> SMOKE, NOT DATA: pipeline test on the smallest case; no statement below is a result.

# L1 hash-membership factor base: results table [pcc:unverified]

Ratios are verified-hit-rate ratios (MC), 95 % bootstrap intervals over bases; "exact" is the ratio of exact mean covered points.

| k | N | B | comparison | MC ratio | 95 % CI | exact ratio |
|--:|--:|--:|:--|--:|:--|--:|
| 16 | 18 | 40 | hash/rand | 1.0296 | [0.9553, 1.1090] | 1.0291 |
| 16 | 18 | 40 | hash/bits_rand | 0.9816 | [0.9087, 1.0644] | 0.9659 |
| 16 | 18 | 40 | bits_rand/rand | 1.0489 | [0.9768, 1.1206] | 1.0654 |
| 16 | 18 | 40 | hash_proj/rand_proj | 1.0231 | [0.9894, 1.0569] | 1.0053 |
| 16 | 18 | 40 | rand_proj/rand | 7.1206 | [6.7569, 7.5137] | 7.4194 |
| 16 | 18 | 40 | rand/rand2 (A/A) | 0.9719 | [0.9084, 1.0412] | 0.9859 |
| 16 | 18 | 40 | hash/hash2 (A/A) | 1.0512 | [0.9788, 1.1331] | 1.0428 |
| 16 | 18 | 40 | plant3/rand | 1.0337 | [0.9700, 1.0993] | 1.0607 |
| 16 | 18 | 40 | plant5/rand | 1.0846 | [1.0091, 1.1644] | 1.0940 |
| 16 | 18 | 40 | T0/blind | 2.0104 | [1.9409, 2.0790] | 2.0089 |

## Pooled over cells (geometric mean)

| comparison | MC ratio | 95 % CI | exact ratio |
|:--|--:|:--|--:|
| hash/rand | 1.0296 | [0.9553, 1.1090] | 1.0291 |
| hash/bits_rand | 0.9816 | [0.9087, 1.0644] | 0.9659 |
| bits_rand/rand | 1.0489 | [0.9768, 1.1206] | 1.0654 |
| hash_proj/rand_proj | 1.0231 | [0.9894, 1.0569] | 1.0053 |
| rand_proj/rand | 7.1206 | [6.7569, 7.5137] | 7.4194 |
| rand/rand2 (A/A) | 0.9719 | [0.9084, 1.0412] | 0.9859 |
| hash/hash2 (A/A) | 1.0512 | [0.9788, 1.1331] | 1.0428 |
| plant3/rand | 1.0337 | [0.9700, 1.0993] | 1.0607 |
| plant5/rand | 1.0846 | [1.0091, 1.1644] | 1.0940 |
| T0/blind | 2.0104 | [1.9409, 2.0790] | 2.0089 |

## Predictions (thresholds frozen in PROTOCOL.md section 6)

- **FAILS**: P1 hash/rand pooled in [0.98,1.02], CI inside [0.97,1.03]; >= 8/10 cell CIs contain 1 [pcc:unverified]
- **FAILS**: P1p power: hash/rand half-width <= 0.03 in >= 8/10 cells (else underpowered, null not claimable) [pcc:unverified]
- **FAILS**: P2 hash/bits_rand and bits_rand/rand pooled in [0.98,1.02], CI inside [0.97,1.03] [pcc:unverified]
- **HOLDS**: P2b single draws (bits lo=0; literal theta hash base) inside the 1-99 percentile of their distribution in >= 8/10 cells [pcc:unverified]
- **FAILS**: P3 ceiling: no base above 2N^2/(r-1); projected arms in [0.78,1.0] of it; rand_proj/rand in [6.0,8.4] every cell; hash_proj/rand_proj pooled in [0.98,1.02] [pcc:unverified]
- **HOLDS**: P4 positive control T0/blind exact in [1.80,2.20] every cell; MC CI excludes 1 in every cell [pcc:unverified]
- **FAILS**: P5 planted: plant5/rand CI excludes 1 in >= 9/10 cells and pooled within 0.015 of design; plant3/rand pooled CI excludes 1, pooled within 0.01 of design, >= 6/10 cells exclude 1 [pcc:unverified]
- **FAILS**: P6 A/A: rand/rand2 and hash/hash2 pooled in [0.985,1.015], CI inside [0.97,1.03]; >= 8/10 cell CIs contain 1 [pcc:unverified]
- **HOLDS**: P7 instrument: 0 verification failures; every hit verified; 0 instrument-check failures; MC vs exact |z| <= 4 in every arm and cell [pcc:unverified]
- **HOLDS**: P8 cost: hash arm priced total >= rand arm in every cell at w = 0, 0.1, 1 (equal at 0, strictly greater above); ranking holds only where the order holds across the band [pcc:unverified]

## Cost, hash membership priced separately (GAE = group-add equivalents)

| k | N | table group ops (N^2) | hash evals to build base (p) | hash evals per column | hash/rand total at w=0 | w=0.1 | w=1 |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 18 | 324 | 65437 | 3635 | 1.00 | 21.20 | 202.97 |

Totals exclude the shared unpriced native work (Legendre/sqrt per column, matched across arms) and the T0 extra inversions; cost per relation = total / (Q * rate) for Q probed targets, rates in summary.json. No distance to rho is stated here: the generic baseline is 7.8x rho at k = 20 (curve25519_frame_search_20261103/README.md, imported).
