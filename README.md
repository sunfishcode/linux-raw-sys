<div align="center">
  <h1><code>linux-raw-sys</code></h1>

  <p>
    <strong>Generated bindings for Linux's userspace API</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/linux-raw-sys/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/linux-raw-sys/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/linux-raw-sys"><img src="https://img.shields.io/crates/v/linux-raw-sys.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/linux-raw-sys"><img src="https://docs.rs/linux-raw-sys/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate contains bindgen-generated bindings for Linux's userspace API.

This is primarily of interest if you want to make raw system calls directly,
which is tedious and error prone and not necessary for most use cases. For a
minimal type-safe, memory-safe, and I/O-safe API to the Linux system calls
built on these bindings, see the [rustix crate].

The full bindings are quite large, so they've been split up into modules and
cargo features. By default, `general` and `errno` are enabled, which provide
most things needed by general-purpose code.

The default bindings are generated from Linux 2.3.36, as it is the
[oldest version supported by Rust]. Modules `v5_4` and `v5_11`, enabled by
features `v5_4` and `v5_11` respectively, contain generated bindings for
Linux 5.4 and Linux 5.11, which provide access to numerous new features, though
they may not be available on all supported Linux versions.

To regenerate the generated bindings, run `cargo update && cd gen && cargo run --release`.

## Similar crates

This is similar to [linux-sys], but supports multiple Linux versions at once,
including the oldest version of Linux that Rust supports, and also new
versions, making it easy to selectively mix in new features. Also, the bindings
are generated offline, rather than in a build.rs, making downstream builds
simpler. And, this crate has bindings for more headers, as well as
supplementary definitions not exported by Linux's headers but nonetheless
needed by userspace.

[linux-sys]: https://crates.io/crates/linux-sys

[oldest version supported by Rust]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
[rustix crate]: https://github.com/bytecodealliance/rustix#linux-raw-syscall-support
