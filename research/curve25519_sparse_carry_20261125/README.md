# Curve25519 sparse carry search

Status: native evaluator and correctness controls implemented; the protocol's
ordered-pair search has not been run. No qualifying quadruple or yield claim is
recorded here.

The program enumerates `{-1,0,1}` radix-8 abscissae with at most two nonzero
digits, lifts those on Curve25519, and checks the actual `P+Q` and `P-Q`
coordinates. It then excludes 8-torsion and computes the exact cyclic carries
of

`(x(P)-x(Q))^2*x(P+Q)*x(P-Q) - (x(P)*x(Q)-1)^2`.

The [frozen protocol](PROTOCOL.md) defines the candidate order, anchored
population, success threshold and evidence requirements. There is no random
stream. The [Rust source](src/main.rs) contains the evaluator and three unit
controls. Its dependency versions are pinned in `Cargo.lock`.

```sh
cargo test --release --locked
cargo run --release --locked -- --max-weight 2 --anchor-weight 1
```

Run the second command only under the repository's authorized experiment
backend and record the complete output and report required by the protocol.
Earlier Python counts from a separate workspace were exploratory and are not
replayed or promoted here.

## Limits for index calculus

The A-free differential quartic fixes the product of the two outputs, not
their individual values. When `x1 != x2`, genuine outputs also satisfy

`(x1-x2)^2*(x(P+Q)+x(P-Q)) = 2*((x1+x2)*(x1*x2+1)+2*A*x1*x2)`,

where `A=486662`. The quartic alone therefore admits spurious tuples as a
relation-finding equation. The evaluator computes both outputs from actual
lifted curve points.

This weight-two population has at most
`1 + 2*85 + 4*binom(85,2) = 14,451` abscissae and at most twice as many
signed points. Consequently, four-summand combinations cover fewer than
`(2*14,451)^4 < 2^60` group elements, even before collisions. This is a
cardinality bound for the restricted search, not a general barrier to sparse
representations. The weight-two run is an arithmetic control and cannot
supply full-size four-summand index-calculus coverage.

## Relation to prior work

The affine Montgomery addition formula and X/Z doubling controls follow the
[Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/auto-montgom-xz.html)
and [RFC 7748](https://www.rfc-editor.org/rfc/rfc7748). The summation-polynomial
context is [Semaev 2004](https://eprint.iacr.org/2004/031.pdf). Related
repository studies are
`research/prime_field_summation_polynomials_20261119/` and
`research/curve25519_wave7_untried_20261122/`. This study tests a restricted
representation and does not establish a new summation-polynomial method or a
solving-cost improvement.
