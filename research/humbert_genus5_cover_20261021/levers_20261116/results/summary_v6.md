# Summary v6 (levers_20261116)

6 runs; q = [16001]

**Correctness:** 6/6 solved with e2e_ok; failures: []

**C0:** base == v5 dlpf50_l1 on 0/0 instances; diffs: []
**R0:** 1/1 flagged runs counted-identical; diffs: []

| q | R1 rel/base t_rel | L1 la4/base t_LA | K1 comb/base t_ic | A0 comb_aa/comb t_ic | G1' lp_comb/comb | X4 dlpxs10/comb | dlpxs05/comb | dlpxs02/comb |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 16001 | – | – | – | 0.930 [0.930, 0.930] n=1 | 1.108 [1.108, 1.108] n=1 | 1.046 [1.046, 1.046] n=1 | – | – |

**X3:** 1/1 dlpxs runs with precheck_extensions <= 5 and t_filter < 1 s; violations: []

| q | dlpxs10 mulmods/comb | dlpxs05 | dlpxs02 | dlpxs10 cand/comb | dlpxs05 | dlpxs02 |
|:--|:--|:--|:--|:--|:--|:--|
| 16001 | 0.971 | nan | nan | 1.057 | nan | nan |

| q | tag | t_setup | t_rel | t_filter | t_LA | t_desc | t_ic | ic/rho | ic/rho_neg |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 16001 | rel | 1.77 | 0.74 | 0.015 | 5.75 | 0.21 | 8.49 | 0.376 | 0.532 |
| 16001 | la4 | 1.88 | 17.35 | 0.017 | 1.77 | 0.23 | 21.25 | 0.942 | 1.332 |
| 16001 | comb | 1.63 | 0.71 | 0.016 | 1.83 | 0.26 | 4.44 | 0.197 | 0.278 |
| 16001 | comb_aa | 1.54 | 0.73 | 0.017 | 1.62 | 0.23 | 4.13 | 0.183 | 0.259 |
| 16001 | lp_comb | 1.67 | 1.43 | 0.000 | 1.13 | 0.69 | 4.92 | 0.218 | 0.308 |
| 16001 | dlpxs10 | 1.60 | 0.84 | 0.031 | 1.54 | 0.63 | 4.65 | 0.206 | 0.291 |

- load1 median 1.00, max 1.36
