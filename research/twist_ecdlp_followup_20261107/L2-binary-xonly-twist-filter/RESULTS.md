# RESULTS: L2-binary-xonly-twist-filter

Frozen protocol: `PREREGISTRATION.md` (unchanged; `FROZEN.sha256`). Deviations: `DEVIATIONS.md` (read it with this file: rho baseline
changes D-01/D-02, k = 29 m = 3 full solve not run D-03, stop-rule handling D-04). Driver: `run.sh`; raw per-curve counters:
`results/*.tsv`; GP cross-check output: `results/gp_verify.txt`; machine-readable summary: `results.json`; figures: `figs/`.
Every number in sections A-I below is produced by the `analyze` subcommand from `results/*.tsv` (code output, not typed); the prose
only points at them.

## Summary (what the data say; numbers are in the tables below)

* **What was tested.** In x-only relation search over binary curves E: y^2 + xy = x^3 + x^2 + b (k odd, prime n, #E = 2n), a solver that
  enumerates Semaev S_3 solutions over a factor base V (an F_2-subspace) wastes work on factor-base elements that are twist-type
  (no point of E over F_q has that x). The **filter** arm F restricts the enumerated x-values to the E-type part V_E of V. It is compared with
  the same solver without a filter (arm U), and with the solution-side check (arm S). This is a comparison inside this instrument:
  whether the repository's index-calculus baseline already restricts to V_E was **not checked** (open item O1), so a gain here
  is "vs the unfiltered arm of this instrument", not a claim against the repository baseline, and may be a relabelling if the baseline already filters.
* **H1 (exact zero waste) was not falsified.** Over all 9 binary cells the number of non-degenerate solutions with a twist-type coordinate is
  0 (rule-of-three style 95% upper bounds per cell in section A), against 0.75 / 0.875 for the independent-lift model; the 8 prime cells give 0 as well (section E); the prime call ratios match 2 and 4 under the declared rule (one cell fails the stricter variant, see section K).
  The only solutions with a twist coordinate are the degenerate family D (flagged targets). m = 3, k = 29 is **underpowered** (fewer than 1000 unique solutions).
* **H2** (about half the columns are dead): 0 of 9 cells reject, pooled sd of z within [0.8, 1.25] (section B).
* **H3** (counted cost): the measured native-operation ratio U/F is within +-10% of the registered table in all 9 cells (about 1.31-1.33 for m = 2, 1.67-1.75 for m = 3),
  with cluster-bootstrap CIs excluding 1 (section C). Ranking across the 0 / 0.1 / 1 GAE band: F < U in every cell, **weak** at c = 0 because both arms have
  identical group-operation counts (tie at c = 0). The solver-call ratios are about 2 (m = 2) and 4 (m = 3).
* **H4**: the solution-side filter S avoids no lifts (nothing to avoid) and costs slightly more than U (section C, S/U column), so U < S.
* **Instrument validity (F7)**: PC-A, PC-B (mirror lemma: every twist-target solution is all-twist), PC-C, PC-D (exact count |V_tw|) and the GP cross-check all pass (section F). Results are therefore reportable.
* **F8 fails at k = 17 and 19** (rho mean G is outside [0.8, 1.6] x sqrt(pi n)/2 because of fixed per-instance table setup that dominates tiny instances).
  At those sizes no ranking against rho is reported. At k >= 23 rho is within the band.
* **Distance remaining to matched rho (the success criterion, reported as distance, not pass/fail).** The enumeration solver is Theta(q); end-to-end cost is
  far above rho at c >= 0.1 in every cell (section H gap columns: measured complete-solve rows are about 10^3 to 10^5 times rho at c = 1 for k = 17-23;
  extrapolated rows reach 10^6-10^7 at k = 29-31). At c = 0 (group operations only) the measured gaps are about 2-4 (rho cheaper); the m = 3, k = 29
  extrapolation has fewer group operations than rho at c = 0 but is unordered across the band, and it is not a ranked win. The filter lowers each IC row
  by a constant factor of about 1.3 (m = 2) to 1.7 (m = 3) in the relation phase only; it does not change the exponent.

## A. Binary arms (H1-H4): per cell counters

| m | k | l | curves | targets | unique sol. | per target (pred) | flagged tg | violations (non-D twist coord.) | 95% upper bound | D-family sol. | all-E | F4 U!=F | avoided lifts (non-D) | lift-fail sol. = twist sol. | powered |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 17 | 9 | 20 | 20000 | 19522 | 0.976 (1.000) | 152 | 0 | 1.89e-4 | 0 | 19522 | 0 | 0 | consistent | yes |
| 2 | 19 | 10 | 20 | 20000 | 20099 | 1.005 (1.000) | 102 | 0 | 1.84e-4 | 0 | 20099 | 0 | 0 | consistent | yes |
| 2 | 23 | 12 | 20 | 20000 | 19691 | 0.985 (1.000) | 28 | 0 | 1.87e-4 | 0 | 19691 | 0 | 0 | consistent | yes |
| 2 | 29 | 15 | 20 | 10000 | 9884 | 0.988 (1.000) | 0 | 0 | 3.73e-4 | 0 | 9884 | 0 | 0 | consistent | yes |
| 2 | 31 | 16 | 20 | 6000 | 5918 | 0.986 (1.000) | 0 | 0 | 6.23e-4 | 0 | 5918 | 0 | 0 | consistent | yes |
| 3 | 17 | 6 | 20 | 20000 | 7039 | 0.352 (0.333) | 18 | 0 | 5.24e-4 | 377 | 6662 | 0 | 0 | consistent | yes |
| 3 | 19 | 7 | 20 | 20000 | 13260 | 0.663 (0.667) | 7 | 0 | 2.78e-4 | 66 | 13194 | 0 | 0 | consistent | yes |
| 3 | 23 | 8 | 20 | 8000 | 2692 | 0.337 (0.333) | 1 | 0 | 1.37e-3 | 0 | 2692 | 0 | 0 | consistent | yes |
| 3 | 29 | 10 | 10 | 2000 | 659 | 0.330 (0.333) | 0 | 0 | 5.58e-3 | 0 | 659 | 0 | 0 | consistent | UNDERPOWERED |

_Violations = unique solutions with a twist-type coordinate that do not match the degenerate family D. Coin-flip model would give 0.75 (m=2) / 0.875 (m=3) of solutions as non-liftable. 'F4 U!=F' counts unflagged targets whose unique solution sets differ between arms U and F._

## B. H2: factor-base column census (z = (f - (1-mu_E))/sd, sd = sqrt(mu(1-mu)/s)); call ratio check

| m | k | l | N_z | mean z | sd z | reject (alpha=0.01) | mean f(twist) arm V | 1-mu_E mean | call ratio U/F (mean per curve) | ref | mean z_call | sd z_call | within band (declared rule) | known-sd variant (|mean z| <= 2.576/sqrt(curves)) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 17 | 9 | 4020 | -0.0338 | 0.9997 | no | 0.50460 | 0.49975 | 2.0243 | 2 | +0.274 | 1.259 | yes | yes |
| 2 | 19 | 10 | 4020 | -0.0086 | 1.0018 | no | 0.49736 | 0.49989 | 1.9915 | 2 | -0.136 | 1.034 | yes | yes |
| 2 | 23 | 12 | 4020 | +0.0076 | 1.0041 | no | 0.50255 | 0.50005 | 2.0106 | 2 | +0.339 | 0.869 | yes | yes |
| 2 | 29 | 15 | 4020 | -0.0087 | 1.0076 | no | 0.50090 | 0.50000 | 2.0037 | 2 | +0.333 | 1.051 | yes | yes |
| 2 | 31 | 16 | 4020 | +0.0093 | 1.0039 | no | 0.50010 | 0.50000 | 2.0004 | 2 | +0.056 | 1.044 | yes | yes |
| 3 | 17 | 6 | 4020 | -0.0191 | 0.9938 | no | 0.50635 | 0.49975 | 4.1385 | 4 | +0.137 | 0.782 | yes | yes |
| 3 | 19 | 7 | 4020 | -0.0020 | 0.9910 | no | 0.50236 | 0.49989 | 4.1100 | 4 | +0.155 | 1.117 | yes | yes |
| 3 | 23 | 8 | 4020 | +0.0286 | 0.9922 | no | 0.49863 | 0.50005 | 3.9993 | 4 | -0.001 | 0.937 | yes | yes |
| 3 | 29 | 10 | 2010 | -0.0193 | 1.0148 | no | 0.49589 | 0.50000 | 3.9445 | 4 | -0.222 | 1.109 | yes | yes |

## C. H3/H4: counted cost ratios (native operations; G common to both arms enters the cost columns)

| m | k | R_registered | R_instrument-model(a,h from measured) | R_hat native (U/F, pre amortised) | in +-10% | cost ratio c=0.1 [95% CI] | cost ratio c=1 [95% CI] | CI excludes 1 (c>=0.1) | a_hat | h1_hat | h2_hat(mem incl.) | S/U native | S/U in [1,1.05] |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 17 | 1.333 | 1.333 | 1.3388 | yes | 1.3379 [1.3232, 1.3543] | 1.3387 [1.3239, 1.3551] | yes | 27.00 | 38.00 | 54.00 | 1.00055 | yes |
| 2 | 19 | 1.330 | 1.330 | 1.3255 | yes | 1.3251 [1.3162, 1.3342] | 1.3254 [1.3166, 1.3345] | yes | 30.00 | 43.00 | 61.00 | 1.00048 | yes |
| 2 | 23 | 1.321 | 1.321 | 1.3237 | yes | 1.3236 [1.3201, 1.3274] | 1.3237 [1.3202, 1.3275] | yes | 35.00 | 52.00 | 74.00 | 1.00043 | yes |
| 2 | 29 | 1.308 | 1.308 | 1.3080 | yes | 1.3079 [1.3065, 1.3095] | 1.3080 [1.3065, 1.3095] | yes | 41.00 | 64.00 | 92.00 | 1.00083 | yes |
| 2 | 31 | 1.308 | 1.308 | 1.3054 | yes | 1.3054 [1.3044, 1.3065] | 1.3054 [1.3044, 1.3065] | yes | 44.00 | 69.00 | 99.00 | 1.00139 | yes |
| 3 | 17 | 1.724 | 1.730 | 1.7519 | yes | 1.7516 [1.6613, 1.8585] | 1.7519 [1.6616, 1.8588] | yes | 27.00 | 36.71 | 58.71 | 1.00002 | yes |
| 3 | 19 | 1.723 | 1.726 | 1.7307 | yes | 1.7306 [1.6389, 1.8358] | 1.7307 [1.6390, 1.8359] | yes | 30.00 | 42.28 | 66.28 | 1.00001 | yes |
| 3 | 23 | 1.707 | 1.709 | 1.6994 | yes | 1.6994 [1.6480, 1.7588] | 1.6994 [1.6480, 1.7589] | yes | 35.00 | 51.57 | 81.57 | 1.00000 | yes |
| 3 | 29 | 1.688 | 1.688 | 1.6687 | yes | 1.6687 [1.6237, 1.7172] | 1.6687 [1.6237, 1.7172] | yes | 41.00 | 63.87 | 101.87 | 1.00000 | yes |

## D. GAE band per target (cost(c) = G + c N; N = S+M+T+X+L native operations of the solver incl. precompute; lifts and LA excluded)

| m | k | arm | G/target | N/target | cost c=0 | cost c=0.1 | cost c=1 | job wall s per target (whole job: U+F+S+census+verification; contention-affected, secondary) |
|---|---|---|---|---|---|---|---|---|
| 2 | 17 | U | 5.278 | 27466.9 | 5.278 | 2751.967 | 27472.169 | 0.00043 |
| 2 | 17 | F | 5.278 | 20516.2 | 5.278 | 2056.896 | 20521.462 | 0.00043 |
| 2 | 17 | S | 5.278 | 27482.0 | 5.278 | 2753.480 | 27487.302 | 0.00043 |
| 2 | 19 | U | 5.720 | 62056.0 | 5.720 | 6211.324 | 62061.762 | 0.00095 |
| 2 | 19 | F | 5.720 | 46817.6 | 5.720 | 4687.482 | 46823.336 | 0.00095 |
| 2 | 19 | S | 5.720 | 62085.6 | 5.720 | 6214.283 | 62091.348 | 0.00095 |
| 2 | 23 | U | 6.479 | 294066.6 | 6.479 | 29413.142 | 294073.116 | 0.00450 |
| 2 | 23 | F | 6.479 | 222161.2 | 6.479 | 22222.602 | 222167.716 | 0.00450 |
| 2 | 23 | S | 6.479 | 294193.4 | 6.479 | 29425.821 | 294199.902 | 0.00450 |
| 2 | 29 | U | 12.827 | 2848010.4 | 12.827 | 284813.867 | 2848023.227 | 0.04095 |
| 2 | 29 | F | 12.827 | 2177434.6 | 12.827 | 217756.285 | 2177447.401 | 0.04095 |
| 2 | 29 | S | 12.827 | 2850373.6 | 12.827 | 285050.185 | 2850386.405 | 0.04095 |
| 2 | 31 | U | 21.051 | 6126859.2 | 21.051 | 612706.971 | 6126880.251 | 0.07387 |
| 2 | 31 | F | 21.051 | 4693314.0 | 21.051 | 469352.446 | 4693335.001 | 0.07387 |
| 2 | 31 | S | 21.051 | 6135382.7 | 21.051 | 613559.321 | 6135403.746 | 0.07387 |
| 3 | 17 | U | 5.099 | 204256.5 | 5.099 | 20430.746 | 204261.575 | 0.00314 |
| 3 | 17 | F | 5.099 | 116590.7 | 5.099 | 11664.170 | 116595.811 | 0.00314 |
| 3 | 17 | S | 5.099 | 204260.8 | 5.099 | 20431.181 | 204265.921 | 0.00314 |
| 3 | 19 | U | 6.823 | 930969.7 | 6.823 | 93103.796 | 930976.554 | 0.01322 |
| 3 | 19 | F | 6.823 | 537924.8 | 6.823 | 53799.304 | 537931.630 | 0.01322 |
| 3 | 19 | S | 6.823 | 930978.8 | 6.823 | 93104.699 | 930985.582 | 0.01322 |
| 3 | 23 | U | 12.355 | 4472266.1 | 12.355 | 447238.970 | 4472278.501 | 0.06414 |
| 3 | 23 | F | 12.355 | 2631663.1 | 12.355 | 263178.667 | 2631675.476 | 0.06414 |
| 3 | 23 | S | 12.355 | 4472288.3 | 12.355 | 447241.183 | 4472300.632 | 0.06414 |
| 3 | 29 | U | 28.190 | 86859234.7 | 28.190 | 8685951.660 | 86859262.895 | 0.80843 |
| 3 | 29 | F | 28.190 | 52052874.3 | 28.190 | 5205315.624 | 52052902.534 | 0.80843 |
| 3 | 29 | S | 28.190 | 86859421.8 | 28.190 | 8685970.371 | 86859449.997 | 0.80843 |

### Ranking across the band (section 3 rule)

| m | k | F vs U | S vs U |
|---|---|---|---|
| 2 | 17 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 2 | 19 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 2 | 23 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 2 | 29 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 2 | 31 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 3 | 17 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 3 | 19 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 3 | 23 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |
| 3 | 29 | F<U (weak: tie at c=0) | U<S (weak: tie at c=0) |

## E. Prime-field control (H5)

| m | k | l | targets | unique sol. | violations (non-D) | 95% upper | D-family | all-E | F4 U!=F | call ratio mean (ref) | mean z_call | sd z_call | within band (declared rule) | known-sd variant | R_hat native | R registered-formula from measured a,h | z pooled mean (sd) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 17 | 9 | 10000 | 9911 | 0 | 3.72e-4 | 0 | 9911 | 0 | 2.0220 (2) | +0.249 | 1.532 | yes | yes | 1.4003 | 1.3966 | -0.019 (1.017) |
| 2 | 19 | 10 | 10000 | 10179 | 0 | 3.62e-4 | 0 | 10179 | 0 | 1.9869 (2) | -0.209 | 0.716 | yes | yes | 1.3836 | 1.3906 | -0.023 (0.979) |
| 2 | 23 | 12 | 10000 | 10238 | 0 | 3.60e-4 | 0 | 10238 | 0 | 1.9928 (2) | -0.230 | 0.846 | yes | yes | 1.5056 | 1.5109 | -0.040 (1.029) |
| 2 | 29 | 15 | 5000 | 4993 | 0 | 7.39e-4 | 0 | 4993 | 0 | 2.0010 (2) | +0.087 | 1.113 | yes | yes | 1.4932 | 1.4957 | -0.002 (1.009) |
| 2 | 31 | 16 | 3000 | 2942 | 0 | 1.25e-3 | 0 | 2942 | 0 | 1.9934 (2) | -0.848 | 1.106 | yes | NO | 1.3620 | 1.3700 | +0.005 (0.996) |
| 3 | 17 | 6 | 10000 | 3149 | 0 | 1.17e-3 | 136 | 3013 | 0 | 4.6715 (4) | +0.671 | 1.107 | yes | yes | 2.0674 | 1.8659 | +0.037 (0.957) |
| 3 | 19 | 7 | 10000 | 8153 | 0 | 4.52e-4 | 380 | 7773 | 0 | 3.8480 (4) | -0.215 | 0.885 | yes | yes | 1.7812 | 1.8551 | +0.019 (1.010) |
| 3 | 23 | 8 | 4000 | 1453 | 0 | 2.54e-3 | 0 | 1453 | 0 | 3.8032 (4) | -0.394 | 1.024 | yes | yes | 1.9173 | 2.0170 | -0.009 (1.025) |

## F. Instrument validity: positive controls (F7)

* PC-A: Rust classifier vs Rust explicit lift: 100000/100000 agree; Rust labels vs GP `ellordinate`: 100000 points, 0 mismatches (binary and prime, k in 17,19,23,29,31).

**PC-B (twist-type targets; every non-degenerate solution must have all coordinates twist-type, blindness criterion waste < 0.99):**

| field | m | k | curves | targets | solutions | all twist | mixed | with E coords only | waste fraction |
|---|---|---|---|---|---|---|---|---|---|
| binary | 2 | 17 | 10 | 10000 | 9783 | 9783 | 0 | 0 | 1.0000 |
| binary | 2 | 19 | 10 | 10000 | 10059 | 10059 | 0 | 0 | 1.0000 |
| binary | 2 | 23 | 10 | 10000 | 10171 | 10171 | 0 | 0 | 1.0000 |
| binary | 3 | 17 | 10 | 3000 | 1005 | 1005 | 0 | 0 | 1.0000 |
| binary | 3 | 19 | 10 | 3000 | 2004 | 2004 | 0 | 0 | 1.0000 |
| binary | 3 | 23 | 10 | 3000 | 969 | 969 | 0 | 0 | 1.0000 |
| prime | 2 | 17 | 10 | 10000 | 10193 | 10193 | 0 | 0 | 1.0000 |
| prime | 2 | 19 | 10 | 10000 | 9846 | 9846 | 0 | 0 | 1.0000 |
| prime | 2 | 23 | 10 | 10000 | 10027 | 10027 | 0 | 0 | 1.0000 |
| prime | 3 | 17 | 10 | 3000 | 1234 | 1234 | 0 | 0 | 1.0000 |
| prime | 3 | 19 | 10 | 3000 | 1859 | 1859 | 0 | 0 | 1.0000 |
| prime | 3 | 23 | 10 | 3000 | 949 | 949 | 0 | 0 | 1.0000 |

**PC-C (injection into stored U solutions):**

| field | m | k | rho | records | injected | TP | FP | FN | TN | sensitivity | specificity | observed waste | expected |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| binary | 2 | 17 | 0.01 | 994 | 10 | 10 | 0 | 0 | 984 | 1.0 | 1.0 | 0.010060 | 0.010060 |
| binary | 2 | 17 | 0.05 | 994 | 50 | 50 | 0 | 0 | 944 | 1.0 | 1.0 | 0.050302 | 0.050302 |
| binary | 2 | 17 | 0.25 | 994 | 249 | 249 | 0 | 0 | 745 | 1.0 | 1.0 | 0.250503 | 0.250503 |
| binary | 3 | 17 | 0.01 | 464 | 5 | 5 | 0 | 0 | 459 | 1.0 | 1.0 | 0.010776 | 0.010776 |
| binary | 3 | 17 | 0.05 | 464 | 23 | 23 | 0 | 0 | 441 | 1.0 | 1.0 | 0.049569 | 0.049569 |
| binary | 3 | 17 | 0.25 | 464 | 116 | 116 | 0 | 0 | 348 | 1.0 | 1.0 | 0.250000 | 0.250000 |
| prime | 2 | 17 | 0.01 | 897 | 9 | 9 | 0 | 0 | 888 | 1.0 | 1.0 | 0.010033 | 0.010033 |
| prime | 2 | 17 | 0.05 | 897 | 45 | 45 | 0 | 0 | 852 | 1.0 | 1.0 | 0.050167 | 0.050167 |
| prime | 2 | 17 | 0.25 | 897 | 224 | 224 | 0 | 0 | 673 | 1.0 | 1.0 | 0.249721 | 0.249721 |
| prime | 3 | 17 | 0.01 | 452 | 5 | 5 | 0 | 0 | 447 | 1.0 | 1.0 | 0.011062 | 0.011062 |
| prime | 3 | 17 | 0.05 | 452 | 23 | 23 | 0 | 0 | 429 | 1.0 | 1.0 | 0.050885 | 0.050885 |
| prime | 3 | 17 | 0.25 | 452 | 113 | 113 | 0 | 0 | 339 | 1.0 | 1.0 | 0.250000 | 0.250000 |

**PC-D (degenerate family, m=3, x(R) forced into V; predicted mixed D-multisets = |V_tw|, nothing else mixed, no pure-twist):**

| field | k | curves | targets | sum |V_tw| | mixed D found | mixed non-D | pure twist | targets with exact count |
|---|---|---|---|---|---|---|---|---|
| binary | 17 | 10 | 200 | 6248 | 6248 | 0 | 0 | 200/200 |
| binary | 19 | 10 | 200 | 12737 | 12737 | 0 | 0 | 200/200 |
| binary | 23 | 10 | 200 | 25332 | 25332 | 0 | 0 | 200/200 |
| prime | 17 | 10 | 200 | 6223 | 6223 | 0 | 0 | 200/200 |
| prime | 19 | 10 | 200 | 12731 | 12731 | 0 | 0 | 200/200 |
| prime | 23 | 10 | 200 | 25606 | 25606 | 0 | 0 | 200/200 |

* GP independent cross-check of stored samples: 3807 unique solutions checked (3796 full relation checks by `elladd`, remainder label-only for degenerate/twist coordinates); label mismatches 0; relation failures 0.

## G. Matched Pollard rho (negation map), per size

| k | curves x instances | solved | mean G | mean G / (sqrt(pi n)/2) | F8 band [0.8,1.6] | mean N (native) | mean lookups | steps/instance | walks/instance | cost c=0 | c=0.1 | c=1 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 17 | 10x50 | 500/500 | 822.5 | 3.623 | OUTSIDE (flagged) | 1458.2 | 16.3 | 288.4 | 1.40 | 822.5 | 969.9 | 2296.9 |
| 19 | 10x50 | 500/500 | 1147.6 | 2.529 | OUTSIDE (flagged) | 3024.0 | 23.8 | 600.0 | 1.77 | 1147.6 | 1452.3 | 4195.4 |
| 23 | 10x50 | 500/500 | 2725.8 | 1.502 | in band | 10658.7 | 56.6 | 2120.4 | 3.47 | 2725.8 | 3797.3 | 13441.2 |
| 29 | 10x50 | 500/500 | 19129.8 | 1.317 | in band | 90475.4 | 122.3 | 18070.6 | 7.63 | 19129.8 | 28189.5 | 109727.4 |
| 31 | 10x50 | 500/500 | 43261.6 | 1.490 | in band | 208424.5 | 164.8 | 41651.9 | 10.92 | 43261.6 | 64120.6 | 251851.0 |

## H. Index calculus end-to-end vs matched rho (distance remaining)

Gap = cost_IC / cost_rho at c = 0, 0.1, 1. Ranking only where the order holds at all three prices.

| m | k | source | arm | N_rel | G | N | cost c=0 | c=0.1 | c=1 | gap c=0 | gap c=0.1 | gap c=1 | vs rho (rank rule) | rho F8 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 17 | measured, d-determined (exploratory stop rule) | U | 108 | 3121 | 3197493 | 3121 | 322870 | 3200614 | 3.8 | 332.9 | 1393.4 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 17 | measured, d-determined (exploratory stop rule) | F | 108 | 3121 | 2383404 | 3121 | 241462 | 2386525 | 3.8 | 249.0 | 1039.0 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 17 | extrapolated, N_need = |V_E|+11 (registered) | U | 264 | 3484 | 13597869 | 3484 | 1363271 | 13601353 | 4.2 | 1405.6 | 5921.6 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 17 | extrapolated, N_dd = 0.44 |V_E| (ratio measured at full-solve cells, exploratory) | U | 110 | 3096 | 3553104 | 3096 | 358406 | 3556199 | 3.8 | 369.5 | 1548.2 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 17 | extrapolated, N_need = |V_E|+11 (registered) | F | 264 | 3484 | 11716884 | 3484 | 1175172 | 11720368 | 4.2 | 1211.6 | 5102.6 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 17 | extrapolated, N_dd = 0.44 |V_E| (ratio measured at full-solve cells, exploratory) | F | 110 | 3096 | 2768869 | 3096 | 279983 | 2771965 | 3.8 | 288.7 | 1206.8 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 19 | measured, d-determined (exploratory stop rule) | U | 210 | 3771 | 13211738 | 3771 | 1324945 | 13215509 | 3.3 | 912.3 | 3150.0 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 19 | measured, d-determined (exploratory stop rule) | F | 210 | 3771 | 9908801 | 3771 | 994651 | 9912572 | 3.3 | 684.9 | 2362.7 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 19 | extrapolated, N_need = |V_E|+11 (registered) | U | 525 | 4524 | 80768087 | 4524 | 8081332 | 80772611 | 3.9 | 5564.3 | 19252.7 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 19 | extrapolated, N_dd = 0.41 |V_E| (ratio measured at full-solve cells, exploratory) | U | 213 | 3744 | 16351599 | 3744 | 1638904 | 16355342 | 3.3 | 1128.5 | 3898.4 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 19 | extrapolated, N_need = |V_E|+11 (registered) | F | 525 | 4524 | 72804287 | 4524 | 7284952 | 72808811 | 3.9 | 5016.0 | 17354.5 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 19 | extrapolated, N_dd = 0.41 |V_E| (ratio measured at full-solve cells, exploratory) | F | 213 | 3744 | 13127690 | 3744 | 1316513 | 13131434 | 3.3 | 906.5 | 3130.0 | withheld (rho flagged by F8: not reported) | flagged |
| 2 | 23 | measured, d-determined (exploratory stop rule) | U | 828 | 6106 | 247520222 | 6106 | 24758128 | 247526328 | 2.2 | 6519.9 | 18415.5 | rho<IC | in band |
| 2 | 23 | measured, d-determined (exploratory stop rule) | F | 828 | 6106 | 187480235 | 6106 | 18754129 | 187486341 | 2.2 | 4938.8 | 13948.7 | rho<IC | in band |
| 2 | 23 | extrapolated, N_need = |V_E|+11 (registered) | U | 2048 | 9152 | 3475456602 | 9152 | 347554812 | 3475465754 | 3.4 | 91526.0 | 258568.5 | rho<IC | in band |
| 2 | 23 | extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory) | U | 824 | 6076 | 432883456 | 6076 | 43294421 | 432889532 | 2.2 | 11401.3 | 32206.2 | rho<IC | in band |
| 2 | 23 | extrapolated, N_need = |V_E|+11 (registered) | F | 2048 | 9152 | 3325879786 | 9152 | 332597130 | 3325888937 | 3.4 | 87587.0 | 247440.2 | rho<IC | in band |
| 2 | 23 | extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory) | F | 824 | 6076 | 372689511 | 6076 | 37275027 | 372695587 | 2.2 | 9816.1 | 27727.9 | rho<IC | in band |
| 2 | 29 | extrapolated, N_need = |V_E|+11 (registered) | U | 16365 | 46219 | 1508064452393 | 46219 | 150806491458 | 1508064498612 | 2.4 | 5349733.1 | 13743738.2 | rho<IC | in band |
| 2 | 29 | extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory) | U | 6617 | 21770 | 115633880023 | 21770 | 11563409772 | 115633901793 | 1.1 | 410202.2 | 1053829.0 | rho<IC | in band |
| 2 | 29 | extrapolated, N_need = |V_E|+11 (registered) | F | 16365 | 46219 | 1496961720843 | 46219 | 149696218303 | 1496961767062 | 2.4 | 5310347.1 | 13642553.6 | rho<IC | in band |
| 2 | 29 | extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory) | F | 6617 | 21770 | 111144718720 | 21770 | 11114493642 | 111144740490 | 1.1 | 394277.3 | 1012917.0 | rho<IC | in band |
| 2 | 31 | extrapolated, N_need = |V_E|+11 (registered) | U | 32772 | 87566 | 11935779707530 | 87566 | 1193578058318 | 11935779795095 | 2.0 | 18614589.5 | 47392224.5 | rho<IC | in band |
| 2 | 31 | extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory) | U | 13255 | 38737 | 858629819492 | 38737 | 85863020687 | 858629858229 | 0.9 | 1339087.0 | 3409277.0 | unordered across the band | in band |
| 2 | 31 | extrapolated, N_need = |V_E|+11 (registered) | F | 32772 | 87566 | 11888148894866 | 87566 | 1188814977052 | 11888148982432 | 2.0 | 18540306.3 | 47203101.5 | rho<IC | in band |
| 2 | 31 | extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory) | F | 13255 | 38737 | 839364794243 | 38737 | 83936518162 | 839364832981 | 0.9 | 1309042.0 | 3332783.2 | unordered across the band | in band |
| 3 | 17 | measured, d-determined (exploratory stop rule) | U | 31 | 3068 | 22522696 | 3068 | 2255338 | 22525764 | 3.7 | 2325.3 | 9807.0 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | measured, registered stop rule (mean over the 10 of 10 instances that succeeded) | U | 44 | 3171 | 31955199 | 3171 | 3198690 | 31958369 | 3.9 | 3297.9 | 13913.6 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | measured, d-determined (exploratory stop rule) | F | 31 | 3068 | 12474948 | 3068 | 1250563 | 12478016 | 3.7 | 1289.4 | 5432.5 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | measured, registered stop rule (mean over the 10 of 10 instances that succeeded) | F | 44 | 3171 | 17567464 | 3171 | 1759917 | 17570635 | 3.9 | 1814.5 | 7649.7 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | extrapolated, N_need = |V_E|+11 (registered) | U | 42 | 3082 | 24462779 | 3082 | 2449360 | 24465861 | 3.7 | 2525.3 | 10651.6 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | extrapolated, N_dd = 1.00 |V_E| (ratio measured at full-solve cells, exploratory) | U | 31 | 3011 | 18121826 | 3011 | 1815194 | 18124838 | 3.7 | 1871.5 | 7890.9 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | extrapolated, N_need = |V_E|+11 (registered) | F | 42 | 3082 | 13976265 | 3082 | 1400709 | 13979347 | 3.7 | 1444.2 | 6086.1 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 17 | extrapolated, N_dd = 1.00 |V_E| (ratio measured at full-solve cells, exploratory) | F | 31 | 3011 | 10349941 | 3011 | 1038006 | 10352952 | 3.7 | 1070.2 | 4507.3 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | measured, d-determined (exploratory stop rule) | U | 64 | 3592 | 98516056 | 3592 | 9855198 | 98519648 | 3.1 | 6785.7 | 23482.9 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | measured, registered stop rule (mean over the 4 of 10 instances that succeeded) | U | 95 | 3799 | 162919822 | 3799 | 16295781 | 162923621 | 3.3 | 11220.3 | 38834.0 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | measured, d-determined (exploratory stop rule) | F | 64 | 3592 | 52838786 | 3592 | 5287471 | 52842378 | 3.1 | 3640.7 | 12595.4 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | measured, registered stop rule (mean over the 4 of 10 instances that succeeded) | F | 95 | 3799 | 84451985 | 3799 | 8448997 | 84455784 | 3.3 | 5817.5 | 20130.7 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | extrapolated, N_need = |V_E|+11 (registered) | U | 74 | 3617 | 104336126 | 3617 | 10437229 | 104339742 | 3.2 | 7186.5 | 24870.1 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | extrapolated, N_dd = 1.05 |V_E| (ratio measured at full-solve cells, exploratory) | U | 66 | 3573 | 92922679 | 3573 | 9295841 | 92926252 | 3.1 | 6400.6 | 22149.7 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | extrapolated, N_need = |V_E|+11 (registered) | F | 74 | 3617 | 60348292 | 3617 | 6038446 | 60351909 | 3.2 | 4157.7 | 14385.3 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 19 | extrapolated, N_dd = 1.05 |V_E| (ratio measured at full-solve cells, exploratory) | F | 66 | 3573 | 53736169 | 3573 | 5377190 | 53739742 | 3.1 | 3702.4 | 12809.3 | withheld (rho flagged by F8: not reported) | flagged |
| 3 | 23 | measured, d-determined (exploratory stop rule) | U | 128 | 4940 | 1736171058 | 4940 | 173622046 | 1736175998 | 1.8 | 45722.1 | 129168.4 | rho<IC | in band |
| 3 | 23 | measured, registered stop rule (mean over the 6 of 10 instances that succeeded) | U | 186 | 5436 | 2854831633 | 5436 | 285488600 | 2854837070 | 2.0 | 75181.4 | 212394.8 | rho<IC | in band |
| 3 | 23 | measured, d-determined (exploratory stop rule) | F | 128 | 4940 | 981055474 | 4940 | 98110488 | 981060414 | 1.8 | 25836.7 | 72989.2 | rho<IC | in band |
| 3 | 23 | measured, registered stop rule (mean over the 6 of 10 instances that succeeded) | F | 186 | 5436 | 1600929213 | 5436 | 160098358 | 1600934649 | 2.0 | 42160.7 | 119106.7 | rho<IC | in band |
| 3 | 23 | extrapolated, N_need = |V_E|+11 (registered) | U | 139 | 4972 | 1846306051 | 4972 | 184635577 | 1846311023 | 1.8 | 48622.4 | 137362.3 | rho<IC | in band |
| 3 | 23 | extrapolated, N_dd = 1.03 |V_E| (ratio measured at full-solve cells, exploratory) | U | 131 | 4918 | 1743397864 | 4918 | 174344704 | 1743402782 | 1.8 | 45912.4 | 129706.1 | rho<IC | in band |
| 3 | 23 | extrapolated, N_need = |V_E|+11 (registered) | F | 139 | 4972 | 1086817998 | 4972 | 108686772 | 1086822969 | 1.8 | 28621.9 | 80857.7 | rho<IC | in band |
| 3 | 23 | extrapolated, N_dd = 1.03 |V_E| (ratio measured at full-solve cells, exploratory) | F | 131 | 4918 | 1026204124 | 4918 | 102625330 | 1026209042 | 1.8 | 27025.6 | 76348.1 | rho<IC | in band |
| 3 | 29 | extrapolated, N_need = |V_E|+11 (registered) | U | 527 | 8885 | 138891779665 | 8885 | 13889186852 | 138891788550 | 0.5 | 492707.2 | 1265789.6 | unordered across the band | in band |
| 3 | 29 | extrapolated, N_dd = 1.03 |V_E| (ratio measured at full-solve cells, exploratory) | U | 529 | 8901 | 139467274646 | 8901 | 13946736365 | 139467283547 | 0.5 | 494748.7 | 1271034.4 | unordered across the band | in band |
| 3 | 29 | extrapolated, N_need = |V_E|+11 (registered) | F | 527 | 8885 | 83254420024 | 8885 | 8325450888 | 83254428910 | 0.5 | 295338.4 | 758738.8 | unordered across the band | in band |
| 3 | 29 | extrapolated, N_dd = 1.03 |V_E| (ratio measured at full-solve cells, exploratory) | F | 529 | 8901 | 83599545421 | 8901 | 8359963443 | 83599554321 | 0.5 | 296562.7 | 761884.1 | unordered across the band | in band |

