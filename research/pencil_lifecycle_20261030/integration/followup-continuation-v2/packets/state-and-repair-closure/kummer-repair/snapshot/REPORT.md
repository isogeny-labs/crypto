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
by the Mumford curve equations. The [chart proof](CHART_NONVANISHING.md)
also expresses the direct chart as the recorded invertible conversion
matrix applied to a vector with first coordinate 1. Consequently it is
nonzero for **every** degree-two polynomial input over each of the 21
recorded fields, including `a0=0`; this is stronger than the finite
exceptional replay. The same argument establishes nonvanishing of the
degree-one candidate chart. It does not prove the degree-one chart's
geometric interpretation or any action identity.

The degree-one candidate chart used for the finite check is the fixed
conversion of `(0,1,x,x²)`. Its 27,470 comparisons use native Jacobian
addition to construct translated divisors, and the expected projection
does not use the action matrix being tested. This is still a finite check
of the chart's geometric interpretation, not an all-state theorem.

## Historical generic proof gap (subsequently closed for the declared chart)

For a branch root `r`, a generic divisor with `U=u(r)!=0` can be translated
by taking `c=-v(r)/U`, `L=v+c*u`, and
`u'=(f-L²)/(u*(x-r))`; the reduced second coordinate is `v'=-L mod u'`.
These formulas give the exact rational identity to test:

```
K(u',v')  ~  M_r K(u,v)
```

after clearing powers of `U` and reducing every cross-product modulo the
two coefficients of `v²-f mod u`. At the original N16 freeze, this packet
did not contain a multivariate ideal-reduction certificate. Matrix
involutions, commutation, product relations, and finite exceptional checks
alone did not imply the generic identity. The later exact packets below
close the declared-chart action identities. No new signed collector or
1,260-target cohort was run; its rho and end-to-end cost ratios remain
**null**.

The native validator recomputes all finite row counts, class/action
coverage, source digests, and packet checksums. Hydrate artifact-store files
before validation:

```sh
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n16-action-certificate
cargo run --offline --locked --release --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml --bin validate
```

## Subsequent exact generic-branch certificate (25 October 2026)

The open generic reduction described above was subsequently executed for all
105 frozen model/root pairs. The [new packet](GENERIC_IDEAL.md),
[PDF](GENERIC_IDEAL.pdf), [review graph](generic-ideal-graph.svg),
[standardized CSV](results/generic-ideal.csv), and
[raw ideal witnesses](results/generic-ideal.json) retain the separate result.
An independently implemented native checker verified **630 of 630** exact
six-cross-product polynomial-ideal identities, with zero nonzero remainders.
Each cleared translated vector uses `U^6`; in every row its action-side
first coordinate is a nonzero scalar times `U=u(r)`. This establishes the
recorded projective action for the **declared degree-two chart** on the
generic `U!=0` branch under the stated Mumford construction. The generic
packet by itself does not cover `U=0`, degree one, or degree zero; those
declared-chart cases are addressed in the exceptional packet below.
Geometric interpretation of the charts and a new collector/rho ratio
remain unproved/unmeasured.

## Corrected degree-one basis and exceptional branches

The earlier [chart note](CHART_NONVANISHING.md) incorrectly identified the
degree-two direct chart matrix `C2` with the source degree-one
`conversion_matrix` (`C1`). Their third columns differ, but
`C2(1,-a1,-a0,q3)=C1(1,-a1,a0,q3+sigma*a0)` with
`sigma=1+lambda+mu+nu=-f4`; N15 correctly inverts this alternate `C1`
degree-two vector. The earlier N16 determinant in `results/summary.json` was for `C1`. The corrected
[exceptional packet](EXCEPTIONAL_IDEAL.md) checks nonzero determinants of
**both** matrices on all 21 fields and retains the
[wrong-basis raw diagnostic](results/exceptional-ideal-wrong-basis.json):
when `C2` is wrongly used for degree one, all 105 exceptional pairs have
six nonzero cross-products, and 84 of 105 `O`/torsion pairs mismatch.
The original finite replay already used source `C1`, so its recorded
27,470 matches remain unchanged.

With the source `C1`, the corrected exact reduction gives six zero
univariate cross-products for every one of the 105 `U=0` model/root pairs.
The `O` to torsion and torsion to `O` projective checks are also exact in
all 105 cases. The recorded matrix involutions derive the reverse
degree-one action away from the branch root. This closes the action
identities for the **declared charts** on every valid reduced degree-zero,
one, and two divisor class of the 21 models, conditional on those charts
representing the geometric Kummer embedding. That geometric chart
interpretation, a signed relation-label-preserving collector using the
action, and a new matched-rho end-to-end ratio remain unproved/unmeasured.
