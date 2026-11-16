# Summary (coded rules, PROTOCOL.md)

- **A/A** (R1 = main run, total_muls and dim): **PASS** []
- **Q1-a** (plain: dim < 4,096 iff a ≤ 3-point decomposition exists): **PASS** []
- **Q1-b** (transformed: dim = 4,096): **FAIL** ["curve 1 conj planted[5, 6, 12, 19] (R2, transformed) dim 4095", "curve 1 conj common0 (R2, transformed) dim 4094", "curve 1 conj planted3_0[8, 13, 26] (R3, transformed) dim 4095", "curve 2 std planted3_0[0, 14, 14] (R3, transformed) dim 4130"]
- **Q1-c** (R3 plain: dim < 4,096 on both arms): **PASS** []
- **Q2** (transformed: solver = oracle): **PASS** []
- **Q2-cost** (R2 / main-run generic cost in [0.9, 1.1]): **PASS** []

## All solves (Macaulay degree 29)

| curve | arm | target | set | transform | dim | at ∞ | short 1/2/3 | total_muls | solver/oracle | outcome |
|--:|:--|:--|:--|:--|--:|--:|:--|--:|--:|:--|
| 1 | conj | planted[5, 6, 12, 19] | R1 | false | 4090 | 0 | 0/0/1 | 1.0404e11 | 0/3 | border unreachable at this degree |
| 1 | conj | planted[5, 6, 12, 19] | R2 | true | 4095 | 1 | 0/0/1 | 1.1914e11 | 3/3 | solved |
| 1 | conj | common0 | R1 | false | 4090 | 0 | 0/0/1 | 1.0415e11 | 0/2 | border unreachable at this degree |
| 1 | conj | common0 | R2 | true | 4094 | 1 | 0/0/1 | 1.1780e11 | 2/2 | solved |
| 1 | conj | common1 | R1 | false | 4096 | 0 | 0/0/0 | 1.1590e11 | 0/0 | solved |
| 1 | conj | common1 | R2 | true | 4096 | 0 | 0/0/0 | 1.1237e11 | 0/0 | solved |
| 1 | std | planted[16, 17, 18, 18] | R1 | false | 4096 | 0 | 0/0/0 | 1.1518e11 | 1/1 | solved |
| 1 | std | planted[16, 17, 18, 18] | R2 | true | 4096 | 0 | 0/0/0 | 1.1695e11 | 1/1 | solved |
| 1 | std | common0 | R1 | false | 4096 | 0 | 0/0/0 | 1.1247e11 | 0/0 | solved |
| 1 | std | common0 | R2 | true | 4096 | 0 | 0/0/0 | 1.1589e11 | 0/0 | solved |
| 1 | std | common1 | R1 | false | 4096 | 0 | 0/0/0 | 1.1311e11 | 0/0 | solved |
| 1 | std | common1 | R2 | true | 4096 | 0 | 0/0/0 | 1.1292e11 | 0/0 | solved |
| 1 | conj | planted3_0[8, 13, 26] | R3 | false | 4090 | 0 | 0/0/1 | 1.0335e11 | 0/2 | border unreachable at this degree |
| 1 | conj | planted3_0[8, 13, 26] | R3 | true | 4095 | 1 | 0/0/1 | 1.1881e11 | 2/2 | solved |
| 1 | conj | planted3_1[0, 2, 15] | R3 | false | 4084 | 0 | 0/0/1 | 1.0409e11 | 0/0 | border unreachable at this degree |
| 1 | conj | planted3_1[0, 2, 15] | R3 | true | 4096 | 2 | 0/0/1 | 1.1637e11 | 0/0 | solved |
| 1 | std | planted3_0[2, 17, 24] | R3 | false | 4090 | 0 | 0/0/1 | 1.0615e11 | 0/1 | border unreachable at this degree |
| 1 | std | planted3_0[2, 17, 24] | R3 | true | 4096 | 1 | 0/0/1 | 1.1498e11 | 1/1 | solved |
| 1 | std | planted3_1[16, 17, 24] | R3 | false | 4090 | 0 | 0/0/1 | 1.0388e11 | 0/0 | border unreachable at this degree |
| 1 | std | planted3_1[16, 17, 24] | R3 | true | 4096 | 1 | 0/0/1 | 1.1617e11 | 0/0 | solved |
| 2 | conj | planted[5, 15, 16, 26] | R1 | false | 4096 | 0 | 0/0/0 | 1.1933e11 | 3/3 | solved |
| 2 | conj | planted[5, 15, 16, 26] | R2 | true | 4096 | 0 | 0/0/0 | 1.1774e11 | 3/3 | solved |
| 2 | conj | common0 | R1 | false | 4096 | 0 | 0/0/0 | 1.1360e11 | 0/0 | solved |
| 2 | conj | common0 | R2 | true | 4096 | 0 | 0/0/0 | 1.1375e11 | 0/0 | solved |
| 2 | conj | common1 | R1 | false | 4096 | 0 | 0/0/0 | 1.1627e11 | 0/0 | solved |
| 2 | conj | common1 | R2 | true | 4096 | 0 | 0/0/0 | 1.1649e11 | 0/0 | solved |
| 2 | std | planted[1, 9, 13, 17] | R1 | false | 4096 | 0 | 0/0/0 | 1.1322e11 | 1/1 | solved |
| 2 | std | planted[1, 9, 13, 17] | R2 | true | 4096 | 0 | 0/0/0 | 1.1633e11 | 1/1 | solved |
| 2 | std | common0 | R1 | false | 4096 | 0 | 0/0/0 | 1.1573e11 | 0/0 | solved |
| 2 | std | common0 | R2 | true | 4096 | 0 | 0/0/0 | 1.1482e11 | 0/0 | solved |
| 2 | std | common1 | R1 | false | 4096 | 0 | 0/0/0 | 1.1346e11 | 0/0 | solved |
| 2 | std | common1 | R2 | true | 4096 | 0 | 0/0/0 | 1.1574e11 | 0/0 | solved |
| 2 | conj | planted3_0[0, 17, 27] | R3 | false | 4078 | 0 | 0/0/1 | 1.0711e11 | 0/2 | chain structure: standard monomials are not an order ideal in x |
| 2 | conj | planted3_0[0, 17, 27] | R3 | true | 4096 | 3 | 0/0/1 | 1.2000e11 | 2/2 | solved |
| 2 | conj | planted3_1[17, 25, 26] | R3 | false | 4078 | 0 | 0/0/3 | 1.0376e11 | 0/3 | chain structure: standard monomials are not an order ideal in x |
| 2 | conj | planted3_1[17, 25, 26] | R3 | true | 4096 | 3 | 0/0/3 | 1.2032e11 | 3/3 | solved |
| 2 | std | planted3_0[0, 14, 14] | R3 | false | 3797 | 0 | 1/0/16 | 2.1927e11 | 0/0 | chain structure: standard monomials are not an order ideal in x |
| 2 | std | planted3_0[0, 14, 14] | R3 | true | 4130 | 0 | 1/0/16 | 1.0683e11 | 0/0 | border unreachable at this degree |
| 2 | std | planted3_1[3, 6, 14] | R3 | false | 4090 | 0 | 0/0/1 | 1.0263e11 | 0/0 | border unreachable at this degree |
| 2 | std | planted3_1[3, 6, 14] | R3 | true | 4096 | 1 | 0/0/1 | 1.1683e11 | 0/0 | solved |

