# libefi

`libefi` is a safe, idiomatic Rust implementation of the Unified Extensible Firmware Interface,
making it possible to write low-level EFI applications using purely Rust code. It also provides
target specifications that can be used to build EFI images directly with Xargo.

# Overview

UEFI is the modern replacement for PC BIOS systems. It specifies a standard interface that platform
firmware must honor, so that any software written against the specification is portable to any UEFI
platform.

This crate uses Rust's rich FFI support to implement types and functions for interacting with UEFI
firmware. Most of the structures, functions, and types described in the UEFI specification are
implemented purely in Rust, and on top of these are a set of functions for interacting using
idiomatic Rust code. This layered approach is used throughout the library, resulting in an API that
is ergonomic wherever possible but still incredibly flexible when necessary.

# Dependencies

Cargo itself isn't quite up to the task of building EFI images by itself, but it comes very close
and can be configured to do almost all of the work itself. There are just a few extra pieces needed.

### Xargo

`libefi` depends on Rust's `libcore`, which is currently not automatically cross-compiled by Cargo.
Xargo exists to fill this gap, and essentially acts as a drop-in replacement for the `cargo` command
with added support for cross-compiling `libcore`.

Xargo is available via Cargo:

```bash
$ cargo install xargo
```

### LLD

Rust's toolchain currently uses GNU ld to link programs. Unfortunately, ld typically isn't build
with support for the PE32+ file format used by UEFI, so another linker must be used. LLVM's LLD is
known to work for this purpose, and is readily available for most platforms.

[This page](http://releases.llvm.org/download.html) links to pre-built binaries for supported
platforms. Linux users may also have the option of using a package manager to download LLVM. In
particular, the LLVM project provides [APT repositories](https://apt.llvm.org/) suitable for use on
Debian and Ubuntu based systems.

Once installed, ensure that the `lld-link` command is available in your shell of choice. The target
specifications provided in this repo (see next section) are configured to use `lld-link` to link the
final EFI image. To do this, you may need to create a symlink to the real LLD executable, or you can
simply modify the target specification to point to the installed binary.

### Target Specification

A target specification is a configuration file for the Rust toolchain. It's used to customize the
way Rust programs are compiled into native code. This repository contains target specifications for
producing EFI applications.

To use them, they must be available locally, and the `RUST_TARGET_PATH` environment variable should
be set appropriately. For example:

```bash
$ git clone https://github.com/reynoldsbd/libefi /path/to/libefi
$ export RUST_TARGET_PATH=/path/to/libefi/targets
```

# Building

Building functional EFI images is using this crate is relatively easy, because wherever possible
standard Rust tooling is used.

The first step is to create a binary crate using Cargo and add a dependency on `libefi` in
*Cargo.toml*:

```toml
[dependencies]
efi = { git = "https://github.com/reynoldsbd/libefi" }
```

Second, add some crate attributes in *src/main.rs* and provide the expected entry point:

```rust
#![no_std]
#![no_main]


extern crate efi;
use efi::runtime;
use efi::types::Status;


/// EFI image entry point
#[no_mangle]
pub extern fn efi_main() -> Status {

    // your code goes here
    runtime::system_table().con_out.output_string("hello, world!\r\n");

    Status::Success
}
```

You're now ready to build an EFI image. Assuming you've setup dependencies as described above:

```bash
$ xargo build --target x86_64-pc-uefi
```

This will compile an EFI image and place it under the *target/* directory.

# Examples

See the *libefi-test/* directory for an example of an EFI application that uses this crate. It also
contains examples of usage and a Makefile demonstrating how to test using qemu and OVMF.
