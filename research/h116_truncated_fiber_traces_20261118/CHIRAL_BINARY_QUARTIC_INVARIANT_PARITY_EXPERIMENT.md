# H116 chiral binary-quartic invariant-parity experiment

Protocol: `H116-CHIRAL-BINARY-QUARTIC-INVARIANT-PARITY-001`

Source/launch gate commit: `965a002f6c`

Disposition: **the classical `I,J` character tuple is strongly
sign-sensitive, but its selected affine rule has no calibrated association
with scalar parity and no rule is exact**. [pcc:unverified]

All 12,160 inherited cubic reconstructions, 24,320 inherited discriminant
controls, 42,560 inherited resultant controls, and 5,992 inherited smooth
genus-one trace controls pass.  For each admitted quartic, the direct
binary-quartic invariants also pass the independent finite-field identity

```text
4*I^3-J^2 = 27*disc(H_i)*disc(H_j)*Res(H_i,H_j)^2.
```

This gives 5,992 invariant checks and 11,926 nonzero invariant features.  The
ordered eight-bit tuple distinguishes 1,426/1,520 targets from their
negations.  The planted scorer is 1,520/1,520 and replay is byte-identical.

The selected rule has mask `39`, hence uses feature indices `[0,1,2,5]`, and
polarity `1`.  It covers 1,431 targets and matches 764, for binary excess 97.
No total or covered rule is exact.  In 3,806/4,096 deterministic within-curve
shuffles, the reselected family maximum is at least as large, giving plus-one
`p=0.929216499878`.

| subgroup order | correct / covered |
| ---: | ---: |
| 127 | 51 / 109 |
| 197 | 84 / 177 |
| 257 | 135 / 244 |
| 397 | 196 / 369 |
| 547 | 298 / 532 |

Lean proves the specialized binary-quartic discriminant identity for a
product of two monic quadratics.  The 242-theorem audit has zero failures and
zero trusted `native_decide` declarations.  This formalizes the native
invariant control, not a scalar decoder.

This closes the frozen eight-bit affine `I,J` character family.  The next
lead uses the same factorization as extra structure: each partition
`H_i*H_j` selects a rational two-torsion class on the auxiliary genus-one
Jacobian.  We will test the induced two-isogeny Kummer/covering class rather
than another invariant of the unmarked quartic. [pcc:unverified]
