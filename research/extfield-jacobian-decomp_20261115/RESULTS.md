# RESULTS (all numbers [pcc:unverified], local development check)

git HEAD 3c24520f7cdc1c47bac26cc7778d058e655b79a8; bootstrap: 2000 resamples of the 3 instance seeds within each q cell (seed 20261115).

## Per-cell means over 3 instance seeds (S in group-addition equivalents)

| q | ell (3 seeds) | IC S(w=0) | IC S(0.1) | IC S(1) | IC S(w_cal) | rho S(0) | rho S(0.1) | rho S(1) | rho S(w_cal) |
|---|---|---|---|---|---|---|---|---|---|
| 67 | 2.3e+06, 3.4e+06, 3.4e+06 | 2.48e+03 | 7.31e+07 | 7.31e+08 | 2.47e+06 | 8.7e+03 | 9.35e+03 | 1.52e+04 | 8.72e+03 |
| 131 | 4.9e+07, 2.3e+07, 1.5e+08 | 3.42e+03 | 9.27e+07 | 9.27e+08 | 2.92e+06 | 1.68e+04 | 2e+04 | 4.97e+04 | 1.69e+04 |
| 251 | 2.5e+08, 3.3e+08, 1e+09 | 5.3e+03 | 1.62e+08 | 1.62e+09 | 5.34e+06 | 4.4e+04 | 5.67e+04 | 1.71e+05 | 4.44e+04 |
| 509 | 4.2e+09, 4.2e+09, 1.1e+10 | 8.8e+03 | 2.86e+08 | 2.86e+09 | 8.02e+06 | 8.54e+04 | 1.15e+05 | 3.83e+05 | 8.64e+04 |
| 1021 | 1.4e+11, 2.2e+11, 1.2e+11 | 1.64e+04 | 5.75e+08 | 5.75e+09 | 1.96e+07 | 4.18e+05 | 5.8e+05 | 2.03e+06 | 4.24e+05 |
| 2039 | 2.2e+12, 1.7e+12, 1.9e+12 | 3.04e+04 | 1.13e+09 | 1.13e+10 | 3.88e+07 | 8.11e+05 | 1.13e+06 | 3.99e+06 | 8.22e+05 |

Distance to matched rho, S_IC / S_rho (ratio of cell means; <1 means IC cheaper):

| q | w=0 | w=0.1 | w=1 | w_cal |
|---|---|---|---|---|
| 67 | 0.285 [0.272, 0.319] | 7.82e+03 [6.56e+03, 8.7e+03] | 4.8e+04 [4.57e+04, 5.07e+04] | 284 [232, 327] |
| 131 | 0.204 [0.147, 0.307] | 4.63e+03 [3.06e+03, 7.37e+03] | 1.87e+04 [1.13e+04, 3.94e+04] | 173 [128, 213] |
| 251 | 0.121 [0.091, 0.208] | 2.86e+03 [2.22e+03, 4.51e+03] | 9.49e+03 [7.27e+03, 1.68e+04] | 120 [91.3, 181] |
| 509 | 0.103 [0.0641, 0.292] | 2.48e+03 [1.49e+03, 7.67e+03] | 7.47e+03 [4.37e+03, 2.49e+04] | 92.8 [67.6, 183] |
| 1021 | 0.0391 [0.0351, 0.0478] | 991 [903, 1.21e+03] | 2.83e+03 [2.57e+03, 3.45e+03] | 46.2 [41.2, 54.6] |
| 2039 | 0.0375 [0.0288, 0.0537] | 1e+03 [757, 1.44e+03] | 2.83e+03 [2.14e+03, 4.09e+03] | 47.2 [34.4, 72.3] |

w_cal (best-of-3 scalar F_q mulmod time / group-addition time, per run, noisy shared host): q=67: 0.0034, q=131: 0.0031, q=251: 0.0033, q=509: 0.0029, q=1021: 0.0034, q=2039: 0.0034

## Decomposition probability per trial (degenerate R counted as failures)

| q | trials | R with verified relation | p | Wilson 95% | 24 p | degenerate-R rate | solver cost per trial (counted F_q mulmods) | solver time per trial (ms) |
|---|---|---|---|---|---|---|---|---|
| 67 | 4017 | 123 | 0.0306 | [0.0257, 0.0364] | 0.73 | 0.0162 | 5.5e+05 | 0.492 |
| 131 | 5771 | 220 | 0.0381 | [0.0335, 0.0434] | 0.91 | 0.0064 | 4.83e+05 | 0.465 |
| 251 | 10312 | 395 | 0.0383 | [0.0348, 0.0422] | 0.92 | 0.0043 | 4.7e+05 | 0.454 |
| 509 | 18707 | 773 | 0.0413 | [0.0386, 0.0443] | 0.99 | 0.0019 | 4.56e+05 | 0.449 |
| 1021 | 37705 | 1488 | 0.0395 | [0.0375, 0.0415] | 0.95 | 0.0011 | 4.52e+05 | 0.452 |
| 2039 | 73344 | 2980 | 0.0406 | [0.0392, 0.0421] | 0.98 | 0.0006 | 4.52e+05 | 0.447 |

