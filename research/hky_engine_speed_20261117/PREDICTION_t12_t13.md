# Held-out prediction for t = 12 and t = 13, written before either was run

Written 2026-11-17 07:50 UTC, before any `hky2` run at `t >= 12` on this machine.
Everything here is an **estimate** from the model below, `[pcc:unverified]`.

Fit points (measured, `raw/hky2_first_t9_t10.jsonl`, `raw/hky2_first_t11.jsonl`; `hky2` defaults:
level 4, split, sparse-first queue, batch 32; 4 vCPU, another job running):

| t | c | columns n | `+` part columns n₊ | muladds M of the closure c = 6 | κ = M / (2 n₊³) | closure wall s | ns per muladd (wall, 4 threads) | stored words | words / n² |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 10 | 6 | 60,460 | 30,318 | 92,397,155,280 | 0.001658 | 39.4 | 0.426 | 34,449,151 | 0.00942 |
| 11 | 6 | 110,056 | 55,144 | 539,254,650,848 | 0.001608 | 231.1 | 0.429 | 111,433,956 | 0.00920 |

Model: for the closure in which 1 enters at `c = 6` (t = 10..13 by the other engine's values),
`M(t) = 2 κ n₊(t)³` with κ = 0.00163 (mean of the two), wall = M × 0.43 ns on this machine,
stored words = 0.0093 n². `n₊ = (n + s)/2`, `s = Σ_{k ≤ 3} C(t, k)` the swap-fixed monomials.

| t | n | n₊ | predicted M | predicted wall of the c = 6 closure here | predicted stored words | predicted answer |
|--:|--:|--:|--:|--:|--:|:--|
| 12 | 190,051 | 95,175 | 2.81e12 | 1,210 s (band 850 s to 1,700 s) | 3.4e8 | 1 enters at c = 6 (measured by the reference on rented pods: 6,857 s and 7,269 s on 16 vCPU) |
| 13 | 313,912 | 157,145 | 1.27e13 | 5,440 s (band 3,800 s to 8,200 s) | 9.2e8 (at most 7.3 GB) | 1 enters at c = 6 (the other repository's engine) |

The band is ±30 % on κ and on the rate together (two fit points; the rate can fall when the
rows no longer fit the caches, and the other job's load varies). The closures `c <= 5` add
about 30 s (t = 12) to 2 min (t = 13).

Refuted if the t = 12 closure takes more than 1,700 s or fewer than 850 s on this machine at a
comparable load, or if `M` is outside 2.0e12 .. 3.7e12.
