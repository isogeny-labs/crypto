# Reproduce native controls

Read the two protocols and FROZEN.md. Extract each whole raw archive into NEW
scratch directories and compare every member size/hash in its manifest. The
native aggregate and separate runtime-contract results are tracked at their
real paths. Never overwrite source snapshots, pre-call records, raw directories
or copied immutable executables/qualifiers. A rebuilt binary gets a new identity.

```sh
sha256sum --check research/curve25519_torus_native_20261115/monomial-cancellation-v1/SOURCES.sha256
cargo test --release --lib --manifest-path tools/torus-monomial-cancellation/Cargo.toml
cargo fmt --manifest-path tools/torus-monomial-cancellation/Cargo.toml -- --check
cargo clippy --release --all-targets --manifest-path tools/torus-monomial-cancellation/Cargo.toml -- -D warnings
cargo build --release --manifest-path tools/torus-monomial-cancellation/Cargo.toml
```

Separate helper: tools/torus-monomial-rollback, with its own Cargo.lock,fmt,
strict Clippy/build and ENGINE_ROLLBACK_{SOURCES,BUILD,PRECALL,QUALIFICATION}.
The original shared build cache was /workspace/curve25519-leased-build;
previous copied binaries stay immutable when that cache changes.

Both programs take a NEW output directory, create it themselves and seal native
pre-call data before solver calls. Actual copied shell qualifiers retain their
hashes before calls and send console output to separate .control.log files.
No timing is admitted from Cargo/stdout. Core loops through35fixed systems,
including every ordinary cap/Unsupported; Invalid stops with failure evidence.
Separate helper runs exactly its one public Fp29 contract, complete truth set
and two trace mutations. No roots/labels/signs/scalars/tables enter solve.

Python is restricted to file/hash/archive metadata and plot/layout of already
recorded native values. All cryptographic code, controls, truth/replay/checking
and result aggregation are Rust. Full cold and allocator accounting remains
open; these source/byte checks are not signed execution or mathematical proof.
