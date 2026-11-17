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

## Outcome at t = 12, and the revised prediction for t = 13

Appended 2026-11-17 08:36 UTC, after the t = 12 run (`raw/t12/`) and while the t = 13 run
(started 08:33:18 UTC) was in its first minutes; nothing of its c = 6 closure was known.

| quantity (closure c = 6, t = 12) | predicted | band | measured | verdict |
|:--|--:|:--|--:|:--|
| answer | 1 enters at c = 6, 190,051 columns | | 1 enters at c = 6, 190,051 columns; falls 4, 5, 6 | as predicted (and as the reference found on the pods) |
| multiply-adds M | 2.81e12 | 2.0e12 .. 3.7e12 | 3,690,966,694,706 | inside, at the upper edge (31 % above the centre) |
| wall of the closure | 1,210 s | 850 .. 1,700 s | 1,722 s (load 3.6 to 4.0, one neighbour) | **outside the band by 1.3 %: the prediction is refuted on time** |
| stored words | 3.4e8 | | 352,689,051 (339,806,373 entries) | 4 % above |
| peak resident memory | not predicted | | 3.2 GB | |

κ at t = 12 is 0.00214, not 0.00163, and the rate is 0.467 ns per multiply-add, not 0.43. The
constant-κ model is too optimistic. Revised (**estimate**):

- least squares on the three points in logs: `M ∝ n₊^3.22`, which gives 1.75e13 at t = 13;
- the last step alone (t = 11 → 12) has exponent 3.52, which gives 2.16e13;
- at 0.47 to 0.50 ns per multiply-add: **8,200 s to 10,800 s** for the c = 6 closure at t = 13
  on this machine, about 9.3e8 stored words, about 9 GB resident.

Refuted if the closure takes more than 13,000 s or fewer than 6,500 s, or if 1 does not enter at
c = 6.

## Outcome at t = 13

Appended 2026-11-17 11:55 UTC, after the run (`raw/t13/`, started 08:33:18, ended 11:49:46 UTC).

| quantity (closure c = 6, t = 13) | revised prediction | measured | verdict |
|:--|--:|--:|:--|
| answer | 1 enters at c = 6 | 1 enters at c = 6, 313,912 columns; falls 4, 5, 6 | as predicted |
| multiply-adds M | 1.75e13 .. 2.16e13 | 24,217,016,013,719 | **12 % above the upper value** |
| wall of the closure | 8,200 .. 10,800 s; refuted above 13,000 s | 11,775 s (load 3.2 to 4.8) | above the stated range by 9 %, below the refutation bound |
| stored words | about 9.3e8 | 1,070,913,677 (1,034,680,005 entries) | 15 % above |
| peak resident memory | about 9 GB | 9.6 GB | |

κ = 0.00312 and the local exponent of M in n₊ from t = 12 to t = 13 is 3.75 (2.95, 3.52 before).
Both predictions erred low. A model with a fixed exponent underestimates this engine's work.
