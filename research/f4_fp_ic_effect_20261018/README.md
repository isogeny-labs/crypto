# The faster `f4_fp` (PR #1717) in the end-to-end index calculus (2026-10-18)

Results report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`; graphs `dlp.svg`, `ccov.svg` with PNG renders).
Run report: [`reports/run-20261018T1548Z/report.pdf`](reports/run-20261018T1548Z/report.pdf), raw records in `reports/run-20261018T1548Z/raw/`.
Frozen summary for the leaderboard: [`summary.json`](summary.json). Reproduce: `run.sh` with release builds of
`examples/jv_cover.rs` against `f4_fp.rs` before (`4b6e09b78`) and after #1717; graphs with `plot.py <run dir>`.

`examples/jv_cover.rs` (Nagao decompositions on a genus-2 cover of E(F_{p^6}), every decomposition one `f4_fp` solve),
same seeds, same relations, every logarithm correct in both arms:

| p, seed | S before ÷ after | S/rho before | S/rho after (distance to rho) |
|:--|--:|--:|--:|
| 53, 1 | 5.05 | 4297 | 850 |
| 53, 2 | 4.99 | 1843 | 370 |
| 101, 1 | 5.06 | 981 | 194 |
| 101, 2 | 5.08 | 1897 | 373 |

Per decomposition test: F4 multiplications 7.7× fewer, C_cov 5.0× fewer (p = 53, 101, 251). Class engineering.
S/rho is a lower-bound comparison: neither arm emits its unpriced native work (AGENTS.md §5a).
The Koblitz binary-field pipeline does not use `f4_fp` and is unaffected.
