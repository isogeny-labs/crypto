# Native torus/S4 quadratic circuit

Reusable Rust compiler and public-input checks for Chebyshev membership plus
Edwards-y S4 decomposition on a Montgomery curve. It stores only monomials
of degree at most two; it does not allocate a dense exponent vector for each
term. Arithmetic uses the repository's num-bigint convention. This standalone
crate avoids pulling the entire crypto library into a coefficient serializer.

```sh
cargo test --locked --release --manifest-path tools/torus-s4/Cargo.toml
cargo run --locked --release --manifest-path tools/torus-s4/Cargo.toml -- \
  compile tools/torus-s4/tests/fixtures/actual_planted.json /tmp/torus-new-output
```

The output directory must be new. It contains separate pre-call and post-call
manifests, canonical sparse JSON and native Singular input. The latter uses
`std` over `(integer,p)`; compatibility of `slimgb` and FGLM with that interface
must not be assumed. A basis computation alone is not a recovered decomposition
or independently checked absence certificate.

`Case` accepts decimal JSON numbers without machine-word truncation. `M` is
positive and fits u128. The caller supplies a prime p; primality is not tested.
The compiler rejects singular Montgomery models and invalid/noncanonical target
points. The target chart requires `u+1 != 0`. The rational membership bridge
is restricted to nonzero denominators: `D(y)=(A+2)-(A-2)y`, with
`D(y)^2*(X+1)=2*A^2*(1-y)^2`. The equations exclude `D=0` for these nonsingular
models: at `D=0`, `1-y != 0`, so their second equality is impossible.
Candidates require finite y != 1, rational curve lifts, Chebyshev membership,
and a checked signed sum. `Curve::verify_triple` performs the membership/lift/
sign checks using separate scalar arithmetic; its square-root helper supports
p congruent to 5 modulo 8. This is variable-time research arithmetic.

The producer tests compare every coefficient and variable/equation order
against three frozen historical serializer fixtures, and check a planted
actual-field witness, group sum, cofactor-cleared target, recurrences and
mutations. They report six passes. [pcc:unverified]

The optional `tests/backend.rs` control requires Singular plus GNU `timeout`
and `prlimit`. Set `TORUS_SINGULAR` to the native binary and
`TORUS_BACKEND_LOG_DIR` to a fresh directory, preserve the backend's library
paths, then run `cargo test --release --manifest-path tools/torus-s4/Cargo.toml
--test backend -- --ignored`. This checks the two M=1 actual-field basis
classifications under a short resource cap. It is a development correctness
control, with no performance assertion. [pcc:unverified]

Historical `.sing` fixtures are parsed as data by Rust tests, never executed.
They preserve the earlier quadratic serializer, not an earlier timing claim.
See the [source-linked report](../../research/curve25519_torus_native_20261115/README.md)
and the consuming cryptanalysis study for the full protocol and route criteria.
