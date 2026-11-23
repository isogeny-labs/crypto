# Fast hash / combined binary: tables (analyze.py)  [pcc:unverified]

Gate (counted fields identical, full-line, every ic record, REF/HASH/COMBO/REF2 and both passes): 79 unit comparisons, 237 records, 0 mismatches, 0 missing, 0 verification failures, 0 retried units: **PASS**.

A/A floor on relations.cpu (REF2/REF, 4 single-pass pairs and 2 pass-means): 20.69%. q=2097091: pass1 1.010, pass2 0.793; q=4194271: pass1 1.030, pass2 1.015

Decision: HASH: NOT SUPPORTED (mean ratio 0.594..0.923); COMBO: SUPPORTED (mean ratio 0.430..0.561)

## relations.cpu ratio to REF (DL record), mean of the two passes; seeds 1 / 2 / 3

| arm | q=262111 | 524287 | 1048507 | 2097091 | 4194271 |
|:--|:--|:--|:--|:--|:--|
| HASH | 0.859 / 0.643 / 0.808 | 0.702 / 0.676 / 0.705 | 0.629 / 0.923 / 0.635 | 0.603 / 0.601 / 0.627 | 0.594 / 0.623 / 0.671 |
| COMBO | 0.521 / 0.445 / 0.517 | 0.520 / 0.555 / 0.561 | 0.499 / 0.492 / 0.509 | 0.477 / 0.430 / 0.496 | 0.455 / 0.462 / 0.470 |

Geometric mean over seeds: HASH/262111 0.764; HASH/524287 0.694; HASH/1048507 0.717; HASH/2097091 0.610; HASH/4194271 0.629; COMBO/262111 0.493; COMBO/524287 0.545; COMBO/1048507 0.500; COMBO/2097091 0.467; COMBO/4194271 0.463

## Total CPU ratio to REF (all phases, DL record; geomean, min..max over 3 seeds x 2 passes)

| arm | q=262111 | 524287 | 1048507 | 2097091 | 4194271 |
|:--|:--|:--|:--|:--|:--|
| HASH | 0.958 (0.80..1.09) | 0.917 (0.78..1.07) | 0.935 (0.85..1.08) | 0.932 (0.88..1.00) | 0.858 (0.82..0.93) |
| COMBO | 0.838 (0.75..0.89) | 0.884 (0.83..0.97) | 0.881 (0.82..0.98) | 0.874 (0.76..0.97) | 0.796 (0.71..0.87) |

A/A total CPU (REF2/REF, pass 1, pass 2): q=2097091: 1.111, 0.849; q=4194271: 1.032, 0.912

## Peak RSS (VmHWM) ratio to REF (geomean, min..max)

| arm | q=262111 | 524287 | 1048507 | 2097091 | 4194271 |
|:--|:--|:--|:--|:--|:--|
| HASH | 0.956 (0.85..1.12) | 0.979 (0.86..1.16) | 0.989 (0.86..1.16) | 1.027 (1.00..1.12) | 1.000 (1.00..1.00) |
| COMBO | 0.946 (0.85..1.06) | 0.982 (0.86..1.16) | 1.063 (0.93..1.16) | 1.002 (0.92..1.07) | 1.000 (1.00..1.00) |

REF peak RSS (kB, geomean): q=262111: 57748; q=524287: 108804; q=1048507: 211039; q=2097091: 447220; q=4194271: 916545
load1 at unit start (inside the lock): 1.00..2.30
