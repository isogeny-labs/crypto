# H116 chiral branch-residual parity experiment

Protocol: `H116-CHIRAL-BRANCH-RESIDUAL-PARITY-001`

Source/launch gate commit: `c064ddde6b`

Disposition: **the target-tethered ramification tuple is sign-sensitive and
has near-total coverage, but its frozen affine Boolean family has no exact
scalar-parity rule and no calibrated association**. [pcc:unverified]

All 12,160 cubic reconstructions, 24,320 residual/total discriminant checks,
12,160 root-count checks and 12,160 chiral negation swaps pass.  In 5,568
smooth split cases, the two residual roots lift to curve points satisfying the
certified chord law `T+R+S=O`.  The planted scorer is 1,520/1,520 and replay
from the frozen binary is byte-identical.

Of 12,160 feature instances, 12,112 are smooth; the singular census contains
24 tangent-at-the-known-root and 24 repeated-residual instances.  The ordered
tuple distinguishes 1,432/1,520 targets from their negations.

The selected decimal mask `77` uses features 0, 2, 3 and 6 with polarity zero.
It covers 1,502 targets and matches 796, for binary excess 90.  No total or
covered rule is exact.  In 4,072/4,096 deterministic within-curve shuffles,
the reselected family maximum is at least as large, giving plus-one
`p=0.994142055162`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 65 / 126 |
| 197 | 95 / 190 |
| 257 | 132 / 256 |
| 397 | 207 / 384 |
| 547 | 297 / 546 |

Lean proves the closed residual discriminant, its exact chiral swap, and the
general discriminant factorization for a linear factor times a quadratic.
The 236-theorem audit has zero failures and zero trusted `native_decide`
declarations.  These theorems certify the geometric controls, not a decoder.

This closes affine xors of the individual residual split characters at four
multiples.  The next lead measures pairwise resultants between the eight
residual quadratics.  Their characters encode mutual branch-divisor position
(norms of cross-differences), information not determined by the individual
quadratic splitting bits. [pcc:unverified]
