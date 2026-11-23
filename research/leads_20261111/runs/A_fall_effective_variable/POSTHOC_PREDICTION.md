# Post-hoc hypothesis, written BEFORE the follow-up runs (2026-11-12)

Status: generated after seeing the preregistered cells (i)-(iv) and the low-degree scan, so it is
NOT part of the preregistration of LEAD-A-fall-effective-variable-law. It is frozen here before any
of the follow-up cells below are run. [pcc:unverified]

## Hypothesis ("excess law")

Let v = Boolean variables after eliminating the trace form (v = N - 1 for R-type, v = N for T-type,
N = 2k) and m = n - 1 = number of non-trivial equations left after that elimination (both families).
Define the excess e = 2v - m. Then for unsatisfiable, non-degenerate instances

    d_F <= 2  iff  e <= 4,     d_F <= 3  iff  e <= 14,     d_F <= 4  iff  e <= 35.

Fitted on: low-degree scan n = 13, 15, 17 (out/lowdeg.jsonl), the four preregistered cells, and the
treatise points R33 (e=34 fall), T31 (34 fall), T33 (36 stall), R35 (36 stall), R37 (38 stall).

## Follow-up predictions (first two unsatisfiable seeds, same generators and tool)

| cell | type | n | k | N | v | m | e | excess-law | v-law (lead) |
|:--|:--|--:|--:|--:|--:|--:|--:|:--|:--|
| F1 | T | 34 | 17 | 34 | 34 | 33 | 35 | fall | stall |
| F2 | T | 37 | 18 | 36 | 36 | 36 | 36 | stall | stall |
| F3 | R | 37 | 18 | 36 | 35 | 36 | 34 | fall | stall |
| L19 | R/T | 19 | 4..9 | | | 18 | | d_F=2 iff v<=11, d_F<=3 iff v<=16 | monotone |
| L11 | R/T | 11 | 3..7 | | | 10 | | d_F=2 iff v<=7, d_F<=3 iff v<=12 | monotone |

Falsifier for the excess law: any of F1-F3 or any non-degenerate low-degree instance at n = 11, 19
off the stated thresholds.
