# Reproduction and exact boundary

Use the source95 inventory and THREE archives in SOURCE_QUALIFICATION.json;
restore sparse-missing inputs exactly before interpreting their absence.
The point crate uses native Rust and the unchanged original constructors.
Its public solve API receives Spec/R/Q/limits, never an arbitrary System or
known labels/signs/coefficients/witness. Model acceptance rechecks canonical
values, all original sparse rows, indexed sign/raw-column decoding and group
replay. No private Poly modulus accessor was added.

Development commands, before the frozen call:

```sh
cargo test --release --manifest-path tools/torus-point-solver/Cargo.toml
cargo fmt --manifest-path tools/torus-point-solver/Cargo.toml --check
cargo clippy --release --all-targets --manifest-path tools/torus-point-solver/Cargo.toml -- -D warnings
```

Use an explicitly recorded target directory and CARGO_INCREMENTAL=0. Final
qualified logs are in native-v1. Do not rerun scientific controls just to
refresh metadata. The original root library has unrelated baseline failures;
these focused local checks are not its gate or a hosted pass.

The immutable qualifier and separate BUILD/PRECALL/POSTCALL records show the
actual retained source/binary/transport identities. The fixed final native
shell call exited0; the public curve query inside it returned refusal. Limits:
5000ms child plus1000ms watchdog grace,2GiB address space,2MiB query,1MiB stdout,
64KiB stderr. Full captured streams roundtrip through the54020B native archive;
the deliberate output-overflow case is explicitly truncated. This is not an
uncapped server-byte receipt, timing trial or independently signed execution.

Read native-v1/RAW_MANIFEST.json and RESULTS.json instead of treating the
transport's final exit0 as a successful curve solve. No query retry is part
of reproduction of this recorded observation. A new campaign needs a separate
protocol and the authorized EKS route. [pcc:PCC-20261123-2033f1]
