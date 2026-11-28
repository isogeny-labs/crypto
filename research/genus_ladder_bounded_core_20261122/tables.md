# Tables (every figure `[pcc:unverified]`; 3 curves x 2 seeds = 6 runs per arm per prime; cell mean = mean over curves of the mean over seeds)

FPM3 = cantor_mul + 10 cantor_inv + rel_mul + 3 la_mul; relation phase = cantor_mul + 10 cantor_inv + rel_mul. Weights: mean row weight of the peeled core (before the step), of the core the step hands on (after the step), and of the merged system SGE hands to Wiedemann (nnz_out / rows_out). Ratios to plain: ratio of cell means, 95% hierarchical bootstrap (same curve and seed resample for every arm).


## p = 32771

| arm | runs | relations | core rows x cols (peeled) | rows x cols after step | w before | w after step | rows x cols after SGE | nnz after SGE | w after SGE | nnz / col | la_mul | relation phase | FPM3 | FPM3 / plain [95%] | la_mul / plain [95%] | probes | E | capped |
|---|--:|--:|--|--|--:|--:|--|--:|--:|--:|--:|--:|--:|--|--|--:|--:|--:|
| unmerged | 6 | 1.33e+04 | 2e+03 x 1.93e+03 | - | - | - | - | - | - | - | 4.643e+07 | 1.239e+08 | 2.632e+08 | - | - | - | - | - |
| plain SGE(8,3) | 6 | 1.33e+04 | 2e+03 x 1.93e+03 | - | 9.55 | - | 1.22e+03 x 1.15e+03 | 2.47e+04 | 20.28 | 21.41 | 1.798e+07 | 1.239e+08 | 1.778e+08 | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | - | - | - |
| ex(0.3,0.01) (R4) | 6 | 1.73e+04 | 4.71e+03 x 2.04e+03 | 1.87e+03 x 1.84e+03 | 9.14 | 7.13 | 902 x 865 | 1.7e+04 | 18.85 | 19.66 | 1.030e+07 | 1.607e+08 | 1.916e+08 | 1.077 [1.073, 1.082] | 0.573 [0.555, 0.591] | - | - | - |
| bd(8) cover 2 (spec) | 6 | 1.6e+04 | 3.74e+03 x 2.03e+03 | 2.06e+03 x 2.03e+03 | 9.29 | 7.69 | 1e+03 x 976 | 1.96e+04 | 19.54 | 20.11 | 1.303e+07 | 1.486e+08 | 1.877e+08 | 1.056 [1.052, 1.059] | 0.725 [0.711, 0.741] | 1.0 | 0.20 | 0.00 |
| bd(8) cover 1 | 6 | 1.6e+04 | 3.74e+03 x 2.03e+03 | 1.88e+03 x 1.84e+03 | 9.29 | 7.53 | 946 x 912 | 1.81e+04 | 19.17 | 19.89 | 1.132e+07 | 1.486e+08 | 1.826e+08 | 1.027 [1.025, 1.028] | 0.630 [0.619, 0.641] | 1.0 | 0.20 | 0.00 |

