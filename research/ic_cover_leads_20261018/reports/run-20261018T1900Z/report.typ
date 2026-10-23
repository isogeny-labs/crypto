#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Run report: run-20261018T1900Z

Study `research/ic_cover_leads_20261018`. Started 2026-10-18 19:00 UTC on a 4 vCPU shared VM (Intel Xeon), isolation L0. Observations only.

*Binary.* Release build of `examples/jv_cover.rs` at commit `536aba4ce`, whose `f4_fp.rs` and `jv_cover.rs` match main `3c192c34c`. The linear-stage split uses the same build with `raw/instrument.patch` (`eprintln` under `LEADS_DBG=1`, no change to the computation).

*Commands* (`run.sh`; common flags `--exp ccov --seeds 1 --residuals 100 --max-degree 24 --budget 300`):
- `--sizes 53,101 --constructed 20` → `raw/ccov.json`, and again with `--trace` → `raw/ccov_trace.json`;
- `F4_DEBUG=1 --sizes 101 --constructed 0` → `raw/f4steps.txt` (701 step lines, 100 systems);
- `LEADS_DBG=1 --sizes 101 --constructed 20` (instrumented) → `raw/lin_breakdown_p101.log` (120 systems).
Aggregation with awk/jq (`aggregate.sh`) → `breakdown_p101.txt`, `lin_breakdown.txt`, `f4_by_degree.txt`, `ccov_stock.txt`, `ccov_trace.txt`.

*Runs requested and completed.* Four of four. Every arm: 120 tests per p, planted decompositions 20/20, oracle mismatches 0. Stock: 0 incomplete, 0 timed out.

*Measured values* (means per test, field multiplications):
#table(columns: 5, inset: 4pt,
  [p], [C_cov], [F4], [linear], [`--trace` F4 (replayed / diverged)],
  [53], [1,224,315], [737,960], [482,159], [967,067 (95 / 24)],
  [101], [1,222,627], [736,927], [481,526], [995,628 (106 / 13)])

Linear stage at p = 101: characteristic polynomial 244,693, combination plus eigenvectors 160,337, normal forms 57,168, roots 19,328 (sum 481,526). Non-unit columns of `M_x1..M_x6`: 52.0, 49.0, 54.8, 45.9, 38.9, 20.0 (of δ ≈ 64). F4 rows per system: 1,014; reducing to zero: 124.

#figure(image("../../breakdown.svg", width: 100%), caption: [Per-test cost by part, p = 101, 120 tests, run-20261018T1900Z.])
#figure(image("../../f4_by_degree.svg", width: 80%), caption: [F4 rows per degree, p = 101, 100 systems, run-20261018T1900Z.])

*Missing data.* No wall-time spread recorded; one seed per p.
