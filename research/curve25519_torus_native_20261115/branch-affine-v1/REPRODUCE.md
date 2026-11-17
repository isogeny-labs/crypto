# Reproduce fixed development controls

Read PROTOCOL.md/FROZEN.md/POSTCALL_PROVENANCE.md before calls. Core and audit
qualifiers refuse an existing output directory. Never overwrite native-v1 or
the frozen archive/qualifier. Hydrate with maintained artifact tools only once
a genuine receipt exists; PUBLISHING.md distinguishes tracked data from store
publication. The actual source archive and binary hashes are authoritative.

```sh
sha256sum --check research/curve25519_torus_native_20261115/branch-affine-v1/SOURCES.sha256
cargo test --release --lib --manifest-path tools/torus-branch-affine/Cargo.toml
cargo fmt --manifest-path tools/torus-branch-affine/Cargo.toml -- --check
cargo clippy --release --all-targets --manifest-path tools/torus-branch-affine/Cargo.toml -- -D warnings
cargo build --release --manifest-path tools/torus-branch-affine/Cargo.toml
```

Original development reused `tools/torus-affine-closure/target` as Cargo's build
cache; separately retained old executables remain immutable. A freshly rebuilt
binary is not automatically the old qualified executable; record its identity.
The core executable takes a NEW output directory and internally runs only the
declared fixed controls, sealing pre-call bytes before queries. Query interface
is System/semantic roles/limits/mode, never fixture roots or labels.

The native post-call reader accepts RAW_DIR and NEW_AUDIT_OUTPUT_DIR. It pins
controls.json/member-manifest digests, reads retained per-case files and
aggregates in Rust. Rebuild using AUDIT_BUILD_MANIFEST.json's actual rlib paths
and `-L dependency=` for their directories; this Cargo cache uses
`release/build/<crate>/<id>/out`, not a release/deps directory. The initial
failed library lookup/build log remains. Rust source did not change.

Core archive contains complete requests, old/disabled/enabled reports,
conditional certificates, assignment/drop/undo trace, replay, truth/refusal
controls and pre-call data. Separate audit archive contains native totals,
pre-call and altered-input refusal bytes. Extract into scratch and compare
member sizes/hashes; never edit or relabel historical diagnostics. All standing
remains producer unsigned/unverified.
