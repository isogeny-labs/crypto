# Tables (faster Wiedemann) [pcc:unverified]

One VM, single thread, wall clock, no PMU. Medians of 15 trials after 2 warm-ups; A/A = split-half (even vs odd trials).

## Before / after per p

| p | r bits | t_R in situ before ns/la_mul (min-max, A/A) | after (min-max, A/A) | speedup (median ratio; extreme range) | t_F in situ before / after ns | kappa in situ before | kappa after (trial range; A/A) | LA wall before / after s |
|---|---|---|---|---|---|---|---|---|
| 65537 | 46 | 100.7 (89.11-111.4, 1.6%) | 16.61 (9.185-23.59, 14.1%) | 6.06x (3.78-12.1) | 5.547 / 5.808 | 18.2 | 2.86 (1.29-4.34; 28.8%) | 5.25 / 0.866 |
| 131101 | 49 | 120.4 (106.2-139.5, 4.8%) | 20.31 (14.82-23.58, 5.1%) | 5.93x (4.51-9.41) | 7.246 / 6.859 | 16.6 | 2.96 (1.74-3.51; 15.9%) | 17.9 / 3.01 |
| 262147 | 53 | 104.4 (99.74-110.1, 1.3%) | 19.03 (15.98-22.94, 2.1%) | 5.49x (4.35-6.89) | 6.423 / 7.153 | 16.3 | 2.66 (2.24-3.49; 4.9%) | 44.5 / 8.11 |

## Load average (1 min), trials with load >= 1.5

| p | before min-max | after min-max | trials >= 1.5 |
|---|---|---|---|
| 65537 | 0.87-0.99 | 1.00-1.00 | 0 / 0 |
| 131101 | 1.00-1.00 | 1.00-1.00 | 0 / 0 |
| 262147 | 1.00-1.11 | 1.00-1.00 | 0 / 0 |

## Count identity on the timing instances (meta of before vs after)

| p | all counted fields equal | la_mul | la_add | la_wied_mul | la_sge_mul | rel_mul | cantor_mul | trials |
|---|---|---|---|---|---|---|---|---|
| 65537 | True | 52,122,811 | 553,133,264 | 52,122,807 | 4 | 147,841,444 | 132,214,399 | 582,790 |
| 131101 | True | 148,300,903 | 1,639,444,399 | 148,300,901 | 2 | 377,019,823 | 317,779,886 | 1,397,302 |
| 262147 | True | 426,192,389 | 4,951,428,437 | 426,192,385 | 4 | 921,021,897 | 768,769,059 | 3,373,332 |

## Re-priced frontier row: R2 SGE(8,3) arm over negation rho (FPM = cantor_mul + 10 cantor_inv + rel_mul + kappa la_mul)

| p | kappa = 9 | old study kappa (value) | new in-situ kappa (value) |
|---|---|---|---|
| 65537 | 0.300 | 17.9 (0.479) | 2.86 (0.176) |
| 131101 | 0.399 | 17.0 (0.625) | 2.96 (0.228) |
| 262147 | 0.377 | 15.8 (0.568) | 2.66 (0.198) |