_Extrapolated rows: relation-phase cost = measured per-unique-solution cost of the arms cell (G: target walk + sign checks; N: solver + lifts + hash lookups) x N_rel; LA = N_rel^3/3 multiply-adds (formula model, not measured); setup G measured; final dP=Q check (about 1.5k G) not added. Never a measured full solve._

**Complete-solve certificate cells (measured; 5 curves x 2 instances at k=17,19, reduced at larger k, see DEVIATIONS.md):**

| m | k | instances | registered stop rule (full column rank): success | d determined (exploratory) | d correct (dP=Q) | mean unknowns u | mean N_need | mean relations at d-determined | mean relations at registered stop/fail | mean targets at d-determined | F4/F9 failures |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 17 | 10 | 0/10 | 10/10 | 10/10 | 249.4 | 260.4 | 108.5 | 422.4 | 116.9 | 0 |
| 2 | 19 | 10 | 0/10 | 10/10 | 10/10 | 507.4 | 518.4 | 209.8 | 838.6 | 214.4 | 0 |
| 2 | 23 | 10 | 0/10 | 10/10 | 10/10 | 2047.2 | 2058.2 | 828.3 | 3317.6 | 839.3 | 0 |
| 3 | 17 | 10 | 10/10 | 10/10 | 10/10 | 30.6 | 41.6 | 30.7 | 44.1 | 111.8 | 0 |
| 3 | 19 | 10 | 4/10 | 10/10 | 10/10 | 61.0 | 72.0 | 63.8 | 110.9 | 108.8 | 0 |
| 3 | 23 | 10 | 6/10 | 10/10 | 10/10 | 125.2 | 136.2 | 128.4 | 203.8 | 394.6 | 0 |