## Q2-cost (R2)

| curve | arm | target | transformed total_muls | of which transform | main-run generic mean | ratio |
|--:|:--|:--|--:|--:|--:|--:|
| 1 | conj | planted[5, 6, 12, 19] | 1.1914e11 | 2.9350e6 | 1.1590e11 | 1.028 |
| 1 | conj | common0 | 1.1780e11 | 2.5250e6 | 1.1590e11 | 1.016 |
| 1 | conj | common1 | 1.1237e11 | 2.5227e6 | 1.1590e11 | 0.970 |
| 1 | std | planted[16, 17, 18, 18] | 1.1695e11 | 2.3253e6 | 1.1359e11 | 1.030 |
| 1 | std | common0 | 1.1589e11 | 2.7150e6 | 1.1359e11 | 1.020 |
| 1 | std | common1 | 1.1292e11 | 2.7097e6 | 1.1359e11 | 0.994 |
| 2 | conj | planted[5, 15, 16, 26] | 1.1774e11 | 2.9359e6 | 1.1640e11 | 1.012 |
| 2 | conj | common0 | 1.1375e11 | 2.9281e6 | 1.1640e11 | 0.977 |
| 2 | conj | common1 | 1.1649e11 | 2.9205e6 | 1.1640e11 | 1.001 |
| 2 | std | planted[1, 9, 13, 17] | 1.1633e11 | 2.6250e6 | 1.1413e11 | 1.019 |
| 2 | std | common0 | 1.1482e11 | 2.7208e6 | 1.1413e11 | 1.006 |
| 2 | std | common1 | 1.1574e11 | 2.7133e6 | 1.1413e11 | 1.014 |

## Amendment A1: transformed solves at Macaulay degree 30–32

| curve | target | degree | matrix | dim | at ∞ | total_muls | solver/oracle | outcome |
|--:|:--|--:|:--|--:|--:|--:|--:|:--|
| 1 | planted[5, 6, 12, 19] | 30 | 48580 × 46376 | 4095 | 1 | 1.5879e11 | 3/3 | solved |
| 1 | planted[5, 6, 12, 19] | 31 | 56124 × 52360 | 4095 | 1 | 2.0890e11 | 3/3 | solved |
| 1 | planted[5, 6, 12, 19] | 32 | 64458 × 58905 | 4095 | 1 | 2.7122e11 | 3/3 | solved |
| 1 | common0 | 30 | 48580 × 46376 | 4094 | 1 | 1.5710e11 | 2/2 | solved |
| 1 | common0 | 31 | 56124 × 52360 | 4094 | 1 | 2.0677e11 | 2/2 | solved |
| 1 | common0 | 32 | 64458 × 58905 | 4094 | 1 | 2.6851e11 | 2/2 | solved |

M2:

- curve 1 planted[5, 6, 12, 19]: dim stays below 4,096 through degree 32, so the shortfall is not truncation (cause reported, not decided)
- curve 1 common0: dim stays below 4,096 through degree 32, so the shortfall is not truncation (cause reported, not decided)
