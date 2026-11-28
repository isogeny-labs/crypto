# H116 chiral genus-three Prym digit experiment

Protocol: `H116-CHIRAL-GENUS3-PRYM-DIGIT-001`

Source/launch gate commit: `0aca317d20`

Disposition: **the frozen Prym discriminant rule is the strongest association
in the chiral sequence but is not exact and does not pass its family-wise
calibration threshold**. [pcc:unverified]

The recomputed oracle passes all prior extension-field controls and all 3,148
new low-coefficient factorization and Prym real-Weil checks.  The Prym middle
coefficient distinguishes 1,322/1,520 targets from their negations; the
real-Weil discriminant distinguishes 1,340/1,520.  The planted scorer is
1,520/1,520 and the replay is byte-identical.

Family-wise selection across 486 frozen rules retains the real-Weil
discriminant coefficients `[0,2,0,2]` and offset two.  It scores 570/1,508
covered targets (`3*M-C=202`).  No total or covered rule is exact.  In
254/4,096 within-curve shuffles, the reselected family maximum is at least as
large, giving plus-one `p=0.062240663900`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 45 / 126 |
| 197 | 64 / 192 |
| 257 | 92 / 256 |
| 397 | 147 / 388 |
| 547 | 222 / 546 |

The rule is neither a digit oracle nor evidence of a subexponential ECDLP
algorithm.  Its score is close enough to the frozen calibration boundary to
justify one unchanged out-of-sample test.  That test must retain observable,
coefficients and offset, score no competing rule, and use fresh ordinary
prime-order curves. [pcc:unverified]

The accompanying [Lean proof](CHIRAL_GENUS3_PRYM_DIGIT_PROOF.md) checks the
low-coefficient factor removal and discriminant normalization over an
arbitrary field.  The 231-theorem audit has zero failures and zero
trusted/native decisions.  It deliberately leaves the geometric Prym
isogeny as an explicit premise. [pcc:unverified]
