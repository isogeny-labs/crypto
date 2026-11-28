# Scalar-preserving singleton peeling on the cross-curve panel

Both batch solves recovered the online scalar on all 48 targets. All online answers passed original-curve multiplication and independently reproduced the integrated arm's recorded anchor count for the same public input. The three groups below each contain sixteen targets.

| Genus | Total rows | Rows after peeling | Entries before → after | First-round rows with a singleton large place | Batch mulmods before → after |
|---|---:|---:|---:|---:|---:|
| 2 | 3,951 | 1,341 | 17,295 → 5,762 | 2,462 | 15,285 → 15,285 |
| 3 | 1,633 | 1,186 | 8,740 → 6,306 | 332 | 27,303 → 27,303 |
| 4 | 516 | 507 | 3,081 → 3,027 | 0 | 11,469 → 11,469 |

In genus two, 2,462 rows (62.3% of all admitted rows) already had a private large-place variable before any deletion. The complete peeling process removed 2,610 (66.1%). Genus three lost 447 rows (27.4%); genus four lost nine (1.7%). A maximum of six simultaneous rounds were needed. The count of batch modular multiplications stayed identical even per fixture: eliminating a singleton pivot has no other row to update. The available saving is in constructing or materializing rows, coefficient storage and bookkeeping. This experiment measures no wall-time improvement.

The concrete online opportunity is deferring the small-place coefficients of a first large-place occurrence until that variable is used again. Any implementation must account for a later public row or constraint that references the deferred variable and retain enough information to reconstruct its coefficients and sign. The final-system singleton census does not identify disposable rows at arrival time.

RUN.txt identifies the remote freeze. CONTROL.txt records every full/core/online scalar, dimension count and anchor count. The tested input families, seeds and prior timings are in the cross-curve holdout study.
