//! A program which generates a linux-headers installation and runs bindgen
//! over each public header, for each supported architecture, for a selection
//! of Linux kernel versions.

use bindgen::{builder, EnumVariation};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use std::process::Command;

#[allow(unused_doc_comments)]
const LINUX_VERSIONS: [&str; 3] = [
    /// Rust supports Linux versions back to this.
    /// <https://doc.rust-lang.org/nightly/rustc/platform-support.html>
    "v2.6.32",
    /// This is the oldest kernel version available on Github Actions.
    /// <https://github.com/actions/virtual-environments#available-environments>
    "v5.4",
    /// Linux 5.6 has `openat2` so pick something newer than that.
    "v5.11",
];

/// Rust supports Linux versions back to this this.
const DEFAULT_LINUX_VERSION: &str = "v2.6.32";

/// Some commonly used features.
const DEFAULT_FEATURES: &str = "\"general\",\"errno\"";

fn main() {
    let mut args = env::args();
    let _exe = args.next().unwrap();
    let cmd = args.next();

    // This is the main invocation path.
    assert!(cmd.is_none());
    assert!(args.next().is_none());

    let out = tempdir::TempDir::new("linux-raw-sys").unwrap();
    let out_dir = out.path();
    let linux_headers = out_dir.join("linux-headers");
    let linux_include = linux_headers.join("include");

    // Clean up any modules from previous builds.
    for entry in fs::read_dir("../src").unwrap() {
        let entry = entry.unwrap();
        assert!(!entry.path().to_str().unwrap().ends_with("."));
        if entry.file_type().unwrap().is_dir() {
            fs::remove_dir_all(entry.path()).ok();
        }
    }

    // Edit ../src/lib.rs
    let mut src_lib_rs_in = File::open("../src/lib.rs").unwrap();
    let mut src_lib_rs_contents = String::new();
    src_lib_rs_in.read_to_string(&mut src_lib_rs_contents).unwrap();
    let edit_at = src_lib_rs_contents.find("// The rest of this file is auto-generated!\n").unwrap();
    src_lib_rs_contents = src_lib_rs_contents[..edit_at].to_owned();

    let mut src_lib_rs = File::create("../src/lib.rs").unwrap();
    src_lib_rs.write_all(src_lib_rs_contents.as_bytes()).unwrap();
    src_lib_rs.write_all("// The rest of this file is auto-generated!\n".as_bytes()).unwrap();

    // Edit ../Cargo.toml
    let mut cargo_toml_in = File::open("../Cargo.toml").unwrap();
    let mut cargo_toml_contents = String::new();
    cargo_toml_in.read_to_string(&mut cargo_toml_contents).unwrap();
    let edit_at = cargo_toml_contents.find("# The rest of this file is auto-generated!\n").unwrap();
    cargo_toml_contents = cargo_toml_contents[..edit_at].to_owned();

    // Generate Cargo.toml
    let mut cargo_toml = File::create("../Cargo.toml").unwrap();
    cargo_toml.write_all(cargo_toml_contents.as_bytes()).unwrap();
    cargo_toml.write_all("# The rest of this file is auto-generated!\n".as_bytes()).unwrap();
    writeln!(cargo_toml, "[features]").unwrap();

    let mut features: HashSet<String> = HashSet::new();

    for linux_version in &LINUX_VERSIONS {
        let linux_version_mod = linux_version.replace('.', "_");

        // Collect all unique feature names across all architectures.
        if features.insert(linux_version_mod.clone()) {
            writeln!(cargo_toml, "{} = []", linux_version_mod).unwrap();
        }

        let cfg_version = format!("#[cfg(feature = \"{}\")]", linux_version_mod);
        if linux_version != &DEFAULT_LINUX_VERSION {
            writeln!(src_lib_rs, "{}", cfg_version).unwrap();
        }
        writeln!(src_lib_rs, "pub mod {};", linux_version_mod).unwrap();
        if linux_version == &DEFAULT_LINUX_VERSION {
            writeln!(src_lib_rs, "pub use {}::*;", linux_version_mod).unwrap();
        }

        let src_vers = format!("../src/{}", linux_version_mod);
        fs::create_dir_all(&src_vers).unwrap();
        let mut src_vers_mod_rs = File::create(&format!("{}/mod.rs", src_vers)).unwrap();

        // Checkout a specific version of Linux.
        git_checkout(linux_version);

        for linux_arch_entry in fs::read_dir(&format!("linux/arch")).unwrap() {
            let linux_arch_entry = linux_arch_entry.unwrap();
            if !linux_arch_entry.file_type().unwrap().is_dir() {
                continue;
            }
            let linux_arch = linux_arch_entry.file_name().to_str().unwrap().to_owned();

            let rust_arches = rust_arches(&linux_arch);
            if rust_arches.is_empty() {
                continue;
            }

            fs::create_dir_all(&linux_headers).unwrap();

            make_headers_install(&linux_arch, &linux_headers);

            for rust_arch in rust_arches {
                eprintln!(
                    "Generating all bindings for Linux {} architecture {}",
                    linux_version, rust_arch
                );

                let src_arch = format!("{}/{}", src_vers, rust_arch);
                fs::create_dir_all(&src_arch).unwrap();
                let mut src_arch_mod_rs = File::create(&format!("{}/mod.rs", src_arch)).unwrap();

                let cfg_arch = format!("#[cfg(target_arch = \"{}\")]", rust_arch);
                writeln!(src_vers_mod_rs, "{}", cfg_arch).unwrap();
                writeln!(src_vers_mod_rs, "mod {};", rust_arch).unwrap();
                writeln!(src_vers_mod_rs, "{}", cfg_arch).unwrap();
                writeln!(src_vers_mod_rs, "pub use {}::*;", rust_arch).unwrap();

                for mod_entry in fs::read_dir("modules").unwrap() {
                    let mod_entry = mod_entry.unwrap();
                    let header_name = mod_entry.path();
                    let mod_name = header_name.file_stem().unwrap().to_str().unwrap();
                    let mod_rs = format!("{}/{}.rs", src_arch, mod_name);

                    run_bindgen(
                        linux_include.to_str().unwrap(),
                        header_name.to_str().unwrap(),
                        &mod_rs,
                        mod_name,
                        rust_arch,
                        linux_version,
                    );

                    writeln!(src_arch_mod_rs, "/// {}", header_name.to_str().unwrap()).unwrap();
                    writeln!(src_arch_mod_rs, "#[cfg(feature = \"{}\")]", mod_name).unwrap();
                    writeln!(src_arch_mod_rs, "pub mod r#{};", mod_name).unwrap();
                    // Collect all unique feature names across all architectures.
                    if features.insert(mod_name.to_owned()) {
                        writeln!(cargo_toml, "{} = []", mod_name).unwrap();
                    }
                }
            }

            fs::remove_dir_all(&linux_headers).unwrap();
        }
    }

    writeln!(cargo_toml, "default = [{}]", DEFAULT_FEATURES).unwrap();

    // Reset the `linux` directory back to the default branch.
    git_checkout(DEFAULT_LINUX_VERSION);

    eprintln!("All bindings generated!");
}

