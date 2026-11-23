## Cells (k = 20, flat presentation, N = 6 targets per cell)

| m | D | T = mean field_ops | W = mean ms/target | ns per op | SD | wall s | load before/after | container | chain T (recorded) | chain/flat T |
|---|---|---|---|---|---|---|---|---|---|---|
| 3 | 9 | 7.589e+06 | 74.56 | 9.82 | 10 | 0 | 1.0/1.0 | first container | 1.476e+08 | 19.5 |
| 3 | 12 | 2.726e+07 | 198 | 7.26 | 13 | 1 | 1.0/1.0 | first container | 7.748e+08 | 28.4 |
| 3 | 14 | 4.686e+07 | 347 | 7.40 | 15 | 2 | 1.0/1.0 | first container | 7.766e+08 | 16.6 |
| 3 | 18 | 1.424e+08 | 950 | 6.67 | 19 | 6 | 1.0/1.0 | first container | 2.428e+10 | 170.5 |
| 3 | 19 | 2.204e+08 | 1331 | 6.04 | 20 | 8 | 1.0/0.919921875 | first container | 2.204e+08 | 1.0 |
| 3 | 21 | 2.649e+08 | 1795 | 6.77 | 22 | 11 | 0.919921875/1.005859375 | first container | 2.788e+09 | 10.5 |
| 3 | 27 | 7.266e+08 | 5864 | 8.07 | 28 | 36 | 1.005859375/1.07666015625 | first container | 2.635e+11 | 362.6 |
| 3 | 28 | 1.003e+09 | 6966 | 6.95 | 29 | 42 | 1.07666015625/1.0869140625 | first container | 2.016e+11 | 201.0 |
| 3 | 36 | 4.133e+09 | 2.978e+04 | 7.21 | 37 | 180 | 1.0869140625/1.08203125 | first container | censored |  |
| 3 | 38 | 6.72e+09 | 4.499e+04 | 6.69 | 39 | 272 | 1.08203125/1.01123046875 | first container | n/a |  |
| 3 | 42 | 8.231e+09 | 9.251e+04 | 11.24 | 43 | 558 | 1.01123046875/1.0 | first container | n/a |  |
| 3 | 54 | 2.506e+10 | 8.041e+05 | 32.08 | 55 | 4832 | 1.0/1.0087890625 | first container | n/a |  |
| 3 | 57 | 3.168e+10 | 1.393e+06 | 43.97 | 58 | 8367 | 1.8818359375/1.0 | second container | censored |  |
| 3 | 63 | censored (exit -6) | | | | 13904 | 1.0/1.0 | second container | n/a | |
| 4 | 3 | 4.233e+05 | 9.151 | 21.62 | 8 | 2 | 0.78515625/0.78515625 | first container | 4.233e+05 | 1.0 |
| 4 | 7 | 2.498e+07 | 369.3 | 14.79 | 8 | 4 | 0.78515625/0.802734375 | first container | 2.498e+07 | 1.0 |
| 4 | 9 | 1.333e+08 | 1432 | 10.75 | 10 | 11 | 0.802734375/0.83349609375 | first container | 1.421e+11 | 1065.8 |
| 4 | 12 | 3.133e+09 | 1.783e+04 | 5.69 | 13 | 110 | 0.83349609375/1.05322265625 | first container | censored |  |
| 4 | 14 | 2.738e+10 | 1.241e+05 | 4.53 | 15 | 750 | 1.05322265625/1.0 | first container | n/a |  |
| 4 | 18 | censored (exit -6) | | | | 1287 | 1.0/1.0 | first container | n/a | |

## Estimator (parent section 5; OLS of ln unit on ln D, complete cells D >= 9, 95 % t interval)

| fit | n | D range | slope | 95 % interval | balanced total exponent 2/(m+1-s) (derived) |
|---|---|---|---|---|---|
| m3_T_all | 13 | 9..57 | 4.598 | [4.402, 4.794] | -3.343 |
| m3_W_all | 12 | 9..54 | 4.977 | [4.391, 5.563] | -2.047 |
| m4_T_all | 3 | [9, 12, 14] | not applicable | | |
| m4_W_all | 3 | [9, 12, 14] | not applicable | | |
| m3_W_first_container | 12 | 9..54 | 4.977 | [4.391, 5.563] | -2.047 |
| m3_T_first_container | 12 | 9..54 | 4.597 | [4.366, 4.827] | -3.353 |

## Two-point local slopes

- m3_T: 9->12: 4.45, 12->14: 3.51, 14->18: 4.42, 18->19: 8.09, 19->21: 1.84, 21->27: 4.01, 27->28: 8.87, 28->36: 5.63, 36->38: 8.99, 38->42: 2.03, 42->54: 4.43, 54->57: 4.33
- m3_W_ms: 9->12: 3.39, 12->14: 3.64, 14->18: 4.01, 18->19: 6.24, 19->21: 2.99, 21->27: 4.71, 27->28: 4.74, 28->36: 5.78, 36->38: 7.63, 38->42: 7.20, 42->54: 8.60, 54->57: 10.16
- m4_T: 3->7: 4.81, 7->9: 6.66, 9->12: 10.97, 12->14: 14.06
- m4_W_ms: 3->7: 4.36, 7->9: 5.39, 9->12: 8.77, 12->14: 12.58

## k = 24 controls (p = 16777141, A = 7842; m = 3)

| D | T | SD | W ms | k = 20 fitted T | ratio |
|---|---|---|---|---|---|
| 10 | 1.49e+07 | 11 | 111 | 1.053e+07 | 1.415 |
| 12 | 2.726e+07 | 13 | 215.3 | 2.435e+07 | 1.119 |
| 15 | 7.043e+07 | 16 | 477 | 6.795e+07 | 1.037 |
| 20 | 2.022e+08 | 21 | 1568 | 2.551e+08 | 0.793 |
| 30 | 1.917e+09 | 31 | 1.151e+04 | 1.646e+09 | 1.165 |

k = 20 vs k = 24 at D = 12: T = 2.726394e+07 vs 2.726398e+07 (relative difference 1.61e-06).

## Predictions

- **P1** (m = 3 estimator applies, s_T in [4.0, 4.8], lower bound > 3; verdict no lever): **held**. applicable = True, s_T = 4.598196022512757, lo = 4.4021860308819925, hi = 4.794206014143521, lever_m3 = False
- **P2** (m = 3: s_W >= s_T + 0.5 and s_W in [4.5, 7]): **FAILED**. s_W = 4.976881792862899, lo = 4.391115992570686, hi = 5.562647593155111, s_T = 4.598196022512757
- **P3** (m = 3 chain reaches D = 57 and censors at or before D = 84): **held**. max_complete = 57, first_censored = 63
- **P4** (m = 4: 3, 7, 9 complete; 12 completes; censors at or before 28; estimator inapplicable; local slopes (D >= 9) > 2): **held**. complete = [3, 7, 9, 12, 14], first_censored = 18, estimator_applicable = False
- **P5** (k = 24 controls equal k = 20 at D = 12 to four digits; all within 1.5x of the k = 20 m = 3 law): **held**. four_digit_equal = True, max_ratio_dev = 1.415029166479931
- **P6** (binary sha256 matches at launch and in every cell record): **held**. launch_lines = 2, launch_ok = True, cells_ok = True

Load flags (any 1-minute load > 1.5): ['20_subflat_m3_sym_d57.json'].
Wall-time regimes: first container D = [9, 12, 14, 18, 19, 21, 27, 28, 36, 38, 42, 54], second container D = [57, 63].
