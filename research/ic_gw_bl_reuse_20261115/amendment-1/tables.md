# BLR / REF tables (analyze.py) [pcc:unverified]

G1 pass (30 pairs, 30 with a changed DL linear-algebra field, 0 other mismatches); G2 pass; G3 pass (0 failures); G4 pass (30 unit files, 0 mismatches).

Decision (every (q, seed) and w_add in 0, 0.5, 1, w_h = 0): **SUPPORTED**; also below 1 at every w_h in 0, 0.1, 1: True.

DL cold M-eq BLR/REF (w_h = 0): geomean (max) over seeds 1-6

| w_add | k = 18 | 19 | 20 | 21 | 22 |
|--:|--:|--:|--:|--:|--:|
| 0.0 | 0.8184 (0.8223) | 0.8105 (0.8136) | 0.8152 (0.8163) | 0.8189 (0.8194) | 0.8127 (0.8129) |
| 0.5 | 0.8635 (0.8656) | 0.8622 (0.8651) | 0.8656 (0.8663) | 0.8688 (0.8700) | 0.8677 (0.8687) |
| 1.0 | 0.8834 (0.8859) | 0.8837 (0.8863) | 0.8864 (0.8870) | 0.8893 (0.8905) | 0.8894 (0.8905) |

Same, w_h = 0.1 and 1 GAE on hash + cmp (geomean):

| w_add | w_h | k = 18 | 19 | 20 | 21 | 22 |
|--:|--:|--:|--:|--:|--:|--:|
| 0.0 | 0.1 | 0.8410 | 0.8360 | 0.8399 | 0.8433 | 0.8390 |
| 0.0 | 1.0 | 0.9249 | 0.9258 | 0.9274 | 0.9292 | 0.9290 |
| 0.5 | 0.1 | 0.8744 | 0.8737 | 0.8767 | 0.8797 | 0.8789 |
| 0.5 | 1.0 | 0.9269 | 0.9279 | 0.9294 | 0.9312 | 0.9313 |
| 1.0 | 0.1 | 0.8903 | 0.8908 | 0.8932 | 0.8959 | 0.8961 |
| 1.0 | 1.0 | 0.9282 | 0.9293 | 0.9307 | 0.9324 | 0.9328 |

DL1 and D identical to 1 at every cell: DL1 True, D True.

DL linear_algebra.lmul BLR/REF, geomean over seeds: k=18: 0.7387, k=19: 0.7351, k=20: 0.7318, k=21: 0.7302, k=22: 0.7288

Gap at k = 22 (seeds 1-3, w_add = 0, hash+cmp at w*gae). REF reproduces the frozen combo band (rel. err < 1e-9): **yes**.

| w | rho (frozen neg-map) | IC REF | IC BLR | rho/IC REF | rho/IC BLR | IC/rho BLR |
|--:|--:|--:|--:|--:|--:|--:|
| 0 | 3.7959e+11 | 1.0385e+09 | 8.4401e+08 | 365.5 | 449.7 | 0.00222 |
| 0.1 | 4.9091e+11 | 1.2085e+09 | 1.0140e+09 | 406.2 | 484.1 | 0.00207 |
| 1 | 1.4928e+12 | 2.7385e+09 | 2.5440e+09 | 545.1 | 586.8 | 0.00170 |
