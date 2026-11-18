# Results: plain XL at degree 5 on the m·ℓ = 21 cells (lead L2)

Class: stage diagnostic. No `S`, no rho ratio, no speed claim.
Preregistration: `PREREGISTRATION.md` (commit 88451b3e2, before any registered
or control run). Records: `runs/`; scorer output: `score-output.txt`
(`xl_dense_degree score`). Report: `reports/run-20261018T2104Z/`.

## Outcome: unmeasured on all 8 registered draws

| cell | N | rows (dense) | columns | draws | outcome | rows inserted at the cap |
|---|--:|--:|--:|---|---|---|
| m3 (19, 7) | 40 | 218,848 | 619,720 | 0, 3, 5, 7 | **unmeasured** x4 (6.3 GB pivot cap, no verdict) | 104,192–104,640 (47.6–47.8%) |
| m3 (21, 7) | 42 | 278,978 | 778,324 | 0, 2, 3, 6 | **unmeasured** x4 (6.3 GB pivot cap, no verdict) | 78,016–79,488 (28.0–28.5%) |

Each run took 242–292 s of engine time, peak RSS 6.28–6.32 GB, exit 0
(`runs/queue.log`). No draw refuted before the cap. A run stopped at the cap
says nothing about XL at degree 5: it is neither "refuted" nor "XL degree
>= 6". The question (does plain XL at 5 refute the system on these cells)
is **unanswered** under the 8 GB address-space / 7 GB-hold limit, and the
limit was not raised. So there is no closure-degree-5 conclusion and no
degree-5 point or slope to derive.

## The brief's size was wrong (measured)

The brief expected 34,482 rows (42 x C(40, <=2)) and 3.3 GB. The systems carry
lower-degree equations, so the degree-5 Macaulay matrix at N = 40 has 218,918
rows (218,848 after exact singleton peeling) over 619,790 occurring columns
(of C(40, <=5) = 760,099): a 17.0 GB dense matrix. N = 42: 279,048 rows,
778,394 columns, 27.1 GB (`runs/sizes-count-only.jsonl`). Echelon pivots store
from their leading word on; on the unregistered N = 34 smoke they took 61% of
the dense size.

## Controls (all pass; `runs/ctl-*`)

- Positive, degree 5: m3 (10, 2) and (12, 2), draws 0–3 each (committed XL
  degree 5): open at degree 4, refuted at 5, 8 of 8 each way.
- Positive, degree 6: m3 (5, 3) draws 9, 14: open at 5, refuted at 6.
- Soundness: m3 (4, 3) satisfiable draws 0–3, degrees 5 and 6: 0 of 8 refuted.
- Soundness on the registered cells' satisfiable draws (7 of 7) was cut off at
  the same cap, unmeasured: it adds no evidence either way.
- Unit tests: engine agrees with `closure_refutation_scan` on 48 random
  systems (n = 6–10).

## Predictions

| | prediction | confidence | outcome |
|---|---|--:|---|
| P1 | N = 40: >= 3 of 4 draws unmeasured at the cap | 75% | held (4 of 4) |
| P2 | no draw refuted at 5 | 85% | held (none refuted; but none finished, so the claim is vacuous: not tested) |
| P3 | all four N = 42 unmeasured | 90% | held (4 of 4) |
| P4 | all controls pass | 95% | held |
| P5 | a measured draw is "not refuted" | 85% | untested (no draw measured) |

## Unmeasured and what would settle it

Degree 5 at N = 40 needs more than 6.3 GB of dense pivots (47.7% of the rows
were in when the cap hit, and the rank was about equal to the rows inserted:
about 104 k pivots). Settling it needs either more memory than the limit
allows (this dense engine) or an engine that does not store dense pivots,
such as a sparse-with-Markowitz or Wiedemann-style rank test of the same
matrix. Smoke (unregistered, disclosed in the preregistration): m3 (16, 6)
draw 0, N = 34, degree 5, completes in 72 s with 2.4 GB and is **not
refuted** (rank 108,150 of 114,776 rows).
