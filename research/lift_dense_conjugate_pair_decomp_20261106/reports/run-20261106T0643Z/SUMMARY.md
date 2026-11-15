# Summary (coded rules, PROTOCOL.md)

- Control: std H equals `SymmetrisedS5::precompute` on every curve: **PASS**
- **K1** (rows 41,780, cols 40,920, dim 4,096 on every solve): **FAIL** ["curve 1 conj planted[5, 6, 12, 19] (dim 4090)", "curve 1 conj common0 (dim 4090)"]
- **K2** (common-target ratio in [0.9, 1.1] with both solves correct): **FAIL**; median ratio over all solves 1.012 (band [0.95, 1.05]: **PASS**)
- **K3** (solver = oracle on every solve): **FAIL** ["curve 1 conj planted[5, 6, 12, 19]: solver 0 vs oracle 3", "curve 1 conj common0: solver 0 vs oracle 2"]
- **A/A** (repeated solve, identical count): **PASS**

## Common targets

| curve | target | conj total_muls | std total_muls | conj/std |
|--:|:--|--:|--:|--:|
| 1 | common0 | 1.0415e11 (FAILED) | 1.1247e11 (ok) | 0.926 |
| 1 | common1 | 1.1590e11 (ok) | 1.1311e11 (ok) | 1.025 |
| 2 | common0 | 1.1360e11 (ok) | 1.1573e11 (ok) | 0.982 |
| 2 | common1 | 1.1627e11 (ok) | 1.1346e11 (ok) | 1.025 |

## All solves (Macaulay degree 29)

| curve | arm | target | dim | total_muls | solver/oracle | ok |
|--:|:--|:--|--:|--:|--:|:--|
| 1 | conj | planted[5, 6, 12, 19] | 4090 | 1.0404e11 | 0/3 | FAILED |
| 1 | conj | common0 | 4090 | 1.0415e11 | 0/2 | FAILED |
| 1 | conj | common1 | 4096 | 1.1590e11 | 0/0 | ok |
| 1 | std | planted[16, 17, 18, 18] | 4096 | 1.1518e11 | 1/1 | ok |
| 1 | std | common0 | 4096 | 1.1247e11 | 0/0 | ok |
| 1 | std | common1 | 4096 | 1.1311e11 | 0/0 | ok |
| 2 | conj | planted[5, 15, 16, 26] | 4096 | 1.1933e11 | 3/3 | ok |
| 2 | conj | common0 | 4096 | 1.1360e11 | 0/0 | ok |
| 2 | conj | common1 | 4096 | 1.1627e11 | 0/0 | ok |
| 2 | std | planted[1, 9, 13, 17] | 4096 | 1.1322e11 | 1/1 | ok |
| 2 | std | common0 | 4096 | 1.1573e11 | 0/0 | ok |
| 2 | std | common1 | 4096 | 1.1346e11 | 0/0 | ok |

## Amendment A1: decomposable conj targets at Macaulay degree 30–32

| curve | target | degree | rows | dim | total_muls | solver/oracle | ok |
|--:|:--|--:|--:|--:|--:|--:|:--|
| 1 | planted[5, 6, 12, 19] | 30 | 48580 | 4090 | 1.4394e11 | 0/3 | FAILED |
| 1 | planted[5, 6, 12, 19] | 31 | 56124 | 4090 | 1.9437e11 | 0/3 | FAILED |
| 1 | planted[5, 6, 12, 19] | 32 | 64458 | 4090 | 2.5705e11 | 0/3 | FAILED |
| 1 | common0 | 30 | 48580 | 4090 | 1.4407e11 | 0/2 | FAILED |
| 1 | common0 | 31 | 56124 | 4090 | 1.9451e11 | 0/2 | FAILED |
| 1 | common0 | 32 | 64458 | 4090 | 2.5721e11 | 0/2 | FAILED |
| 2 | planted[5, 15, 16, 26] | 30 | 48580 | 4096 | 1.5963e11 | 3/3 | ok |
| 2 | planted[5, 15, 16, 26] | 31 | 56124 | 4096 | 2.1057e11 | 3/3 | ok |
| 2 | planted[5, 15, 16, 26] | 32 | 64458 | 4096 | 2.7391e11 | 3/3 | ok |

M1:

- curve 1 planted[5, 6, 12, 19]: no degree ≤ 32 works
- curve 1 common0: no degree ≤ 32 works
- curve 2 planted[5, 15, 16, 26]: first working degree 30, 1.5963e11 muls = 1.40× the std arm's mean degree-29 cost on that curve
