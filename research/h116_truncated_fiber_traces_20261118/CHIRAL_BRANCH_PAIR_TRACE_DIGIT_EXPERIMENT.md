# H116 chiral branch-pair genus-one trace-digit experiment

Protocol: `H116-CHIRAL-BRANCH-PAIR-TRACE-DIGIT-001`

Source/launch gate commit: `2fd6d6b00b`

Disposition: **the four auxiliary genus-one traces are sign-sensitive and
their selected affine rule has positive excess, but no rule is exact and the
family is not calibrated significant**. [pcc:unverified]

All five square-root-table censuses, 12,160 inherited cubic reconstructions,
24,320 inherited discriminant controls, 42,560 inherited resultant controls,
and 2,401,920 factored/expanded quartic evaluations pass.  The oracle admits
5,992 smooth genus-one traces, each reconciled between direct affine root
counts and character sums, normalized with two rational infinity points, and
checked against Hasse's bound.  The ordered trace tuple distinguishes
1,318/1,520 targets from their negations.  The planted scorer is 1,520/1,520
and replay is byte-identical.

The selected rule has coefficient vector `[2,2,2,1]` and offset `2`.  It covers
1,444 targets and matches 536, for ternary excess 164.  No total or covered
rule is exact.  In 1,038/4,096 deterministic within-curve shuffles, the
reselected family maximum is at least as large, giving plus-one
`p=0.253600195265`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 46 / 114 |
| 197 | 70 / 178 |
| 257 | 91 / 248 |
| 397 | 141 / 366 |
| 547 | 188 / 538 |

Lean proves the explicit monic-quadratic product coefficients and the
two-points-at-infinity trace normalization.  The 241-theorem audit has zero
failures and zero trusted `native_decide` declarations.  These formalize
native controls, not a scalar decoder.

This closes the frozen four-trace affine `F_3` family.  The next lead retains
the classical binary-quartic invariants `I` and `J` of the same four genus-one
models and tests their ordered quadratic-character tuple.  Those invariants
capture isomorphism/`j` geometry that is not determined by the Frobenius trace
alone. [pcc:unverified]
