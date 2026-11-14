### Result per prime, primary estimator (b) pooled new primes (frozen choice: homogeneity holds)

S/rho = geometric mean over the 3 cover seeds of S, divided by the reference mean; 95% percentile bootstrap, 20000 resamples of seeds and reference runs jointly (derived). Distance remaining = the factor by which the cover must improve to reach the matched rho (S/rho itself when above 1; the cover is ahead by 1/(S/rho) otherwise).

| unit | p | cover geo-mean S (measured) | reference mean (derived) | S/rho [95% boot] | verdict | distance remaining |
|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.389 | 0.9322 | **0.417** [0.381, 0.459] | below | none: ahead by x2.40 [2.18, 2.62] |
| U1 group additions | 1451 | 0.325 | 0.9322 | **0.348** [0.321, 0.379] | below | none: ahead by x2.87 [2.64, 3.12] |
| U1 group additions | 1601 | 0.270 | 0.9322 | **0.289** [0.267, 0.314] | below | none: ahead by x3.46 [3.18, 3.74] |
| U2 textbook F_p mults | 1301 | 0.389 | 0.3879 | **1.003** [0.916, 1.104] | not resolved | x1.00 [0.92, 1.10] |
| U2 textbook F_p mults | 1451 | 0.325 | 0.3879 | **0.837** [0.771, 0.910] | below | none: ahead by x1.20 [1.10, 1.30] |
| U2 textbook F_p mults | 1601 | 0.270 | 0.3879 | **0.695** [0.642, 0.756] | below | none: ahead by x1.44 [1.32, 1.56] |

### All three reference estimators (w = 0)

| unit | estimator | n | ref mean | p=1301 | p=1451 | p=1601 |
|---|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 42/42/42 | 0.9203/1.0095/0.8669 | 0.423 [0.369, 0.489] below | 0.321 [0.283, 0.371] below | 0.311 [0.270, 0.363] below |
| U1 group additions | (b) pooled new primes | 126 | 0.9322 | 0.417 [0.381, 0.459] below | 0.348 [0.321, 0.379] below | 0.289 [0.267, 0.314] below |
| U1 group additions | (c) pooled + p=251 | 190 | 0.9111 | 0.427 [0.393, 0.466] below | 0.356 [0.331, 0.384] below | 0.296 [0.276, 0.319] below |
| U2 textbook F_p mults | (a) per-prime | 42/42/42 | 0.3830/0.4201/0.3607 | 1.016 [0.886, 1.175] not resolved | 0.773 [0.680, 0.891] below | 0.748 [0.650, 0.871] below |
| U2 textbook F_p mults | (b) pooled new primes | 126 | 0.3879 | 1.003 [0.916, 1.104] not resolved | 0.837 [0.771, 0.910] below | 0.695 [0.642, 0.756] below |
| U2 textbook F_p mults | (c) pooled + p=251 | 190 | 0.3791 | 1.026 [0.944, 1.121] not resolved | 0.856 [0.795, 0.923] below | 0.712 [0.662, 0.766] below |

### Section 5a band (primary estimator; native operations priced at w group-addition equivalents each, on both arms)

Cover native count = `f4_native_ops` (emitted, a LOWER BOUND: F_p additions, pair selection, memory traffic not emitted). Reference native count = canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp (emitted by the walk; `fp_adds` listed separately below, not priced in the band).

| unit | p | w=0 | w=0.1 | w=1 | same verdict at all w | cover native/sqrt(l) (lower bound) | rho native/sqrt(l) |
|---|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.417 [0.381, 0.459] below | 0.675 [0.616, 0.743] below | 0.804 [0.734, 0.885] below | yes (below) | 13.08 | 15.81 |
| U1 group additions | 1451 | 0.348 [0.321, 0.379] below | 0.563 [0.519, 0.612] below | 0.670 [0.618, 0.729] below | yes (below) | 10.90 | 15.81 |
| U1 group additions | 1601 | 0.289 [0.267, 0.314] below | 0.468 [0.432, 0.508] below | 0.557 [0.515, 0.606] below | yes (below) | 9.06 | 15.81 |
| U2 textbook F_p mults | 1301 | 1.003 [0.916, 1.104] not resolved | 0.861 [0.786, 0.949] below | 0.831 [0.758, 0.915] below | NO | 13.08 | 15.81 |
| U2 textbook F_p mults | 1451 | 0.837 [0.771, 0.910] below | 0.718 [0.662, 0.782] below | 0.693 [0.638, 0.754] below | yes (below) | 10.90 | 15.81 |
| U2 textbook F_p mults | 1601 | 0.695 [0.642, 0.756] below | 0.597 [0.551, 0.649] below | 0.576 [0.532, 0.626] below | yes (below) | 9.06 | 15.81 |

Unpriced counters listed (section 5a): reference `fp_adds` 42.1 per sqrt(l) (about 45.1 per step), emitted; cover F_p additions: NOT emitted (cover is a lower bound); cover residual-table and factor-base bytes: not operations, not charged (5a, 2026-11-01). Because the cover count is a lower bound, an "above" verdict at w = 0 can only widen under complete counts; a "below" or "not resolved" verdict could flip.

### Extrapolated fair crossover p* (model)

Model: S/rho(p) = G (p/p_i)^-2.10 through the three measured points (slope from the crossover study, not re-fitted here), p* where S/rho = 1. Interval: 2.5/97.5 percentile of the same bootstrap draws (no slope uncertainty). Slope sensitivity: -1.99 / -2.21 (the crossover study's 95% t interval).

| unit | estimator | p* (model) [95% boot] | p* at slope -1.99 | p* at slope -2.21 | below the u64 limit p <= 2039? |
|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 875 [843, 911] | 851 | 897 | yes (whole interval) |
| U1 group additions | (b) pooled new primes | 874 [842, 910] | 850 | 896 | yes (whole interval) |
| U1 group additions | (c) pooled + p=251 | 884 [854, 916] | 860 | 906 | yes (whole interval) |
| U2 textbook F_p mults | (a) per-prime | 1329 [1280, 1384] | 1322 | 1334 | yes (whole interval) |
| U2 textbook F_p mults | (b) pooled new primes | 1327 [1278, 1381] | 1321 | 1333 | yes (whole interval) |
| U2 textbook F_p mults | (c) pooled + p=251 | 1342 [1297, 1390] | 1336 | 1347 | yes (whole interval) |
