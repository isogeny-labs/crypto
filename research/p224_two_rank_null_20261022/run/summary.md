## Null arms

| arm | N | censored (any cofactor) | exact ω | ω_lo median [2.5%,97.5%] | ω_mid median [2.5%,97.5%] | ω_hi median [2.5%,97.5%] | ω_small median [2.5%,97.5%] | overlap flags |
|:--|--:|--:|--:|:--|:--|:--|:--|--:|
| R224u | 1000 | 577 (57.7%; CP95 54.6%..60.8%) | 423 | 4 [1,8] | 4 [2,8] | 11 [2,14] | 2 [0,5] | 0 |
| R224s | 1000 | 596 (59.6%; CP95 56.5%..62.7%) | 404 | 4 [1,7] | 4 [2,8] | 11 [2,14] | 2 [0,5] | 0 |
| F224u | 1000 | 611 (61.1%; CP95 58.0%..64.1%) | 389 | 4 [1,8] | 5 [2,8] | 11 [2,14] | 2 [0,6] | 0 |
| F224s | 1000 | 597 (59.7%; CP95 56.6%..62.8%) | 403 | 4 [1,8] | 5 [2,8] | 11 [2,14] | 2 [0,6] | 0 |
| R256u | 1000 | 658 (65.8%; CP95 62.8%..68.7%) | 342 | 4 [1,7] | 4 [2,8] | 12 [2,15] | 2 [0,5] | 0 |
| R256s | 1000 | 663 (66.3%; CP95 63.3%..69.2%) | 337 | 4 [1,7] | 4 [2,8] | 13 [2,15] | 2 [0,5] | 0 |
| R384u | 100 | 80 (80.0%; CP95 70.8%..87.3%) | 20 | 4 [1,8] | 5 [2,9] | 19 [2,22] | 2 [0,6] | 0 |
| R521u | 100 | 85 (85.0%; CP95 76.5%..91.4%) | 15 | 4 [1,8] | 5 [2,9] | 26 [2,29] | 2 [0,6] | 0 |

## Target discriminants

| target | |Δ| bits | ω_lo | ω_mid | ω_hi | ω_small | 2-rank = ω−1 | unsplit cofactor bits |
|:--|--:|--:|--:|--:|--:|--:|:--|
| P-224 | 226 | 7 | 7 | 7 | 5 | 6..6 | [] |
| P-256 | 258 | 4 | 5 | 13 | 2 | 3..12 | [215.3359858395596] |
| P-384 | 386 | 2 | 3 | 19 | 0 | 1..18 | [361.93803872630735] |
| P-521 | 523 | 2 | 3 | 27 | 1 | 1..26 | [520.6551515061761] |
| secp224k1 | 224 | 1 | 1 | 1 | 1 | 0..0 | [] |
| secp256k1 | 258 | 1 | 1 | 1 | 1 | 0..0 | [] |
| brainpoolP224r1 | 226 | 5 | 5 | 5 | 3 | 4..4 | [] |
| brainpoolP256r1 | 257 | 4 | 4 | 4 | 1 | 3..3 | [] |
| brainpoolP384r1 | 385 | 3 | 4 | 18 | 1 | 2..17 | [326.24237163509144] |

## Pre-registered family (uniform trace; threshold alpha/m = 0.005, m = 10)

