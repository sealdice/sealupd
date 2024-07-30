# Sealupd

Updater for [SealDice](https://github.com/sealdice), successor of [seal-updater](https://github.com/sealdice/seal-updater).

> [!WARNING]
> Please use Rust 1.80.0 and above to build this project, as it uses new features such as `std::sync::LazyLock`.

## TODO

- [x] Implement business logic.
- [x] Add log information.
- [x] Add manifest and static build for Windows (via `embed_manifest` and `static_vcruntime`).
- [ ] CI/CD
- [ ] Tests