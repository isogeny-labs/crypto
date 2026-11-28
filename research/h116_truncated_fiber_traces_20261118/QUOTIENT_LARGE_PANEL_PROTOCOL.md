# H116 fixed-ratio large-panel quotient-rank protocol

Protocol ID: `H116-QUOTIENT-LARGE-PANEL-001`

Filed after the original growing-radix result exposed a radix/order boundary
confound, before generating any moment on the new curves. [pcc:unverified]

## Curve selection independent of the feature

Fix base primes `5003, 10007, 20011` and `A=1`. For each prime, enumerate
positive `B` from one and select the first nonsingular curve
`y^2=x^3+x+B` whose full base-field group order is prime. Exact Legendre-sum
point counting and deterministic trial-division primality testing must certify
both the chosen order and the absence of an earlier eligible `B`.

The resulting frozen cohort is:

| p | A | B | prime order r | trace | ord_r(p) |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 5,003 | 1 | 15 | 5,081 | -77 | 635 |
| 10,007 | 1 | 28 | 9,851 | 157 | 4,925 |
| 20,011 | 1 | 36 | 19,927 | 85 | 19,926 |

Use the lexicographically first nonidentity affine point as generator and one
generator label per curve. The selection rule uses no quotient moment, rank,
target scalar, or result from the earlier panel.

## Producer

For every nonzero scalar `1 <= k < r`, generate the target `[k]P`, an opaque
protocol-bound coordinate digest, the separate scalar label, and the same
sixteen frozen formal-jet moments in native and `lambda=2` coordinate models.
Require exact group-order checks, 34,856 targets, 69,712 model records, zero
repeat mismatches, and zero native/isomorphic invariant mismatches.

## Radix experiment

Use radices

```text
B in {2,3,4,5,6,8,10,12,16}.
```

For every target with `k>=B`, rank the public candidates
`[B^-1](Q-[d]P)`, `0<=d<B`, by `(canonical IX1 representative,d)` ascending.
The true candidate is `[floor(k/B)]P`. Retain full rank histograms, mean and
normalized mean rank, p50/p90/p99, per-curve rank sums, every-width recursive
path survival, maximum required width, and recursion depth.

All radices satisfy `B/r < 0.00315`; radix 16 retains three scored recursion
levels on every curve. The preregistered scaling statistic is the OLS slope of
`log(mean rank)` on `log(B)`, pooled and separately by curve. The primary
completeness statistic is `max_required_width/B`.

## Controls

Use 4,096 deterministic true-digit shuffles within each fixed curve row.
Report exact rank-sum and top-`ceil(log2 B)` exceedances at every radix, and
cross-radix slope exceedances. Self-tests cover curve selection, ranking ties,
radix decomposition, slope recovery and census arithmetic. A clean replay
must be byte-identical.

## Decision boundary

The candidate advances only if worst-case complete width is plausibly
polylogarithmic while the subgroup and radix grow. Average-rank association,
a slope below one, or top-log enrichment is insufficient. A width proportional
to `B` retains a positive constant search exponent. This finite panel cannot
prove an asymptotic, exact generic decoder, or ECDLP speedup. [pcc:unverified]
