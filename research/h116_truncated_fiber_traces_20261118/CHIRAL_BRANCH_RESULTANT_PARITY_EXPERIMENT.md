# H116 chiral branch-resultant parity experiment

Protocol: `H116-CHIRAL-BRANCH-RESULTANT-PARITY-001`

Source/launch gate commit: `74140510e6`

Disposition: **pairwise residual-branch resultants distinguish every target
from its negation, but the frozen one-/two-resultant affine family has no exact
scalar-parity rule and no calibrated association**. [pcc:unverified]

All 42,560 closed-form/Sylvester determinant controls, 42,560
resultant/common-root controls, and inherited branch-geometry controls pass.
There are 42,030 smooth nonzero resultant features and 218 zero-resultant
abstentions.  The ordered resultant tuple distinguishes all 1,520 targets from
their negations.  The planted scorer is 1,520/1,520 and replay from the frozen
binary is byte-identical.

The selected rule xors lexicographic resultant features 14 and 15 and then
complements.  It covers 1,482 targets and matches 799, for binary excess 116.
No total or covered rule is exact.  In 2,256/4,096 deterministic within-curve
shuffles, the reselected family maximum is at least as large, giving plus-one
`p=0.550890895777`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 64 / 118 |
| 197 | 101 / 188 |
| 257 | 153 / 252 |
| 397 | 208 / 380 |
| 547 | 273 / 544 |

Lean proves the monic-quadratic resultant closed form, its symmetry, and
vanishing at a common root.  The 239-theorem audit has zero failures and zero
trusted `native_decide` declarations.  These theorems certify the native
controls, not a decoder.

This closes the prespecified one-/two-resultant character family.  The next
lead promotes selected branch-pair quartics `Z^2=H_i(X)H_j(X)` to auxiliary
genus-one curves and tests their Frobenius traces modulo three.  Those traces
retain the cross-ratio/isomorphism geometry of four ramification points rather
than only discriminants and pairwise norms. [pcc:unverified]
