Per instance: the matched counted rho (mean S over its runs) and every configuration run end to end. `S` = total group-addition equivalents / sqrt(r), mean over repeats (min–max); `S / rho` = mean s_over_rho; `ok` = every repeat recovered the planted logarithm; `hit` = decomposition hit rate; `tried` = targets tried; `solver` = engine, priced_by, mean calls, budget hits, share of the row's GAE, wall per call.

| instance (r) | configuration | S mean (min–max) | S / rho | ok | hit | tried | rows / rank | solver |
|:--|:--|--:|--:|:--|--:|--:|--:|:--|
| `icv1-f2m15-tm249-9e24ae34` (r = 5503) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.28 | 1.00 | y | | | | |
| | `binary-subspace[dimension=8] + mitm[m=2] + — + walk` | 473.71 (473.13–474.62) | 207.78 | y | 0.6141 | 46 | 28 / 28 |  |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4 + walk` | 262.68 (188.85–365.78) | 115.22 | y | 0.6141 | 46 | 28 / 28 | `inherited-f4` measured, 45 calls, 0 over budget, 96% of GAE, 1.38 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4-batched + walk` | 223.90 (162.23–311.58) | 98.21 | y | 0.6141 | 46 | 28 / 28 | `inherited-f4-batched` measured, 45 calls, 0 over budget, 95% of GAE, 1.52 ms/call |
| `icv1-f2m21-tm1325-b6925563` (r = 1049239) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.24 | 1.00 | y | | | | |
| | `binary-subspace[dimension=11] + mitm[m=2] + — + walk` | 1975.87 (1975.84–1975.93) | 1597.50 | y | 0.6254 | 847 | 529 / 529 |  |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4 + walk` | 4299.15 (4130.60–4530.25) | 3475.88 | y | 0.6254 | 847 | 529 / 529 | `inherited-f4` measured, 847 calls, 0 over budget, 100% of GAE, 7.68 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4-batched + walk` | 3631.12 (3490.02–3825.55) | 2935.77 | y | 0.6254 | 847 | 529 / 529 | `inherited-f4-batched` measured, 847 calls, 0 over budget, 100% of GAE, 7.80 ms/call |
| `icv1-f2m13-t181-515ee569` (r = 2003) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.89 | 1.00 | y | | | | |
| | `binary-subspace[dimension=7] + mitm[m=2] + — + walk` | 197.78 (197.66–198.01) | 68.38 | y | 0.6603 | 47 | 31 / 31 |  |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4 + walk` | 137.95 (117.79–167.62) | 47.70 | y | 0.6603 | 47 | 31 / 31 | `inherited-f4` pinned, 47 calls, 0 over budget, 88% of GAE, 0.83 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4-batched + walk` | 120.21 (102.87–145.90) | 41.57 | y | 0.6603 | 47 | 31 / 31 | `inherited-f4-batched` pinned, 47 calls, 0 over budget, 87% of GAE, 0.93 ms/call |
| `icv1-f2m17-tm101-00378d4e` (r = 65587) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.73 | 1.00 | y | | | | |
| | `binary-subspace[dimension=9] + mitm[m=2] + — + walk` | 493.17 (492.55–493.54) | 285.87 | y | 0.6377 | 150 | 96 / 96 |  |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4 + walk` | 304.50 (102.55–415.33) | 176.51 | y | 0.6377 | 150 | 96 / 96 | `inherited-f4` pinned, 150 calls, 0 over budget, 98% of GAE, 2.78 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4-batched + walk` | 249.66 (84.46–340.52) | 144.72 | y | 0.6377 | 150 | 96 / 96 | `inherited-f4-batched` pinned, 150 calls, 0 over budget, 98% of GAE, 2.76 ms/call |
| `icv1-f2m19-tm797-9c54981b` (r = 262543) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.35 | 1.00 | y | | | | |
| | `binary-subspace[dimension=10] + mitm[m=2] + — + walk` | 1063.26 (1062.78–1063.52) | 785.16 | y | 0.6861 | 276 | 186 / 186 |  |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4 + walk` | 465.45 (227.52–594.50) | 343.71 | y | 0.6861 | 276 | 186 / 186 | `inherited-f4` pinned, 276 calls, 0 over budget, 99% of GAE, 4.28 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4-batched + walk` | 413.64 (202.26–527.86) | 305.45 | y | 0.6861 | 276 | 186 / 186 | `inherited-f4-batched` pinned, 276 calls, 0 over budget, 99% of GAE, 4.45 ms/call |
| `icv1-f2m23-tm5197-1f85e9e1` (r = 4196903) | **matched rho** `signed-Frobenius r-adding walk, distinguished points (koblitz_signed_frobenius_rho_reference)`, 16 runs | 0.88 | 1.00 | y | | | | |
| | `binary-subspace[dimension=12] + mitm[m=2] + — + walk` | 3986.33 (3986.00–3986.59) | 4542.18 | y | 0.6253 | 1376 | 861 / 861 |  |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4 + walk` | 2051.17 (1498.17–2448.60) | 2337.18 | y | 0.6253 | 1376 | 861 / 861 | `inherited-f4` pinned, 1376 calls, 0 over budget, 100% of GAE, 13.81 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4-batched + walk` | 1674.33 (1223.11–1998.99) | 1907.79 | y | 0.6253 | 1376 | 861 / 861 | `inherited-f4-batched` pinned, 1376 calls, 0 over budget, 100% of GAE, 12.55 ms/call |
