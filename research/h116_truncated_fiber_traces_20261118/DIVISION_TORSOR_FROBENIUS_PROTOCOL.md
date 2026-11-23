# H116 selected division-torsor Frobenius protocol

Protocol ID: `H116-DIVISION-TORSOR-FROBENIUS-001`

Filed after the fixed division-polynomial characters calibrated null and
before constructing any division torsor. [pcc:unverified]

## Related-work boundary

The torsor identity below is the standard Kummer connecting-cocycle
construction, not a new cohomology theorem.  A repository search and a
targeted OpenAlex query for elliptic-curve Kummer/Frobenius cocycles located
the adjacent generalized-Weil-pairing DLP literature (including DOI
`10.1016/S0304-3975(04)00121-5`) and arithmetic Kummer descent, but no exact
coordinate-minimum selector decoder matching this frozen experiment.  This
screen separates the new bounded test from its standard ingredients; it is
not a publication-novelty or absence claim. [pcc:unverified]

## Candidate

For each public `Q=[k]P` on an odd prime-order subgroup, form its four
geometric halves

```text
H_Q = { S : [2]S=Q } = R + E[2],
R = [(r+1)/2]Q in E(F_p).
```

The full two-torsion is constructed in the degree-one, two or three splitting
field of `x^3+Ax+B`.  Order affine extension coordinates by their polynomial-
basis integer encodings and select the least point `S_Q` in `H_Q`.  Encode the
Frobenius cocycle

```text
c(Q) = pi(S_Q)-S_Q in E[2]
```

by the sorted two-torsion label `0..3`.  Test all 16 fixed maps from that label
to `k mod 2`.  This selector is representation-dependent, so the same selected
map is also scored after coordinate rescalings `(x,y)->(u^2 x,u^3 y)` for
`u=2,3`; stability is evidence, not an acceptance repair.

## Exact controls

For every one of the 1,520 targets, require all four candidate points to be
distinct and to double to `Q`.  Require every cocycle to be two-torsion.  Since
`R` is rational,

```text
pi(R+T)-(R+T) = pi(T)-T.
```

Therefore the *unselected multiset* of four cocycles must equal the fixed
multiset obtained from `E[2]`, independently of `Q`.  All 6,080 half/cocycle
checks and all 1,520 multiset checks must pass before scoring.  A planted
two-label scorer control must be exact.

Use the same one-sided statistic and 4,096 deterministic within-curve label
shuffles, reselecting over all 16 maps in every replicate.  Retain per-curve
scores, rescaled-model scores, selector agreement, exact-rule counts and a
byte-identical replay.

## Boundary

Any target dependence can enter only through the non-geometric selector; the
unselected torsor invariant is frozen as a negative control.  A favorable
finite result would still require a model-canonical selector, a generic proof,
an exact recursive decoder and full asymptotic accounting.  A null rejects
only this selector/cocycle construction. [pcc:unverified]
