Per instance: the matched counted rho (mean S over its runs) and every configuration run end to end. `S` = total group-addition equivalents / sqrt(r), mean over repeats (min–max); `S / rho` = mean s_over_rho; `ok` = every repeat recovered the planted logarithm; `hit` = decomposition hit rate; `tried` = targets tried; `solver` = engine, priced_by, mean calls, budget hits, share of the row's GAE, wall per call.

| instance (r) | configuration | S mean (min–max) | S / rho | ok | hit | tried | rows / rank | solver |
|:--|:--|--:|--:|:--|--:|--:|--:|:--|
| `icv1-f2m15-tm249-9e24ae34` (r = 5503) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.64 | 1.00 | y | | | | |
| | `binary-subspace[dimension=8] + mitm[m=2] + — + walk` | 474.05 (473.30–474.80) | 179.76 | y | 0.6406 | 48 | 30 / 30 |  |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4 + walk` | 357.54 (242.85–472.24) | 135.58 | y | 0.6406 | 48 | 30 / 30 | `inherited-f4` measured, 48 calls, 0 over budget, 97% of GAE, 1.91 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4-next + walk` | 357.54 (242.85–472.24) | 135.58 | y | 0.6406 | 48 | 30 / 30 | `inherited-f4-next` measured, 48 calls, 0 over budget, 97% of GAE, 1.41 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + fes-f2 + walk` | 426.51 (289.74–563.28) | 161.74 | y | 0.6406 | 48 | 30 / 30 | `fes-f2` measured, 48 calls, 0 over budget, 97% of GAE, 0.18 ms/call |
| `icv1-f2m21-tm1325-b6925563` (r = 1049239) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.36 | 1.00 | y | | | | |
| | `binary-subspace[dimension=11] + mitm[m=2] + — + walk` | 1975.75 (1975.75–1975.75) | 1449.00 | y | 0.6330 | 822 | 520 / 520 |  |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4 + walk` | 2758.31 (2723.40–2793.21) | 2022.93 | y | 0.6330 | 822 | 520 / 520 | `inherited-f4` measured, 822 calls, 0 over budget, 100% of GAE, 8.83 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4-next + walk` | 2758.31 (2723.40–2793.21) | 2022.93 | y | 0.6330 | 822 | 520 / 520 | `inherited-f4-next` measured, 822 calls, 0 over budget, 100% of GAE, 7.70 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + fes-f2 + walk` | 21807.90 (20967.80–22647.99) | 15993.80 | y | 0.6324 | 844 | 533 / 533 | `fes-f2` measured, 844 calls, 0 over budget, 100% of GAE, 3.13 ms/call |
| `icv1-f2m13-t181-515ee569` (r = 2003) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.76 | 1.00 | y | | | | |
| | `binary-subspace[dimension=7] + mitm[m=2] + — + walk` | 197.66 (197.66–197.66) | 71.59 | y | 0.6543 | 42 | 28 / 28 |  |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4 + walk` | 123.11 (117.79–128.43) | 44.59 | y | 0.6543 | 42 | 28 / 28 | `inherited-f4` pinned, 42 calls, 0 over budget, 88% of GAE, 0.76 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4-next + walk` | 123.11 (117.79–128.43) | 44.59 | y | 0.6543 | 42 | 28 / 28 | `inherited-f4-next` pinned, 42 calls, 0 over budget, 88% of GAE, 0.74 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + fes-f2 + walk` | 94.70 (83.71–105.69) | 34.30 | y | 0.6567 | 47 | 31 / 31 | `fes-f2` pinned, 46 calls, 0 over budget, 83% of GAE, 0.06 ms/call |
| `icv1-f2m17-tm101-00378d4e` (r = 65587) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.02 | 1.00 | y | | | | |
| | `binary-subspace[dimension=9] + mitm[m=2] + — + walk` | 493.04 (492.55–493.54) | 244.44 | y | 0.6394 | 122 | 78 / 78 |  |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4 + walk` | 249.09 (102.55–395.63) | 123.49 | y | 0.6394 | 122 | 78 / 78 | `inherited-f4` pinned, 122 calls, 0 over budget, 98% of GAE, 2.80 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4-next + walk` | 249.09 (102.55–395.63) | 123.49 | y | 0.6394 | 122 | 78 / 78 | `inherited-f4-next` pinned, 122 calls, 0 over budget, 98% of GAE, 2.78 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + fes-f2 + walk` | 493.97 (200.24–787.71) | 244.90 | y | 0.6394 | 122 | 78 / 78 | `fes-f2` pinned, 122 calls, 0 over budget, 99% of GAE, 0.30 ms/call |
| `icv1-f2m19-tm797-9c54981b` (r = 262543) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.48 | 1.00 | y | | | | |
| | `binary-subspace[dimension=10] + mitm[m=2] + — + walk` | 1063.15 (1062.78–1063.52) | 719.20 | y | 0.6900 | 244 | 164 / 164 |  |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4 + walk` | 411.01 (227.52–594.50) | 278.04 | y | 0.6900 | 244 | 164 / 164 | `inherited-f4` pinned, 244 calls, 0 over budget, 99% of GAE, 4.57 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4-next + walk` | 411.01 (227.52–594.50) | 278.04 | y | 0.6900 | 244 | 164 / 164 | `inherited-f4-next` pinned, 244 calls, 0 over budget, 99% of GAE, 4.12 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + fes-f2 + walk` | 2547.95 (2010.28–3085.63) | 1723.63 | y | 0.6719 | 393 | 262 / 262 | `fes-f2` pinned, 393 calls, 0 over budget, 100% of GAE, 0.73 ms/call |
| `icv1-f2m23-tm5197-1f85e9e1` (r = 4196903) | **matched rho** `signed-Frobenius r-adding walk, distinguished points (koblitz_signed_frobenius_rho_reference)`, 16 runs | 0.92 | 1.00 | y | | | | |
| | `binary-subspace[dimension=12] + mitm[m=2] + — + walk` | 3986.20 (3986.00–3986.40) | 4334.12 | y | 0.6238 | 1244 | 776 / 776 |  |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4 + walk` | 1852.45 (1498.17–2206.73) | 2014.13 | y | 0.6238 | 1244 | 776 / 776 | `inherited-f4` pinned, 1244 calls, 0 over budget, 100% of GAE, 16.14 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4-next + walk` | 1852.45 (1498.17–2206.73) | 2014.13 | y | 0.6238 | 1244 | 776 / 776 | `inherited-f4-next` pinned, 1244 calls, 0 over budget, 100% of GAE, 14.82 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + fes-f2 + walk` | 18444.03 (17128.04–19760.03) | 20053.83 | y | 0.6225 | 939 | 584 / 584 | `fes-f2` pinned, 939 calls, 0 over budget, 100% of GAE, 12.00 ms/call |
