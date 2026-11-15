Per instance: the matched counted rho (mean S over its runs) and every configuration run end to end. `S` = total group-addition equivalents / sqrt(r), mean over repeats (min–max); `S / rho` = mean s_over_rho; `ok` = every repeat recovered the planted logarithm; `hit` = decomposition hit rate; `tried` = targets tried; `solver` = engine, priced_by, mean calls, budget hits, share of the row's GAE, wall per call.

| instance (r) | configuration | S mean (min–max) | S / rho | ok | hit | tried | rows / rank | solver |
|:--|:--|--:|--:|:--|--:|--:|--:|:--|
| `icv1-f2m15-tm249-9e24ae34` (r = 5503) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.28 | 1.00 | y | | | | |
| | `binary-subspace[dimension=8] + mitm[m=2] + — + walk` | 473.67 (473.09–474.59) | 207.76 | y | 0.6141 | 46 | 28 / 28 |  |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4 + walk` | 278.79 (200.30–388.41) | 122.29 | y | 0.6141 | 46 | 28 / 28 | `inherited-f4` measured, 45 calls, 0 over budget, 96% of GAE, 1.39 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4-legacy + walk` | 278.79 (200.30–388.41) | 122.29 | y | 0.6141 | 46 | 28 / 28 | `inherited-f4-legacy` measured, 45 calls, 0 over budget, 96% of GAE, 1.52 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + fes-then-inherited + walk` | 278.79 (200.30–388.41) | 122.29 | y | 0.6141 | 46 | 28 / 28 | `fes-then-inherited` measured, 45 calls, 0 over budget, 96% of GAE, 1.39 ms/call |
| `icv1-f2m21-tm1325-b6925563` (r = 1049239) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.24 | 1.00 | y | | | | |
| | `binary-subspace[dimension=11] + mitm[m=2] + — + walk` | 1975.81 (1975.78–1975.87) | 1597.45 | y | 0.6254 | 847 | 529 / 529 |  |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4 + walk` | 4763.28 (4576.51–5019.34) | 3851.13 | y | 0.6254 | 847 | 529 / 529 | `inherited-f4` measured, 847 calls, 0 over budget, 100% of GAE, 8.72 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4-legacy + walk` | 4763.28 (4576.51–5019.34) | 3851.13 | y | 0.6254 | 847 | 529 / 529 | `inherited-f4-legacy` measured, 847 calls, 0 over budget, 100% of GAE, 9.52 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + fes-then-inherited + walk` | 4763.28 (4576.51–5019.34) | 3851.13 | y | 0.6254 | 847 | 529 / 529 | `fes-then-inherited` measured, 847 calls, 0 over budget, 100% of GAE, 8.80 ms/call |
| `icv1-f2m13-t181-515ee569` (r = 2003) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.89 | 1.00 | y | | | | |
| | `binary-subspace[dimension=7] + mitm[m=2] + — + walk` | 197.78 (197.66–198.01) | 68.38 | y | 0.6603 | 47 | 31 / 31 |  |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4 + walk` | 137.95 (117.79–167.62) | 47.70 | y | 0.6603 | 47 | 31 / 31 | `inherited-f4` pinned, 47 calls, 0 over budget, 88% of GAE, 0.91 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4-legacy + walk` | 137.95 (117.79–167.62) | 47.70 | y | 0.6603 | 47 | 31 / 31 | `inherited-f4-legacy` pinned, 47 calls, 0 over budget, 88% of GAE, 0.87 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + fes-then-inherited + walk` | 101.91 (83.71–116.31) | 35.24 | y | 0.6619 | 51 | 34 / 34 | `fes-then-inherited` pinned, 50 calls, 0 over budget, 84% of GAE, 0.06 ms/call |
| `icv1-f2m17-tm101-00378d4e` (r = 65587) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.73 | 1.00 | y | | | | |
| | `binary-subspace[dimension=9] + mitm[m=2] + — + walk` | 493.17 (492.55–493.54) | 285.87 | y | 0.6377 | 150 | 96 / 96 |  |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4 + walk` | 304.50 (102.55–415.33) | 176.51 | y | 0.6377 | 150 | 96 / 96 | `inherited-f4` pinned, 150 calls, 0 over budget, 98% of GAE, 3.36 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4-legacy + walk` | 304.50 (102.55–415.33) | 176.51 | y | 0.6377 | 150 | 96 / 96 | `inherited-f4-legacy` pinned, 150 calls, 0 over budget, 98% of GAE, 3.11 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + fes-then-inherited + walk` | 304.50 (102.55–415.33) | 176.51 | y | 0.6377 | 150 | 96 / 96 | `fes-then-inherited` pinned, 150 calls, 0 over budget, 98% of GAE, 2.68 ms/call |
| `icv1-f2m19-tm797-9c54981b` (r = 262543) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.35 | 1.00 | y | | | | |
| | `binary-subspace[dimension=10] + mitm[m=2] + — + walk` | 1063.26 (1062.78–1063.52) | 785.16 | y | 0.6861 | 276 | 186 / 186 |  |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4 + walk` | 465.45 (227.52–594.50) | 343.71 | y | 0.6861 | 276 | 186 / 186 | `inherited-f4` pinned, 276 calls, 0 over budget, 99% of GAE, 4.53 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4-legacy + walk` | 465.45 (227.52–594.50) | 343.71 | y | 0.6861 | 276 | 186 / 186 | `inherited-f4-legacy` pinned, 276 calls, 0 over budget, 99% of GAE, 5.08 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + fes-then-inherited + walk` | 465.45 (227.52–594.50) | 343.71 | y | 0.6861 | 276 | 186 / 186 | `fes-then-inherited` pinned, 276 calls, 0 over budget, 99% of GAE, 4.52 ms/call |
| `icv1-f2m23-tm5197-1f85e9e1` (r = 4196903) | **matched rho** `signed-Frobenius r-adding walk, distinguished points (koblitz_signed_frobenius_rho_reference)`, 16 runs | 0.88 | 1.00 | y | | | | |
| | `binary-subspace[dimension=12] + mitm[m=2] + — + walk` | 3986.33 (3986.00–3986.59) | 4542.18 | y | 0.6253 | 1376 | 861 / 861 |  |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4 + walk` | 2051.17 (1498.17–2448.60) | 2337.18 | y | 0.6253 | 1376 | 861 / 861 | `inherited-f4` pinned, 1376 calls, 0 over budget, 100% of GAE, 15.49 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4-legacy + walk` | 2051.17 (1498.17–2448.60) | 2337.18 | y | 0.6253 | 1376 | 861 / 861 | `inherited-f4-legacy` pinned, 1376 calls, 0 over budget, 100% of GAE, 17.02 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + fes-then-inherited + walk` | 2051.17 (1498.17–2448.60) | 2337.18 | y | 0.6253 | 1376 | 861 / 861 | `fes-then-inherited` pinned, 1376 calls, 0 over budget, 100% of GAE, 16.92 ms/call |
