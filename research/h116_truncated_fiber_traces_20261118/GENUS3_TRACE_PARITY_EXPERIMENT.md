# H116 genus-three cover trace parity experiment

Protocol: `H116-GENUS3-TRACE-PARITY-001`

Source/launch gate commit: `39eb52f33c`

Disposition: **the frozen family is exactly balanced, and a sign-twist
symmetry proves that no deterministic decoder of these trace parities can
recover scalar parity on an odd-order subgroup**. [pcc:unverified]

The run admitted 1,577 smooth genus-three cover levels.  All 1,577 Weil-bound
and point-count/trace identities, 3,154 coordinate-rescaling checks, 12,160
target-feature smoothness decisions, and the 1,520/1,520 planted scorer
control passed.  The deterministic replay is byte-identical.

The retained empty rule scores 760/1,520, exactly half on every curve.  No
total or covered rule is exact.  Every one of 4,096 within-curve shuffled
family maxima meets the observed zero statistic, giving plus-one
`p=1.000000`.

| subgroup order | correct / targets |
| ---: | ---: |
| 127 | 63 / 126 |
| 197 | 98 / 196 |
| 257 | 128 / 256 |
| 397 | 198 / 396 |
| 547 | 273 / 546 |

The exact mechanism is broader than the affine rule family.  Write

```text
S(y0) = sum_(R != O) chi(y(R)-y0),
a(y0) = t_E-S(y0).
```

The substitution `R -> -R` gives
`S(-y0)=chi(-1) S(y0)`.  Hence `a(-y0)=t_E-chi(-1)S(y0)` has the same parity
as `a(y0)`, whether `-1` is square or nonsquare.  Every feature at `[-Q]` is
therefore equal to the corresponding feature at `Q`.  For odd subgroup order
`r`, the paired scalars `k` and `r-k` have opposite parity.  Thus *any*
deterministic function of any number of these mod-two traces fails on at least
one member of each pair.  The accompanying Lean theorem abstracts precisely
the sign-twist parity step and composes it with the existing odd-orbit decoder
obstruction.

This closes mod two for the entire signed-twist trace family, not fixed-genus
trace observables in general.  The next preregistered lead must use an odd
modulus, where negating a character sum is visible, and must target a radix
digit rather than scalar parity. [pcc:unverified]
