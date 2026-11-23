# H116 quadratic-sandwich fiber source gate

Protocol `H116-QUADRATIC-SANDWICH-FIBERS-001` was committed at
`24bada56a5` before either implementation existed and before any affected
fiber count was computed. This gate seals the only source paths permitted to
generate and replay the frozen three-cell result. [pcc:unverified]

```text
1c03e16848e1aa7d5519f196f6bda068c8f8540383e0ca65ced514c791c2efc9  QUADRATIC_SANDWICH_FIBER_PROTOCOL.json
3b64c005d8ee46fc99958087c2e082c1ffb27062c70f667a06328e17d0d6b611  h116_quadratic_sandwich_fibers.rs
9b07de61a85c7e87a062b7a60e32ea658b7e04682da85a1a36104e4e36c500c7  h116_quadratic_sandwich_fibers_verify.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
```

The producer uses the previously sealed H116 polynomial-basis field and group
module. The verifier does not import either source. It independently uses a
dynamically sized polynomial-basis field, a no-root irreducibility test for the
locked degrees at most three, brute-force `(x,y)` point enumeration, and its own
affine group law. It compares every mathematical result column and parses every
operation-count column. [pcc:unverified]

## Pre-execution checks

Both sources compiled under warnings-as-errors with
`rustc 1.100.0 (0ef1bc386 2026-11-09)`:

```text
rustc --edition=2021 -O -D warnings h116_quadratic_sandwich_fibers.rs \
  -o /tmp/h116_quadratic_sandwich_fibers
rustc --edition=2021 -O -D warnings h116_quadratic_sandwich_fibers_verify.rs \
  -o /tmp/h116_quadratic_sandwich_fibers_verify
/tmp/h116_quadratic_sandwich_fibers --self-test
h116 quadratic sandwich producer self-test ok
/tmp/h116_quadratic_sandwich_fibers_verify --self-test
h116 quadratic sandwich verifier self-test ok
```

The producer self-test uses `E/F_13: y^2=x^3+2x+3`; the verifier self-test
uses `E/F_(17^2): y^2=x^3+3x+5`. Neither is a frozen experiment cell. The first
permitted affected execution is the exact producer above, followed by the exact
verifier against its retained TSV. Every row is retained regardless of its
acceptance value. [pcc:unverified]

This gate establishes source identity only. It predicts no observed count and
does not turn rational-point counts into a function-field, separability, ECDLP,
or complexity theorem. [pcc:unverified]
