# T13: the t = 4 candidate end to end, plain and with trace replay (results, 2026-11-04)

**Class.** End-to-end ecbench comparison in counted units (GAE plus unpriced native work at
w = 0, 0.1, 1) on the k = 20 Curve25519 miniature. Algebraic-track component result. Not a
leaderboard row, not a `benchmarks/` row. Rho remains ahead.

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen at `b86032edee` before any data. Instrument:
`/home/user/ctarget/release/ecbench`, hash in `ecbench/binary.sha256`. Execution: 16 one-target
chunk sessions (`scripts/chunked_run.py`, the T12 copy), pooled band (`ecbench band --dir`),
paired ratios (`scripts/pooled_compare.py`, the T12 copy), one `verify --replay 1` per chunk.
Every number is read from `ecbench/{band_pooled.txt, pooled_compare.jsonl, verify/}` or the
chunk `records.jsonl`.

## Headline

- **t = 4 with trace replay costs 0.284× the T12 winner at w ≥ 0.1.** The pooled paired ratio
  S(t4-trace)/S(t5 winner) is **0.2837 [0.2413, 0.3335]** at w = 0.1 and 0.2834 at w = 1, with
  16 of 16 workloads won. That is a 3.5× measured reduction in counted F4 work per solve on the
  algebraic track, against T12's incumbent arm, on the same curve and targets.
- The two ingredients compose. The t = 4 base alone gives **0.598 [0.509, 0.704]**, against the
  model's 0.49. Trace replay on that base gives **0.4740**, matching L12's 0.474 at t = 5.
- **At w = 0 the order flips, as predicted.** The t = 4 arms cost **1.567× [1.359, 1.807]** the
  winner, against the model's 1.47, because 8 columns cover 52 points against 22 columns
  covering 210.
- 6 of 6 predictions held. R2 held at 0.598 against its 0.6 threshold; the point prediction of
  0.49 missed.

## Band (pooled, 16 chunks × 2 rounds = 32 runs per arm, √r = 362.22)

| arm | verified | S(w = 0) | S(w = 0.1) | S(w = 1) |
|:--|--:|--:|--:|--:|
| pkm-iso-t5-g4-s1307 (T12 winner) | 32/32 | 28.49 | 110,556.6 | 1,105,309.9 |
| pkm-iso-t4-g3-s1193 | 32/32 | 44.64 | 66,158.6 | 661,184.7 |
| pkm-iso-t4-g3-s1193-trace | 32/32 | 44.64 | 31,361.0 | 313,208.6 |
| proj-a | 32/32 | 11.88 | 12.75 | 20.60 |
| rho-neg | 32/32 | 1.600 | 1.716 | 2.764 |

## Predictions (PROTOCOL.md)

Ratios are pooled paired ratios with 95 % intervals (K = 16 workloads, 32 pairs).

| | rule | outcome |
|:--|:--|:--|
| R1 | every arm 32/32 verified; every chunk's replay audit 0 problems | **held**: 32/32 verified in all five arms; 16/16 chunk receipts `ok: true`, 0 problems, every replay `identical`. The 16 replays cover rho-neg 4, proj-a 3, t4 plain 5 and the winner 4 runs; **no t4-trace run was replayed** |
| R2 | S(t4)/S(winner) ≤ 0.6 at w = 0.1 and 1, upper 95 % < 1 (point 0.49) | **held**: 0.59841 [0.50903, 0.70350] at w = 0.1, 0.59819 [0.50883, 0.70324] at w = 1; 16/16 workloads. The point 0.49 missed: measured / model = 1.22 |
| R3 | S(t4)/S(winner) ≥ 1.2 at w = 0 (point 1.47) | **held**: 1.5668 [1.3586, 1.8070]; the winner is cheaper on 14 of 16 workloads |
| R4 | S(t4-trace)/S(t4) ≤ 0.6 at w = 0.1 and 1, upper < 1 (point 0.47–0.57) | **held**: 0.47403 [0.47402, 0.47404] at w = 0.1, 0.47371 at w = 1; 16/16 |
| R5 | per pair: total_gae and recovered scalar identical between the two t = 4 arms; w = 0 ratio 1.000 | **held**: algorithm seeds paired in 32/32 pairs; total_gae and recovered scalar equal in 32/32; w = 0 ratio 1.000 [1.000, 1.000] |
| R6 | every PKM arm behind proj-a and rho-neg at w = 0.1 and 1 (band ranks) | **held**: "proj-a < arm" and "rho-neg < arm" for all three PKM arms, across the band |