## I. Falsification criteria (registered thresholds, mechanically evaluated)

| criterion | not falsified / valid | detail |
|---|---|---|
| F7 instrument validity | yes | all controls pass |
| F1 H1 (zero waste, exact) | yes | 0 non-degenerate solutions with a twist coordinate over all binary cells |
| F2 H2 (column density) | yes | 0 of 9 cells reject; pooled-sd out of [0.8,1.25]: false |
| F3 H3 (cost ratio) | yes | 0 of 9 cells outside +-10% of the registered R; CI contains 1 at c>=0.1 in some cell: false |
| F4 set equality U vs F | yes | 0 unflagged targets with differing solution sets (all cells, both fields, full-solve) |
| F5 H4 (solution-side filter) | yes | avoided non-degenerate lifts > 0: false; S/U outside [1,1.05] in some cell (magnitude only): false |
| F6 H5 (prime control) | yes | 0 non-degenerate prime waste solutions; 0 prime cells outside the call-ratio band |
| F8 rho baseline band | NO | sizes outside [0.8,1.6]: [17, 19] |
| F9 verification | yes | 0 failed group-law checks or wrong recovered d |

**H6 (qualitative, distance remaining):** smallest gap cost_IC/cost_rho at c = 1 over every IC row of section H: 1039.0. Rows whose order flips inside the band (IC cheaper at some prices, rho at others): m=2 k=31 U [extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory)]; m=2 k=31 F [extrapolated, N_dd = 0.40 |V_E| (ratio measured at full-solve cells, exploratory)]; m=3 k=29 U [extrapolated, N_need = |V_E|+11 (registered)]; m=3 k=29 U [extrapolated, N_dd = 1.03 |V_E| (ratio measured at full-solve cells, exploratory)]; m=3 k=29 F [extrapolated, N_need = |V_E|+11 (registered)]; m=3 k=29 F [extrapolated, N_dd = 1.03 |V_E| (ratio measured at full-solve cells, exploratory)].


