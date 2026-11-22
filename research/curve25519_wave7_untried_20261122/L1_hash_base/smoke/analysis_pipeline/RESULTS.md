> SMOKE, NOT DATA: pipeline test on the smallest case; no statement below is a result.

# L1 hash-membership factor base: results table [pcc:unverified]

Ratios are verified-hit-rate ratios (MC), 95 % bootstrap intervals over bases; "exact" is the ratio of exact mean covered points.

| k | N | B | comparison | MC ratio | 95 % CI | exact ratio |
|--:|--:|--:|:--|--:|:--|--:|
| 16 | 18 | 300 | hash/rand | 0.9974 | [0.9704, 1.0247] | 1.0035 |
| 16 | 18 | 300 | hash/bits_rand | 0.9964 | [0.9698, 1.0236] | 0.9977 |
| 16 | 18 | 300 | bits_rand/rand | 1.0010 | [0.9737, 1.0284] | 1.0058 |
| 16 | 18 | 300 | hash_proj/rand_proj | 1.0037 | [0.9918, 1.0151] | 0.9996 |
| 16 | 18 | 300 | rand_proj/rand | 7.2989 | [7.1498, 7.4612] | 7.3544 |
| 16 | 18 | 300 | rand/rand2 (A/A) | 1.0068 | [0.9799, 1.0334] | 1.0115 |
| 16 | 18 | 300 | hash/hash2 (A/A) | 0.9949 | [0.9682, 1.0212] | 0.9951 |
| 16 | 18 | 300 | plant3/rand | 1.0264 | [0.9997, 1.0542] | 1.0311 |
| 16 | 18 | 300 | plant5/rand | 1.0428 | [1.0166, 1.0705] | 1.0489 |
| 16 | 18 | 300 | T0/blind | 1.9882 | [1.9620, 2.0138] | 1.9920 |
| 16 | 36 | 300 | hash/rand | 1.0100 | [0.9941, 1.0264] | 1.0029 |
| 16 | 36 | 300 | hash/bits_rand | 1.0092 | [0.9929, 1.0244] | 1.0037 |
| 16 | 36 | 300 | bits_rand/rand | 1.0008 | [0.9844, 1.0166] | 0.9993 |
| 16 | 36 | 300 | hash_proj/rand_proj | 0.9868 | [0.9762, 0.9975] | 0.9984 |
| 16 | 36 | 300 | rand_proj/rand | 6.9207 | [6.8243, 7.0136] | 6.8714 |
| 16 | 36 | 300 | rand/rand2 (A/A) | 0.9933 | [0.9760, 1.0099] | 0.9983 |
| 16 | 36 | 300 | hash/hash2 (A/A) | 1.0109 | [0.9951, 1.0275] | 1.0064 |
| 16 | 36 | 300 | plant3/rand | 1.0421 | [1.0241, 1.0614] | 1.0359 |
| 16 | 36 | 300 | plant5/rand | 1.0442 | [1.0271, 1.0609] | 1.0471 |
| 16 | 36 | 300 | T0/blind | 1.9657 | [1.9498, 1.9818] | 1.9570 |

## Pooled over cells (geometric mean)

| comparison | MC ratio | 95 % CI | exact ratio |
|:--|--:|:--|--:|
| hash/rand | 1.0037 | [0.9882, 1.0200] | 1.0032 |
| hash/bits_rand | 1.0028 | [0.9873, 1.0188] | 1.0007 |
| bits_rand/rand | 1.0009 | [0.9853, 1.0171] | 1.0025 |
| hash_proj/rand_proj | 0.9952 | [0.9872, 1.0029] | 0.9990 |
| rand_proj/rand | 7.1073 | [7.0204, 7.1971] | 7.1088 |
| rand/rand2 (A/A) | 1.0000 | [0.9844, 1.0156] | 1.0049 |
| hash/hash2 (A/A) | 1.0029 | [0.9871, 1.0186] | 1.0007 |
| plant3/rand | 1.0342 | [1.0176, 1.0513] | 1.0335 |
| plant5/rand | 1.0435 | [1.0278, 1.0599] | 1.0480 |
| T0/blind | 1.9769 | [1.9620, 1.9922] | 1.9744 |

## Predictions (thresholds frozen in PROTOCOL.md section 6)

- **HOLDS**: P1 hash/rand pooled in [0.98,1.02], CI inside [0.97,1.03]; >= 8/10 cell CIs contain 1 [pcc:unverified]
- **HOLDS**: P1p power: hash/rand half-width <= 0.03 in >= 8/10 cells (else underpowered, null not claimable) [pcc:unverified]
- **HOLDS**: P2 hash/bits_rand and bits_rand/rand pooled in [0.98,1.02], CI inside [0.97,1.03] [pcc:unverified]
- **HOLDS**: P2b single draws (bits lo=0; literal theta hash base) inside the 1-99 percentile of their distribution in >= 8/10 cells [pcc:unverified]
- **HOLDS**: P3 ceiling: no base above 2N^2/(r-1); projected arms in [0.78,1.0] of it; rand_proj/rand in [6.0,8.4] every cell; hash_proj/rand_proj pooled in [0.98,1.02] [pcc:unverified]
- **HOLDS**: P4 positive control T0/blind exact in [1.80,2.20] every cell; MC CI excludes 1 in every cell [pcc:unverified]
- **FAILS**: P5 planted: plant5/rand CI excludes 1 in >= 9/10 cells and pooled within 0.015 of design; plant3/rand pooled CI excludes 1, pooled within 0.01 of design, >= 6/10 cells exclude 1 [pcc:unverified]
- **HOLDS**: P6 A/A: rand/rand2 and hash/hash2 pooled in [0.985,1.015], CI inside [0.97,1.03]; >= 8/10 cell CIs contain 1 [pcc:unverified]
- **HOLDS**: P7 instrument: 0 verification failures; every hit verified; 0 instrument-check failures; MC vs exact |z| <= 4 in every arm and cell [pcc:unverified]
- **HOLDS**: P8 cost: hash arm priced total >= rand arm in every cell at w = 0, 0.1, 1 (equal at 0, strictly greater above); ranking holds only where the order holds across the band [pcc:unverified]

## Cost, hash membership priced separately (GAE = group-add equivalents)

| k | N | table group ops (N^2) | hash evals to build base (p) | hash evals per column | hash/rand total at w=0 | w=0.1 | w=1 |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 18 | 324 | 65437 | 3635 | 1.00 | 21.20 | 202.97 |
| 16 | 36 | 1296 | 65437 | 1818 | 1.00 | 6.05 | 51.49 |

Totals exclude the shared unpriced native work (Legendre/sqrt per column, matched across arms) and the T0 extra inversions; cost per relation = total / (Q * rate) for Q probed targets, rates in summary.json. No distance to rho is stated here: the generic baseline is 7.8x rho at k = 20 (curve25519_frame_search_20261103/README.md, imported).
