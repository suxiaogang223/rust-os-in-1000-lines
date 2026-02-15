# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` is the boot entry, and `src/lib.rs` wires all modules. Core kernel logic lives in `src/kernel/` (exceptions, memory, paging, process, syscalls, user mode). Device code is in `src/drivers/` (`uart.rs`, `disk.rs`). Architecture-specific code is in `src/arch/` (RISC-V), shared helpers are in `src/common/`, filesystem code is in `src/fs/`, and user program examples are in `src/user/`.

Top-level build/runtime assets include `Makefile`, `linker.ld`, `.cargo/config.toml`, and `test.sh`. Tutorial content is in `docs/` and built with mdBook to `docs/book/`.

## Build, Test, and Development Commands
- `make install-toolchain`: installs the RISC-V target and required Rust components.
- `make build`: builds release kernel and creates `kernel.stripped`.
- `make run`: boots the kernel in QEMU (`Ctrl+A`, then `X` to exit).
- `make debug`: runs QEMU paused with GDB stub on `:1234`.
- `./test.sh`: full local validation (toolchain checks, `cargo check`, `cargo fmt -- --check`, release build, binary image check).
- `make fmt`, `make clippy`, `make check`: fast formatting/lint/type-check loop.
- `cd docs && mdbook serve`: preview docs locally; `mdbook build` creates static output.

## Sandbox & Elevation
- In restricted sandbox environments (such as Codex CLI), `make build` usually works without elevation.
- `make debug` requires elevated permissions because QEMU `-s` must bind the GDB stub port (`:1234`).
- `make run` may require elevation in restricted environments if QEMU/device access is sandbox-blocked.

## Coding Style & Naming Conventions
Use Rust 2021 and keep code `no_std` friendly. Format with `cargo fmt` (4-space indentation, no tabs). Use `snake_case` for files/modules/functions, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants (especially MMIO addresses and bit masks). Keep `unsafe` blocks minimal and close to hardware access; add short invariants/comments when safety is non-obvious.

## Testing Guidelines
This repo mainly relies on build and boot smoke checks. Run `./test.sh` before opening a PR. For behavior changes, also run `make run` and confirm expected UART/QEMU output. If adding tests, prefer module-local `#[cfg(test)]` tests that do not depend on host-specific tooling.

## Commit & Pull Request Guidelines
Current history uses short imperative subjects (for example, `fix deploy`, `fix theme`). Follow that pattern and keep commits focused. PRs should include: what changed, why it changed, related issue/link (if any), commands executed, and terminal output or screenshots when behavior/docs output changes.
