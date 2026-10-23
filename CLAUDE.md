# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Krabby, a minimal x86_64 operating system kernel in Rust, built on the `bootloader` 0.11 crate and run under QEMU. Two workspace crates:

- `kernel/` — the `#![no_std]` / `#![no_main]` kernel. Compiled for `x86_64-unknown-none`. Entry is `kernel_main` via `bootloader_api::entry_point!`.
- `runner/` — a normal host binary. Its `build.rs` wraps the compiled kernel into bootable disk images, and its `main.rs` launches QEMU on the BIOS image.

Long-term direction: run Linux programs and tools on Krabby (Linux-compatible syscalls, ELF loading). Near-term work is deliberately small and incremental; do not build toward that early, but avoid designs that would block it.

## Commands

Requires nightly Rust (pinned in `rust-toolchain.toml`, which also installs the `x86_64-unknown-none` target) and `qemu-system-x86_64` on PATH.

```sh
cargo run -p runner          # build kernel + disk images, boot in QEMU (BIOS)
cargo build                  # build everything without launching QEMU
cargo build -p kernel --target x86_64-unknown-none   # kernel only
```

Always build from the workspace root. `.cargo/config.toml` enables `-Z bindeps`, which the runner needs; building from inside `kernel/` creates a stray `kernel/target` and `kernel/Cargo.lock`.

There are no tests or lint configuration yet. The kernel is `no_std`, so `cargo test` on it would not work without a custom test harness.

## How the build fits together

1. `runner/Cargo.toml` lists `kernel` as a **binary artifact build-dependency** (`artifact = "bin", target = "x86_64-unknown-none"`). This is the unstable `bindeps` feature, which is why nightly and `.cargo/config.toml` are required.
2. Cargo cross-compiles the kernel first and exposes its path to `runner/build.rs` as `CARGO_BIN_FILE_KERNEL_kernel`.
3. `build.rs` uses `bootloader::UefiBoot` and `bootloader::BiosBoot` to write `uefi.img` and `bios.img` into `OUT_DIR`, then exports their paths as the `UEFI_PATH` and `BIOS_PATH` env vars.
4. `runner/src/main.rs` reads those via `env!()` at compile time and passes `bios.img` to QEMU as a raw drive. The UEFI image is produced but not currently used; booting it needs OVMF firmware passed to QEMU.

Kernel changes trigger a rebuild of the runner's build script, so `cargo run -p runner` is always enough to pick them up.
