# Amendment A2: fresh random charts (coded rule M3)

- **Q2 re-check** (solver = oracle on every A2 solve): **FAIL** (1 violations)
- **Control** (`planted3_1` stays at 4,096): **FAIL**

M3:

- curve 1 conj planted[5, 6, 12, 19]: first A dim 4095; chart-dependent: some A_j reaches 4,096
- curve 1 conj common0: first A dim 4094; chart-dependent: some A_j reaches 4,096
- curve 1 conj planted3_0[8, 13, 26]: first A dim 4095; chart-dependent: some A_j reaches 4,096
- curve 1 conj planted3_1[0, 2, 15]: first A dim 4096; control (first A gave 4,096): A_j dims [4096, 4095, 4096], NOT all 4,096

| curve | arm | target | j | A seed | dim | at ∞ | solver/oracle | total_muls | outcome |
|--:|:--|:--|--:|--:|--:|--:|--:|--:|:--|
| 1 | conj | planted[5, 6, 12, 19] | 1 | 10626833 | 4095 | 0 | 0/3 | 1.0388e11 | border unreachable at this degree |
| 1 | conj | planted[5, 6, 12, 19] | 2 | 10626834 | 4096 | 1 | 3/3 | 1.2119e11 | solved |
| 1 | conj | planted[5, 6, 12, 19] | 3 | 10626835 | 4094 | 1 | 3/3 | 1.1835e11 | solved |
| 1 | conj | common0 | 1 | 10626933 | 4096 | 1 | 2/2 | 1.2109e11 | solved |
| 1 | conj | common0 | 2 | 10626934 | 4095 | 1 | 2/2 | 1.1901e11 | solved |
| 1 | conj | common0 | 3 | 10626935 | 4095 | 1 | 2/2 | 1.1833e11 | solved |
| 1 | conj | planted3_0[8, 13, 26] | 1 | 10627833 | 4096 | 1 | 2/2 | 1.1789e11 | solved |
| 1 | conj | planted3_0[8, 13, 26] | 2 | 10627834 | 4096 | 1 | 2/2 | 1.1964e11 | solved |
| 1 | conj | planted3_0[8, 13, 26] | 3 | 10627835 | 4095 | 1 | 2/2 | 1.1793e11 | solved |
| 1 | conj | planted3_1[0, 2, 15] | 1 | 10627933 | 4096 | 2 | 0/0 | 1.1977e11 | solved |
| 1 | conj | planted3_1[0, 2, 15] | 2 | 10627934 | 4095 | 2 | 0/0 | 1.1911e11 | solved |
| 1 | conj | planted3_1[0, 2, 15] | 3 | 10627935 | 4096 | 2 | 0/0 | 1.1831e11 | solved |