fn git_checkout(rev: &str) {
    // Check out the given revision.
    assert!(Command::new("git")
        .arg("checkout")
        .arg(rev)
        .current_dir("linux")
        .status()
        .unwrap()
        .success());

    // Delete any untracked generated files from other versions.
    assert!(Command::new("git")
        .arg("clean")
        .arg("-f")
        .arg("-d")
        .current_dir("linux")
        .status()
        .unwrap()
        .success());

    // Restore any deleted files.
    assert!(Command::new("git")
        .arg("restore")
        .arg(".")
        .current_dir("linux")
        .status()
        .unwrap()
        .success());
}

fn make_headers_install(linux_arch: &str, linux_headers: &Path) {
    assert!(Command::new("make")
        .arg(format!("headers_install"))
        .arg(format!("ARCH={}", linux_arch))
        .arg(format!(
            "INSTALL_HDR_PATH={}",
            fs::canonicalize(&linux_headers).unwrap().to_str().unwrap()
        ))
        .current_dir("linux")
        .status()
        .unwrap()
        .success());
}

fn rust_arches(linux_arch: &str) -> &[&str] {
    match linux_arch {
        "arm" => &["arm"],
        "arm64" => &["aarch64"],
        "avr32" => &["avr"],
        "hexagon" => &["hexagon"],
        "mips" => &["mips", "mips64"],
        "powerpc" => &["powerpc", "powerpc64"],
        "riscv" => &["riscv32", "riscv64"],
        "s390" => &["s390x"],
        "sparc" => &["sparc", "sparc64"],
        "x86" => &["x86", "x86_64"],
        "alpha" | "cris" | "h8300" | "m68k" | "microblaze" | "mn10300" | "score" | "blackfin"
        | "frv" | "ia64" | "m32r" | "m68knommu" | "parisc" | "sh" | "um" | "xtensa"
        | "unicore32" | "c6x" | "nios2" | "openrisc" | "csky" | "arc" | "nds32" => &[],
        _ => panic!("unrecognized arch: {}", linux_arch),
    }
}

fn run_bindgen(
    linux_include: &str,
    header_name: &str,
    mod_rs: &str,
    mod_name: &str,
    rust_arch: &str,
    linux_version: &str,
) {
    let clang_arch = compute_clang_arch(rust_arch);

    eprintln!(
        "Generating bindings for {} on Linux {} architecture {}",
        mod_name, linux_version, rust_arch
    );

    let builder = builder()
        // The generated bindings are quite large, so use a few simple options
        // to keep the file sizes down.
        .rustfmt_configuration_file(Some(Path::new("bindgen-rustfmt.toml").to_owned()))
        .layout_tests(false)
        .generate_comments(false)

        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .array_pointers_in_arguments(true)

        .derive_debug(true)
        .clang_arg(&format!("--target={}-unknown-linux", clang_arch))
        .clang_arg("-nostdinc")
        .clang_arg("-I")
        .clang_arg("include")
        .clang_arg("-I")
        .clang_arg(linux_include)
        .blocklist_item("NULL");

    let bindings = builder
        .header(header_name)
        .generate()
        .expect(&format!("generate bindings for {}", mod_name));
    bindings
        .write_to_file(mod_rs)
        .expect(&format!("write_to_file for {}", mod_name));
}

fn compute_clang_arch(rust_arch: &str) -> &str {
    if rust_arch == "x86" {
        "i686"
    } else {
        rust_arch
    }
}