| target | null arm | metric | target value | control median [2.5%,97.5%] | rank r/N | mid-rank percentile | upper tail (ω ≥ x) frac, CP95 | lower tail (ω ≤ x) frac, CP95 | p (min / mid / max over censoring) | verdict |
|:--|:--|:--|--:|:--|--:|--:|:--|:--|:--|:--|
| P-224 | R224u | omega | 7 | 4 [2,8] | 915/1000 | 91.5% | 12.3% (10.3%..14.5%) | 95.3% (93.8%..96.5%) | 0.1359 / 0.2478 / 1.0000 | within the control distribution |
| P-224 | R224u | omega_small | 5 | 2 [0,5] | 956/1000 | 95.5% | 6.9% (5.4%..8.7%) | 98.0% (96.9%..98.8%) | 0.1399 / 0.1399 / 0.1399 | within the control distribution |
| P-224 | F224u | omega | 7 | 5 [2,8] | 900/1000 | 90.0% | 14.5% (12.4%..16.8%) | 94.5% (92.9%..95.8%) | 0.1618 / 0.2917 / 1.0000 | within the control distribution |
| P-224 | F224u | omega_small | 5 | 2 [0,6] | 946/1000 | 94.5% | 8.0% (6.4%..9.9%) | 97.1% (95.9%..98.0%) | 0.1618 / 0.1618 / 0.1618 | within the control distribution |
| P-256 | R256u | omega | 4..13 | 4 [2,8] | 622/1000 | 62.2% | 48.9% (45.8%..52.0%) | 73.3% (70.4%..76.0%) | 0.0020 / 0.9790 / 1.0000 | inconclusive: censoring |
| P-256 | R256u | omega_small | 2 | 2 [0,5] | 502/1000 | 50.2% | 63.5% (60.4%..66.5%) | 63.9% (60.8%..66.9%) | 1.0000 / 1.0000 / 1.0000 | within the control distribution |
| brainpoolP224r1 | R224u | omega | 5 | 4 [2,8] | 648/1000 | 64.8% | 45.6% (42.5%..48.7%) | 75.3% (72.5%..77.9%) | 0.6513 / 0.9131 / 1.0000 | within the control distribution |
| brainpoolP224r1 | R224u | omega_small | 3 | 2 [0,5] | 732/1000 | 73.2% | 36.2% (33.2%..39.3%) | 82.7% (80.2%..85.0%) | 0.7253 / 0.7253 / 0.7253 | within the control distribution |
| brainpoolP256r1 | R256u | omega | 4 | 4 [2,8] | 396/1000 | 39.6% | 71.9% (69.0%..74.7%) | 51.1% (48.0%..54.2%) | 0.4476 / 1.0000 / 1.0000 | within the control distribution |
| brainpoolP256r1 | R256u | omega_small | 1 | 2 [0,5] | 235/1000 | 23.5% | 89.5% (87.4%..91.3%) | 36.5% (33.5%..39.6%) | 0.7313 / 0.7313 / 0.7313 | within the control distribution |

## Sensitivity: semicircle-weighted trace (not in the family; same rule and threshold)

| target | null arm | metric | target value | control median [2.5%,97.5%] | rank r/N | mid-rank percentile | upper tail (ω ≥ x) frac, CP95 | lower tail (ω ≤ x) frac, CP95 | p (min / mid / max over censoring) | verdict |
|:--|:--|:--|--:|:--|--:|--:|:--|:--|:--|:--|
| P-224 | R224s | omega | 7 | 4 [2,8] | 926/1000 | 92.5% | 11.0% (9.1%..13.1%) | 96.1% (94.7%..97.2%) | 0.1199 / 0.2218 / 1.0000 | within the control distribution |
| P-224 | R224s | omega_small | 5 | 2 [0,5] | 963/1000 | 96.3% | 6.0% (4.6%..7.7%) | 98.6% (97.7%..99.2%) | 0.1219 / 0.1219 / 0.1219 | within the control distribution |
| P-224 | F224s | omega | 7 | 5 [2,8] | 888/1000 | 88.8% | 15.0% (12.8%..17.4%) | 92.7% (90.9%..94.2%) | 0.1998 / 0.3017 / 1.0000 | within the control distribution |
| P-224 | F224s | omega_small | 5 | 2 [0,6] | 946/1000 | 94.6% | 8.0% (6.4%..9.9%) | 97.2% (96.0%..98.1%) | 0.1618 / 0.1618 / 0.1618 | within the control distribution |
| P-256 | R256s | omega | 4..13 | 4 [2,8] | 634/1000 | 63.4% | 46.7% (43.6%..49.8%) | 73.5% (70.6%..76.2%) | 0.0020 / 0.9351 / 1.0000 | inconclusive: censoring |
| P-256 | R256s | omega_small | 2 | 2 [0,5] | 501/1000 | 50.1% | 63.9% (60.8%..66.9%) | 64.2% (61.1%..67.2%) | 1.0000 / 1.0000 / 1.0000 | within the control distribution |
| brainpoolP224r1 | R224s | omega | 5 | 4 [2,8] | 645/1000 | 64.5% | 45.7% (42.6%..48.8%) | 74.7% (71.9%..77.4%) | 0.6494 / 0.9151 / 1.0000 | within the control distribution |
| brainpoolP224r1 | R224s | omega_small | 3 | 2 [0,5] | 742/1000 | 74.2% | 34.9% (31.9%..37.9%) | 83.3% (80.8%..85.6%) | 0.6993 / 0.6993 / 0.6993 | within the control distribution |
| brainpoolP256r1 | R256s | omega | 4 | 4 [2,8] | 410/1000 | 41.0% | 71.2% (68.3%..74.0%) | 53.3% (50.2%..56.4%) | 0.4356 / 1.0000 / 1.0000 | within the control distribution |
| brainpoolP256r1 | R256s | omega_small | 1 | 2 [0,5] | 233/1000 | 23.3% | 89.5% (87.4%..91.3%) | 36.1% (33.1%..39.2%) | 0.7233 / 0.7233 / 0.7233 | within the control distribution |

