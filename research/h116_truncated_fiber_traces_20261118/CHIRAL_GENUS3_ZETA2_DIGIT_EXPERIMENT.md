# H116 chiral genus-three second-zeta digit experiment

Protocol: `H116-CHIRAL-GENUS3-ZETA2-DIGIT-001`

Source/launch gate commit: `7957926e46`

Disposition: **the exact quadratic-extension oracle exposes new
negation-sensitive second-zeta data, but neither frozen affine ternary family
is an exact digit decoder or family-wise significant**. [pcc:unverified]

The run independently enumerates 644,381 quadratic-extension field elements
and 643,818 affine points across the five extension curves.  All five curve
censuses and all five Frobenius-fixed base-point censuses pass.  Both chiral
histograms pass on every curve.  All 2,575,272 pointwise coordinate-rescaling
identities, 3,148 base trace identities/bounds, 3,148 extension trace
identities/bounds, 3,148 Newton integrality/coefficient bounds, 3,040 exact
chirality swaps, and the 1,520/1,520 planted scorer pass.  The replay is
byte-identical.

The second-power-sum vector distinguishes 1,326/1,520 targets from their
negations; the second zeta coefficient distinguishes 1,356/1,520.  Family-wise
selection across all 486 rules retains the `e2` coefficients `[1,1,1,0]` with
offset zero.  It scores 562/1,505 covered targets (`3*M-C=181`).  No total or
covered rule is exact.  In 926/4,096 within-curve shuffles, the reselected
family maximum is at least as large, giving plus-one `p=0.226263119356`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 51 / 126 |
| 197 | 74 / 191 |
| 257 | 97 / 256 |
| 397 | 135 / 386 |
| 547 | 205 / 546 |

This closes the two preregistered second-zeta affine families, not the cover's
Prym geometry.  Since `Jac(C)` contains the known elliptic factor, `e2(C)`
mixes public elliptic coefficients with the middle coefficient of the
abelian-surface Prym factor.  The next frozen step removes the elliptic factor
and tests the Prym middle coefficient and real-Weil discriminant directly.
[pcc:unverified]