## J. Extrapolation (labelled; no data-like numbers beyond the registered model table)

Not measured. Values below are the registered model (`frozen_pre_data/prediction_table.txt`), not results.
* Lemma B is size independent (E[2] = {O, T0}); the waste fraction of non-degenerate solutions is predicted to stay 0 and the dead-column density 1/2 at ECC2K-130 (k = 131).
* Enumeration-instrument model ratios at k = 131: R2 = 1.266 (l = 66) and R3 = 1.625 (l = 44). Moot: enumeration needs 2^66 or 2^88 attempts per target.
* Groebner / Weil-descent solvers: the effect of restricting x_i to V_E is not modelled and is open (V_E is not an F_2-subspace; the trace condition adds lift variables). Koblitz orbit structure: no separate prediction.

## K. Limits and open items

* O1: the repository baseline may already filter non-lifting x-values; then the F-over-U gain is a relabelling and not an improvement.
* O2: the enumeration solver is Theta(q) in total; the gain is a constant factor on that phase. O3: Weil descent, m >= 4, Koblitz orbits not measured.
* Extrapolated IC rows (k = 29, 31 and m = 3, k = 29) use measured per-relation cost times N_rel and a formula model for linear algebra; they are not measured full solves.
* The registered stop rule (full column rank) failed for every m = 2 instance and for part of the m = 3 instances at k = 19, 23 (section H table "Complete-solve certificate cells"); the d-determined stop rule is exploratory.
* One prime cell (m = 2, k = 31) fails the stricter known-sd variant of the call-ratio check although the ratio is 1.993 against 2.0 and it passes the declared rule (section E); see D-08.
* Rho at k = 17, 19 is flagged by F8. Wall times (contention-affected: the host load average was above 8) are secondary and not used for any conclusion.
* No PDF of this report: no LaTeX toolchain is present; `figs/RESULTS_figures.pdf` contains the four figures (matplotlib).

## L. Replication

`VERIFICATION.json` holds (i) a same-seed `./run.sh quick` replay compared column by column with this run (all 19 TSVs of the subset are identical
apart from the contention-affected wall time, and the sample files and curve list are byte-identical), (ii) an independent replay with seed offset
`L2SEED_XOR=777` (different curves, factor bases, targets and DLP instances; k <= 19 binary and prime arms, all controls, rho k <= 23, full solves k <= 19)
with its mechanical verdict rows: F1, F2, F3, F4, F5, F7, F9 hold; F8 flags k = 17 and 19 as in the main run; F6 reports one prime cell (m = 2, k = 19) outside the
declared call-ratio band (mean z = -0.51 with the sd estimated from 10 curves; it passes the known-sd variant), while the structural part of F6 (zero non-degenerate prime waste) holds.
Under the null, 17 cells at alpha = 0.01 give a few percent chance of such a fluctuation; it is reported, not reconciled. (iii) The GP cross-check summary (`results/gp_verify.txt`).
