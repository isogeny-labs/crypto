# H116 fixed-ratio large-panel quotient-rank experiment

Protocol: `H116-QUOTIENT-LARGE-PANEL-001`

Source/launch gate commit: `2727ba4716`

Disposition: **the apparent growing-radix compression does not replicate when
radix/order ratio is controlled. Every radix requires full width, and ranks
are null-like or slightly worse than shuffled labels**. This closes the frozen
`IX1` quotient-ordering route to a complete subexponential decoder on the
tested family. [pcc:unverified]

## Selection and execution gates

Before data generation, the producer independently counted all points and
verified that `B=15,28,36` are respectively the first positive nonsingular
coefficients giving prime group order at `p=5003,10007,20011`. The admitted
orders are 5,081, 9,851 and 19,927. Curve selection therefore did not inspect
the rank feature.

The producer emitted 34,856 targets and 69,712 native/`lambda=2` model records
with zero repeat mismatches. The scorer found zero mismatches across all
sixteen coordinate invariants.

## Fixed-ratio rank result

| radix `B` | targets | mean rank | uniform mean | p50/p90/p99 | required width | max depth | lower-rank p |
| ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| 2 | 34,853 | 1.499871 | 1.5 | 1/2/2 | 2 | 14 | 0.492556 |
| 3 | 34,850 | 2.009010 | 2 | 2/3/3 | 3 | 9 | 0.979497 |
| 4 | 34,847 | 2.505869 | 2.5 | 3/4/4 | 4 | 7 | 0.838418 |
| 5 | 34,844 | 3.015670 | 3 | 3/5/5 | 5 | 6 | 0.983891 |
| 6 | 34,841 | 3.525817 | 3.5 | 4/6/6 | 6 | 5 | 0.998047 |
| 8 | 34,835 | 4.565954 | 4.5 | 5/8/8 | 8 | 4 | 1.000000 |
| 10 | 34,829 | 5.606190 | 5.5 | 6/10/10 | 10 | 4 | 1.000000 |
| 12 | 34,823 | 6.628550 | 6.5 | 7/11/12 | 12 | 3 | 1.000000 |
| 16 | 34,811 | 8.689265 | 8.5 | 9/15/16 | 16 | 3 | 1.000000 |

Here a small rank and a small slope are favorable. The plus-one probabilities
compare shuffled rank sums no larger than observed. Radix two is indistinguish-
able from its control; from radix three onward the observed mean is above the
uniform mean, and at radices eight through sixteen every one of 4,096 shuffled
rank sums is at least as favorable as the observation. [pcc:unverified]

The pooled log-mean-rank/log-radix slope is `0.851668230314`, but all 4,096
shuffled slopes are no larger, so its lower-tail empirical p is exactly one.
The same holds separately on all curves:

| subgroup order | slope | lower-tail p |
| ---: | ---: | ---: |
| 5,081 | 0.851400129196 | 1 |
| 9,851 | 0.856011275391 | 1 |
| 19,927 | 0.849564576005 | 1 |

This explains why a raw slope below one was misleading: even uniformly random
ranks have mean `(B+1)/2`, whose finite log-log slope over small radices is
below one. The frozen shuffle distribution, not comparison to the number one,
is the correct control.

## Exact-path cost

The maximum required widths are exactly
`2,3,4,5,6,8,10,12,16`. Thus every tested radix has at least one recursive
true path whose local correct candidate ranks last. Width `B-1` still misses
5,739 of 34,811 radix-16 paths, and width `ceil(log2 16)=4` retains only
1,768. All curves exercise three scored levels at radix 16, so this is not the
one-level artifact in the earlier panel.

An exhaustive exact recursion therefore needs `B` branches and has exponent
`log_B(B)=1`. No fixed or polylogarithmic width is complete in the data. The
large-panel result refutes extrapolation of the earlier average-rank effect;
it does not refute every possible observable or division-tower construction.
[pcc:unverified]

## Reproducibility

```text
9fdb7e9b8bbbd970de2a418ec5b6db73f283682bd3b964973f278eacf57fb79b  QUOTIENT_LARGE_PANEL_MOMENTS.tsv
42367e44d6af276b93b789c4f3e6b4c139f648d101c8a5f76cfc538584f07eb7  QUOTIENT_LARGE_PANEL_LABELS.tsv
d917fe0fce5348394a462f5cd20cff2212243cce998d5b656a08afdd402759e8  QUOTIENT_LARGE_PANEL_RESULT.json
```

A second full 4,096-replicate scorer run reproduced the result byte-for-byte.
