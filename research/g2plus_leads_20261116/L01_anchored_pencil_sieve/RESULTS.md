# L01 anchored pencil sieve: RESULTS (executor; all numbers `[pcc:unverified]`)

Counts only (FPM = mul + 10 inv, squarings in mul). Not a benchmark. One producer, one thread, Python counters.
Source: `raw/*.jsonl` (hashes in `raw/data.sha256`, commands in `raw/COMMANDS.log`), analysis in `raw/analysis.json` and `raw/summary.md`.
No interpretation is given here. The output of the pre-registered decision rule of `analyze.py` is in `raw/summary.md` (last section) and is not restated.
`frozen.sha256` re-verified after the runs: PROTOCOL.md OK, SPEC.md OK. Instances: p in {1031, 4099, 16411, 65537}, g in {2, 3}, curves 1-3, solver seeds 1-2; curve seeds are in each record (`curve_seed`).

## Gates

| gate | outcome |
|---|---|
| G1 unit tests (12 tests, SPEC 11 items 1-13) | `raw/unittest.log` ends `OK` (12 tests) |
| G2 V1/V2 failures over all relphase, A/A and lp2 records | 0 failures, 0 gate_failure records |
| G3 tiny DLP | pass at (2,131), (2,257), (3,67), (3,97): details below |
| G4 P_naive1 relation set == P_full tuple 0 | equal in all 48 relphase runs and both A/A reruns (`relset_equal` true in 50 of 50 records) |
| G5 bucket <= g+1, no S-root, zero remainders, count formula | no violation (max bucket 3 at g=2, 4 at g=3; no GateFailure) |
| G6 A/A rerun (g=2, p=4099, curve 1, seeds 1 and 2), 3 arms each | all integer/string fields and rows digests identical (6 of 6 records) |
| G7 P_full candidates per tuple / (p/(g+1)!) pooled per cell | g=2: 1.0038 (1031), 1.0078 (4099), 0.9985 (16411), 0.9941 (65537); g=3: 0.9928 (1031), 1.0602 (4099), 1.0071 (16411), 1.0085 (65537) |
| Positive control PC = (naive FPM per rel)/(P_full tuple-0 FPM per cand) | min over all 48 pairs: see PC columns below (all values > 5) |

G3 records (`raw/gate_*.jsonl`):

| g | p | curve seed | #J | h | r | n_cols | pencil rows (bad) | labelled rels (bad) | hom_rank | L determined (mismatch) | t planted | t recovered | pass |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 131 | 1 | 19816 | 8 | 2477 | 74 | 228 (0) | 50 (0) | 73 | 74 (0) | 29 | 29 | true |
| 2 | 257 | 1 | 66485 | 5 | 13297 | 129 | 418 (0) | 50 (0) | 128 | 129 (0) | 12740 | 12740 | true |
| 3 | 67 | 5 | 280848 | 48 | 5851 | 30 | 90 (0) | 50 (0) | 29 | 30 (0) | 131 | 131 | true |
| 3 | 97 | 1 | 899301 | 39 | 23059 | 47 | 145 (0) | 50 (0) | 46 | 47 (0) | 2239 | 2239 | true |

## Primary metric: FPM per distinct relation, relation phase (R1 = B_lab / P_full, I = 10)

Cell statistics over the 6 pairs (3 curves x 2 solver seeds); GM = geometric mean.

| g | p | P_full FPM/rel GM | B_lab FPM/rel GM | R1 GM | R1 MIN | R1 MAX | R2 GM (B_strong, g=2) | R3 GM (P charged adds/fd/bucket) | R4 I=1 GM | R4 I=30 GM | PC GM | PC MIN |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 1031 | 15.209 | 1042.663 | 68.558 | 29.415 | 225.847 | 12.224 | 20.369 | 53.253 | 101.828 | 71.237 | 69.515 |
| 2 | 4099 | 14.989 | 474.436 | 31.652 | 29.907 | 38.246 | 5.352 | 9.355 | 24.754 | 46.896 | 80.974 | 78.970 |
| 2 | 16411 | 15.050 | 582.912 | 38.732 | 32.801 | 48.852 | 6.020 | 11.402 | 30.933 | 56.038 | 95.252 | 93.666 |
| 2 | 65537 | 15.052 | 505.314 | 33.571 | 33.182 | 34.158 | 5.247 | 9.862 | 26.784 | 48.649 | 101.008 | 100.886 |
| 3 | 1031 | 62.041 | 2609.976 | 42.068 | 39.922 | 48.223 | n/a | 10.175 | 37.093 | 52.886 | 93.137 | 90.499 |
| 3 | 4099 | 58.069 | 2683.988 | 46.221 | 45.550 | 47.155 | n/a | 11.170 | 40.805 | 58.190 | 106.533 | 104.442 |
| 3 | 16411 | 59.824 | 2979.908 | 49.811 | 49.215 | 50.644 | n/a | 11.892 | 44.314 | 62.010 | 124.676 | 122.496 |
| 3 | 65537 | 59.732 | 3059.027 | 51.213 | 50.715 | 51.774 | n/a | 12.223 | 45.687 | 63.487 | 133.488 | 132.779 |

