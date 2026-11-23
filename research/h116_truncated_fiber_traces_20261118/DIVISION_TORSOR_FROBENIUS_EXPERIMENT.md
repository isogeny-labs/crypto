# H116 selected division-torsor Frobenius experiment

Protocol: `H116-DIVISION-TORSOR-FROBENIUS-001`

Source/launch gate commit: `a0ea413b26`

Disposition: **the frozen coordinate selector collapses to the rational half,
so its Frobenius cocycle is identically zero and carries no scalar bit**.
[pcc:unverified]

All 6,080 half/cocycle checks and all 1,520 target-independent multiset checks
pass.  The planted scorer is 1,520/1,520.  Each two-torsion cubic is
irreducible, so all five splitting fields have degree three.  Polynomial-basis
integer ordering places the rational half before every non-base half; the
selected cocycle is therefore zero.  Coordinate scales one, two and three
select the same cocycle label on all 1,520 targets.

Every one of the 16 maps is noninformative.  The deterministic tie-break
retains constant map zero, which predicts 760/1,520 labels.  Every curve is
exactly half correct, all 4,096 shuffled family maxima meet the zero observed
statistic, plus-one `p=1`, and no rule is exact.

| subgroup order | correct / targets |
| ---: | ---: |
| 127 | 63 / 126 |
| 197 | 98 / 196 |
| 257 | 128 / 256 |
| 397 | 198 / 396 |
| 547 | 273 / 546 |

This is a mechanism-level wall for the frozen selector, not a null rejection
of every division-torsor selector.  Target dependence can only be introduced
by a non-geometric choice inside a torsor whose complete cocycle multiset is
constant.

The next candidate uses a different cohomology class: the first `p`-adic carry
of a canonical lift under multiplication by the subgroup order.  The anomalous
curve Smart quotient supplies a required positive control; the unchanged
quotient is then tested on nonanomalous prime-order curves.
[pcc:unverified]

```text
36f2963b6bc1ded84d907e9452a6ec06770329aa63f4eed8343ecf305029e07e  DIVISION_TORSOR_FROBENIUS_RESULT.json
```

The complete run reproduces byte-for-byte.
