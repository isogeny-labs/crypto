### Result per prime, primary estimator (b) pooled new primes (frozen choice: homogeneity holds)

S/rho = geometric mean over the 3 cover seeds of S, divided by the reference mean; 95% percentile bootstrap, 20000 resamples of seeds and reference runs jointly (derived). Distance remaining = the factor by which the cover must improve to reach the matched rho (S/rho itself when above 1; the cover is ahead by 1/(S/rho) otherwise).

| unit | p | cover geo-mean S (measured) | reference mean (derived) | S/rho [95% boot] | verdict | distance remaining |
|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.389 | 0.8907 | **0.437** [0.398, 0.481] | below | none: ahead by x2.29 [2.08, 2.51] |
| U1 group additions | 1451 | 0.325 | 0.8907 | **0.364** [0.336, 0.398] | below | none: ahead by x2.74 [2.51, 2.98] |
| U1 group additions | 1601 | 0.270 | 0.8907 | **0.303** [0.280, 0.330] | below | none: ahead by x3.30 [3.03, 3.58] |
| U2 textbook F_p mults | 1301 | 0.389 | 0.3707 | **1.050** [0.957, 1.155] | not resolved | x1.05 [0.96, 1.16] |
| U2 textbook F_p mults | 1451 | 0.325 | 0.3707 | **0.876** [0.807, 0.956] | below | none: ahead by x1.14 [1.05, 1.24] |
| U2 textbook F_p mults | 1601 | 0.270 | 0.3707 | **0.728** [0.672, 0.793] | below | none: ahead by x1.37 [1.26, 1.49] |

### All three reference estimators (w = 0)

| unit | estimator | n | ref mean | p=1301 | p=1451 | p=1601 |
|---|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 42/42/42 | 0.8793/0.8458/0.9471 | 0.442 [0.382, 0.521] below | 0.384 [0.336, 0.447] below | 0.285 [0.250, 0.330] below |
| U1 group additions | (b) pooled new primes | 126 | 0.8907 | 0.437 [0.398, 0.481] below | 0.364 [0.336, 0.398] below | 0.303 [0.280, 0.330] below |
| U1 group additions | (c) pooled + p=251 | 190 | 0.8836 | 0.440 [0.404, 0.481] below | 0.367 [0.341, 0.397] below | 0.305 [0.284, 0.330] below |
| U2 textbook F_p mults | (a) per-prime | 42/42/42 | 0.3659/0.3520/0.3941 | 1.063 [0.919, 1.251] not resolved | 0.922 [0.807, 1.073] not resolved | 0.684 [0.601, 0.792] below |
| U2 textbook F_p mults | (b) pooled new primes | 126 | 0.3707 | 1.050 [0.957, 1.155] not resolved | 0.876 [0.807, 0.956] below | 0.728 [0.672, 0.793] below |
| U2 textbook F_p mults | (c) pooled + p=251 | 190 | 0.3676 | 1.058 [0.972, 1.155] not resolved | 0.883 [0.820, 0.954] below | 0.734 [0.683, 0.792] below |

### Section 5a band (primary estimator; native operations priced at w group-addition equivalents each, on both arms)

Cover native count = `f4_native_ops` (emitted, a LOWER BOUND: F_p additions, pair selection, memory traffic not emitted). Reference native count = canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp (emitted by the walk; `fp_adds` listed separately below, not priced in the band).

| unit | p | w=0 | w=0.1 | w=1 | same verdict at all w | cover native/sqrt(l) (lower bound) | rho native/sqrt(l) |
|---|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.437 [0.398, 0.481] below | 0.706 [0.644, 0.778] below | 0.841 [0.767, 0.927] below | yes (below) | 13.08 | 15.11 |
| U1 group additions | 1451 | 0.364 [0.336, 0.398] below | 0.589 [0.543, 0.643] below | 0.701 [0.646, 0.766] below | yes (below) | 10.90 | 15.11 |
| U1 group additions | 1601 | 0.303 [0.280, 0.330] below | 0.490 [0.452, 0.534] below | 0.583 [0.539, 0.636] below | yes (below) | 9.06 | 15.11 |
| U2 textbook F_p mults | 1301 | 1.050 [0.957, 1.155] not resolved | 0.901 [0.822, 0.993] below | 0.870 [0.793, 0.958] below | NO | 13.08 | 15.11 |
| U2 textbook F_p mults | 1451 | 0.876 [0.807, 0.956] below | 0.752 [0.693, 0.821] below | 0.725 [0.668, 0.792] below | yes (below) | 10.90 | 15.11 |
| U2 textbook F_p mults | 1601 | 0.728 [0.672, 0.793] below | 0.625 [0.577, 0.681] below | 0.603 [0.557, 0.657] below | yes (below) | 9.06 | 15.11 |

Unpriced counters listed (section 5a): reference `fp_adds` 40.2 per sqrt(l) (about 45.1 per step), emitted; cover F_p additions: NOT emitted (cover is a lower bound); cover residual-table and factor-base bytes: not operations, not charged (5a, 2026-11-01). Because the cover count is a lower bound, an "above" verdict at w = 0 can only widen under complete counts; a "below" or "not resolved" verdict could flip.

### Extrapolated fair crossover p* (model)

Model: S/rho(p) = G (p/p_i)^-2.10 through the three measured points (slope from the crossover study, not re-fitted here), p* where S/rho = 1. Interval: 2.5/97.5 percentile of the same bootstrap draws (no slope uncertainty). Slope sensitivity: -1.99 / -2.21 (the crossover study's 95% t interval).

| unit | estimator | p* (model) [95% boot] | p* at slope -1.99 | p* at slope -2.21 | below the u64 limit p <= 2039? |
|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 894 [861, 932] | 870 | 916 | yes (whole interval) |
| U1 group additions | (b) pooled new primes | 893 [860, 931] | 870 | 915 | yes (whole interval) |
| U1 group additions | (c) pooled + p=251 | 897 [867, 930] | 874 | 918 | yes (whole interval) |
| U2 textbook F_p mults | (a) per-prime | 1357 [1307, 1415] | 1352 | 1361 | yes (whole interval) |
| U2 textbook F_p mults | (b) pooled new primes | 1356 [1305, 1413] | 1352 | 1361 | yes (whole interval) |
| U2 textbook F_p mults | (c) pooled + p=251 | 1362 [1316, 1412] | 1357 | 1366 | yes (whole interval) |