## Descriptive comparison points (no verdict; outside the tested family)

| target | null arm | N | target ω (lo..hi) | control ω_mid median [2.5%,97.5%] | fraction of controls with ω_hi ≥ x_lo / ω_lo ≤ x_hi |
|:--|:--|--:|--:|:--|:--|
| P-384 | R384u | 100 | 2..19 | 5 [2,9] | 99.0% / 100.0% |
| brainpoolP384r1 | R384u | 100 | 3..18 | 5 [2,9] | 97.0% / 100.0% |
| P-521 | R521u | 100 | 2..27 | 5 [2,9] | 99.0% / 100.0% |
| secp224k1 | R224u | 1000 | 1..1 | 4 [2,8] | 100.0% / 4.7% |
| secp256k1 | R256u | 1000 | 1..1 | 4 [2,8] | 100.0% / 4.6% |

## Positive control

(a) Population shift: fresh R224u samples with k planted small primes vs the R224u null (Mann-Whitney on ω_mid, two-sided).

| planted k | N | mean ω_mid planted | mean ω_mid null | shift | MW z | MW p | detected (p < 0.001 and |shift − k| ≤ 0.5) |
|--:|--:|--:|--:|--:|--:|--:|:--|
| 2 | 300 | 6.52 | 4.47 | +2.05 | 15.9 | 8.09e-57 | yes |
| 4 | 300 | 8.52 | 4.47 | +4.05 | 23.8 | 3.63e-125 | yes |

(b) Target sweep: P-224's discriminant times the first k primes >= 5 not dividing it, ranked in R224u (the P-224 vs R224u rule, threshold 0.005).

| k | planted ω (lo..hi) | midrank percentile | p min / mid / max | verdict | ω_small | p (ω_small) | flagged (ω_small) |
|--:|--:|--:|:--|:--|--:|--:|:--|
| 0 | 7..7 | 91.5% | 0.1359 / 0.2478 / 1.0000 | within the control distribution | 5 | 0.1399 | no |
| 1 | 8..8 | 96.7% | 0.0579 / 0.0959 / 1.0000 | within the control distribution | 6 | 0.0420 | no |
| 2 | 9..9 | 98.8% | 0.0200 / 0.0420 / 1.0000 | within the control distribution | 7 | 0.0180 | no |
| 3 | 10..10 | 99.8% | 0.0020 / 0.0100 / 1.0000 | inconclusive: censoring | 8 | 0.0020 | yes |
| 4 | 11..11 | 100.0% | 0.0020 / 0.0020 / 1.0000 | inconclusive: censoring | 9 | 0.0020 | yes |
| 5 | 12..12 | 100.0% | 0.0020 / 0.0020 / 0.6533 | inconclusive: censoring | 10 | 0.0020 | yes |
| 6 | 13..13 | 100.0% | 0.0020 / 0.0020 / 0.2557 | inconclusive: censoring | 11 | 0.0020 | yes |
| 7 | 14..14 | 100.0% | 0.0020 / 0.0020 / 0.0699 | inconclusive: censoring | 12 | 0.0020 | yes |
| 8 | 15..15 | 100.0% | 0.0020 / 0.0020 / 0.0220 | inconclusive: censoring | 13 | 0.0020 | yes |
| 9 | 16..16 | 100.0% | 0.0020 / 0.0020 / 0.0040 | more extreme than 100.0% of controls (high) | 14 | 0.0020 | yes |
| 10 | 17..17 | 100.0% | 0.0020 / 0.0020 / 0.0020 | more extreme than 100.0% of controls (high) | 15 | 0.0020 | yes |
| 11 | 18..18 | 100.0% | 0.0020 / 0.0020 / 0.0020 | more extreme than 100.0% of controls (high) | 16 | 0.0020 | yes |
| 12 | 19..19 | 100.0% | 0.0020 / 0.0020 / 0.0020 | more extreme than 100.0% of controls (high) | 17 | 0.0020 | yes |

Pre-registered pass rule: both population shifts detected and the target sweep flagged (p_max < alpha/m) at some k <= 10 on both omega and omega_small. First flagged k: omega Some(9), omega_small Some(3). **Positive control PASSED.**
