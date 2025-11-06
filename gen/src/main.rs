//! A program which generates a linux-headers installation and runs bindgen
//! over the headers, for each supported architecture.

use bindgen::{builder, EnumVariation};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

#[allow(unused_doc_comments)]
const LINUX_VERSION: &str = "v6.17";

/// Some commonly used features.
const DEFAULT_FEATURES: &str = "\"general\", \"errno\"";

fn main() {
    let mut args = env::args();
    let _exe = args.next().unwrap();
    let cmd = args.next();

    // This is the main invocation path.
    assert!(cmd.is_none());
    assert!(args.next().is_none());

    git_init();

    let out = tempfile::TempDir::with_prefix("linux-raw-sys").unwrap();
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
    src_lib_rs_in
        .read_to_string(&mut src_lib_rs_contents)
        .unwrap();
    let edit_at = src_lib_rs_contents
        .find("// The rest of this file is auto-generated!\n")
        .unwrap();
    src_lib_rs_contents = src_lib_rs_contents[..edit_at].to_owned();

    let mut src_lib_rs = File::create("../src/lib.rs").unwrap();
    src_lib_rs
        .write_all(src_lib_rs_contents.as_bytes())
        .unwrap();
    src_lib_rs
        .write_all("// The rest of this file is auto-generated!\n".as_bytes())
        .unwrap();

    // Edit ../Cargo.toml
    let mut cargo_toml_in = File::open("../Cargo.toml").unwrap();
    let mut cargo_toml_contents = String::new();
    cargo_toml_in
        .read_to_string(&mut cargo_toml_contents)
        .unwrap();
    let edit_at = cargo_toml_contents
        .find("# The rest of this file is auto-generated!\n")
        .unwrap();
    cargo_toml_contents = cargo_toml_contents[..edit_at].to_owned();

    // Generate Cargo.toml
    let mut cargo_toml = File::create("../Cargo.toml").unwrap();
    cargo_toml
        .write_all(cargo_toml_contents.as_bytes())
        .unwrap();
    cargo_toml
        .write_all("# The rest of this file is auto-generated!\n".as_bytes())
        .unwrap();
    writeln!(cargo_toml, "[features]").unwrap();

    let mut features: HashSet<String> = HashSet::new();

    let linux_version = LINUX_VERSION;
    // Checkout a specific version of Linux.
    git_checkout(linux_version);

    let mut linux_archs = fs::read_dir(&format!("linux/arch"))
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();
    // Sort archs list as filesystem iteration order is non-deterministic
    linux_archs.sort_by_key(|entry| entry.file_name());
    for linux_arch_entry in linux_archs {
        if !linux_arch_entry.file_type().unwrap().is_dir() {
            continue;
        }
        let linux_arch = linux_arch_entry.file_name().to_str().unwrap().to_owned();

        let rust_arches = rust_arches(&linux_arch);
        if rust_arches.is_empty() {
            continue;
        }

        fs::create_dir_all(&linux_headers).unwrap();

        let mut headers_made = false;
        for rust_arch in rust_arches {
            if !headers_made {
                make_headers_install(&linux_arch, &linux_headers);
                headers_made = true;
            }

            eprintln!(
                "Generating all bindings for Linux {} architecture {}",
                linux_version, rust_arch
            );

            let src_arch = format!("../src/{}", rust_arch);

            fs::create_dir_all(&src_arch).unwrap();

            let mut modules = fs::read_dir("modules")
                .unwrap()
                .map(|entry| entry.unwrap())
                .collect::<Vec<_>>();
            // Sort module list as filesystem iteration order is non-deterministic
            modules.sort_by_key(|entry| entry.file_name());
            for mod_entry in modules {
                let header_name = mod_entry.path();
                let mod_name = header_name.file_stem().unwrap().to_str().unwrap();
                let mod_rs = format!("{}/{}.rs", src_arch, mod_name);

                writeln!(src_lib_rs, "#[cfg(feature = \"{}\")]", mod_name).unwrap();
                if *rust_arch == "x32" {
                    writeln!(
                        src_lib_rs,
                        "#[cfg(all(target_arch = \"x86_64\", target_pointer_width = \"32\"))]"
                    )
                    .unwrap();
                } else if *rust_arch == "x86_64" {
                    writeln!(
                        src_lib_rs,
                        "#[cfg(all(target_arch = \"x86_64\", target_pointer_width = \"64\"))]"
                    )
                    .unwrap();
                } else {
                    writeln!(src_lib_rs, "#[cfg(target_arch = \"{}\")]", rust_arch).unwrap();
                }
                writeln!(src_lib_rs, "#[path = \"{}/{}.rs\"]", rust_arch, mod_name).unwrap();
                writeln!(src_lib_rs, "pub mod {};", mod_name).unwrap();

                run_bindgen(
                    linux_include.to_str().unwrap(),
                    header_name.to_str().unwrap(),
                    &mod_rs,
                    mod_name,
                    rust_arch,
                    linux_version,
                );

                // Collect all unique feature names across all architectures.
                if features.insert(mod_name.to_owned()) {
                    writeln!(cargo_toml, "{} = []", mod_name).unwrap();
                }
            }
        }

        fs::remove_dir_all(&linux_headers).unwrap();
    }

    writeln!(cargo_toml, "default = [\"std\", {}]", DEFAULT_FEATURES).unwrap();
    writeln!(cargo_toml, "std = []").unwrap();
    writeln!(cargo_toml, "no_std = []").unwrap();
    writeln!(cargo_toml, "elf = []").unwrap();
    writeln!(cargo_toml, "rustc-dep-of-std = [\"core\", \"no_std\"]").unwrap();

    eprintln!("All bindings generated!");
}

