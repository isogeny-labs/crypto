# resgcd size-aware NTT dispatch — result

Follows `RESULT_RESGCD_HALFGCD_NTT_20261128.md`. Code: isogeny-labs/crypto#5353.
Every claim is [pcc:unverified]. Claude Code, local container, one thread,
16-bit curve `icv1-fp16-t351-d433f24e`, CLI seed 20261400, binary `c5220353`.

## Change

The fixed NTT thresholds (512 for single products, 256 for half-GCD matrix
steps) are replaced by a per-call cost comparison from operand lengths:

- `ug_ntt_sums_cost` reproduces `ug_ntt_sums`'s counted multiplications
  exactly: padded transform length, number of CRT primes, transform reuse,
  pointwise products and CRT. Both share `ug_ntt_plan`, so they cannot drift
  apart.
- `ug_kar_cost` reproduces `ug_karatsuba`'s count from its recursion on
  lengths (memoized). It is exact on factors without zero coefficients and an
  upper bound otherwise.
- `ug_mul` and both matrix steps take the cheaper estimate. A matrix step
  weighs shared-transform NTT against the best choice for each separate
  product.

Test `ntt_and_karatsuba_cost_models_match_counts`: the NTT model is exact on
random lengths up to 3000, single products and a 2×2 matrix shape with an
empty entry, p = 65521 and 2^31 − 1; the Karatsuba model is within 1 %.

## Microbenchmarks (counted multiplications, p = 65521)

Product tree of L quartic leaves, NTT mode / Karatsuba:

| L | 256 | 512 | 1024 |
|--:|--:|--:|--:|
| fixed threshold 512 | 1.139 | 1.090 | 0.960 |
| cost model | 1.000 | 0.998 | 0.899 |

gcd of random monic degree-d pairs, half-GCD on NTT / classical Euclid:
1.327 / 1.046 / 0.772 / 0.507 / 0.314 at d = 512 / 1024 / 2048 / 4096 / 8192
(fixed thresholds: 1.345 / 1.051 / 0.773 / 0.507 / 0.314).

## m = 3 rerun (per-call F_p operations; relation counts equal across arms)

| t | \|V\| | classical | NTT, classical gcd | half + NTT | grid | best resgcd / grid |
|--:|--:|--:|--:|--:|--:|--:|
| 7 | 128 | 18,334,518 | 18,334,518 | 18,387,893 | 2,912,384 | 6.3 |
| 8 | 256 | 127,299,689 | 127,278,328 | 127,483,380 | 22,831,360 | 5.6 |
| 9 | 512 | 908,074,605 | 906,984,120 | 976,147,358 | 180,797,952 | 5.0 |
| 10 | 1024 | 6,632,116,107 | 6,453,702,003 | 5,322,721,401 | 1,439,007,744 | 3.7 |

With the fixed threshold 512 the NTT arm was 133.8M / 933.2M / 6558.6M at
t = 8 / 9 / 10 and half + NTT 134.0M / 1006.2M / 5435.3M. With the cost model
the NTT arm is never above classical (0.9998×, 0.9988×, 0.973×), and
half + NTT at t = 10 improves from 0.820× to 0.803× classical. For t ≤ 6 all
modes are as before (operands below 64 coefficients, so the model is not
consulted).

Data: `t_scaling_engines_m3_halfgcd_ntt_costmodel_16bit_20261400.jsonl`. The
classical, half-Karatsuba and grid rows are carried over from the earlier
runs with their binary hashes (`4b0e7305`, `c02404e9`). With `mul=ntt` off,
the dispatch takes the same Karatsuba path as before.

## Reading

The cost model removes the padding loss and makes the NTT never worse,
lowering the best m = 3 cost at t = 10 by a further 2 %. The structural picture
is unchanged: m = 3 resgcd is still 3.7× the grid at t = 10, and its
per-target cost is p^{2/3+o(1)} even at the asymptotic rate.
