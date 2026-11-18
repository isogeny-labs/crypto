# Held-out prediction for v2 (hky3, default) at m = 2, t = 12, c = 6

Written and committed **before** the t = 12 run. Instrument: `bin/pilot_v2`
(sha256 in README §1), `--engine hky3` default (deg_weight 1, interreduce 0.25,
adaptive batch 32..256, par_second on, bitmap on), prime 2013265921, Kummer
tower, draw 0, 4 shared vCPU, a neighbour job present.

## Model

`M = 2 κ n₊³` multiply-adds for the closure in which 1 enters (c = 6), from the
two v2 fit points measured here (default engine, draw 0):

| t | n₊ (part 0 cols) | M (measured, c = 6) | κ | wall s | ρ ns/muladd |
|--:|--:|--:|--:|--:|--:|
| 10 | 30,318 | 4.371e10 | 7.84e-4 | 15.3 | 0.35 |
| 11 | 55,144 | 3.215e11 | 9.59e-4 | 128.3 | 0.399 |

Local exponent of M in n₊: ln(3.215e11/4.371e10)/ln(55144/30318) = **3.34**.
κ rises with depth (as it did for v1), so the point estimate uses 3.34 and the
band spans exponent 3.2 to 3.7. n₊(t=12, c=6) = 95,175 (from v1 README §8);
n₊ ratio t12/t11 = 1.726.

## Prediction

| quantity | point | band |
|:--|--:|:--|
| answer | 1 enters at c = 6 | (d_F = 5 at t = 12) |
| multiply-adds, c = 6 | **2.0e12** | 1.8e12 – 2.5e12 |
| ρ (ns/muladd, 4 vCPU, neighbour) | 0.42 | 0.40 – 0.47 |
| wall, c = 6 closure | **850 s** | 720 – 1180 s |
| wall, whole cell (c ≤ 6) | **870 s** | 735 – 1200 s |
| v2/v1 muladds (v1 c6 = 3.69e12) | 0.54 | 0.49 – 0.68 |
| v2/v1 wall (v1 cell = 1722 s) | 0.51 | 0.43 – 0.70 |
| peak resident memory | 3.5 GB | 3 – 5 GB |
| stored words, c = 6 (peak) | 3.5e8 | 2.8e8 – 4.5e8 |

Refuted if: 1 does not enter at c = 6; or muladds outside 1.8e12–2.5e12; or
wall outside 720–1180 s (c = 6) once the recorded load is accounted for.
