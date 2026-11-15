# Commands and environment

All research execution in this continuation is native C++ or Rust. Python only
renders recorded graph values and compiles the syntax of a metadata assembly adapter.
Compiler/runtime: GCC 14.2.0, OpenSSL 3.5.7, Rust/Cargo 1.99.1, Lean 4.34.1;
Linux x86-64, uncontrolled host. No benchmark timing or speedup is admitted.

```sh
bash research/quadratic_twist_evidence_20261021/run-native.sh
bash research/quadratic_twist_evidence_20261021/test-native.sh NATIVE_RUN FRESH_CONTROLS
cargo test --release --lib --locked
cargo test --release --lib --locked cryptanalysis::curve_traits::tests::extension_registry_preserves_field_and_subgroup_records -- --exact
cargo test --release --locked --manifest-path research/quadratic_twist_evidence_20261021/native/metadata/Cargo.toml
bash research/quadratic_twist_evidence_20261021/test-metadata.sh NATIVE_RUN FRESH_METADATA
XDG_CACHE_HOME=WRITABLE_CACHE tools/lean/proof-gate.sh --out RECEIPTS research/quadratic_twist_evidence_20261021/lean
```

Rust was installed into task-local `RUSTUP_HOME`/`CARGO_HOME`; Lean into task-local
`ELAN_HOME`; their bin directories were added only to the corresponding command's
PATH. Cargo used `CARGO_BUILD_JOBS=2`. No system home variable was repurposed.
The passing full-library run had networking permission for its localhost tests.
No shared Conductor service or replacement local instance was started.

The first registry generation used `--write-registry`; replay omits that option
and requires all records to be present and exact. Existing registry rows were
preserved. The native source catalog is reproduced byte for byte from fresh
native metadata output. The legacy metadata harvester only assembles those records;
the full legacy Python arithmetic builder was not executed.
