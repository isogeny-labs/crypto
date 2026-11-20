# Summary v7 (next_leads_20261120)

8 runs; q = [16001]

**Correctness:** 8/8 solved (IC: solved and e2e_ok; rho: rho_solved); failures: []

**C0:** comb6 == v6 comb on 0/0; diffs []
**C0':** comb7/comb7_aa == comb6 on 0/0; diffs []

| q | S1 setup comb7/comb6 | S2 t_ic comb7/comb6 | A/A comb7_aa/comb7 | d25/comb7 | d30/comb7 | d35/comb7 | lp40/lp50 | lp50/comb7 | M1 rho_m/rho_s steps/s |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 16001 | – | – | 0.987 [0.987, 0.987] n=1 | 1.065 [1.065, 1.065] n=1 | 0.898 [0.898, 0.898] n=1 | 0.945 [0.945, 0.945] n=1 | 1.192 [1.192, 1.192] n=1 | 1.224 [1.224, 1.224] n=1 | – |

**M2:** rho_m solved 1/1, abandoned_walks max 0; steps_over_expected median 0.237 [0.237, 0.237]


| q | tag | t_setup | t_rel | t_filter | t_LA | t_desc | t_ic | / rho_m exp | / rho_m actual | / rho_s exp |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 16001 | comb7 | 0.13 | 0.75 | 0.017 | 1.54 | 0.25 | 2.69 | 1.976 | 8.434 | nan |
| 16001 | comb7_aa | 0.16 | 0.74 | 0.015 | 1.51 | 0.22 | 2.65 | 1.950 | 8.325 | nan |
| 16001 | d25 | 0.11 | 1.72 | 0.016 | 0.56 | 0.45 | 2.86 | 2.105 | 8.985 | nan |
| 16001 | d30 | 0.11 | 1.37 | 0.018 | 0.68 | 0.23 | 2.41 | 1.774 | 7.574 | nan |
| 16001 | d35 | 0.11 | 1.12 | 0.014 | 0.99 | 0.31 | 2.54 | 1.866 | 7.968 | nan |
| 16001 | lp40 | 0.11 | 2.36 | 0.000 | 0.63 | 0.82 | 3.92 | 2.883 | 12.307 | nan |
| 16001 | lp50 | 0.11 | 1.49 | 0.000 | 1.04 | 0.65 | 3.29 | 2.418 | 10.321 | nan |

- rho_m q=16001: steps/s median 2.108e+07, expected time median 1.36 s, actual median 0.32 s

§5a band: median (w * IC native mulmods) / (rho_m expected group operations), best IC arm per q:

- q=16001 best=d30: w=0 0.000, w=0.1 8.364, w=1 83.643 (native 2.398e+09, rho group ops 2.867e+07)
- load1 median 0.70, max 0.98
