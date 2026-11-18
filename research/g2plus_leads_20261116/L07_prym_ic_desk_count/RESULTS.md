# L07 RESULTS

All numbers `[pcc:unverified]`. Counts only; no wall time; no benchmark row. Single thread, split_law.py about 22 s CPU.

## Gates (raw/gates.txt)

| gate | outcome |
|---|---|
| 1 TABLE | PASS |
| 2 POS d=1 (frac = 1.0, both seeds) | PASS |
| 2 POS d=2 (z = 0.609, 0.604 vs 50/101; limit 3) | PASS |
| 3 AA (gaps d=1: 0; d=2: 0.000005 (tol 0.004472); d=3: 0.000505 (tol 0.003293)) | PASS |
| 4 monotone decreasing in d, p in {101,1009,10007} | PASS |
| 5 determinism (re-run counts identical) | PASS |

Unit tests: 9 run, OK (raw/tests.txt).

## TABLE arm (raw/genus_table.tsv)

```
g	h	genus_D	dim_P	dominant	prym_2lp_exp	hyper_g_2lp_exp	rho_prym_exp	prym_beats_hyper	prym_2lp_dec	hyper_g_2lp_dec	rho_prym_dec
1	2	3	1	true	4/3	0/1	1/2	false	1.333333	0.000000	0.500000
2	3	5	2	true	8/5	1/1	1/1	false	1.600000	1.000000	1.000000
3	4	7	3	true	12/7	4/3	3/2	false	1.714286	1.333333	1.500000
4	5	9	4	true	16/9	3/2	2/1	false	1.777778	1.500000	2.000000
5	6	11	5	true	20/11	8/5	5/2	false	1.818182	1.600000	2.500000
6	7	13	6	false	24/13	5/3	3/1	false	1.846154	1.666667	3.000000
```

## Exact complete-splitting fraction C(p,d)/p^d (raw/split_law.jsonl, kind=exact)

| d | 1/d! | p=101 | p=1009 | p=10007 |
|--:|---|---|---|---|
| 1 | 1.000000e+00 | 1.000000e+00 | 1.000000e+00 | 1.000000e+00 |
| 2 | 5.000000e-01 | 4.950495e-01 | 4.995045e-01 | 4.999500e-01 |
| 3 | 1.666667e-01 | 1.617488e-01 | 1.661715e-01 | 1.666167e-01 |
| 4 | 4.166667e-02 | 3.923611e-02 | 4.141935e-02 | 4.164169e-02 |
| 5 | 8.333333e-03 | 7.536440e-03 | 8.251029e-03 | 8.325009e-03 |
| 8 | 2.480159e-05 | 1.866318e-05 | 2.412113e-05 | 2.473227e-05 |
| 9 | 2.755732e-06 | 1.909434e-06 | 2.658876e-06 | 2.745833e-06 |
| 10 | 2.755732e-07 | 1.739286e-07 | 2.635160e-07 | 2.743364e-07 |
| 11 | 2.505211e-08 | 1.424618e-08 | 2.371858e-08 | 2.491475e-08 |

## Penalty ratio splitfrac(p,g)/splitfrac(p,2g+1) (kind=penalty)

| p | g=4 (d 4 vs 9) | g=5 (d 5 vs 11) |
|--:|---|---|
| 101 | 20548.5549 | 529014.8393 |
| 1009 | 15577.7638 | 347872.0499 |
| 10007 | 15165.4107 | 334139.7938 |

## Monte-Carlo (p=101, 200000 trials per cell; kind=mc)

| d | seed | derived seed | n_complete | frac_mc | exact |
|--:|--:|--:|--:|---|---|
| 1 | 20261116 | 20261116001 | 200000 | 1.000000 | 1.000000 |
| 1 | 70261116 | 70261116001 | 200000 | 1.000000 | 1.000000 |
| 2 | 20261116 | 20261116002 | 99146 | 0.495730 | 0.495050 |
| 2 | 70261116 | 70261116002 | 99145 | 0.495725 | 0.495050 |
| 3 | 20261116 | 20261116003 | 32292 | 0.161460 | 0.161749 |
| 3 | 70261116 | 70261116003 | 32191 | 0.160955 | 0.161749 |

MC not run for d >= 4 (per SPEC). The TABLE arm column prym_beats_hyper reads false for g=1..6.

Deviations: see DEVIATIONS.md. Plot: plot.svg (exact fraction vs d, log axis, one line per p). Hashes: raw/COMMANDS.log.
