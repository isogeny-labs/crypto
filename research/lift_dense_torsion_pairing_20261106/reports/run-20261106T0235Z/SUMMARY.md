## P1: T-stable valid lines = [Tr_{N/d}(b^{1/4}) = 0], and equal b^{1/4}F_q

90 curves, 48 with a T-stable valid line, 0 mismatches: **PASS**

- post hoc, n = 2: D = (k-1)/2 on 0/24 T-stable all-lift lines
- post hoc, n = 3: D = (k-1)/2 on 12/12 T-stable all-lift lines
- post hoc, n = 4: D = (k-1)/2 on 7/12 T-stable all-lift lines
## P2: pairing on T-stable all-lift lines

48 lines; pairing/translation exact on all but 0; D <= (k-1)/2 violated on 0; equality on 19/48. Positive control (F_q, b in F_q): 18 lines, 0 violations. **FAIL**

## P2': no pairing elsewhere

106 non-T-stable lines, D >= k-2 on 101: **PASS**

## P3 / P4 per cell (m = n; pooled over curves)

| N | d | n | group | lines | k (mean) | D (mean) | raw | rows | raw/rows | rho = rows/(targets D) | rho / rho(std) | rows_vec (post hoc) |
|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 12 | 6 | 2 | L_T (T-stable) | 4 | 63.0 | 22.5 | 15134 | 6504 | 2.327 | 3.613e-2 | 2.245 | 6488 |
| 12 | 6 | 2 | std (generic b) | 4 | 31.0 | 29.2 | 3771 | 3766 | 1.001 | 1.609e-2 | 1.000 | 3765 |
| 12 | 6 | 2 | poscontrol F_q, b in F_q | 2 | 63.0 | 18.0 | 7290 | 2904 | 2.510 | 4.033e-2 | 2.506 | 2892 |
| 16 | 8 | 2 | L_T (T-stable) | 4 | 255.0 | 85.0 | 14996 | 6276 | 2.389 | 9.229e-3 | 2.255 | 6269 |
| 16 | 8 | 2 | std (generic b) | 4 | 127.0 | 120.5 | 3947 | 3946 | 1.000 | 4.093e-3 | 1.000 | 3944 |
| 16 | 8 | 2 | poscontrol F_q, b in F_q | 2 | 255.0 | 78.0 | 7371 | 2766 | 2.665 | 8.865e-3 | 2.166 | 2751 |
| 20 | 10 | 2 | L_T (T-stable) | 4 | 1023.0 | 362.5 | 16749 | 6649 | 2.519 | 2.293e-3 | 2.319 | 6649 |
| 20 | 10 | 2 | std (generic b) | 4 | 511.0 | 511.0 | 4041 | 4041 | 1.000 | 9.885e-4 | 1.000 | 4041 |
| 20 | 10 | 2 | poscontrol F_q, b in F_q | 2 | 1023.0 | 335.0 | 10525 | 4540 | 2.318 | 3.388e-3 | 3.427 | 4540 |
| 18 | 9 | 2 | L_T (T-stable) | 4 | 511.0 | 151.2 | 15576 | 6109 | 2.550 | 5.049e-3 | 2.591 | 6109 |
| 18 | 9 | 2 | std (generic b) | 4 | 255.0 | 253.8 | 3955 | 3955 | 1.000 | 1.948e-3 | 1.000 | 3955 |
| 18 | 9 | 2 | poscontrol F_q, b in F_q | 2 | 511.0 | 193.0 | 7402 | 3464 | 2.137 | 4.487e-3 | 2.303 | 3464 |
| 15 | 5 | 3 | L_T (T-stable) | 4 | 31.0 | 15.0 | 2468 | 645 | 3.826 | 2.687e-2 | 2.364 | 510 |
| 15 | 5 | 3 | valid, not T-stable | 8 | 31.0 | 31.0 | 3889 | 3889 | 1.000 | 3.920e-2 | 3.449 | 3769 |
| 15 | 5 | 3 | std (generic b) | 4 | 16.0 | 16.0 | 291 | 291 | 1.000 | 1.137e-2 | 1.000 | 291 |
| 15 | 5 | 3 | poscontrol F_q, b in F_q | 2 | 13.0 | 0.5 | 0 | 0 | NaN | 0.000e0 | 0.000 | 0 |
| 21 | 7 | 3 | L_T (T-stable) | 4 | 127.0 | 63.0 | 2110 | 532 | 3.966 | 5.278e-3 | 1.762 | 532 |
| 21 | 7 | 3 | valid, not T-stable | 2 | 127.0 | 126.0 | 958 | 958 | 1.000 | 9.504e-3 | 3.174 | 958 |
| 21 | 7 | 3 | std (generic b) | 4 | 69.5 | 69.5 | 333 | 333 | 1.000 | 2.995e-3 | 1.000 | 333 |
| 21 | 7 | 3 | poscontrol F_q, b in F_q | 2 | 54.0 | 8.0 | 0 | 0 | NaN | 0.000e0 | 0.000 | 0 |
| 18 | 6 | 3 | L_T (T-stable) | 4 | 63.0 | 31.0 | 2180 | 559 | 3.900 | 1.127e-2 | 2.622 | 528 |
| 18 | 6 | 3 | valid, not T-stable | 4 | 63.0 | 62.0 | 2007 | 2005 | 1.001 | 2.021e-2 | 4.702 | 1945 |
| 18 | 6 | 3 | std (generic b) | 4 | 28.5 | 28.5 | 196 | 196 | 1.000 | 4.298e-3 | 1.000 | 168 |
| 18 | 6 | 3 | poscontrol F_q, b in F_q | 2 | 30.0 | 3.5 | 0 | 0 | NaN | 0.000e0 | 0.000 | 0 |
| 16 | 4 | 4 | L_T (T-stable) | 4 | 15.0 | 6.8 | 288 | 42 | 6.857 | 1.556e-2 | 1.320 | 28 |
| 16 | 4 | 4 | valid, not T-stable | 28 | 15.0 | 14.9 | 1833 | 1833 | 1.000 | 4.385e-2 | 3.721 | 1511 |
| 16 | 4 | 4 | std (generic b) | 4 | 7.0 | 7.0 | 33 | 33 | 1.000 | 1.179e-2 | 1.000 | 15 |
| 16 | 4 | 4 | poscontrol F_q, b in F_q | 2 | 15.0 | 5.0 | 0 | 0 | NaN | 0.000e0 | 0.000 | 0 |
| 20 | 5 | 4 | L_T (T-stable) | 4 | 31.0 | 15.0 | 580 | 78 | 7.436 | 1.300e-2 | 4.875 | 33 |
| 20 | 5 | 4 | valid, not T-stable | 28 | 31.0 | 30.9 | 1942 | 1938 | 1.002 | 2.243e-2 | 8.411 | 1702 |
| 20 | 5 | 4 | std (generic b) | 4 | 15.0 | 15.0 | 16 | 16 | 1.000 | 2.667e-3 | 1.000 | 16 |
| 20 | 5 | 4 | poscontrol F_q, b in F_q | 2 | 31.0 | 10.0 | 0 | 0 | NaN | 0.000e0 | 0.000 | 0 |

