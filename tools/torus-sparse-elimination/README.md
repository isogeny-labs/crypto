# Bounded sparse elimination baseline

`solve(&System, Limits)` receives only public equations and limits, retaining
the exact BigUint modulus and dynamic sparse monomials. It combines literal
linear/quadratic propagation with an ordinary sparse Buchberger fallback.
It is not matrix F4. Higher-degree extraction is explicitly unsupported.

ROOT contains a full assignment accepted by the unchanged System checker.
CAP, UNSUPPORTED and NoRootObservation are not absence certificates. Prime
field validity is external. Counters/represented-storage reservations are
partial modeling data, not elapsed time, peak RSS or complete cold cost.

```sh
cargo test --release --locked --manifest-path tools/torus-sparse-elimination/Cargo.toml
cargo fmt --check --manifest-path tools/torus-sparse-elimination/Cargo.toml
cargo clippy --release --locked --all-targets --manifest-path tools/torus-sparse-elimination/Cargo.toml -- -D warnings
cargo build --release --locked --manifest-path tools/torus-sparse-elimination/Cargo.toml
tools/torus-sparse-elimination/target/release/torus-sparse-elimination NEW-OUTPUT-DIRECTORY
```

Fixed native qualification and its limits are in
`research/curve25519_torus_native_20261115/direct-elimination-v1/`.
PCC-20261116-9dda07 is unsigned/unverified producer evidence. The actual
root control has71 variables over a TWO-column q1/k2 base; it does not solve
the huge original factor base. Its root used propagation, not Buchberger.
The1100-variable test has only one constrained variable. The1059-variable
input receives a deliberately forced work1 refusal, not a full solve attempt.
