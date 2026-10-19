# Sparse FGLM for the linear stage of jv_cover (2026-10-19)

Results report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`; graphs `ccov.svg`, `dlp.svg` with PNG renders).
Run report: [`reports/run-20261018T2138Z/report.pdf`](reports/run-20261018T2138Z/report.pdf), raw records in `reports/run-20261018T2138Z/raw/`.
Frozen summary (ccov + dlp arrays, same shape as the f4_fp study's, before = dense, after = sparse; L1a as extra fields): [`summary.json`](summary.json).
Reproduce: `run.sh <release jv_cover> <outdir>`; graphs with `plot.py <run dir>`. Arms are env-selected in one binary: `JV_LIN=dense|l1a|sparse` (default sparse).

Measured, per Nagao test (120 tests per prime): linear stage 4.87e5 / 4.86e5 / 5.35e5 -> 2.30e5 / 1.99e5 / 2.30e5 (p = 53, 101, 251);
C_cov 1.265x / 1.305x / 1.311x cheaper (L1a: 1.241x / 1.250x / 1.261x). Gates: planted 20/20, mismatches 0, unverified 0, incomplete 0 in every arm.
End to end (jv_cover --exp dlp): S 1.28x cheaper at all four (p, seed), same residuals and relations, all logarithms correct;
S/rho 851/369/194/373 -> 664/286/151/293 (lower-bound comparison, AGENTS.md section 5a). Class engineering.
Tests: the `jv_cover` test module passes in release (16 passed, 1 ignored), including the new `sparse_linear_stage_agrees_with_dense` (48 systems, dense = sparse = L1a solution sets). The linear stage draws one `u64` from the caller's rng, which shifts later residuals; that exposed one residual of ideal degree 62 (a solution at infinity) to `stopped_f4_…` and `replayed_f4_…`, which asserted a stop at exactly 64. F4 stops at *at most* the bound (`F4Options::stop_staircase`), so both now assert `<= 64`.

IC leaderboard: component section "Linear stage after F4 (sparse FGLM) in the Jacobian-cover IC", outside tables A–C, read from `summary.json`.
