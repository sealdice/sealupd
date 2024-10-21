# Sealupd

Updater for [SealDice], successor of [seal-updater].

# Usage

The program accepts to principal arguments: `--package` (short `-p`, alias `--upgrade`) and `--pid`. It waits for the process with the specified PID to terminate before extracting files from the provided package into the current directory. Finally, it tries to start the executable named `sealdice-core` or `sealdice-core.exe` unless the `--skip-launch` flag is set.

For a full definition of acceptable arguments and flags, see `src/cli.rs`.

# Development

> [!WARNING]
> Please use Rust 1.80.0 and above to build this project, as it leverages new features such as `std::sync::LazyLock`.

## Style Guides

1. Use `cargo fmt` with configurations speficied in `.rustfmt.toml` to format the project. Do not run `rustfmt` directly as it formats on a per-file basis and may remove disambiguate imports.

2. Import types directly; prefer `File` instead of `std::fs::File` or `fs::File`, unless ambiguity forbids (`Error` and `io::Error`).

3. Import functions by their immediate parent module; prefer `io::stdin()` instead of `std::io::stdin()` or `stdin()`.

## TODO

- [x] Implement business logic.
- [x] Add log information.
- [x] Add manifest and static build for Windows (via `embed_manifest` and `static_vcruntime`).
- [ ] CI/CD & auto-release
- [ ] Tests
- [ ] Make [SealDice] migrate from the old [seal-updater].

## Other Notes

- We might need a graceful way to log and print information rather than littering `debug!()` and `println!()` everywhere.

- The current implementation of `proc::wait()` is the best we can do, but feedback we got from [seal-updater] proves it is not always reliable.

[SealDice]: https://github.com/sealdice
[seal-updater]: https://github.com/sealdice/seal-updater
