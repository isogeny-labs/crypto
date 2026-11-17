# Exact generic two-torsion action certificates

The frozen 21 Rosenhain models and their five recorded rational two-torsion
matrices per model were checked by exact sparse polynomial arithmetic over
each recorded prime field. The [raw certificates](results/generic-ideal.json)
contain 105 model/root rows. For each row, all six cleared Kummer
cross-products have explicit quotient polynomials against the two Mumford
generators and an empty remainder. The separate native verifier reconstructed
and checked all 630 ideal-membership equalities. The standardized
[CSV](results/generic-ideal.csv), [SVG](generic-ideal-graph.svg),
[PNG](generic-ideal-graph.png), and [PDF](GENERIC_IDEAL.pdf) give the same
per-row result without changing the original N16 finite replay.

## Exact branch and identities

Write `u=x²+a1*x+a0`, `v=b1*x+b0`, and `f mod u=R0+R1*x` over the
recorded field. The ideal generators retained in each row are

```text
E0 = b0² - a0*b1² - R0
E1 = 2*b0*b1 - a1*b1² - R1.
```

For each recorded root `r`, put `U=u(r)`, `V=v(r)`, and
`L=(U*v-V*u)/U`. Exact polynomial division of
`U²*f-(U*v-V*u)²` by `u*(x-r)` gives a quadratic quotient
`q2*x²+q1*x+q0`; its three residual coefficient polynomials have retained
membership witnesses in `(E0,E1)`, and `q2=U²`. On `U!=0`, this yields
`u'=x²+(q1/U²)*x+q0/U²`. Reducing `-L` modulo `u'` supplies the
translated `b1'`. The direct cancelled degree-two chart matrix `C2`
(distinct from the source degree-one conversion matrix `C1`) then gives a
four-vector for `K(D+T_r)` with common denominator
`U^6`.

For all 105 rows the recorded converted action has first coordinate
`W0=kappa_r*U` as an exact polynomial identity, with the retained
`kappa_r != 0`. Each of the six projective cross-products between the
translated four-vector and `M_r*K(D)` is explicitly represented as
`A_ij*E0+B_ij*E1`; the retained remainder is empty. The conversion
degree-two matrix `C2` is invertible by the later
[separate determinant check](results/exceptional-ideal.json); the earlier
N16 determinant field referred to `C1`. Thus the
recorded matrix gives the same projective chart as the generic degree-two
Jacobian translation on the open locus `U!=0`, conditional on the stated
Mumford addition construction and recorded Rosenhain model.

The branch `U=0`, degree-one and degree-zero geometric chart/action
identities, and a complete signed collector remain **unproved** here. The
finite N16 exceptional replay remains evidence only for its recorded
classes. No new collector ran, so no new end-to-end or matched-rho cost
ratio exists; corresponding standardized fields are `null`.

## Reproduction and checking

From the repository root, with the stored source data hydrated:

```sh
CARGO_TARGET_DIR=/dev/shm/n16-ideal-target TMPDIR=/dev/shm \
  cargo run --release --offline --locked \
  --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml \
  --bin generic-ideal
CARGO_TARGET_DIR=/dev/shm/n16-ideal-target TMPDIR=/dev/shm \
  cargo run --release --offline --locked \
  --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml \
  --bin validate-generic-ideal
```

The producer is [generic_ideal.rs](src/generic_ideal.rs). The independent
checker is [validate_generic_ideal.rs](src/validate_generic_ideal.rs); it
uses its own polynomial representation and independently reconstructs the
fixed curve, conversion matrix, source vector, rational translation
numerator, and all cross-products before verifying every ideal-membership
witness. The PDF source is [GENERIC_IDEAL.tex](GENERIC_IDEAL.tex). The
`SHA256SUMS` file pins the packet and the two frozen source JSONL files.
