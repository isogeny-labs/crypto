### Result per prime, primary estimator (b) pooled new primes (frozen choice: homogeneity holds)

S/rho = geometric mean over the 3 cover seeds of S, divided by the reference mean; 95% percentile bootstrap, 20000 resamples of seeds and reference runs jointly (derived). Distance remaining = the factor by which the cover must improve to reach the matched rho (S/rho itself when above 1; the cover is ahead by 1/(S/rho) otherwise).

| unit | p | cover geo-mean S (measured) | reference mean (derived) | S/rho [95% boot] | verdict | distance remaining |
|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 1.310 | 0.9322 | **1.405** [1.206, 1.629] | above | x1.40 [1.21, 1.63] |
| U1 group additions | 1451 | 1.081 | 0.9322 | **1.160** [1.020, 1.338] | above | x1.16 [1.02, 1.34] |
| U1 group additions | 1601 | 0.811 | 0.9322 | **0.870** [0.697, 1.183] | not resolved | none: ahead by x1.15 [0.85, 1.44] |
| U2 textbook F_p mults | 1301 | 1.310 | 0.3879 | **3.376** [2.899, 3.915] | above | x3.38 [2.90, 3.92] |
| U2 textbook F_p mults | 1451 | 1.081 | 0.3879 | **2.787** [2.452, 3.215] | above | x2.79 [2.45, 3.21] |
| U2 textbook F_p mults | 1601 | 0.811 | 0.3879 | **2.090** [1.674, 2.842] | above | x2.09 [1.67, 2.84] |

### All three reference estimators (w = 0)

| unit | estimator | n | ref mean | p=1301 | p=1451 | p=1601 |
|---|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 42/42/42 | 0.9203/1.0095/0.8669 | 1.423 [1.183, 1.716] above | 1.071 [0.906, 1.286] not resolved | 0.935 [0.717, 1.285] not resolved |
| U1 group additions | (b) pooled new primes | 126 | 0.9322 | 1.405 [1.206, 1.629] above | 1.160 [1.020, 1.338] above | 0.870 [0.697, 1.183] not resolved |
| U1 group additions | (c) pooled + p=251 | 190 | 0.9111 | 1.437 [1.239, 1.657] above | 1.187 [1.049, 1.363] above | 0.890 [0.716, 1.213] not resolved |
| U2 textbook F_p mults | (a) per-prime | 42/42/42 | 0.3830/0.4201/0.3607 | 3.420 [2.843, 4.123] above | 2.573 [2.178, 3.090] above | 2.248 [1.723, 3.087] above |
| U2 textbook F_p mults | (b) pooled new primes | 126 | 0.3879 | 3.376 [2.899, 3.915] above | 2.787 [2.452, 3.215] above | 2.090 [1.674, 2.842] above |
| U2 textbook F_p mults | (c) pooled + p=251 | 190 | 0.3791 | 3.455 [2.978, 3.982] above | 2.852 [2.522, 3.277] above | 2.139 [1.721, 2.915] above |

### Section 5a band (primary estimator; native operations priced at w group-addition equivalents each, on both arms)

Cover native count = `f4_native_ops` (emitted, a LOWER BOUND: F_p additions, pair selection, memory traffic not emitted). Reference native count = canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp (emitted by the walk; `fp_adds` listed separately below, not priced in the band).

| unit | p | w=0 | w=0.1 | w=1 | same verdict at all w | cover native/sqrt(l) (lower bound) | rho native/sqrt(l) |
|---|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 1.405 [1.206, 1.629] above | 1.467 [1.260, 1.702] above | 1.498 [1.286, 1.738] above | yes (above) | 23.94 | 15.81 |
| U1 group additions | 1451 | 1.160 [1.020, 1.338] above | 1.211 [1.065, 1.397] above | 1.236 [1.088, 1.426] above | yes (above) | 19.71 | 15.81 |
| U1 group additions | 1601 | 0.870 [0.697, 1.183] not resolved | 0.908 [0.727, 1.235] not resolved | 0.927 [0.742, 1.261] not resolved | yes (not resolved) | 15.12 | 15.81 |
| U2 textbook F_p mults | 1301 | 3.376 [2.899, 3.915] above | 1.873 [1.608, 2.172] above | 1.549 [1.330, 1.796] above | yes (above) | 23.94 | 15.81 |
| U2 textbook F_p mults | 1451 | 2.787 [2.452, 3.215] above | 1.545 [1.360, 1.783] above | 1.278 [1.124, 1.474] above | yes (above) | 19.71 | 15.81 |
| U2 textbook F_p mults | 1601 | 2.090 [1.674, 2.842] above | 1.159 [0.928, 1.576] not resolved | 0.958 [0.767, 1.304] not resolved | NO | 15.12 | 15.81 |

Unpriced counters listed (section 5a): reference `fp_adds` 42.1 per sqrt(l) (about 45.1 per step), emitted; cover F_p additions: NOT emitted (cover is a lower bound); cover residual-table and factor-base bytes: not operations, not charged (5a, 2026-11-01). Because the cover count is a lower bound, an "above" verdict at w = 0 can only widen under complete counts; a "below" or "not resolved" verdict could flip.

### Extrapolated fair crossover p* (model)

Model: S/rho(p) = G (p/p_i)^-2.10 through the three measured points (slope from the crossover study, not re-fitted here), p* where S/rho = 1. Interval: 2.5/97.5 percentile of the same bootstrap draws (no slope uncertainty). Slope sensitivity: -1.99 / -2.21 (the crossover study's 95% t interval).

| unit | estimator | p* (model) [95% boot] | p* at slope -1.99 | p* at slope -2.21 | below the u64 limit p <= 2039? |
|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 1529 [1443, 1633] | 1534 | 1525 | yes (whole interval) |
| U1 group additions | (b) pooled new primes | 1528 [1441, 1629] | 1533 | 1524 | yes (whole interval) |
| U1 group additions | (c) pooled + p=251 | 1545 [1459, 1643] | 1551 | 1540 | yes (whole interval) |
| U2 textbook F_p mults | (a) per-prime | 2322 [2191, 2479] | 2384 | 2268 | no: beyond the limit |
| U2 textbook F_p mults | (b) pooled new primes | 2320 [2187, 2474] | 2381 | 2266 | no: beyond the limit |
| U2 textbook F_p mults | (c) pooled + p=251 | 2346 [2216, 2494] | 2409 | 2290 | no: beyond the limit |
