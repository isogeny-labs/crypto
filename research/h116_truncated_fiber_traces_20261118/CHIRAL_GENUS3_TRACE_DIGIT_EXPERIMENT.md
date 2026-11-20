# H116 chiral genus-three trace digit experiment

Protocol: `H116-CHIRAL-GENUS3-TRACE-DIGIT-001`

Source/launch gate commit: `dbd1bc2f26`

Disposition: **the chiral construction escapes negation invariance, but the
frozen first-trace affine family has no exact ternary digit rule and no
family-wise association**. [pcc:unverified]

All 3,148 admitted smooth-cover discriminant gates, Weil bounds and exact
point-count/trace identities pass.  All 6,296 full-trace coordinate-rescaling
checks, 3,040 full-trace chirality-swap checks, and the 1,520/1,520 planted
scorer pass.  The deterministic replay is byte-identical.

The chiral escape is nonvacuous.  The two full traces differ in 2,920/3,040
target/multiplier comparisons, their mod-three residues differ in
1,940/3,040 comparisons, and 1,296/1,520 targets are distinguished from their
negations by at least one ordered residue pair.

The selected affine rule has coefficients `[1,2,2,0]` and offset zero.  It
scores 554/1,505 covered targets (`3*M-C=157`).  No total or covered rule is
exact.  In 1,765/4,096 deterministic within-curve shuffles, the reselected
family maximum is at least as large, giving plus-one `p=0.431047107640`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 54 / 126 |
| 197 | 63 / 191 |
| 257 | 95 / 256 |
| 397 | 149 / 386 |
| 547 | 193 / 546 |

This rejects only affine `F_3` decoding from the first Frobenius traces of
the four frozen covers.  It does not restore the earlier sign obstruction:
the ordered chiral tuple demonstrably contains negation-sensitive
information.  The next lead keeps the same cover geometry but tests the
second Frobenius power sum and second zeta coefficient, which access new
information in the genus-two Prym factor. [pcc:unverified]