### Post hoc: rows as signed column vectors (rows_vec), relative to std and to other valid lines

| N | d | n | rho_vec(L_T)/rho_vec(std) | rho_vec(valid)/rho_vec(std) | rho_vec(L_T)/rho_vec(valid) | predicted L_T/valid = 2/2^(n-1) |
|--:|--:|--:|--:|--:|--:|--:|
| 12 | 6 | 2 | 2.240 | n/a | n/a | 1.000 |
| 16 | 8 | 2 | 2.253 | n/a | n/a | 1.000 |
| 20 | 10 | 2 | 2.319 | n/a | n/a | 1.000 |
| 18 | 9 | 2 | 2.591 | n/a | n/a | 1.000 |
| 15 | 5 | 3 | 1.869 | 3.342 | 0.559 | 0.500 |
| 21 | 7 | 3 | 1.762 | 3.174 | 0.555 | 0.500 |
| 18 | 6 | 3 | 2.889 | 5.322 | 0.543 | 0.500 |
| 16 | 4 | 4 | 1.936 | 6.748 | 0.287 | 0.250 |
| 20 | 5 | 4 | 2.062 | 7.387 | 0.279 | 0.250 |

P3 (raw/rows in [0.6, 1] x 2^(n-1) on L_T, <= 1.05 elsewhere, poscontrol >= 1.3): **FAIL** ["(12,6) L_T raw/rows 2.327", "(16,8) L_T raw/rows 2.389", "(20,10) L_T raw/rows 2.519", "(18,9) L_T raw/rows 2.550"] poscontrol_ok=false

P4 (rho(L_T)/rho(std) in [1.3, 3]; rho(valid)/rho(std) in [0.6, 1.6] x 2^(n-1)): **FAIL** ["(16,4) rho(valid)/rho(std) 3.721 vs 2^(n-1) = 8", "(20,5) rho(L_T)/rho(std) 4.875"]

## P6: Oct 27 twisted lines on L_T

27 recorded twisted rows (27 valid by this instrument); 0 T-stable (0 of them with Tr(a) = 0).