Pooled q >= 131: 5856/145839 = 0.0402, Wilson 95% [0.0392, 0.0412], 24p = 0.96.

## Fitted exponents (ln S = a + b ln q over the six cell means; 95% seed-bootstrap interval)

| arm | w=0 | w=0.1 | w=1 | w_cal |
|---|---|---|---|---|
| ic | 0.74 [0.68, 0.79] | 0.83 [0.73, 0.92] | 0.83 [0.73, 0.92] | 0.83 [0.73, 0.93] |
| rho | 1.38 [1.27, 1.49] | 1.45 [1.34, 1.57] | 1.66 [1.53, 1.81] | 1.38 [1.27, 1.49] |

Secondary (post hoc, not preregistered): rho walk steps only: 1.82 [1.66, 2.02]; rho S(w=0) over q >= 251 only: 1.48 [1.27, 1.77]; IC solver cost per trial: -0.050 [-0.056, -0.044].

## Extrapolated crossover q* (fit of IC against fit of rho; extrapolation over orders of magnitude, indicative only)

| w | q* | 95% seed-bootstrap |
|---|---|---|
| w0 | 9.94 | [6, 16] |
| w0p1 | 9.53e+07 | [2.2e+07, 1.2e+09] |
| w1 | 1.94e+07 | [5.9e+06, 9.8e+07] |
| wcal | 1.66e+06 | [5.5e+05, 1.4e+07] |

At w = 0 IC is already cheaper than rho over the whole ladder, so the w=0 row gives the q at which the fitted lines would cross from the other side (it is below the ladder or meaningless if the ratio at the ladder is < 1 and falling).

## A/A (two internal seeds, same instance, seed 1)

* q=131: IC S ratios (t1/t0) w0=0.89, w0.1=0.82, w1=0.82; rho S ratios w0=0.50, w1=0.33; rho steps [8781, 2173]
* q=251: IC S ratios (t1/t0) w0=0.90, w0.1=0.81, w1=0.81; rho S ratios w0=0.46, w1=0.40; rho steps [42330, 16026]

## Oracle agreement (Macaulay solver vs pair-hash oracle, non-degenerate R)

| run | q | R tested | agree | oracle decompositions | solver decompositions (group-law verified) | degenerate R | R skipped (root of u in F_q) |
|---|---|---|---|---|---|---|---|
| oracle_q131 | 131 | 99 | 99 | 5 | 5 | 1 | 0 |
| oracle_q67 | 67 | 281 | 281 | 14 | 14 | 6 | 13 |
| oracle_supp_q131 | 131 | 976 | 976 | 50 | 50 | 10 | 14 |
| oracle_supp_q67 | 67 | 2881 | 2881 | 24 | 24 | 43 | 76 |

## Dense LA vs model

| q | relations | classes | dense LA mulmods | block-Wiedemann model 2*m*nnz | dense share of S_IC(0.1) |
|---|---|---|---|---|---|
| 67 | [46, 44, 36] | [36, 34, 26] | [10054, 8222, 3783] | [16272, 14416, 8840] | [2e-05, 2e-05, 0.0] |
| 131 | [69, 79, 78] | [60, 70, 69] | [27203, 34548, 34784] | [40120, 53682, 51680] | [2e-05, 4e-05, 4e-05] |
| 251 | [134, 142, 123] | [125, 132, 115] | [162090, 181612, 131652] | [164424, 186120, 137182] | [0.0001, 0.00015, 6e-05] |
| 509 | [267, 246, 268] | [261, 240, 266] | [1314697, 898005, 1062237] | [684648, 579144, 689376] | [0.0005, 0.00027, 0.0004] |
| 1021 | [508, 530, 486] | [505, 526, 486] | [5377881, 7666866, 5559268] | [2525856, 2749760, 2307648] | [0.00095, 0.00149, 0.00086] |
| 2039 | [1035, 1006, 997] | [1032, 1020, 1010] | [41448794, 41524017, 41341472] | [10598500, 10013784, 9834468] | [0.00385, 0.00367, 0.0035] |

## Preregistered predictions