Noise floor (max/min of FPM per relation within an arm and cell over its 6 runs):
P_full g=2: 1.058, 1.046, 1.017, 1.009 (p = 1031, 4099, 16411, 65537); g=3: 1.122, 1.073, 1.054, 1.025.
B_lab g=2: 7.465, 1.277, 1.499, 1.032; g=3: 1.149, 1.069, 1.025, 1.013.
B_lab runs with duplicate relations (duplicates > 0) are listed below with their trial counts.

## Per-arm raw counts per cell

### Per-cell mean counts per run (6 runs per cell = 3 curves x 2 solver seeds); FPM/rel as geometric mean (min-max)

#### P_full

| g | p | n_cols | N_target | relations | dup | tuples | mul | inv | add | fd_add | bucket_ops | bucket_scan | FPM | FPM/rel GM (min-max) | row wt mean | max col wt | cols unhit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 1031 | 519.7 | 532.7 | 661.2 | 0.0 | 3.8 | 9988 | 7.7 | 4058 | 11845 | 3969 | 3952 | 10065 | 15.209 (14.716-15.567) | 4.997 | 181.5 | 3.5 |
| 2 | 4099 | 2064.3 | 2092.7 | 2524.5 | 0.0 | 3.7 | 37812 | 7.3 | 15185 | 45078 | 15101 | 15030 | 37885 | 14.989 (14.711-15.391) | 5.000 | 703.7 | 15.5 |
| 2 | 16411 | 8215.0 | 8304.7 | 10924.7 | 0.0 | 4.0 | 164331 | 8.0 | 65799 | 196920 | 65707 | 65644 | 164411 | 15.050 (14.894-15.151) | 5.000 | 2754.2 | 28.7 |
| 2 | 65537 | 32680.7 | 33015.0 | 43432.2 | 0.0 | 4.0 | 653649 | 8.0 | 261525 | 786432 | 261433 | 262148 | 653729 | 15.052 (14.977-15.113) | 5.000 | 10906.3 | 133.7 |
| 3 | 1031 | 519.7 | 532.7 | 547.3 | 0.0 | 12.8 | 33740 | 25.7 | 13966 | 66092 | 13273 | 13231 | 33997 | 62.041 (57.609-64.612) | 6.996 | 56.0 | 5.2 |
| 3 | 4099 | 2093.7 | 2122.3 | 2203.0 | 0.0 | 12.2 | 127722 | 24.3 | 51534 | 249295 | 50877 | 49871 | 127965 | 58.069 (56.991-61.121) | 6.998 | 202.2 | 20.2 |
| 3 | 16411 | 8234.7 | 8324.3 | 8608.0 | 0.0 | 12.5 | 514929 | 25.0 | 206434 | 1025625 | 205759 | 205138 | 515179 | 59.824 (58.309-61.469) | 7.000 | 727.2 | 84.8 |
| 3 | 65537 | 32899.0 | 33235.7 | 34423.3 | 0.0 | 12.5 | 2056358 | 25.0 | 822998 | 4096000 | 822323 | 819212 | 2056608 | 59.732 (59.247-60.703) | 7.000 | 2839.2 | 346.3 |

#### B_lab

