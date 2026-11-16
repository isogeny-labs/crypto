# Quadratic summation trees and four-coset torus bases

Standalone native Rust library and development compiler for 2–8 point
decompositions on a Montgomery curve. It reuses `../torus-s4` arithmetic,
polynomials and curve points. Four-point inputs use a chain of three S3
relations in Edwards-y coordinates, with Chebyshev membership circuits.

```sh
cargo test --release --manifest-path tools/torus-s4-tree/Cargo.toml
cargo run --release --manifest-path tools/torus-s4-tree/Cargo.toml -- actual-four /tmp/new-four-point-control
```

The output directory must not exist. `actual-four` constructs a known witness
over p=2^255-19, A=486662, four cosets of M=60824497*213156431, and writes
the public input, quadratic system, native Singular script, arithmetic
records, identity certificates, known-point check and eight raw cofactor
sections. It invokes no solver. The current parameter-record version also
serializes the norm-one multiplier `shift`. Frozen version-one records used
the default multiplier one. [pcc:unverified]

`TorusParameters::shifted` supports admissible randomized bases;
`TreeInput` contains only curve, target, arity and membership values;
`TreeSystem::compile` consumes that public card. `witness` is a separate
development check using known points. Intermediate identity sums are allowed;
the affine leaf and target charts retain their declared denominator guards.
The equations are a candidate-generation representation. Rational point
lifts and actual signed sums must be checked after solving.

The imported native identity-checker and SHA modules remain byte-identical
to their repository owners. Their unit tests are inherited regressions,
not additional four-point mathematical experiments.

Construction counts and finite arithmetic do not give a solver-cost bound.
See the [study](../../research/curve25519_torus_native_20261115/README.md),
[open proof/cost obligations](../../problems/curve25519-torus-four-point-decomposition.md)
and the consuming `isogeny-labs/cryptanalysis` root-recovery harness.
