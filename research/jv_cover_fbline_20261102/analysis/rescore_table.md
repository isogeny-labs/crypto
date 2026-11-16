### Result per prime, primary estimator (b) pooled new primes (frozen choice: homogeneity holds)

S/rho = geometric mean over the 3 cover seeds of S, divided by the reference mean; 95% percentile bootstrap, 20000 resamples of seeds and reference runs jointly (derived). Distance remaining = the factor by which the cover must improve to reach the matched rho (S/rho itself when above 1; the cover is ahead by 1/(S/rho) otherwise).

| unit | p | cover geo-mean S (measured) | reference mean (derived) | S/rho [95% boot] | verdict | distance remaining |
|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.737 | 0.9322 | **0.791** [0.722, 0.871] | below | none: ahead by x1.26 [1.15, 1.39] |
| U1 group additions | 1451 | 0.615 | 0.9322 | **0.659** [0.608, 0.718] | below | none: ahead by x1.52 [1.39, 1.65] |
| U1 group additions | 1601 | 0.511 | 0.9322 | **0.548** [0.506, 0.596] | below | none: ahead by x1.82 [1.68, 1.97] |
| U2 textbook F_p mults | 1301 | 0.737 | 0.3879 | **1.900** [1.735, 2.092] | above | x1.90 [1.73, 2.09] |
| U2 textbook F_p mults | 1451 | 0.615 | 0.3879 | **1.585** [1.460, 1.725] | above | x1.58 [1.46, 1.72] |
| U2 textbook F_p mults | 1601 | 0.511 | 0.3879 | **1.318** [1.217, 1.432] | above | x1.32 [1.22, 1.43] |

### All three reference estimators (w = 0)

| unit | estimator | n | ref mean | p=1301 | p=1451 | p=1601 |
|---|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 42/42/42 | 0.9203/1.0095/0.8669 | 0.801 [0.699, 0.926] below | 0.609 [0.536, 0.703] below | 0.590 [0.512, 0.687] below |
| U1 group additions | (b) pooled new primes | 126 | 0.9322 | 0.791 [0.722, 0.871] below | 0.659 [0.608, 0.718] below | 0.548 [0.506, 0.596] below |
| U1 group additions | (c) pooled + p=251 | 190 | 0.9111 | 0.809 [0.744, 0.884] below | 0.675 [0.627, 0.728] below | 0.561 [0.522, 0.604] below |
| U2 textbook F_p mults | (a) per-prime | 42/42/42 | 0.3830/0.4201/0.3607 | 1.925 [1.679, 2.226] above | 1.463 [1.288, 1.688] above | 1.417 [1.231, 1.651] above |
| U2 textbook F_p mults | (b) pooled new primes | 126 | 0.3879 | 1.900 [1.735, 2.092] above | 1.585 [1.460, 1.725] above | 1.318 [1.217, 1.432] above |
| U2 textbook F_p mults | (c) pooled + p=251 | 190 | 0.3791 | 1.944 [1.788, 2.125] above | 1.622 [1.506, 1.749] above | 1.348 [1.255, 1.452] above |

### Section 5a band (primary estimator; native operations priced at w group-addition equivalents each, on both arms)

Cover native count = `f4_native_ops` (emitted, a LOWER BOUND: F_p additions, pair selection, memory traffic not emitted). Reference native count = canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp (emitted by the walk; `fp_adds` listed separately below, not priced in the band).

| unit | p | w=0 | w=0.1 | w=1 | same verdict at all w | cover native/sqrt(l) (lower bound) | rho native/sqrt(l) |
|---|---|---|---|---|---|---|---|
| U1 group additions | 1301 | 0.791 [0.722, 0.871] below | 0.822 [0.751, 0.906] below | 0.838 [0.765, 0.923] below | yes (below) | 13.31 | 15.81 |
| U1 group additions | 1451 | 0.659 [0.608, 0.718] below | 0.686 [0.632, 0.746] below | 0.699 [0.644, 0.761] below | yes (below) | 11.09 | 15.81 |
| U1 group additions | 1601 | 0.548 [0.506, 0.596] below | 0.570 [0.527, 0.620] below | 0.581 [0.537, 0.632] below | yes (below) | 9.22 | 15.81 |
| U2 textbook F_p mults | 1301 | 1.900 [1.735, 2.092] above | 1.050 [0.958, 1.156] not resolved | 0.866 [0.791, 0.954] below | NO | 13.31 | 15.81 |
| U2 textbook F_p mults | 1451 | 1.585 [1.460, 1.725] above | 0.875 [0.807, 0.952] below | 0.722 [0.666, 0.786] below | NO | 11.09 | 15.81 |
| U2 textbook F_p mults | 1601 | 1.318 [1.217, 1.432] above | 0.728 [0.672, 0.791] below | 0.601 [0.555, 0.653] below | NO | 9.22 | 15.81 |

Unpriced counters listed (section 5a): reference `fp_adds` 42.1 per sqrt(l) (about 45.1 per step), emitted; cover F_p additions: NOT emitted (cover is a lower bound); cover residual-table and factor-base bytes: not operations, not charged (5a, 2026-11-01). Because the cover count is a lower bound, an "above" verdict at w = 0 can only widen under complete counts; a "below" or "not resolved" verdict could flip.

### Extrapolated fair crossover p* (model)

Model: S/rho(p) = G (p/p_i)^-2.10 through the three measured points (slope from the crossover study, not re-fitted here), p* where S/rho = 1. Interval: 2.5/97.5 percentile of the same bootstrap draws (no slope uncertainty). Slope sensitivity: -1.99 / -2.21 (the crossover study's 95% t interval).

| unit | estimator | p* (model) [95% boot] | p* at slope -1.99 | p* at slope -2.21 | below the u64 limit p <= 2039? |
|---|---|---|---|---|---|
| U1 group additions | (a) per-prime | 1186 [1143, 1235] | 1173 | 1198 | yes (whole interval) |
| U1 group additions | (b) pooled new primes | 1185 [1141, 1233] | 1172 | 1197 | yes (whole interval) |
| U1 group additions | (c) pooled + p=251 | 1198 [1158, 1241] | 1186 | 1209 | yes (whole interval) |
| U2 textbook F_p mults | (a) per-prime | 1801 [1735, 1876] | 1823 | 1781 | yes (whole interval) |
| U2 textbook F_p mults | (b) pooled new primes | 1799 [1732, 1872] | 1821 | 1780 | yes (whole interval) |
| U2 textbook F_p mults | (c) pooled + p=251 | 1819 [1758, 1885] | 1842 | 1799 | yes (whole interval) |
