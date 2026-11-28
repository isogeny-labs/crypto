# resgcd with half-GCD and NTT multiplication — result

Follows `RESULT_RESGCD_M3_20261126.md`. Code: isogeny-labs/crypto#5308
(half-GCD), #5312 (loop fix, NTT, division gating) and the NTT product
threshold commit on this branch. Every claim is [pcc:unverified]. Claude
Code, local 4-vCPU container, one thread, 16-bit curve
`icv1-fp16-t351-d433f24e`, CLI seed 20261400.

## What was built

- `gcd=half`: Thull–Yap half-GCD over the counted polynomial arithmetic,
  with classical Euclid steps below degree 512 (`UG_HGCD_MIN`).
- Newton division (power-series inversion) only when the divisor has at
  least 2048 coefficients on Karatsuba, or 1024 on NTT.
- `mul=ntt`: multi-prime NTT (998244353, 167772161, 469762049; the fewest
  primes whose product exceeds c·n·(p−1)², which is two here) with Garner CRT.
  Each input is transformed once and products are summed before one inverse,
  which the half-GCD's 2×2 matrix steps use. Single products go to the NTT
  from 512 coefficients, matrix steps from 256.
- Every modular multiplication counts as one op on every path (Karatsuba,
  NTT butterflies, twiddles, scaling, CRT); an inversion is 32.

## Correctness

- Unit tests: `half_gcd_and_fast_division_match_classical`,
  `ntt_products_and_ntt_half_gcd_match_classical` (NTT = schoolbook up to
  p = 2^31 − 1). Both grid-agreement helpers have `gcd=half` and
  `gcd=half,mul=ntt` arms with zero solution-set disagreements.
- A standalone harness built from the committed source reproduces every
  microbenchmark below exactly.
- In both sweeps every arm finds identical relation counts at every t.
- The first half-GCD (#5308) looped forever when the remainder sequence
  ended inside a half-GCD step. The new unit test found it, and #5312
  fixed it.

## Microbenchmarks (counted multiplications, p = 65521)

gcd of a random monic degree-d pair, half-GCD / classical Euclid:

| d | 512 | 1024 | 2048 | 4096 | 8192 |
|--:|--:|--:|--:|--:|--:|
| half on Karatsuba | 1.50 | 1.30 | 1.17 | 0.95 | 0.75 |
| half on NTT | 1.34 | 1.05 | 0.77 | 0.51 | 0.31 |

A single product, NTT / Karatsuba: 1.03 at 256, 0.70 at 512, 0.49 at 1024,
0.24 at 4096 coefficients (power-of-two lengths).

Product tree of L quartic leaves, NTT / Karatsuba: 1.14 (L = 256), 1.09
(512), 0.96 (1024). Tree products have length 2^k + 1, so their transforms
pad to nearly twice the output length. A size-aware cost model rather than
a fixed threshold would avoid this; it is not implemented.

## m = 2 at large t (gcd degree 2^t), 30 trials

| t | classical | NTT, classical gcd | half (Kar.) | half + NTT | half+NTT / classical |
|--:|--:|--:|--:|--:|--:|
| 12 | 48,132,982 | 47,438,963 | 40,022,171 | 22,051,265 | 0.46 |
| 13 | 190,989,237 | 186,254,544 | 127,195,531 | 58,143,907 | 0.30 |

Data: `t_scaling_m2_halfgcd_ntt_t12_13.jsonl` (binary `4b0e7305`).

## m = 3, t = 3…10 (per-call F_p operations)

| t | \|V\| | classical | half (Kar.) | NTT, classical gcd | half + NTT | grid | best resgcd / grid |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 8 | 11,174 | 11,630 | 11,174 | 11,630 | 1,224 | 9.1 |
| 4 | 16 | 66,976 | 68,272 | 66,976 | 68,272 | 7,440 | 9.0 |
| 5 | 32 | 418,996 | 423,124 | 418,996 | 423,124 | 51,232 | 8.2 |
| 6 | 64 | 2,723,936 | 2,738,336 | 2,723,936 | 2,738,336 | 378,944 | 7.2 |
| 7 | 128 | 18,334,518 | 18,387,893 | 18,334,518 | 18,387,893 | 2,912,384 | 6.3 |
| 8 | 256 | 127,299,689 | 127,504,741 | 133,827,977 | 134,033,029 | 22,831,360 | 5.6 |
| 9 | 512 | 908,074,605 | 1,013,103,359 | 933,192,532 | 1,006,190,369 | 180,797,952 | 5.0 |
| 10 | 1024 | 6,632,116,107 | 7,127,320,566 | 6,558,591,998 | 5,435,280,528 | 1,439,007,744 | 3.8 |

50 trials per point (45 at t = 7, 20 at t ≥ 9); relation counts equal across
arms. Data: `t_scaling_engines_m3_halfgcd_ntt512_16bit_20261400.jsonl`. The
classical, half-Karatsuba and grid rows for t ≤ 9, and classical at t = 10,
were measured with binary `4b0e7305` (they do not touch the NTT code and
were carried over); the NTT rows and the rest of t = 10 use `c02404e9`. The
threshold-256 run is kept as `t_scaling_engines_m3_halfgcd_ntt_16bit_20261400.jsonl`:
there the NTT arm was 15 % and 14 % worse at t = 7 and 8, against 0 % and
5 % now.

## Reading

- The half-GCD only pays once the gcd degree passes ~1100 (on NTT). At m = 3
  the gcd degree is |V| = 2^t, so it first wins at t = 10: half + NTT is
  0.82× classical there. At m = 2 the gcds are larger relative to the work,
  and half + NTT is 0.46× and 0.30× at t = 12 and 13.
- At m = 3 resgcd still costs 3.8× the grid at t = 10. The gap went
  5.6 → 5.0 → 3.8 over t = 8 → 10 (steps of 0.90× and 0.75×), but each target
  still needs |V| eliminations of degree 4|V|. Even at the asymptotic rate the per-target cost is
  |V|^{2+o(1)} = p^{2/3+o(1)} at |V| ≈ p^{1/3}, above rho's p^{1/2} for the
  whole problem.
- Faster arithmetic does not change the earlier verdict: no per-system
  solver moves the PKM chain's exponent.
