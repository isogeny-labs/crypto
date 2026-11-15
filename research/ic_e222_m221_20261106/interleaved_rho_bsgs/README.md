# Interleaved multi-target rho vs folded multi-target BSGS on j=0 curves, 26-40 bits (2026-11-14/15) — `[pcc:unverified]`

Protocol: [`PROTOCOL.md`](PROTOCOL.md), committed in `a8cc24a6fe` before any row. Runner `run.sh`, analysis `analyse.py` (as the protocol fixes it), graph `plot.py` → `rho_vs_bsgs.{svg,png,pdf}`, raw rows `rows/`, summary `summary.json`. Bench record `benchmarks/records/20261114T2202Z-j0-multirho-vs-bsgs-interleaved.json`.

Arms: the batched folded multi-target rho with Kuhn-Struik restarts and the dp rule `DPRULE=ks DPDIV=1.5` (PR #4144), the folded multi-target BSGS at `MC=1` (its author's tuning), and the rho run again as an A/A arm. Six rounds per cell; each round plants fresh targets (`INST0 = 100 + r`, 2 curves x 2 instances) shared by all three arms, with the arm order rotating through all six permutations. Every row recovered all T logs; no round was excluded. Local 4-core container, single thread, not isolated.

| bits | T | rounds | rho ms/problem | BSGS ms/problem | rho/BSGS [95%] | A/A [95%] | verdict | rho steps/W |
|---|---|---|---|---|---|---|---|---|
| 26 | 20 | 6 | 4.2 | 1.9 | 1.40 [0.62, 2.32] | 1.19 [1.05, 1.45] | unresolved (A/A failed: unreliable) | 3.42 |
| 26 | 200 | 6 | 9.3 | 5.0 | 1.98 [1.80, 2.22] | 0.94 [0.86, 1.01] | BSGS faster | 2.05 |
| 26 | 1000 | 6 | 23.6 | 13.4 | 1.76 [1.68, 1.81] | 0.96 [0.89, 1.03] | BSGS faster | 1.66 |
| 28 | 200 | 6 | 15.8 | 12.2 | 1.31 [1.18, 1.46] | 1.05 [0.99, 1.13] | BSGS faster | 1.96 |
| 28 | 1000 | 6 | 34.4 | 27.2 | 1.27 [1.20, 1.34] | 0.98 [0.88, 1.09] | BSGS faster | 1.65 |
| 28 | 3000 | 6 | 88.0 | 62.3 | 1.43 [1.35, 1.54] | 1.03 [0.96, 1.11] | BSGS faster | 1.45 |
| 30 | 200 | 6 | 26.4 | 27.4 | 0.97 [0.88, 1.06] | 1.03 [0.95, 1.14] | unresolved | 1.96 |
| 30 | 1000 | 6 | 60.8 | 63.2 | 0.99 [0.92, 1.04] | 0.98 [0.88, 1.08] | unresolved | 1.65 |
| 30 | 3000 | 6 | 126.8 | 125.2 | 1.00 [0.93, 1.07] | 1.05 [0.97, 1.11] | unresolved | 1.46 |

![rho vs BSGS](rho_vs_bsgs.svg)

## Reading [pcc:unverified]

- **BSGS is faster at 26 and 28 bits** (rho takes 1.27x-1.98x its time, every interval above 1, A/A intervals contain 1). [pcc:unverified]
- **At 30 bits the two are tied**: rho/BSGS 0.97-1.00, every interval contains 1. The separate, non-interleaved runs in `../RESULTS_prime_fast_j0_sym.md` read rho as 0.83x-0.96x of BSGS there; the paired comparison does not confirm that lead, which is withdrawn.
- **26 bits, T = 20 is unreliable**: its A/A interval [1.05, 1.45] excludes 1 (runs of about 2-4 ms per problem, at the timer and scheduling noise floor), so no verdict.
- The ratio falls with size (about 1.8 at 26 bits, 1.3-1.4 at 28, 1.0 at 30). BSGS stores `m = sqrt(Tn/6)` table entries while rho stores only distinguished points, so a crossover in rho's favour at larger sizes is plausible, but nothing here measures it.
- Rho still takes 1.45x-2.05x of `W = sqrt(2nT/6)` steps (3.4x at 26 bits, T = 20, where 128 lanes in flight exceed W).

## Addendum A: 32 and 34 bits [pcc:unverified]

Frozen in `e8c0fd72da` before any 32/34-bit row (PROTOCOL.md, addendum A); same arms, settings, rounds and rule. Record `benchmarks/records/20261114T2217Z-j0-multirho-vs-bsgs-interleaved-32-34.json`. Every row recovered all T logs; no round excluded.

| bits | T | rounds | rho ms/problem | BSGS ms/problem | rho/BSGS [95%] | A/A [95%] | verdict | rho steps/W |
|---|---|---|---|---|---|---|---|---|
| 32 | 200 | 6 | 57.8 | 57.3 | 0.99 [0.91, 1.07] | 1.11 [1.08, 1.15] | unresolved (A/A failed: unreliable) | 1.97 |
| 32 | 1000 | 6 | 119.7 | 146.4 | 0.81 [0.75, 0.85] | 0.94 [0.89, 1.00] | rho faster (A/A failed: unreliable) | 1.63 |
| 32 | 3000 | 6 | 208.5 | 287.5 | 0.75 [0.71, 0.79] | 1.02 [0.98, 1.06] | rho faster | 1.48 |
| 34 | 200 | 6 | 115.2 | 140.7 | 0.81 [0.76, 0.87] | 0.91 [0.87, 0.95] | rho faster (A/A failed: unreliable) | 2.01 |
| 34 | 1000 | 6 | 208.4 | 349.2 | 0.61 [0.56, 0.64] | 0.97 [0.89, 1.03] | rho faster | 1.66 |
| 34 | 3000 | 6 | 361.8 | 611.9 | 0.59 [0.57, 0.62] | 0.99 [0.94, 1.03] | rho faster | 1.47 |

| bits | T | BSGS table entries `m` |
|---|---|---|
| 32 | 200 / 1000 / 3000 | 378k / 846k / 1.47M |
| 34 | 200 / 1000 / 3000 | 757k / 1.69M / 2.93M |

- **Rho is faster at 32 bits, T = 3000 (0.75 [0.71, 0.79]) and at 34 bits, T = 1000 and 3000 (0.61 and 0.59)**, each with an A/A interval containing 1. [pcc:unverified]
- 32/200, 32/1000 and 34/200 have A/A intervals excluding 1 (1.11, 0.94, 0.91: a 5-11% run-to-run drift on this host), so by the rule they carry no verdict, although 32/1000 and 34/200 point the same way (0.81 each).
- Across all 15 cells the ratio falls with size: BSGS ahead by 1.3-2.0x at 26-28 bits, level at 30, rho ahead by 1.3-1.7x at 32-34 bits once T ≥ 1000. A plausible mechanism, not measured here: the BSGS table (13-byte slots, `m` up to 2.9M) outgrows the cache, so each giant-step probe gets slower, while rho's state stays small. Memory traffic was not profiled.
- This changes nothing for index calculus: both generic methods are well ahead of it at every size measured.

## Addendum B: 36 bits [pcc:unverified]

Frozen in `f533b67618` before any 36-bit row (PROTOCOL.md, addendum B); same arms, settings, rounds, rule and binaries. Record `benchmarks/records/20261115T0416Z-j0-multirho-vs-bsgs-interleaved-36.json`. Every row recovered all T logs; no round excluded; every A/A interval contains 1.

| bits | T | rounds | rho ms/problem | BSGS ms/problem | rho/BSGS [95%] | A/A [95%] | verdict | rho steps/W |
|---|---|---|---|---|---|---|---|---|
| 36 | 200 | 6 | 223.3 | 266.8 | 0.85 [0.80, 0.90] | 1.00 [0.94, 1.06] | rho faster | 1.96 |
| 36 | 1000 | 6 | 422.2 | 650.7 | 0.66 [0.63, 0.68] | 0.98 [0.97, 1.00] | rho faster | 1.66 |
| 36 | 3000 | 6 | 697.0 | 1197.0 | 0.58 [0.56, 0.59] | 1.00 [0.96, 1.04] | rho faster | 1.48 |

BSGS table entries `m`: 1.51M / 3.38M / 5.86M at T = 200 / 1000 / 3000.

- **Rho is faster in all three 36-bit cells**: 0.85 [0.80, 0.90] at T = 200, 0.66 [0.63, 0.68] at T = 1000, 0.58 [0.56, 0.59] at T = 3000. [pcc:unverified]
- **The gap stops widening.** At T = 3000 the ratio is 0.75, 0.59, 0.58 at 32, 34, 36 bits; at T = 1000 it is 0.61 at 34 and 0.66 at 36; at T = 200, 0.81 at 34 and 0.85 at 36. Both methods are `O(sqrt(nT))` in group operations, so a constant ratio is what the asymptotics allow once the BSGS table is out of cache; the 26-30-bit BSGS lead and the 32-34-bit swing look like a constant-factor (memory-hierarchy) effect, not a change in growth rate. Not profiled. [pcc:unverified]
- Memory, not time, is the real limit for BSGS: its table grows as `sqrt(nT)` (5.9M entries here) while rho stores only distinguished points.

## Addendum C: 38 bits [pcc:unverified]

Frozen in `d28409195b` before any 38-bit row (PROTOCOL.md, addendum C); same arms, settings, rounds, rule and binaries. Record `benchmarks/records/20261115T0434Z-j0-multirho-vs-bsgs-interleaved-38.json`. Every row recovered all T logs; no round excluded.

| bits | T | rounds | rho ms/problem | BSGS ms/problem | rho/BSGS [95%] | A/A [95%] | verdict | rho steps/W |
|---|---|---|---|---|---|---|---|---|
| 38 | 200 | 6 | 424.6 | 584.6 | 0.72 [0.69, 0.77] | 0.97 [0.93, 1.01] | rho faster | 1.96 |
| 38 | 1000 | 6 | 857.9 | 1367.3 | 0.62 [0.60, 0.64] | 0.99 [0.97, 1.01] | rho faster | 1.69 |
| 38 | 3000 | 6 | 1258.5 | 2685.7 | 0.47 [0.45, 0.49] | 1.05 [1.02, 1.08] | rho faster (A/A failed: unreliable) | 1.47 |

BSGS table entries `m`: 3.03M / 6.77M / 11.7M at T = 200 / 1000 / 3000.

- **Rho is faster at T = 200 (0.72 [0.69, 0.77]) and T = 1000 (0.62 [0.60, 0.64])**, A/A intervals containing 1. [pcc:unverified]
- T = 3000 reads 0.47 [0.45, 0.49], but its A/A interval [1.02, 1.08] excludes 1, so by the rule it carries no verdict.
- **The 36-bit plateau reading is withdrawn.** From 36 to 38 bits the ratio moved again in rho's favour in every cell (0.85 → 0.72, 0.66 → 0.62, 0.58 → 0.47), so addendum B's "the gap stops widening" was a two-point reading that a third point does not support. The trend over 30-38 bits is a slow, uneven widening, which fits BSGS's per-probe cost rising as its table (now 11.7M entries) moves further down the memory hierarchy. Not profiled. [pcc:unverified]

## Addendum D: 40 bits [pcc:unverified]

Frozen in `3f2b071724` before any 40-bit row (PROTOCOL.md, addendum D); same arms, settings, rounds, rule and binaries. Record `benchmarks/records/20261115T0532Z-j0-multirho-vs-bsgs-interleaved-40.json`. Every row recovered all T logs; no round excluded; every A/A interval contains 1.

| bits | T | rounds | rho ms/problem | BSGS ms/problem | rho/BSGS [95%] | A/A [95%] | verdict | rho steps/W |
|---|---|---|---|---|---|---|---|---|
| 40 | 200 | 6 | 900.5 | 1288.6 | 0.71 [0.68, 0.74] | 0.99 [0.91, 1.06] | rho faster | 1.96 |
| 40 | 1000 | 6 | 1660.9 | 3066.7 | 0.55 [0.53, 0.57] | 0.99 [0.96, 1.03] | rho faster | 1.66 |
| 40 | 3000 | 6 | 2621.3 | 5896.9 | 0.45 [0.44, 0.46] | 1.01 [0.98, 1.04] | rho faster | 1.48 |

BSGS table entries `m`: 6.05M / 13.5M / 23.4M at T = 200 / 1000 / 3000.

- **Rho is faster in all three 40-bit cells**: 0.71 [0.68, 0.74], 0.55 [0.53, 0.57] and 0.45 [0.44, 0.46] at T = 200, 1000, 3000; at T = 3000 rho takes less than half of BSGS's time. [pcc:unverified]
- Against 38 bits the ratio held at T = 200 (0.72 → 0.71) and moved further in rho's favour at T = 1000 (0.62 → 0.55) and T = 3000 (0.47 → 0.45, where the 38-bit cell had failed A/A and this one passes).
- Over 30-40 bits the trend is consistent: the larger the BSGS table, the larger rho's lead. Still not profiled.

## Not done

No §5a band (wall time only), no isolated host, no sizes above 40 bits, no cache/memory profile, the S3 checkpoint and raw upload skipped (no AWS credentials in this container).
