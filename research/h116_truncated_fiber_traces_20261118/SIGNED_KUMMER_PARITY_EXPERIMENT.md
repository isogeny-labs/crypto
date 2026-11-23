# H116 signed-y Kummer parity experiment

Protocol: `H116-SIGNED-KUMMER-PARITY-001`

Source/launch gate commit: `8bc95890f0`

Disposition: **breaking negation symmetry removes the exact-half theorem but
does not produce a calibrated parity signal in the bounded signed-y family**.
[pcc:unverified]

The 184 covers, statistic and 4,096 family-wise shuffles were pushed before
any signature was evaluated. The best discovery rule uses anchors four and
six:

```text
z^2 = (y-y([4]P)) (y-y([6]P)).
```

It covers 1,498/1,520 points and predicts 792 correctly, for signed excess 86.
There are zero total-exact rules and zero rules exact on their covered set.
In 4,096 within-curve label permutations, 3,726 family maxima meet or exceed
the observed statistic, giving plus-one p=`0.909690017086`.

| subgroup order | covered | correct | signed excess |
| ---: | ---: | ---: | ---: |
| 127 | 120 | 53 | -14 |
| 197 | 192 | 103 | 14 |
| 257 | 252 | 135 | 18 |
| 397 | 392 | 197 | 2 |
| 547 | 542 | 304 | 66 |

The pooled excess is dominated by the largest curve and is entirely typical
after selection over the frozen family. Unlike the x-only result, exact half
accuracy is not forced: these covers change under point negation. This is an
ordinary calibrated null result, not a new impossibility theorem.

The next cover candidate uses tangent-line/vertical-line Miller quotients,
whose divisors encode doubling rather than arbitrary horizontal y-levels. A
new source gate is required; no OOS test is justified from this result.
[pcc:unverified]

```text
4183eb02a1ea0327ed330f4ca8c58eb17e73743a97ba9d747a9a241e6b458803  SIGNED_KUMMER_PARITY_RESULT.json
```

The complete run reproduces byte-for-byte.
