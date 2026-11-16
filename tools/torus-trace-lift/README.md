# Guarded trace lift

`TraceMap` takes public q/omega, evaluates a Chebyshev trace equation and
reconstructs the unique original imaginary coordinate on the d!=0 chart.
It accepts no original b. It is an adapter, not a relation-root solver.
`None` rejects one scalar candidate; it is not an absence result for a
target, base or route. Prime-field assumptions are external.

Fixed development checks:

```sh
cargo test --release --locked --manifest-path tools/torus-trace-lift/Cargo.toml
cargo fmt --check --manifest-path tools/torus-trace-lift/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-trace-lift/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-trace-lift/Cargo.toml
tools/torus-trace-lift/target/release/torus-trace-lift NEW-OUTPUT-DIRECTORY
```

See the pre-code protocol, derivation, source qualification and report under
`research/curve25519_torus_native_20261115/trace-lift-v1` and
`reports/trace-lift-v1`. Frozen native evidence: PCC-20261116-35f196,
unsigned/unverified. The original compiler/reference source is unchanged.
