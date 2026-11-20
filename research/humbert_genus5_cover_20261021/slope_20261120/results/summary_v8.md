# Summary v8 (slope_20261120)

23 runs; q = [16001]

**Correctness:** 23/23 solved; failures: []

**C0:** best7 == v7 d35 on 3/3; diffs []
**C0':** best8/best8_aa == best7 on 5/5; diffs []

| q | L2 t_LA best8/best7 | t_ic best8/best7 | A/A best8_aa/best8 | d25_8/best8 | d50_8/best8 | lp50_8/best8 | M3 rho_m8/rho_m7 steps/s |
|:--|:--|:--|:--|:--|:--|:--|:--|
| 16001 | 0.214 [0.201, 0.269] n=3 | 0.699 [0.692, 0.748] n=3 | 1.024 [1.015, 1.033] n=2 | 1.484 [1.377, 1.609] n=3 | 0.881 [0.798, 0.891] n=3 | 1.380 [1.263, 1.452] n=3 | 3.629 [3.573, 3.904] n=3 |

**M4:** rho solved 6/6, abandoned max 0; steps_over_expected median rho_m7 1.086, rho_m8 1.057

| q | tag | t_setup | t_rel | t_filter | t_LA | t_desc | t_ic | / rho_m8 exp | / rho_m8 actual | / rho_m7 exp |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 16001 | best7 | 0.09 | 1.03 | 0.015 | 0.83 | 0.29 | 2.33 | 5.918 | 5.902 | 1.658 |
| 16001 | best8 | 0.09 | 1.04 | 0.015 | 0.18 | 0.28 | 1.63 | 4.420 | 4.412 | 1.239 |
| 16001 | best8_aa | 0.08 | 1.05 | 0.014 | 0.18 | 0.26 | 1.57 | 4.595 | 4.119 | 1.230 |
| 16001 | d25_8 | 0.10 | 1.69 | 0.014 | 0.11 | 0.43 | 2.32 | 6.585 | 6.446 | 1.839 |
| 16001 | d50_8 | 0.10 | 0.68 | 0.015 | 0.30 | 0.25 | 1.30 | 3.891 | 3.735 | 1.077 |
| 16001 | lp50_8 | 0.09 | 1.38 | 0.000 | 0.24 | 0.50 | 2.22 | 5.944 | 5.572 | 1.565 |

- rho_m7 q=16001: steps/s median 2.775e+07, expected time median 1.37 s, actual median 1.49 s
- rho_m8 q=16001: steps/s median 1.020e+08, expected time median 0.35 s, actual median 0.40 s


§5a band: (w * IC native mulmods) / (rho_m8 expected group operations), best IC arm per q:
- q=16001 d50_8: w=0 0.000, w=0.1 4.910, w=1 49.100
- load1 median 1.55, max 1.84
