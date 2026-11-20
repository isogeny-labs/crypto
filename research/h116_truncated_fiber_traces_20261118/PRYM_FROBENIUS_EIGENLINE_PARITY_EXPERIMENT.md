# H116 Prym Frobenius-eigenline parity experiment

Protocol: `H116-PRYM-FROBENIUS-EIGENLINE-PARITY-001`

Source/launch gate commit: `4e3e1548b6`

Disposition: **the full-Weil eigenline tuple has high coverage and strong
negation sensitivity, but its frozen affine Boolean family has no exact
scalar-parity rule and no calibrated association**. [pcc:unverified]

The recomputed oracle passes every inherited control, all 6,296 direct
degree-four root predicates, and all 31,480 independently evaluated identities

```text
F(lambda)=lambda^2*W(lambda+p/lambda).
```

The polynomial identity and its nonzero-eigenvalue quotient form are also
proved in Lean as `prymWeil_homogenized_realWeil` and
`prymWeil_eq_sq_mul_realWeil`; the 233-theorem audit has zero failures and zero
trusted `native_decide` declarations.  This formalizes the control identity,
not a scalar decoder.

The ordered eigenline tuple distinguishes 1,424/1,520 targets from their
negations.  The planted scorer is 1,520/1,520, and a complete replay from the
frozen binary is byte-identical.

The selected decimal mask `24` uses features 3 and 4 and polarity one.  It
covers 1,511 targets and matches 809, for binary excess 107.  No total or
covered rule is exact.  In 3,187/4,096 deterministic within-curve shuffles,
the reselected family maximum is at least as large, giving plus-one
`p=0.778130339273`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 64 / 126 |
| 197 | 111 / 193 |
| 257 | 136 / 256 |
| 397 | 199 / 390 |
| 547 | 299 / 546 |

Unlike the preceding real-Weil splitting experiment, the selected rule has
near-total coverage, so repeated-root abstentions do not explain the null.
This closes the two-prime affine family of linear-factor-presence bits, not
the full auxiliary Frobenius conjugacy data.  The next lead retains the
complete degree partition of the quartic modulo each auxiliary prime and
tests parity consistency and leave-one-curve-out transport of the resulting
categorical tuple. [pcc:unverified]
