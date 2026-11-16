# Degree-eight branch-supported sparse screen

The frozen source at `30fa591b826f` completed all four representative
carrier/source cells. It covered the eight denominator shapes in
[DEGREE8_SPARSE_PROTOCOL.md](DEGREE8_SPARSE_PROTOCOL.md), including
`196,020` high-degree and `972` unique-CRT logical candidates per cell.

| carrier mask | source mask | point-filter survivors | exact identities | CPU s |
|---:|---:|---:|---:|---:|
| 32280 | 68 | 1,231 | 0 | 1.734 |
| 32280 | 84 | 1,249 | 0 | 1.731 |
| 35349 | 68 | 1,264 | 0 | 1.744 |
| 35349 | 84 | 1,285 | 0 | 1.760 |

The [four JSON receipts](.) retain all per-shape assignment, square-value
filter, degree, coprimality, divisibility and full-identity counters. Every
point filter has 4,050 direct equivalence controls per high-degree shape
and cell. Source SHA256 is retained in each receipt. There is no explicit
degree-eight map in these eight shapes on the two carrier representatives;
the checked coordinate and Frobenius transformations extend that statement
to the twelve recorded carriers and ten exact source equations.

Other degree-eight denominator shapes remain open, so this is neither a
degree-eight exclusion nor a vulnerability result. [pcc:unverified]
