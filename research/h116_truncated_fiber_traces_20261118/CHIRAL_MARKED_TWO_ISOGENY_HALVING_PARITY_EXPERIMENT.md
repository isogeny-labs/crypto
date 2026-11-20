# H116 chiral marked two-isogeny halving-parity experiment

Protocol: `H116-CHIRAL-MARKED-TWO-ISOGENY-HALVING-PARITY-002`

Corrected source/launch gate commit: `12b1a6a2ed`

Disposition: **the marked source/dual kernel-halving tuple is strongly
sign-sensitive, but its selected affine rule has no calibrated association
with scalar parity and no rule is exact**. [pcc:unverified]

The first `-001` gate stopped before scoring because it incorrectly declared
the features total; the preserved preflight record explains the correction
to inherited singular-model abstention.  The scored `-002` gate admits 5,992
smooth marked quartics and abstains on the other 88 models.

All 12,160 inherited cubic reconstructions, 24,320 inherited discriminant
controls, 42,560 inherited resultant controls, and 5,992 inherited genus-one
trace controls pass.  For each admitted model, the marked resolvent root and
two-isogeny coefficient identities pass.  Exhaustive doubling of 4,818,424
affine points agrees with 11,984 independent eliminated-equation halving
counts; 11,984 source/dual point counts also agree with the inherited
genus-one orders.  There are 4,520 true bits among 11,984 admitted features,
and the ordered eight-bit tuple distinguishes 1,376/1,520 targets from their
negations.  The planted scorer is 1,520/1,520 and replay is byte-identical.

The selected rule has mask `117`, hence uses feature indices `[0,2,4,5,6]`,
and polarity `0`.  It covers 1,444 targets and matches 762, for binary excess
80.  No total or covered rule is exact.  In 4,087/4,096 deterministic
within-curve shuffles, the reselected family maximum is at least as large,
giving plus-one `p=0.997803270686`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 63 / 114 |
| 197 | 99 / 178 |
| 257 | 132 / 248 |
| 397 | 190 / 366 |
| 547 | 278 / 538 |

Lean proves that the factorization marks a resolvent root and that the source
and dual two-isogeny coefficients are respectively 81 times the quadratic
resultant and 81 times the product of quadratic discriminants.  The
243-theorem audit has zero failures and zero trusted `native_decide`
declarations.  This formalizes the native algebraic controls, not a scalar
decoder.

This closes the frozen eight-bit affine rational-halving family.  The next
lead retains the complete Frobenius cycle type on each four-point halving
fiber—`1^4`, `1^2 2`, `2^2`, or `4`—rather than only whether it has a rational
fixed point. [pcc:unverified]
