# RESULT: LEAD-ICP-prime-ic-constant-engineering

## Outcome
- status: ran (single run, no retries). exit code 0, wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- run.py verdict field: "inconclusive". controls_failed = [].

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): max R_c < 1 for every c (about 0.77, 0.50, 0.31), spread of R_c across cells <= 0.05, hit sequences bit-identical across arms. Falsified if any R_c >= 1 or spread > 0.05. Inconclusive if any control fails or the A/A wall ratio leaves [0.8, 1.25].

Measured (9 cells: 12/14/16 bits x seeds 101/202/303):
- max R_5 = 0.7539, max R_10 = 0.4700, max R_20 = 0.2724.
- spread across cells = 0.0 at every c (identical in all cells).
- hits_identical true in 9/9; positive control found 9/9; batch1 == base counts 9/9 (null control, R = 1.000).
- A/A wall ratio per cell ranged 0.685 to 1.143. Cells 12-bit/101 (0.685) and 14-bit/101 (0.766) fall below 0.8; AA_wall_within_band = false.

Verdict: inconclusive, solely because the A/A wall-time band was violated. All counted-cost criteria met the prediction (R < 1, spread 0, ratios slightly below the analytic estimates). Null and positive controls passed. The A/A control did NOT pass.

## Reading
- The toy tests only the counted-cost model of batching. The ratios are essentially the analytic formula (6M+I/W vs 3M+I per add), so spread 0.0 is by construction, as PROTOCOL s6 warns.
- The inconclusive verdict is a wall-time noise artifact on a shared machine (2 s total, tiny timings), not a counted-cost failure. By the preregistered rule it stays inconclusive; I do not reinterpret it as consistent. Wall ratios of batched/base (0.40 to 0.62) are reported only, not decisive, and are Python-overhead dominated.
- Ratios are a bit below the quoted 0.77/0.50/0.31 (0.754/0.470/0.272), likely because the analytic figures were rounded or included partial-batch effects; the direction is consistent.
- No pair-table build, generator, LA filter, planner, native arithmetic or 20-bit holdout is exercised.

## Bearing on the sub-sqrt(p) question
constant factor only: the counted gain is a field-size-independent constant (spread 0), so it cannot change the exponent.

## Standing
[pcc:unverified], single local toy run, no independent replay.
