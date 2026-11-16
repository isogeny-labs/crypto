## P1: T-stable valid lines = [Tr_{N/d}(b^{1/4}) = 0], and equal b^{1/4}F_q

90 curves, 48 with a T-stable valid line, 0 mismatches: **PASS**

## P2: pairing on T-stable all-lift lines

48 lines; pairing/translation exact on all but 0; D <= (k-1)/2 violated on 0; equality on 19/48. Positive control (F_q, b in F_q): 18 lines, 0 violations. **FAIL**

## P2': no pairing elsewhere

106 non-T-stable lines, D >= k-2 on 101: **PASS**

## P3 / P4 per cell (m = n; pooled over curves)

| N | d | n | group | lines | k (mean) | D (mean) | raw | rows | raw/rows | rho = rows/(targets D) | rho / rho(std) |
|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 12 | 6 | 2 | L_T (T-stable) | 4 | 63.0 | 22.5 | 15134 | 6488 | 2.333 | 3.604e-2 | 2.240 |
| 12 | 6 | 2 | std (generic b) | 4 | 31.0 | 29.2 | 3771 | 3765 | 1.002 | 1.609e-2 | 1.000 |
| 12 | 6 | 2 | poscontrol F_q, b in F_q | 2 | 63.0 | 18.0 | 7290 | 2892 | 2.521 | 4.017e-2 | 2.496 |
| 16 | 8 | 2 | L_T (T-stable) | 4 | 255.0 | 85.0 | 14996 | 6269 | 2.392 | 9.219e-3 | 2.253 |
| 16 | 8 | 2 | std (generic b) | 4 | 127.0 | 120.5 | 3947 | 3944 | 1.001 | 4.091e-3 | 1.000 |
| 16 | 8 | 2 | poscontrol F_q, b in F_q | 2 | 255.0 | 78.0 | 7371 | 2751 | 2.679 | 8.817e-3 | 2.155 |
| 20 | 10 | 2 | L_T (T-stable) | 4 | 1023.0 | 362.5 | 16749 | 6649 | 2.519 | 2.293e-3 | 2.319 |
| 20 | 10 | 2 | std (generic b) | 4 | 511.0 | 511.0 | 4041 | 4041 | 1.000 | 9.885e-4 | 1.000 |
| 20 | 10 | 2 | poscontrol F_q, b in F_q | 2 | 1023.0 | 335.0 | 10525 | 4540 | 2.318 | 3.388e-3 | 3.427 |
| 18 | 9 | 2 | L_T (T-stable) | 4 | 511.0 | 151.2 | 15576 | 6109 | 2.550 | 5.049e-3 | 2.591 |
| 18 | 9 | 2 | std (generic b) | 4 | 255.0 | 253.8 | 3955 | 3955 | 1.000 | 1.948e-3 | 1.000 |
| 18 | 9 | 2 | poscontrol F_q, b in F_q | 2 | 511.0 | 193.0 | 7402 | 3464 | 2.137 | 4.487e-3 | 2.303 |
| 15 | 5 | 3 | L_T (T-stable) | 4 | 31.0 | 15.0 | 2468 | 510 | 4.839 | 2.125e-2 | 1.869 |
| 15 | 5 | 3 | valid, not T-stable | 8 | 31.0 | 31.0 | 3889 | 3769 | 1.032 | 3.799e-2 | 3.342 |
| 15 | 5 | 3 | std (generic b) | 4 | 16.0 | 16.0 | 291 | 291 | 1.000 | 1.137e-2 | 1.000 |
| 15 | 5 | 3 | poscontrol F_q, b in F_q | 2 | 13.0 | 0.5 | 0 | 0 | NaN | 0.000e0 | 0.000 |
| 21 | 7 | 3 | L_T (T-stable) | 4 | 127.0 | 63.0 | 2110 | 532 | 3.966 | 5.278e-3 | 1.762 |
| 21 | 7 | 3 | valid, not T-stable | 2 | 127.0 | 126.0 | 958 | 958 | 1.000 | 9.504e-3 | 3.174 |
| 21 | 7 | 3 | std (generic b) | 4 | 69.5 | 69.5 | 333 | 333 | 1.000 | 2.995e-3 | 1.000 |
| 21 | 7 | 3 | poscontrol F_q, b in F_q | 2 | 54.0 | 8.0 | 0 | 0 | NaN | 0.000e0 | 0.000 |
| 18 | 6 | 3 | L_T (T-stable) | 4 | 63.0 | 31.0 | 2180 | 528 | 4.129 | 1.065e-2 | 2.889 |
| 18 | 6 | 3 | valid, not T-stable | 4 | 63.0 | 62.0 | 2007 | 1945 | 1.032 | 1.961e-2 | 5.322 |
| 18 | 6 | 3 | std (generic b) | 4 | 28.5 | 28.5 | 196 | 168 | 1.167 | 3.684e-3 | 1.000 |
| 18 | 6 | 3 | poscontrol F_q, b in F_q | 2 | 30.0 | 3.5 | 0 | 0 | NaN | 0.000e0 | 0.000 |
| 16 | 4 | 4 | L_T (T-stable) | 4 | 15.0 | 6.8 | 288 | 28 | 10.286 | 1.037e-2 | 1.936 |
| 16 | 4 | 4 | valid, not T-stable | 28 | 15.0 | 14.9 | 1833 | 1511 | 1.213 | 3.615e-2 | 6.748 |
| 16 | 4 | 4 | std (generic b) | 4 | 7.0 | 7.0 | 33 | 15 | 2.200 | 5.357e-3 | 1.000 |
| 16 | 4 | 4 | poscontrol F_q, b in F_q | 2 | 15.0 | 5.0 | 0 | 0 | NaN | 0.000e0 | 0.000 |
| 20 | 5 | 4 | L_T (T-stable) | 4 | 31.0 | 15.0 | 580 | 33 | 17.576 | 5.500e-3 | 2.062 |
| 20 | 5 | 4 | valid, not T-stable | 28 | 31.0 | 30.9 | 1942 | 1702 | 1.141 | 1.970e-2 | 7.387 |
| 20 | 5 | 4 | std (generic b) | 4 | 15.0 | 15.0 | 16 | 16 | 1.000 | 2.667e-3 | 1.000 |
| 20 | 5 | 4 | poscontrol F_q, b in F_q | 2 | 31.0 | 10.0 | 0 | 0 | NaN | 0.000e0 | 0.000 |

P3 (raw/rows in [0.6, 1] x 2^(n-1) on L_T, <= 1.05 elsewhere, poscontrol >= 1.3): **FAIL** ["(12,6) L_T raw/rows 2.333", "(16,8) L_T raw/rows 2.392", "(20,10) L_T raw/rows 2.519", "(18,9) L_T raw/rows 2.550", "(15,5) L_T raw/rows 4.839", "(18,6) L_T raw/rows 4.129", "(18,6) g2 raw/rows 1.167", "(16,4) L_T raw/rows 10.286", "(16,4) g1 raw/rows 1.213", "(16,4) g2 raw/rows 2.200", "(20,5) L_T raw/rows 17.576", "(20,5) g1 raw/rows 1.141"] poscontrol_ok=false

P4 (rho(L_T)/rho(std) in [1.3, 3]; rho(valid)/rho(std) in [0.6, 1.6] x 2^(n-1)): **PASS** []

## P6: Oct 27 twisted lines on L_T

27 recorded twisted rows (27 valid by this instrument); 0 T-stable (0 of them with Tr(a) = 0).

