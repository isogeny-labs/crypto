# torus-section-operator

Exact six-candidate section evaluation on the Montgomery cubic, with local
target-pole charts and confluent jets for repeated geometric points. Rust
research code, variable time. It receives all six already oriented points;
it does not discover any factor-base member or solve a target decomposition.

`Operator::analyze(terms,target)` returns the6x6 evaluation matrix, rank and
an optional normalized null-section vector. `verify_section` recomputes every
jet condition. Distinct raw column IDs are required, while distinct IDs may
have equal geometric points. Membership, sign/index recovery and cofactor
provenance must still come from the original pipeline.

Native v1 supports only Fp101 and Fp(2^255-19), nonsingular canonical A,
six finite members with nonzero ordinate, and target identity or finite
nonzero ordinate. Unsupported identity members/two-torsion are errors, not
negative relation decisions. The general classical divisor argument does
not expand this native domain.

```
cargo test --release --lib --manifest-path tools/torus-section-operator/Cargo.toml
cargo clippy --release --all-targets --manifest-path tools/torus-section-operator/Cargo.toml -- -D warnings
cargo build --release --manifest-path tools/torus-section-operator/Cargo.toml
```

The retained separately copied executable supports `--qualify` and
`--controls NEW_OUTPUT`; the latter refuses an existing output file. Use the
frozen study's SOURCE_QUALIFICATION and PRECALL/POSTCALL records to reproduce
the declared fixed controls, not an edited working checkout under old hashes.

Source/derivation/control contract:
`research/curve25519_torus_native_20261115/section-operator-v1/`.
`naive_rank` deliberately retains two incorrect predicates for counterexamples;
it is not an alternative sum checker. Classical elliptic-code theory is prior
art. No low-dimensional matrix or certificate implies a subcubic singular
column search; all-triple Pluecker construction is still cubic.
