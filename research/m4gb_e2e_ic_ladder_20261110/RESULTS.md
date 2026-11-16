Per instance: the matched counted rho (mean S over its runs) and every configuration run end to end. `S` = total group-addition equivalents / sqrt(r), mean over repeats (min–max); `S / rho` = mean s_over_rho; `ok` = every repeat recovered the planted logarithm; `hit` = decomposition hit rate; `tried` = targets tried; `solver` = engine, priced_by, mean calls, budget hits, share of the row's GAE, wall per call.

| instance (r) | configuration | S mean (min–max) | S / rho | ok | hit | tried | rows / rank | solver |
|:--|:--|--:|--:|:--|--:|--:|--:|:--|
| `icv1-f2m15-tm249-9e24ae34` (r = 5503) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.64 | 1.00 | y | | | | |
| | `binary-subspace[dimension=8] + mitm[m=2] + — + walk` | 473.84 (473.10–474.59) | 179.68 | y | 0.6406 | 48 | 30 / 30 |  |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + inherited-f4 + walk` | 287.31 (195.56–379.06) | 108.95 | y | 0.6406 | 48 | 30 / 30 | `inherited-f4` measured, 48 calls, 0 over budget, 96% of GAE, 1.89 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + f4-f2 + walk` | 1735.90 (1166.62–2305.18) | 658.26 | y | 0.6406 | 48 | 30 / 30 | `f4-f2` measured, 48 calls, 0 over budget, 99% of GAE, 7.22 ms/call |
| | `binary-subspace[dimension=8] + descent-algebraic[m=2] + m4gb-f2 + walk` | 124118.48 (84542.80–163694.16) | 47066.49 | y | 0.6406 | 48 | 30 / 30 | `m4gb-f2` measured, 48 calls, 0 over budget, 100% of GAE, 25.79 ms/call |
| `icv1-f2m21-tm1325-b6925563` (r = 1049239) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.36 | 1.00 | y | | | | |
| | `binary-subspace[dimension=11] + mitm[m=2] + — + walk` | 1975.73 (1975.73–1975.74) | 1448.99 | y | 0.6330 | 822 | 520 / 520 |  |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + inherited-f4 + walk` | 2911.76 (2874.91–2948.62) | 2135.47 | y | 0.6330 | 822 | 520 / 520 | `inherited-f4` measured, 822 calls, 0 over budget, 100% of GAE, 9.97 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + f4-f2 + walk` | 705787.23 (705782.84–705791.63) | 517620.85 | y | 0.6330 | 822 | 520 / 520 | `f4-f2` measured, 822 calls, 0 over budget, 100% of GAE, 294.80 ms/call |
| | `binary-subspace[dimension=11] + descent-algebraic[m=2] + m4gb-f2 + walk` | 1285977.24 (1254182.51–1317771.98) | 943129.33 | y | 0.6330 | 822 | 520 / 520 | `m4gb-f2` measured, 822 calls, 0 over budget, 100% of GAE, 237.41 ms/call |
| `icv1-f2m13-t181-515ee569` (r = 2003) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.76 | 1.00 | y | | | | |
| | `binary-subspace[dimension=7] + mitm[m=2] + — + walk` | 197.66 (197.66–197.66) | 71.59 | y | 0.6543 | 42 | 28 / 28 |  |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + inherited-f4 + walk` | 123.11 (117.79–128.43) | 44.59 | y | 0.6543 | 42 | 28 / 28 | `inherited-f4` pinned, 42 calls, 0 over budget, 88% of GAE, 0.97 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + f4-f2 + walk` | 455.44 (424.38–486.50) | 164.96 | y | 0.6543 | 42 | 28 / 28 | `f4-f2` pinned, 42 calls, 0 over budget, 97% of GAE, 2.85 ms/call |
| | `binary-subspace[dimension=7] + descent-algebraic[m=2] + m4gb-f2 + walk` | 136908.29 (128520.02–145296.57) | 49588.79 | y | 0.6543 | 42 | 28 / 28 | `m4gb-f2` measured, 42 calls, 0 over budget, 100% of GAE, 16.06 ms/call |
| `icv1-f2m17-tm101-00378d4e` (r = 65587) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 2.02 | 1.00 | y | | | | |
| | `binary-subspace[dimension=9] + mitm[m=2] + — + walk` | 493.04 (492.55–493.54) | 244.44 | y | 0.6394 | 122 | 78 / 78 |  |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + inherited-f4 + walk` | 249.09 (102.55–395.63) | 123.49 | y | 0.6394 | 122 | 78 / 78 | `inherited-f4` pinned, 122 calls, 0 over budget, 98% of GAE, 3.15 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + f4-f2 + walk` | 1843.65 (745.30–2942.00) | 914.04 | y | 0.6394 | 122 | 78 / 78 | `f4-f2` pinned, 122 calls, 0 over budget, 100% of GAE, 25.38 ms/call |
| | `binary-subspace[dimension=9] + descent-algebraic[m=2] + m4gb-f2 + walk` | 176669.07 (72915.26–280422.88) | 87588.39 | y | 0.6394 | 122 | 78 / 78 | `m4gb-f2` measured, 122 calls, 0 over budget, 100% of GAE, 46.66 ms/call |
| `icv1-f2m19-tm797-9c54981b` (r = 262543) | **matched rho** `negation-map r-adding walk: look-ahead, short-cycle escape by doubling, distinguished points, stride starts`, 16 runs | 1.48 | 1.00 | y | | | | |
| | `binary-subspace[dimension=10] + mitm[m=2] + — + walk` | 1063.15 (1062.78–1063.52) | 719.20 | y | 0.6900 | 244 | 164 / 164 |  |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + inherited-f4 + walk` | 411.01 (227.52–594.50) | 278.04 | y | 0.6900 | 244 | 164 / 164 | `inherited-f4` pinned, 244 calls, 0 over budget, 99% of GAE, 5.25 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + f4-f2 + walk` | 6709.06 (3567.97–9850.15) | 4538.53 | y | 0.6900 | 244 | 164 / 164 | `f4-f2` pinned, 244 calls, 0 over budget, 100% of GAE, 65.08 ms/call |
| | `binary-subspace[dimension=10] + descent-algebraic[m=2] + m4gb-f2 + walk` | 253620.31 (135112.69–372127.93) | 171568.46 | y | 0.6900 | 244 | 164 / 164 | `m4gb-f2` measured, 244 calls, 0 over budget, 100% of GAE, 74.05 ms/call |
| `icv1-f2m23-tm5197-1f85e9e1` (r = 4196903) | **matched rho** `signed-Frobenius r-adding walk, distinguished points (koblitz_signed_frobenius_rho_reference)`, 16 runs | 0.88 | 1.00 | y | | | | |
| | `binary-subspace[dimension=12] + mitm[m=2] + — + walk` | 3986.40 (3986.40–3986.40) | 4555.08 | y | 0.6253 | 1481 | 926 / 926 |  |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4 + walk` | 2206.73 (2206.73–2206.73) | 2521.53 | y | 0.6253 | 1481 | 926 / 926 | `inherited-f4` pinned, 1481 calls, 0 over budget, 100% of GAE, 18.08 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + f4-f2 + walk` | 1657917.78 (1657917.78–1657917.78) | 1894430.25 | y | 0.6253 | 1481 | 926 / 926 | `f4-f2` pinned, 1481 calls, 0 over budget, 100% of GAE, 1733.44 ms/call |
| | `binary-subspace[dimension=12] + descent-algebraic[m=2] + m4gb-f2 + walk` | 1834208.82 (1834208.82–1834208.82) | 2095870.33 | y | 0.6253 | 1481 | 926 / 926 | `m4gb-f2` measured, 1481 calls, 0 over budget, 100% of GAE, 462.38 ms/call |