| id | outcome | evidence |
|---|---|---|
| P1 | confirmed | per-q p*24 for q>=131: 0.91, 0.92, 0.99, 0.95, 0.98; band 0.70-1.30 |
| P2 | confirmed | IC exponents w0/w0.1/w1/wcal: 0.74, 0.83, 0.83, 0.83 |
| P3 | refuted | rho exponents w0/w1: 1.38, 1.66; walk-steps-only 1.82; S_w0 for q>=251 1.48 |
| P4 | confirmed | registered oracle runs agree 380/380; supplemental 976/976, 2881/2881 |
| P5 | confirmed | ratio IC/rho at w=0 by q: [0.285, 0.204, 0.121, 0.103, 0.039, 0.038]; w=0.1: [7816, 4625, 2861, 2483, 991, 1001]; w_cal: [283.7, 172.9, 120.3, 92.8, 46.2, 47.2]; q* (fit) w0=9.94311429668717, w0.1=95328429.81180932, w1=19364488.19501907, wcal=1661607.4942709918 |
| P6 | refuted | solver per-trial exponent -0.050 |
| P7 | confirmed | max dense-LA share of S_IC(0.1) = 0.0039 |
| P8 | confirmed | degenerate-R rate q>=131: 0.0064, 0.0043, 0.0019, 0.0011, 0.0006 |
| P9 | confirmed | IC A/A ratios [0.89, 0.82, 0.82, 0.9, 0.81, 0.81]; rho A/A ratios [0.5, 0.33, 0.46, 0.4] |

## Rho diagnostics and wall time (post hoc)

walk steps / sqrt(ell) per q (3 seeds): q=67: 0.58, 0.51, 1.59; q=131: 1.25, 0.57, 1.07; q=251: 2.68, 0.67, 1.28; q=509: 1.22, 0.38, 1.13; q=1021: 0.88, 0.85, 1.41; q=2039: 0.68, 0.42, 0.60

log-log slope of walk steps on ell over all 18 runs: 0.481 (ideal 0.5). Pooled steps/sqrt(ell) = 0.99.
Rho group additions = 9511 + 1.010 * steps (OLS, 18 runs): the intercept is the 64-entry walk pool and is why S_rho grows with exponent below 2 over this ladder.

Mean wall seconds per run (IC / rho), same host: q=67: 0.7 / 0.02; q=131: 0.9 / 0.04; q=251: 1.6 / 0.12; q=509: 2.9 / 0.20; q=1021: 6.0 / 1.05; q=2039: 11.9 / 1.92

## Component-model projection (extrapolation, NOT a measurement)

Constants fitted from the 18 main runs: rho_steps_over_sqrt_ell=0.9875, ell_over_q4=0.148, rho_adds_intercept=9511, rho_adds_per_step=1.01, rho_native_per_step=4.025, p_decomp=0.04015, classes_over_q=0.496, solver_mulmods_per_trial=4.576e+05, native_over_solver_kappa=1.007, ic_group_adds_fixed=1451, dense_LA_mulmods_over_classes_cubed=0.04552, wiedemann_model_over_classes_squared=10.14, w_cal_median=0.003339

| w | q* (IC cheaper above), Wiedemann LA model | q*, dense LA | ratio IC/rho at q=2039 | at q=1e6 | at q=1e9 |
|---|---|---|---|---|---|
| w0 | none below 1e14 | none below 1e14 | 0.0193 | 3.74e-05 | 3.74e-08 |
| w0p1 | 1.98e+06 | none below 1e14 | 523 | 1.52 | 0.466 |
| w1 | none below 1e14 | none below 1e14 | 1.47e+03 | 4.28 | 1.31 |
| wcal | 5.09e+04 | none below 1e14 | 24.1 | 0.0703 | 0.0215 |

Reading the model: at w = 0 only group additions are counted, IC needs about 12 q of them against rho's ~0.38 q^2, so IC is cheaper at every q (no crossover): this is the zero-priced-work win, an accounting artifact of w = 0. For w > 0 the relation phase costs a constant ~4.6e5 mulmods per trial (linear in q overall) but the linear-algebra phase is Theta(q^2) even when sparse (the block-Wiedemann model 2*m*nnz = ~2.5 q^2 mulmods), the same exponent as rho. The basic algorithm (factor base of size q, no large primes) therefore has exponent 2, not 1.5, asymptotically; the sub-linear measured exponents on this ladder (0.7-0.9) are a small-q artifact of the linear solver term dominating. w = 1: LA alone exceeds the whole rho cost in the model, so no crossover; w = 0.1 and w_cal: crossover only at q* above 5e4, i.e. group order above 1e18 (far beyond the measured ladder and beyond a toy-size study). Projections beyond q = 2039 are extrapolations.
