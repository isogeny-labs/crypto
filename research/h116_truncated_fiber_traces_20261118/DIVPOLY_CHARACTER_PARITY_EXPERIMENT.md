# H116 fixed division-polynomial character experiment

Protocol: `H116-DIVPOLY-CHARACTER-PARITY-001`

Source/launch gate commit: `47b98d04e1`

Disposition: **the canonical multiplication-map divisors pass every arithmetic
control but do not produce a calibrated parity signal in this bounded
family**. [pcc:unverified]

All 12,160 frozen identities

```text
x([m]Q) = x(Q) - psi_(m-1)(Q) psi_(m+1)(Q) / psi_m(Q)^2
```

passed for `m=2..9` before scoring.  The selected rule is
`Legendre(psi_4(Q) psi_9(Q))` with polarity zero.  It is total and predicts
812/1,520 labels, for signed excess 104.  There are zero total-exact rules and
zero rules exact on their covered set.  In 4,096 within-curve label
permutations, 2,284 family maxima meet or exceed the observed statistic,
giving plus-one `p=0.557725164755`.

| subgroup order | covered | correct | signed excess |
| ---: | ---: | ---: | ---: |
| 127 | 126 | 70 | 14 |
| 197 | 196 | 106 | 16 |
| 257 | 256 | 128 | 0 |
| 397 | 396 | 216 | 36 |
| 547 | 546 | 292 | 38 |

The result rejects only products of at most three of the eight frozen
characters.  It does not reject general EDS or division-polynomial
observables, and it supplies no ECDLP speedup.

The next experiment leaves character classifiers.  It canonically selects a
point from the four-point `[2]`-division torsor above `Q`, measures the
Frobenius cocycle of that selected point, and separately verifies the
cohomological control that the *unselected multiset* of cocycles is independent
of `Q`. [pcc:unverified]

```text
665c1f17ef998af27f42774ede640178b2785766a7419e467ddc4e7a5450a7d3  DIVPOLY_CHARACTER_PARITY_RESULT.json
```

The complete run reproduces byte-for-byte.