fn git_init() {
    // Clone the linux kernel source repo if necessary. Ignore exit code as it will
    // be non-zero in case it was already cloned.
    //
    // Use a treeless partial clone to save disk space and clone time.
    // See <https://github.blog/2020-12-21-get-up-to-speed-with-partial-clone-and-shallow-clone/>
    // for more info on partial clones.
    //
    // Note: this is not using the official repo
    // <git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git> but
    // the github fork as the server of the official repo doesn't recognize
    // filtering.
    if !Path::new("linux/.git").exists() {
        assert!(Command::new("git")
            .arg("clone")
            .arg("https://github.com/torvalds/linux.git")
            .arg("--filter=tree:0")
            .arg("--no-checkout")
            .status()
            .unwrap()
            .success());
    }

    // Setup sparse checkout. This greatly reduces the amount of objects necessary
    // to checkout the tree.
    assert!(Command::new("git")
        .arg("sparse-checkout")
        .arg("init")
        .current_dir("linux")
        .status()
        .unwrap()
        .success());

    fs::write(
        "linux/.git/info/sparse-checkout",
        "/*
!/*/
/include/
/arch/
/scripts/
/tools/",
    )
    .unwrap();
}

fn git_checkout(rev: &str) {
    // Delete any generated files from previous versions.
    assert!(Command::new("git")
        .arg("clean")
        .arg("-f")
        .arg("-d")
        .current_dir("linux")
        .status()
        .unwrap()
        .success());

    // Check out the given revision.
    assert!(Command::new("git")
        .arg("checkout")
        .arg(rev)
        .arg("-f")
        .current_dir("linux")
        .status()
        .unwrap()
        .success());

    // Delete any untracked generated files from previous versions.
    assert!(Command::new("git")
        .arg("clean")
        .arg("-f")
        .arg("-d")
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

    // HACK: Header missing from kernel installation - likely an upstream bug that needs to be reported
    if linux_arch == "arm64" {
        fs::copy(
            "linux/arch/arm64/include/asm/image.h",
            linux_headers.join("include/asm/image.h"),
        )
        .expect("Missing headers");
    }
}

fn rust_arches(linux_arch: &str) -> &[&str] {
    match linux_arch {
        "arm" => &["arm"],
        "arm64" => &["aarch64"],
        "avr32" => &["avr"],
        "csky" => &["csky"],
        "hexagon" => &["hexagon"],
        "loongarch" => &["loongarch64"],
        "m68k" => &["m68k"],
        "mips" => &["mips", "mips64", "mips32r6", "mips64r6"],
        "powerpc" => &["powerpc", "powerpc64"],
        "riscv" => &["riscv32", "riscv64"],
        "s390" => &["s390x"],
        "sparc" => &["sparc", "sparc64"],
        "x86" => &["x86", "x86_64", "x32"],
        "alpha" | "cris" | "h8300" | "microblaze" | "mn10300" | "score" | "blackfin" | "frv"
        | "ia64" | "m32r" | "m68knommu" | "parisc" | "sh" | "um" | "xtensa" | "unicore32"
        | "c6x" | "nios2" | "openrisc" | "arc" | "nds32" | "metag" | "tile" => &[],
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
    let clang_target = compute_clang_target(rust_arch);

    eprintln!(
        "Generating bindings for {} on Linux {} architecture {}",
        mod_name, linux_version, rust_arch
    );

    let mut builder = builder()
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
        .sort_semantically(true)
        .blocklist_item("^__UAPI_DEF_.*")
        .blocklist_item("BITS_PER_LONG")
        .blocklist_item("__BITS_PER_LONG")
        .clang_arg(&format!("--target={}", clang_target))
        .clang_arg("-DBITS_PER_LONG=(__SIZEOF_LONG__*__CHAR_BIT__)")
        .clang_arg("-D__WANT_POSIX1B_SIGNALS__")
        .clang_arg("-nostdinc")
        .clang_arg("-I")
        .clang_arg(linux_include)
        .clang_arg("-I")
        .clang_arg("include")
        .blocklist_item("NULL");

    // Avoid duplicating things across multiple modules.
    if mod_name != "ioctl" {
        for ioctl in BufReader::new(File::open("ioctl/generated.txt").unwrap()).lines() {
            builder = builder.blocklist_item(ioctl.unwrap());
        }
    }
    if mod_name != "general" {
        builder = builder.blocklist_item("^LINUX_VERSION_.*");
        builder = builder.blocklist_item("__kernel_fd_set");
        builder = builder.blocklist_item("fds_bits");
        builder = builder.blocklist_item("__FD_SETSIZE");
        builder = builder.blocklist_item("__kernel_sighandler_t");
        builder = builder.blocklist_item("^F_.*");
        builder = builder.blocklist_item("^O_.*");
        builder = builder.blocklist_item("__kernel_fsid_t");
        builder = builder.blocklist_item("^HUGETLB_FLAG_ENCODE_.*");
        builder = builder.blocklist_item("^RESOLVE_.*");
    }
    if rust_arch == "m68k" {
        // Don't emit the `size_t` workaround types for m68k. This should be
        // removed when there's a bindgen release with a fix for [1].
        //
        // [1]: https://github.com/rust-lang/rust-bindgen/issues/3312
        builder = builder.blocklist_type("^s?size_t$");
    }

    let bindings = builder
        .use_core()
        .ctypes_prefix("crate::ctypes")
        .header(header_name)
        .generate()
        .expect(&format!("generate bindings for {}", mod_name));
    bindings
        .write_to_file(mod_rs)
        .expect(&format!("write_to_file for {}", mod_name));
}

fn compute_clang_target(rust_arch: &str) -> String {
    if rust_arch == "x86" {
        format!("i686-unknown-linux")
    } else if rust_arch == "x32" {
        format!("x86_64-unknown-linux-gnux32")
    } else if rust_arch == "mips32r6" {
        format!("mipsisa32r6-unknown-linux-gnu")
    } else if rust_arch == "mips64r6" {
        format!("mipsisa64r6-unknown-linux-gnuabi64")
    } else {
        format!("{}-unknown-linux", rust_arch)
    }
}