| g | p | relations | duplicates (sum of 6) | trials | candidates | splits | walk_add | restarts | cantor_mul | cantor_inv | rel_mul | rel_inv | FPM | FPM/rel GM (min-max) | strong_fpm/rel GM (g=2) | row wt mean | max col wt | cols unhit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 1031 | 532.7 | 7237 | 3520 | 3506 | 1739 | 3518 | 1.7 | 318998 | 14456 | 260906 | 5195 | 776409 | 1042.663 (445.229-3323.650) | 185.902 | 1.998 | 7.3 | 69.0 |
| 2 | 4099 | 2092.7 | 568 | 4304 | 4302 | 2187 | 4302 | 1.8 | 391424 | 17730 | 362201 | 6544 | 996364 | 474.436 (446.448-570.109) | 80.218 | 2.000 | 8.2 | 281.7 |
| 2 | 16411 | 8304.7 | 9299 | 19567 | 19566 | 9854 | 19562 | 5.5 | 1767190 | 80105 | 2031152 | 29582 | 4895211 | 582.912 (493.704-740.148) | 90.598 | 2.000 | 9.0 | 1087.7 |
| 2 | 65537 | 33015.0 | 2298 | 67076 | 67075 | 33398 | 67060 | 17.0 | 6063328 | 274817 | 6871231 | 100134 | 16684066 | 505.314 (496.982-512.972) | 78.975 | 2.000 | 10.0 | 4336.5 |
| 3 | 1031 | 532.7 | 0 | 3154 | 3152 | 533 | 3153 | 1.0 | 707282 | 16082 | 492027 | 3131 | 1391441 | 2609.976 (2418.303-2778.091) | n/a | 2.997 | 9.5 | 20.3 |
| 3 | 4099 | 2122.3 | 0 | 12182 | 12179 | 2122 | 12178 | 3.3 | 2758293 | 62763 | 2187308 | 12362 | 5696848 | 2683.988 (2605.209-2784.096) | n/a | 3.000 | 10.3 | 102.0 |
| 3 | 16411 | 8324.3 | 0 | 49643 | 49640 | 8324 | 49630 | 13.0 | 11277616 | 256599 | 10475730 | 48638 | 24805709 | 2979.908 (2952.993-3025.206) | n/a | 3.000 | 11.7 | 398.5 |
| 3 | 65537 | 33235.7 | 0 | 197983 | 197979 | 33236 | 197934 | 49.0 | 45098654 | 1025898 | 44371143 | 193971 | 101668484 | 3059.027 (3037.809-3078.561) | n/a | 3.000 | 12.2 | 1590.2 |

#### P_naive1 (first tuple only, every lambda in F_p)

| g | p | relations (first tuple) | mul | inv | FPM | FPM/rel GM | relset_equal (all 6) |
|---|---|---|---|---|---|---|---|
| 2 | 1031 | 170.8 | 177294 | 994.0 | 187234 | 1097.357 | True |
| 2 | 4099 | 686.0 | 798560 | 3969.3 | 838254 | 1222.323 | True |
| 2 | 16411 | 2731.5 | 3755692 | 15953.5 | 3915227 | 1433.458 | True |
| 2 | 65537 | 10860.0 | 15874410 | 63356.5 | 16507976 | 1520.088 | True |
| 3 | 1031 | 43.5 | 243004 | 381.5 | 246819 | 5712.894 | True |
| 3 | 4099 | 179.5 | 1104974 | 1571.5 | 1120688 | 6245.698 | True |
| 3 | 16411 | 699.7 | 5077606 | 6181.2 | 5139418 | 7347.891 | True |
| 3 | 65537 | 2763.2 | 21723732 | 24141.0 | 21965142 | 7950.865 | True |

#### B_lab runs with duplicates > 0 (all cells)

- g=2 p=1031 curve 1 seed 2: relations 546, duplicates 3511, trials 8264, restarts 3, FPM/rel 3323.7
- g=2 p=1031 curve 3 seed 1: relations 535, duplicates 1695, trials 4623, restarts 2, FPM/rel 1892.1
- g=2 p=1031 curve 3 seed 2: relations 535, duplicates 2031, trials 4930, restarts 2, FPM/rel 2055.8
- g=2 p=16411 curve 1 seed 1: relations 8257, duplicates 1731, trials 19797, restarts 5, FPM/rel 601.2
- g=2 p=16411 curve 1 seed 2: relations 8257, duplicates 213, trials 16992, restarts 5, FPM/rel 514.0
- g=2 p=16411 curve 2 seed 1: relations 8305, duplicates 4130, trials 24585, restarts 7, FPM/rel 740.1
- g=2 p=16411 curve 2 seed 2: relations 8305, duplicates 2770, trials 22097, restarts 6, FPM/rel 668.7
- g=2 p=16411 curve 3 seed 2: relations 8352, duplicates 455, trials 17356, restarts 5, FPM/rel 519.6
- g=2 p=4099 curve 3 seed 1: relations 2095, duplicates 568, trials 5121, restarts 2, FPM/rel 570.1
- g=2 p=65537 curve 1 seed 2: relations 33089, duplicates 154, trials 66417, restarts 17, FPM/rel 500.6
- g=2 p=65537 curve 2 seed 2: relations 32902, duplicates 392, trials 67184, restarts 17, FPM/rel 507.8
- g=2 p=65537 curve 3 seed 1: relations 33054, duplicates 1001, trials 68283, restarts 17, FPM/rel 513.0
- g=2 p=65537 curve 3 seed 2: relations 33054, duplicates 751, trials 67986, restarts 17, FPM/rel 512.2

