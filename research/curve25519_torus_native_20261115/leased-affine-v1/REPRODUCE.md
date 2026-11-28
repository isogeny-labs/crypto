# Reproduce

Read PROTOCOL.md/FROZEN.md first. Extract native-v1.tar.xz into a NEW scratch
directory and match all member hashes in NATIVE_ARCHIVE.json. controls.json is
also tracked byte-identically for direct inspection of the native aggregate.
Never overwrite raw output, pre-call manifests, the immutable executable or
the executed qualifier. A rebuilt executable needs its own identity record.

```sh
sha256sum --check research/curve25519_torus_native_20261115/leased-affine-v1/SOURCES.sha256
cargo test --release --lib --manifest-path tools/torus-leased-affine/Cargo.toml
cargo fmt --manifest-path tools/torus-leased-affine/Cargo.toml -- --check
cargo clippy --release --all-targets --manifest-path tools/torus-leased-affine/Cargo.toml -- -D warnings
cargo build --release --manifest-path tools/torus-leased-affine/Cargo.toml
```

The original build cache was /workspace/curve25519-leased-build. The binary
takes a NEW output directory and seals its native pre-call before the fixed
controls. The actual copied qualifier was qualifier-executed-v1.sh, with stdout
kept in development/qualification-v1.log and native console output in the
separate native-v1.control.log. No timing is admitted from Cargo/console logs.
Same public System/roles/mode/limits query only; replay tables and target labels
are outside the query. All33 declared systems run even when an arm reports an
ordinary cap/Unsupported. Invalid controls stop with retained failure evidence.

This is local development qualification, not a benchmark/campaign or full-base
solve. Original-root/native indexed-row replay and coefficient/lifetime checks
remain required. Full cold work/live allocation and independent signed standing
are open. Do not carry old hashes forward after source edits.
