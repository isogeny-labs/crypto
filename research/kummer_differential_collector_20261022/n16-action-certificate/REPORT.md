# N16: exact matrix identities and a direct zero-branch chart

The [frozen protocol](PROTOCOL.md) checked the 21 recorded theta models.
Exact arithmetic modulo each model's prime established that all **105**
recorded action matrices are invertible and square to nonzero scalar
identities, all **210** within-model matrix pairs commute projectively, and
the product of the five matrices is scalar identity in each model. All 21
general-conversion matrices have nonzero determinant, and all 84 selected
theta inverse constants are nonzero. These are matrix equalities on the
entire four-dimensional vector space over each field, not point samples.
The raw [summary](results/summary.json) retains every determinant, square
scalar, pair check, and product scalar.

The native Jacobian replay separately evaluated the identity, five rational
degree-one torsion classes, and both available points over each
`x=0..min(p-1,255)` in every model: **5,494** class instances and **27,470**
action comparisons. All 27,470 matched the independent coordinate
construction. The first direct-source run had **5,599** missing expected
coordinates. They were all degree-two divisors with `a0=b0=0`, so the
source's four generic projection numerators vanished together. This was a
chart failure, not a demonstrated action mismatch. The final direct chart
was nonzero in all 5,599, and the other **21,595** degree-two source
projections matched it projectively. The raw
[exceptional rows](results/exceptional.csv) identify every checked class,
action, and status; the [graph](chart-graph.svg) shows the recovered chart
counts by model, and the [PDF](REPORT.pdf) carries the same neutral result.

## Exact partial theorem

Write a valid degree-two Mumford divisor as
`u=x²+a1*x+a0`, `v=b1*x+b0` on the quintic model `f(0)=0`. Let
`f mod u=R0+R1*x`. The constant coefficient of `v²=f mod u` is
`b0²=a0*b1²+R0`. If `x^n mod u=C_n+L_n*x`, then
`C_{n+1}=-a0*L_n`, `L_{n+1}=C_n-a1*L_n`, and `C_0=1,L_0=0`.
Because `f(0)=0`, `R0/a0=-sum_{n>=2} f_n*L_{n-1}` is a polynomial,
including at `a0=0`. Each generic projection numerator is
`a0*T_i-b0²`, where `T_i` is the corresponding term in the source.
It therefore equals

```
a0 * (T_i - b1² - R0/a0).
```

The direct chart takes the parenthesized four-vector, scaled by the
recorded theta diagonal. Thus it agrees projectively with the existing
source projection wherever `a0!=0`, and it remains defined at `a0=0`
without division by `a0` or use of an action matrix. The source expression
and the polynomial recurrence make this an exact identity in the quotient
by the Mumford curve equations. The 21 nonzero fixed conversion
determinants certify that the corresponding general coordinate chart has
full rank; the final finite replay checked nonzero output on the frozen
exceptional classes. A proof of nonvanishing for every valid degree-two
class at `a0=0` remains to be formalized.

The degree-one candidate chart used for the finite check is the fixed
conversion of `(0,1,x,x²)`. Its 27,470 comparisons use native Jacobian
addition to construct translated divisors, and the expected projection
does not use the action matrix being tested. This is still a finite check
of the chart's geometric interpretation, not an all-state theorem.

## Open generic action identity

For a branch root `r`, a generic divisor with `U=u(r)!=0` can be translated
by taking `c=-v(r)/U`, `L=v+c*u`, and
`u'=(f-L²)/(u*(x-r))`; the reduced second coordinate is `v'=-L mod u'`.
These formulas give the exact rational identity to test:

```
K(u',v')  ~  M_r K(u,v)
```

after clearing powers of `U` and reducing every cross-product modulo the
two coefficients of `v²-f mod u`. The N16 packet does not contain that
multivariate ideal-reduction certificate for any of the five roots. Matrix
involutions, commutation, product relations, and finite exceptional checks
do not imply the generic geometric identity. This is the precise remaining
all-state action gate. No collector or 1,260-target cohort was run; rho and
end-to-end cost ratios remain **null**.

The native validator recomputes all finite row counts, class/action
coverage, source digests, and packet checksums. Hydrate artifact-store files
before validation:

```sh
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n16-action-certificate
cargo run --offline --release --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml --bin validate
```
