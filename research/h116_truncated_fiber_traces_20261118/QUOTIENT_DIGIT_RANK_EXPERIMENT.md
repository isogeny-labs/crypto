# H116 quotient-digit rank and search-cost experiment

Protocol: `H116-QUOTIENT-DIGIT-RANK-001`

Source/launch gate commit: `33b56853d6`

Disposition: **the selected coordinate ordering has a reproducible aggregate
rank advantage, but no fixed width below six is complete even on this finite
panel; the resulting exact frozen search is linear, not subexponential**.
[pcc:unverified]

## Frozen execution and reconstruction gates

The protocol, executable, included scorer, launch manifest and retained OOS
inputs were pushed before the first rank run. The executable independently
reconstructed the old selected-rule result before admitting the new data:

| gate | result |
| --- | ---: |
| OOS targets | 4,485 |
| curve/generator rows | 15 |
| invariant mismatches across 16 channels | 0 |
| recomputed unique-minimum coverage | 4,328 |
| recomputed unique-minimum matches | 845 |
| targets with any candidate-value tie | 665 |
| targets with a tied minimum | 157 |

The first four values equal the retained OOS record. Candidate ranks use the
frozen lexicographic `(IX1 value, digit)` order, so every target has one total
rank even when field representatives tie.

## Single-step rank result

| true-digit rank | 1 | 2 | 3 | 4 | 5 | 6 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| count | 870 | 727 | 738 | 763 | 729 | 658 |
| cumulative top-`t` | 870 | 1,597 | 2,335 | 3,098 | 3,827 | 4,485 |

The normalized mean reciprocal rank is `0.429368264586`, versus
`(1+1/2+...+1/6)/6 = 0.408333333333` for a uniform rank. In 4,096 fixed
within-row digit shuffles, zero reciprocal-rank scores and zero top-one counts
meet or exceed the observations; four top-two counts do. The corresponding
plus-one probabilities are `0.000244081035`, `0.000244081035`, and
`0.001220405175`. These are association controls, not probabilities that the
decoder is correct. [pcc:unverified]

## Complete recursive paths

For every canonical scalar `k>=6`, the run follows the true recursion
`k -> floor(k/6)` and records the largest local rank encountered before the
public base case. This measures whether an exhaustive top-`t` tree contains
the true path at every level.

| width `t` | complete paths | rate | exponent `log_6(t)` |
| ---: | ---: | ---: | ---: |
| 1 | 355 | 0.079152731327 | 0 |
| 2 | 744 | 0.165886287625 | 0.386852807235 |
| 3 | 1,460 | 0.325529542921 | 0.613147192765 |
| 4 | 2,345 | 0.522853957637 | 0.773705614469 |
| 5 | 3,437 | 0.766332218506 | 0.898244401704 |
| 6 | 4,485 | 1 | 1 |

Depths are one for 450 targets, two for 2,376, and three for 1,659. Every
curve has at least one path requiring width six. Thus no frozen `t<6` rule is
complete on any claim spanning the panel. Width six is exhaustive: at depth
`L` it has `6^L` leaves, hence exponent one in the scalar range. Fixed widths
two through five have positive constant exponents and still miss observed
solutions. Width one is polynomial-width but misses 92.08% of these paths.

This is the precise wall for this radix-six candidate: the local ordering can
prioritize an exhaustive search, but the evidence supplies neither an exact
digit oracle nor a complete subexponential tree. A new scaling experiment must
change the radix and test whether correct ranks compress as the radix grows;
the present constant-radix association cannot establish that property.
[pcc:unverified]

## Reproducibility

```text
3efc862194feeef84ae76bb8c1e95fcc7fa3227fe8a924cd47d2d9b32f4b4c31  QUOTIENT_DIGIT_RANK_RESULT.json
```

A clean second invocation reproduced the JSON byte-for-byte. The retained
result, run record, frozen sources and exact input hashes are linked by
`QUOTIENT_DIGIT_RANK_SOURCE_GATE.md` and
`QUOTIENT_DIGIT_RANK_SHA256SUMS`.