#### Split-rate check reported only: B_lab splits/candidates divided by C(n,g) 2^g / p^g (mean over 6)

- g=2 p=1031: 0.9661
- g=2 p=4099: 1.0022
- g=2 p=16411: 1.0050
- g=2 p=65537: 1.0012
- g=3 p=1031: 0.9973
- g=3 p=4099: 0.9826
- g=3 p=16411: 0.9960
- g=3 p=65537: 0.9954

## H4: counted sparse elimination on the first N_target rows (seed 1; q = 2^61 - 1; `raw/la_*.jsonl`)

| g | p | curve | P_full LA mul | B_lab LA mul | P/B mul | P inv | B inv | P peak nnz | B peak nnz | status | net FPM (relation-phase saving minus g^2 x LA difference) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 1031 | 1 | 56191 | 975 | 57.632 | 526 | 447 | 2727 | 1090 | ok / ok | 10806.7 |
| 2 | 1031 | 2 | 47875 | 853 | 56.125 | 494 | 422 | 2583 | 1032 | ok / ok | 46120.8 |
| 2 | 1031 | 3 | 48191 | 1157 | 41.652 | 506 | 449 | 2675 | 1070 | ok / ok | 813735.1 |
| 2 | 4099 | 1 | 2075740 | 4468 | 464.579 | 2057 | 1783 | 31333 | 4264 | ok / ok | -7370665.5 |
| 2 | 4099 | 2 | 1422167 | 4115 | 345.606 | 1988 | 1691 | 23749 | 4098 | ok / ok | -4764314.1 |
| 2 | 4099 | 3 | 1590583 | 4842 | 328.497 | 2029 | 1728 | 25115 | 4190 | ok / ok | -5191854.1 |
| 3 | 1031 | 1 | 322305 | 16599 | 19.417 | 515 | 500 | 8402 | 1604 | ok / ok | -1490892.0 |
| 3 | 1031 | 2 | 378665 | 22787 | 16.618 | 518 | 494 | 9162 | 1605 | ok / ok | -1868146.4 |
| 3 | 1031 | 3 | 372862 | 23091 | 16.148 | 509 | 485 | 9360 | 1582 | ok / ok | -1713684.4 |

Net is positive in the 3 cells at g=2, p=1031 and negative in the other 6 cells.

## H5: 2LP arms, FPM per graph cycle (seed 1; `raw/lp2_*.jsonl`; full table in `raw/summary.md`)

B_2lp / P_2lp FPM-per-cycle ratio per curve (curves 1, 2, 3) and GM: g=2 p=4099: 7.235, 6.895, 7.227 (GM 7.117); g=2 p=16411: 8.257, 9.537, 8.192 (GM 8.641);
g=3 p=4099: 27.131, 28.813, 27.887 (GM 27.935); g=3 p=16411: 39.630, 41.145, 39.107 (GM 39.952).
P_2lp FPM per cycle: g=2 p=4099: 696.4, 701.7, 683.3; p=16411: 1250.5, 1286.9, 1264.0; g=3 p=4099: 610.1, 593.7, 584.4; p=16411: 973.7, 965.9, 980.6.
B_2lp FPM per cycle: g=2 p=4099: 5038.3, 4838.6, 4937.9; p=16411: 10324.9, 12272.9, 10355.3; g=3 p=4099: 16553.2, 17106.8, 16297.6; p=16411: 38587.1, 39742.1, 38347.7.
All 24 lp2 records have status ok. P_2lp `repeat_rejects` total 15 (DEVIATIONS item 1).

## Plot

`plot.svg`: FPM per distinct relation (geometric mean over 6 runs) per arm versus p, g=2 (P_full, B_lab, B_strong) and g=3 (P_full, B_lab).

## Deviations

See `DEVIATIONS.md` (10 items; none changes a threshold, arm, seed rule, count rule or decision rule). Implementation notes: `IMPLEMENTATION_NOTES.md`.
