# H116 Miller-doubling Kummer parity experiment

Protocol: `H116-MILLER-KUMMER-PARITY-001`

Source/launch gate commit: `621390e2f8`

Disposition: **replacing arbitrary horizontal covers by structured doubling
Miller divisors does not produce a calibrated parity signal in this bounded
family**. [pcc:unverified]

The 184 rules, statistic and 4,096 family-wise shuffles were pushed before any
Miller signature was evaluated.  The selected discovery rule is the product
of the quadratic characters of

```text
g_[1]P,[1]P(Q) g_[4]P,[4]P(Q) g_[8]P,[8]P(Q)
```

with polarity one.  It covers 1,480/1,520 points and predicts 788 correctly,
for signed excess 96.  There are zero total-exact rules and zero rules exact
on their covered set.  In 4,096 within-curve label permutations, 2,828 family
maxima meet or exceed the observed statistic, giving plus-one
`p=0.690505247742`.

| subgroup order | covered | correct | signed excess |
| ---: | ---: | ---: | ---: |
| 127 | 118 | 66 | 14 |
| 197 | 188 | 99 | 10 |
| 257 | 248 | 122 | -4 |
| 397 | 388 | 222 | 56 |
| 547 | 538 | 279 | 20 |

The finite excess is typical after selection over the frozen family.  This is
an ordinary calibrated null result, not an obstruction to all Miller or
function-field observables.

The next candidate evaluates quadratic characters of fixed small division
polynomials at the target point.  These functions have multiplication-map
divisors and, for even indices, signed `y` dependence.  That candidate is
mathematically distinct from both arbitrary horizontal covers and doubling
Miller quotients, and requires a new source gate. [pcc:unverified]

```text
868be6f3cec29eb983a3b648049926fa60d8fca3dd77d9e3536613531593a254  MILLER_KUMMER_PARITY_RESULT.json
```

The complete run reproduces byte-for-byte.
