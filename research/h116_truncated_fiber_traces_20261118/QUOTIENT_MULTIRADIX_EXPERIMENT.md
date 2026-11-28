# H116 growing-radix quotient-rank experiment

Protocol: `H116-QUOTIENT-MULTIRADIX-001`

Source/launch gate commit: `d538d8b0e3`

Disposition: **mean correct rank grows sublinearly on this fixed finite panel,
but the exact required width grows essentially with the entire radix; the
small apparent slope is also confounded by radices approaching the smaller
group orders**. This does not meet the complete polylogarithmic-width condition
for a subexponential recursion. [pcc:unverified]

## Frozen execution

The protocol, exact radix list, source, launch manifest and all retained inputs
were pushed before any rank at a new radix was computed. The program found
zero mismatches across all sixteen native/isomorphic invariant channels and
exactly reconstructed both prior radix-six arrays:

```text
rank histogram = [870, 727, 738, 763, 729, 658]
path survival  = [355, 744, 1460, 2345, 3437, 4485]
```

## Rank scaling

| radix `B` | targets | mean rank | mean rank / `B` | p50/p90/p99 | required width | max depth |
| ---: | ---: | ---: | ---: | --- | ---: | ---: |
| 2 | 4,545 | 1.499010 | 0.749505 | 1/2/2 | 2 | 9 |
| 3 | 4,530 | 1.979249 | 0.659750 | 2/3/3 | 3 | 5 |
| 4 | 4,515 | 2.420598 | 0.605150 | 2/4/4 | 4 | 4 |
| 5 | 4,500 | 2.882444 | 0.576489 | 3/5/5 | 5 | 3 |
| 6 | 4,485 | 3.385284 | 0.564214 | 3/6/6 | 6 | 3 |
| 8 | 4,455 | 4.240404 | 0.530051 | 4/8/8 | 8 | 3 |
| 12 | 4,395 | 6.226849 | 0.518904 | 6/11/12 | 12 | 2 |
| 16 | 4,335 | 8.207843 | 0.512990 | 8/15/16 | 16 | 2 |
| 24 | 4,215 | 11.874021 | 0.494751 | 12/22/24 | 24 | 1 |
| 32 | 4,095 | 14.689621 | 0.459051 | 15/29/32 | 32 | 1 |
| 48 | 3,855 | 19.675486 | 0.409906 | 18/42/48 | 48 | 1 |
| 64 | 3,615 | 24.481051 | 0.382516 | 23/55/60 | 61 | 1 |

The preregistered OLS slope of `log(mean rank)` on `log(B)` is
`0.835101616673`. None of 4,096 row-preserving shuffled slopes is as small,
giving plus-one p=`0.000244081035`. Rank-sum controls are significant at every
radix from four onward; radix two is null-like and radix three is marginal.
The ordering signal therefore changes with arity rather than being only the
previous six-way effect. [pcc:unverified]

## Completeness and the finite-size control

The favorable average does not control the exhaustive tree. A true path needs
all `B` ranks at every tested radix through 48. At radix 64 it still needs
width 61, or 95.3125% of all candidates. That width has fixed exponent
`log_64(61)=0.988456222927`, not exponent zero. The p99 rank is 60. Thus a
polylogarithmic beam is not complete even in the measured cells.

The pooled slope is also not a clean asymptotic. Per-curve slopes, using the
same twelve radices, are:

| subgroup order | slope |
| ---: | ---: |
| 127 | 0.332054 |
| 197 | 0.790192 |
| 257 | 0.807997 |
| 397 | 0.792385 |
| 547 | 0.910172 |

At `B=64`, every qualifying scalar on the order-127 curve has true quotient
one; its normalized mean rank collapses to `0.020089`. On the largest
order-547 curve, where this boundary is weaker, normalized mean rank is
`0.491212`, close to a uniform ordering. Moreover, all radices at least 24
have recursion depth one on this panel. The pooled `0.835` slope is therefore
evidence for a finite small-quotient ordering effect, not evidence that ranks
compress under simultaneous growth of `r` and `B`. [pcc:unverified]

At `B=64`, 1,118/3,615 targets have rank at most `ceil(log2 B)=6`, but the
other 2,497 are misses and no second recursion level is exercised. The
row-preserving top-log controls reject the no-association null, yet they cannot
turn this incomplete subset into an exact algorithm.

## Next falsifying test

The next panel must grow subgroup order while keeping `B/r` small and retaining
multiple recursion levels. It should freeze new curves independently of the
rank feature, include `B` values satisfying (for example) `B^3 < r`, and test
whether the largest-curve slope and worst-case width still approach one and
`B`. Without such a panel, extrapolating the pooled finite slope would be
invalid. [pcc:unverified]

## Reproducibility

```text
5590c2d459b0a9b47fdda3ffb4fda0794123a84f54272c039ae1a0a266bee859  QUOTIENT_MULTIRADIX_RESULT.json
```

A clean second invocation reproduced the result byte-for-byte.
