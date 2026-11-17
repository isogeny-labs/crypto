### Result per prime, primary estimator (b) pooled new primes (frozen choice: homogeneity holds)

S/rho = geometric mean over the 3 cover seeds of S, divided by the reference mean; 95% percentile bootstrap, 20000 resamples of seeds and reference runs jointly (derived). Distance remaining = the factor by which the cover must improve to reach the matched rho (S/rho itself when above 1; the cover is ahead by 1/(S/rho) otherwise).

| unit | p | cover geo-mean S (measured) | reference mean (derived) | S/rho [95% boot] | verdict | distance remaining |
|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.389 | 0.9115 | **0.427** [0.397, 0.461] | below | none: ahead by x2.34 [2.17, 2.52] |
| U1 group additions | 1451 | 0.325 | 0.9115 | **0.356** [0.335, 0.380] | below | none: ahead by x2.81 [2.63, 2.99] |
| U1 group additions | 1601 | 0.270 | 0.9115 | **0.296** [0.279, 0.315] | below | none: ahead by x3.38 [3.18, 3.58] |
| U2 textbook F_p mults | 1301 | 0.389 | 0.3793 | **1.026** [0.955, 1.108] | not resolved | x1.03 [0.95, 1.11] |
| U2 textbook F_p mults | 1451 | 0.325 | 0.3793 | **0.856** [0.805, 0.912] | below | none: ahead by x1.17 [1.10, 1.24] |
| U2 textbook F_p mults | 1601 | 0.270 | 0.3793 | **0.711** [0.670, 0.756] | below | none: ahead by x1.41 [1.32, 1.49] |

### All three reference estimators (w = 0)

| unit | estimator | n | ref mean | p=1301 | p=1451 | p=1601 |
|---|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 84/84/84 | 0.8998/0.9277/0.9070 | 0.432 [0.389, 0.485] below | 0.350 [0.318, 0.389] below | 0.297 [0.269, 0.332] below |
| U1 group additions | (b) pooled new primes | 252 | 0.9115 | 0.427 [0.397, 0.461] below | 0.356 [0.335, 0.380] below | 0.296 [0.279, 0.315] below |
| U1 group additions | (c) pooled + p=251 | 316 | 0.9030 | 0.431 [0.402, 0.464] below | 0.359 [0.339, 0.382] below | 0.299 [0.282, 0.317] below |
| U2 textbook F_p mults | (a) per-prime | 84/84/84 | 0.3744/0.3860/0.3774 | 1.039 [0.935, 1.165] not resolved | 0.841 [0.763, 0.935] below | 0.715 [0.647, 0.797] below |
| U2 textbook F_p mults | (b) pooled new primes | 252 | 0.3793 | 1.026 [0.955, 1.108] not resolved | 0.856 [0.805, 0.912] below | 0.711 [0.670, 0.756] below |
| U2 textbook F_p mults | (c) pooled + p=251 | 316 | 0.3757 | 1.035 [0.966, 1.116] not resolved | 0.864 [0.815, 0.918] below | 0.718 [0.679, 0.761] below |

### Section 5a band (primary estimator; native operations priced at w group-addition equivalents each, on both arms)

Cover native count = `f4_native_ops` (emitted, a LOWER BOUND: F_p additions, pair selection, memory traffic not emitted). Reference native count = canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp (emitted by the walk; `fp_adds` listed separately below, not priced in the band).

| unit | p | w=0 | w=0.1 | w=1 | same verdict at all w | cover native/sqrt(l) (lower bound) | rho native/sqrt(l) |
|---|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.427 [0.397, 0.461] below | 0.690 [0.642, 0.746] below | 0.822 [0.765, 0.889] below | yes (below) | 13.08 | 15.46 |
| U1 group additions | 1451 | 0.356 [0.335, 0.380] below | 0.576 [0.541, 0.614] below | 0.685 [0.645, 0.731] below | yes (below) | 10.90 | 15.46 |
| U1 group additions | 1601 | 0.296 [0.279, 0.315] below | 0.478 [0.451, 0.509] below | 0.570 [0.537, 0.606] below | yes (below) | 9.06 | 15.46 |
| U2 textbook F_p mults | 1301 | 1.026 [0.955, 1.108] not resolved | 0.881 [0.820, 0.952] below | 0.850 [0.791, 0.919] below | NO | 13.08 | 15.46 |
| U2 textbook F_p mults | 1451 | 0.856 [0.805, 0.912] below | 0.735 [0.691, 0.783] below | 0.709 [0.666, 0.756] below | yes (below) | 10.90 | 15.46 |
| U2 textbook F_p mults | 1601 | 0.711 [0.670, 0.756] below | 0.611 [0.576, 0.650] below | 0.589 [0.555, 0.627] below | yes (below) | 9.06 | 15.46 |

Unpriced counters listed (section 5a): reference `fp_adds` 41.2 per sqrt(l) (about 45.1 per step), emitted; cover F_p additions: NOT emitted (cover is a lower bound); cover residual-table and factor-base bytes: not operations, not charged (5a, 2026-11-01). Because the cover count is a lower bound, an "above" verdict at w = 0 can only widen under complete counts; a "below" or "not resolved" verdict could flip.

### Extrapolated fair crossover p* (model)

Model: S/rho(p) = G (p/p_i)^-2.10 through the three measured points (slope from the crossover study, not re-fitted here), p* where S/rho = 1. Interval: 2.5/97.5 percentile of the same bootstrap draws (no slope uncertainty). Slope sensitivity: -1.99 / -2.21 (the crossover study's 95% t interval).

| unit | estimator | p* (model) [95% boot] | p* at slope -1.99 | p* at slope -2.21 | below the u64 limit p <= 2039? |
|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 884 [860, 911] | 860 | 906 | yes (whole interval) |
| U1 group additions | (b) pooled new primes | 884 [859, 910] | 860 | 906 | yes (whole interval) |
| U1 group additions | (c) pooled + p=251 | 888 [864, 913] | 864 | 909 | yes (whole interval) |
| U2 textbook F_p mults | (a) per-prime | 1342 [1305, 1383] | 1336 | 1347 | yes (whole interval) |
| U2 textbook F_p mults | (b) pooled new primes | 1342 [1305, 1382] | 1336 | 1347 | yes (whole interval) |
| U2 textbook F_p mults | (c) pooled + p=251 | 1348 [1312, 1386] | 1342 | 1352 | yes (whole interval) |