(The `w before` of the plain row is the weight of the cores handed to SGE, nnz_in / rows_in. la_mul includes graph, peel, selection (zero multiplications), SGE and Wiedemann; the step's own additions are `la_sel_add`: ex(0.3,0.01) (R4) 8.76e+04, bd(8) cover 2 (spec) 9.06e+04, bd(8) cover 1 1.259e+05)


## p = 65537

| arm | runs | relations | core rows x cols (peeled) | rows x cols after step | w before | w after step | rows x cols after SGE | nnz after SGE | w after SGE | nnz / col | la_mul | relation phase | FPM3 | FPM3 / plain [95%] | la_mul / plain [95%] | probes | E | capped |
|---|--:|--:|--|--|--:|--:|--|--:|--:|--:|--:|--:|--:|--|--|--:|--:|--:|
| unmerged | 6 | 2.6e+04 | 3.2e+03 x 3.12e+03 | - | - | - | - | - | - | - | 1.198e+08 | 3.082e+08 | 6.677e+08 | - | - | - | - | - |
| plain SGE(8,3) | 6 | 2.6e+04 | 3.2e+03 x 3.12e+03 | - | 10.47 | - | 2.07e+03 x 1.99e+03 | 4.4e+04 | 21.27 | 22.10 | 5.187e+07 | 3.082e+08 | 4.638e+08 | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | - | - | - |
| ex(0.3,0.01) (R4) | 6 | 3.37e+04 | 8.12e+03 x 3.25e+03 | 3.01e+03 x 2.96e+03 | 9.91 | 7.59 | 1.55e+03 x 1.5e+03 | 3e+04 | 19.35 | 20.02 | 2.946e+07 | 4.005e+08 | 4.889e+08 | 1.054 [1.050, 1.061] | 0.568 [0.560, 0.579] | - | - | - |
| bd(8) cover 2 (spec) | 6 | 3.37e+04 | 8.12e+03 x 3.25e+03 | 3.29e+03 x 3.25e+03 | 9.91 | 7.73 | 1.64e+03 x 1.6e+03 | 3.21e+04 | 19.61 | 20.13 | 3.360e+07 | 4.005e+08 | 5.013e+08 | 1.081 [1.077, 1.086] | 0.648 [0.639, 0.656] | 2.0 | 0.30 | 0.00 |
| bd(8) cover 1 | 6 | 3.37e+04 | 8.12e+03 x 3.25e+03 | 3.01e+03 x 2.96e+03 | 9.91 | 7.59 | 1.55e+03 x 1.5e+03 | 3e+04 | 19.35 | 20.02 | 2.946e+07 | 4.005e+08 | 4.889e+08 | 1.054 [1.050, 1.061] | 0.568 [0.560, 0.579] | 2.0 | 0.30 | 0.00 |

(The `w before` of the plain row is the weight of the cores handed to SGE, nnz_in / rows_in. la_mul includes graph, peel, selection (zero multiplications), SGE and Wiedemann; the step's own additions are `la_sel_add`: ex(0.3,0.01) (R4) 1.633e+05, bd(8) cover 2 (spec) 3.042e+05, bd(8) cover 1 3.918e+05)


## p = 131101

| arm | runs | relations | core rows x cols (peeled) | rows x cols after step | w before | w after step | rows x cols after SGE | nnz after SGE | w after SGE | nnz / col | la_mul | relation phase | FPM3 | FPM3 / plain [95%] | la_mul / plain [95%] | probes | E | capped |
|---|--:|--:|--|--|--:|--:|--|--:|--:|--:|--:|--:|--:|--|--|--:|--:|--:|
| unmerged | 6 | 5.06e+04 | 5.12e+03 x 5.02e+03 | - | - | - | - | - | - | - | 3.083e+08 | 7.642e+08 | 1.689e+09 | - | - | - | - | - |
| plain SGE(8,3) | 6 | 5.06e+04 | 5.12e+03 x 5.02e+03 | - | 11.58 | - | 3.53e+03 x 3.43e+03 | 7.86e+04 | 22.27 | 22.90 | 1.496e+08 | 7.642e+08 | 1.213e+09 | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | - | - | - |
| ex(0.3,0.01) (R4) | 6 | 6.57e+04 | 1.41e+04 x 5.16e+03 | 4.86e+03 x 4.8e+03 | 10.71 | 8.11 | 2.65e+03 x 2.58e+03 | 5.29e+04 | 19.95 | 20.49 | 8.536e+07 | 9.923e+08 | 1.248e+09 | 1.029 [1.020, 1.035] | 0.570 [0.558, 0.582] | - | - | - |
| bd(8) cover 2 (spec) | 6 | 7.08e+04 | 1.77e+04 x 5.16e+03 | 5.22e+03 x 5.16e+03 | 10.44 | 7.78 | 2.64e+03 x 2.58e+03 | 5.26e+04 | 19.91 | 20.39 | 8.548e+07 | 1.069e+09 | 1.325e+09 | 1.093 [1.084, 1.099] | 0.571 [0.557, 0.582] | 3.0 | 0.40 | 0.00 |
| bd(8) cover 1 | 6 | 7.08e+04 | 1.77e+04 x 5.16e+03 | 4.82e+03 x 4.74e+03 | 10.44 | 7.67 | 2.52e+03 x 2.44e+03 | 4.99e+04 | 19.82 | 20.41 | 7.680e+07 | 1.069e+09 | 1.299e+09 | 1.071 [1.061, 1.075] | 0.513 [0.494, 0.522] | 3.0 | 0.40 | 0.00 |

(The `w before` of the plain row is the weight of the cores handed to SGE, nnz_in / rows_in. la_mul includes graph, peel, selection (zero multiplications), SGE and Wiedemann; the step's own additions are `la_sel_add`: ex(0.3,0.01) (R4) 3.047e+05, bd(8) cover 2 (spec) 9.032e+05, bd(8) cover 1 1.094e+06)


## p = 262147

| arm | runs | relations | core rows x cols (peeled) | rows x cols after step | w before | w after step | rows x cols after SGE | nnz after SGE | w after SGE | nnz / col | la_mul | relation phase | FPM3 | FPM3 / plain [95%] | la_mul / plain [95%] | probes | E | capped |
|---|--:|--:|--|--|--:|--:|--|--:|--:|--:|--:|--:|--:|--|--|--:|--:|--:|
| unmerged | 6 | 9.85e+04 | 8.16e+03 x 8.03e+03 | - | - | - | - | - | - | - | 7.866e+08 | 1.847e+09 | 4.207e+09 | - | - | - | - | - |
| plain SGE(8,3) | 6 | 9.85e+04 | 8.16e+03 x 8.03e+03 | - | 12.77 | - | 5.97e+03 x 5.85e+03 | 1.373e+05 | 22.98 | 23.48 | 4.274e+08 | 1.847e+09 | 3.129e+09 | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] | - | - | - |
| ex(0.3,0.01) (R4) | 6 | 1.280e+05 | 2.44e+04 x 8.19e+03 | 7.79e+03 x 7.69e+03 | 11.57 | 8.57 | 4.48e+03 x 4.37e+03 | 9.15e+04 | 20.43 | 20.91 | 2.405e+08 | 2.402e+09 | 3.123e+09 | 0.998 [0.994, 1.002] | 0.563 [0.556, 0.567] | - | - | - |
| bd(8) cover 2 (spec) | 6 | 1.477e+05 | 3.83e+04 x 8.19e+03 | 8.28e+03 x 8.19e+03 | 10.92 | 7.84 | 4.3e+03 x 4.21e+03 | 8.5e+04 | 19.77 | 20.20 | 2.237e+08 | 2.772e+09 | 3.443e+09 | 1.100 [1.096, 1.104] | 0.523 [0.521, 0.526] | 4.0 | 0.50 | 0.00 |
| bd(8) cover 1 | 6 | 1.477e+05 | 3.83e+04 x 8.19e+03 | 7.71e+03 x 7.61e+03 | 10.92 | 7.76 | 4.12e+03 x 4.02e+03 | 8.08e+04 | 19.60 | 20.11 | 2.044e+08 | 2.772e+09 | 3.385e+09 | 1.082 [1.077, 1.086] | 0.478 [0.472, 0.484] | 4.0 | 0.50 | 0.00 |

(The `w before` of the plain row is the weight of the cores handed to SGE, nnz_in / rows_in. la_mul includes graph, peel, selection (zero multiplications), SGE and Wiedemann; the step's own additions are `la_sel_add`: ex(0.3,0.01) (R4) 5.687e+05, bd(8) cover 2 (spec) 2.483e+06, bd(8) cover 1 2.873e+06)


## Exponents in p over 32771, 65537, 131101, 262147 (least squares of log cell mean on log p; 95% bootstrap over curves and seeds)

| arm | la_mul | FPM3 | relation phase | FPM1 | FPM9 |
|---|--|--|--|--|--|
| unmerged | 1.361 [1.357, 1.365] | 1.333 [1.332, 1.335] | 1.300 [1.297, 1.304] | 1.318 [1.316, 1.320] | 1.348 [1.345, 1.351] |
| plain SGE(8,3) | 1.524 [1.516, 1.533] | 1.380 [1.377, 1.383] | 1.300 [1.297, 1.304] | 1.335 [1.333, 1.338] | 1.440 [1.435, 1.444] |
| ex(0.3,0.01) (R4) | 1.517 [1.499, 1.531] | 1.343 [1.341, 1.345] | 1.301 [1.299, 1.304] | 1.318 [1.316, 1.320] | 1.392 [1.386, 1.397] |
| bd(8) cover 2 (spec) | 1.365 [1.351, 1.378] | 1.399 [1.396, 1.402] | 1.408 [1.405, 1.411] | 1.405 [1.402, 1.407] | 1.390 [1.384, 1.394] |
| bd(8) cover 1 | 1.390 [1.376, 1.402] | 1.405 [1.402, 1.407] | 1.408 [1.405, 1.411] | 1.407 [1.404, 1.409] | 1.401 [1.396, 1.405] |

Paired difference of exponents against plain (same resamples):

| arm | la_mul | FPM3 |
|---|--|--|
| ex(0.3,0.01) (R4) | -0.007 [-0.022, 0.007] | -0.037 [-0.039, -0.034] |
| bd(8) cover 2 (spec) | -0.159 [-0.169, -0.150] | 0.019 [0.017, 0.022] |
| bd(8) cover 1 | -0.134 [-0.145, -0.123] | 0.025 [0.023, 0.027] |

## Mean row weight after SGE across the primes (B2)

| arm | 32771 | 65537 | 131101 | 262147 | max / min [95%] |
|---|--:|--:|--:|--:|--|
| plain SGE(8,3) | 20.28 | 21.27 | 22.27 | 22.98 | 1.133 [1.116, 1.150] |
| ex(0.3,0.01) (R4) | 18.85 | 19.35 | 19.95 | 20.43 | 1.084 [1.070, 1.099] |
| bd(8) cover 2 (spec) | 19.54 | 19.61 | 19.91 | 19.77 | 1.019 [1.011, 1.034] |
| bd(8) cover 1 | 19.17 | 19.35 | 19.82 | 19.60 | 1.034 [1.020, 1.055] |

## Scorecard

B1 (plain rows equal the earlier studies; all runs certified): **holds** (all certified: True; rows compared 48; mismatches 0; instance lines equal: True)

| arm | B2 w spread max/min (<= 1.10) | B3 la_mul exponent (<= 1.42, upper < 1.47) | B4(i) FPM3/plain at largest p (<= 1) | B4(ii) FPM3 exponent vs plain |
|---|--|--|--|--|
| bd(8) cover 2 (spec) | 1.019: holds | 1.365 (upper 1.378): holds | 1.100: FAILS | 1.399 vs 1.380: FAILS |
| bd(8) cover 1 | 1.034: holds | 1.390 (upper 1.402): holds | 1.082: FAILS | 1.405 vs 1.380: FAILS |