## Distance remaining (pooled ratio arm ÷ reference, 95 % interval)

| arm | vs | w = 0 | w = 0.1 | w = 1 |
|:--|:--|--:|--:|--:|
| t4-trace | rho-neg | 27.9× [23.9, 32.6] | **1.83·10⁴× [1.55·10⁴, 2.15·10⁴]** | 1.13·10⁵× [9.5·10⁴, 1.35·10⁵] |
| t4 (plain) | rho-neg | 27.9× | 3.85·10⁴× | 2.39·10⁵× |
| t4 (plain) | proj-a | 3.76× | 5.19·10³× | 3.21·10⁴× |

The t4-trace arm against proj-a follows from the same ratios: about 2.5·10³× at w = 0.1.
For comparison, T12's winner was 6.06·10⁴× behind rho-neg at w = 0.1. Rho remains ahead.

## Costs not in the band

- **Trace learning.** ecbench does not price `pkm_trace_learn_muladds`. It is 17,592 mul-adds per
  run, one plain F4 solve per branch system, 562,944 in all. Charging it by hand moves
  S(t4-trace)/S(t4) from 0.47403 to 0.47410 at w = 0.1 and from 0.47371 to 0.47378 at w = 1.
- **Trace fallbacks.** 292 fallbacks to plain F4 over 3,462,640 replays, 8.4·10⁻⁵, summed over
  32 runs. Their plain F4 work is inside the measured S.
- **Seed search.** Choosing seed 1193 among 2,000 tower seeds at t = 4 took 4.00·10⁶ group
  operations (`curve25519_isogeny_seed_20261103/analysis/summary.json`). That is 1.10·10⁴ in S
  when charged to a single target, and it vanishes when amortised over targets on the same curve.
  The winner's seed 1307 carries an analogous t = 5 search. Charged to one target, the search
  dominates every PKM arm at w = 0.

## Reading

- The cost model E = columns × r × mpt / covered ranked t = 4 correctly at both ends of the band.
  It underestimated the t = 4 arm at w > 0 by 22 %, so measured S per solve is not exactly
  proportional to mpt × columns / covered.
- L10 found t = 4 optimal at every k on the engine. This run is its end-to-end confirmation at
  k = 20, and trace replay carries over unchanged to a different (t, G).
- The new best algebraic arm at k = 20 is t = 4, G = 3, seed 1193 with trace replay. It is still
  1.83·10⁴× behind rho-neg at w = 0.1, and 27.9× behind at w = 0, where T12's winner is 17.8×
  behind. L13 shows the base, not the solver, sets the w = 0 floor.

## Run record and deviations

- Chunks ran from the T13 launch until 16:17:33Z (`ecbench/chunks/driver.log`), relaunched after
  container restarts. Chunk c07 was rerun from scratch after the 15:27Z restart, and no partial
  chunk is cited. From 15:29Z the L13 chunks ran at the same time. S is a counted unit, so this
  changes no number; no wall-time claim is made.
- Conductor and the AWS CLI are absent in this container: no S3 resume checkpoint was pushed,
  and the run was local rather than on EKS.

## Files

`PROTOCOL.md` (frozen). `scripts/` (`run.sh`, `chunked_run.py`, `pooled_compare.py`).
`ecbench/spec.json`, `plan.txt`, `binary.sha256`; `ecbench/chunks/c00…c15` (sessions, `specs/`,
`driver.log`); `ecbench/band_pooled.{txt,json}`, `pooled_compare.jsonl`, `verify/cNN.{json,txt}`;
`ecbench/SHA256SUMS`.
