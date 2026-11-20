# H116 Prym Frobenius-splitting parity experiment

Protocol: `H116-PRYM-FROBENIUS-SPLITTING-PARITY-001`

Source/launch gate commit: `3702ce7226`

Disposition: **the auxiliary-prime real-Weil splitting tuple is strongly
negation-sensitive but its frozen affine Boolean family has no exact parity
rule and no family-wise association**. [pcc:unverified]

The recomputed oracle passes every inherited control and all 6,296 direct
root-factorization checks for the real Weil polynomials over `F_5` and `F_7`.
The ordered splitting tuple distinguishes 1,486/1,520 targets from their
negations.  The planted scorer is 1,520/1,520 and the replay is byte-identical.

The selected decimal mask `101` uses features 0, 2, 5 and 6, with polarity
zero.  It covers 688 targets and matches 385, for binary excess 82.  No total
or covered rule is exact.  In 1,403/4,096 within-curve shuffles, the reselected
family maximum is at least as large, giving plus-one `p=0.342689773005`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 32 / 59 |
| 197 | 47 / 68 |
| 257 | 52 / 109 |
| 397 | 111 / 195 |
| 547 | 143 / 257 |

Repeated-root abstentions account for the low selected coverage.  This closes
the two-prime affine splitting-bit family, not the full auxiliary-torsion
Frobenius action.  The next lead tests whether the complete degree-four Prym
Weil polynomial has a linear factor modulo each auxiliary prime, which probes
rational Frobenius eigenlines beyond the real quadratic discriminant.
[pcc:unverified]
