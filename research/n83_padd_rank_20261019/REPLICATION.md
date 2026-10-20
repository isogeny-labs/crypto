# Replication of the padd rank-stage panel on the merged head

Requested by the owner 2026-10-20, after `origin/main` was merged into the
PR branch (merge commit `0124cca3b`, `main` at `97a4c5a86` plus seven later
commits). The first panel (Actions run 39296352594, commit `4b2b3efa6`) is
reported in [RESULT.md](RESULT.md) and stays as recorded.

This run is frozen before its timing starts:

- Same protocol as [PROTOCOL.md](PROTOCOL.md), unchanged: same 20 points, same
  arms, order, rho draws and seeds, same gates, same A/A band, same kill
  threshold (rank-stage speedup < 1.2), same `run.py`, `analyze.py` and
  bootstrap seed. Nothing is tuned after seeing the first panel.
- The binary is built from the merged head. `examples/koblitz_orbit_dlp_fast.rs`
  and `examples/n83_rho_optimization_eval.rs` are the same sources as in the
  first run apart from the merge itself; `main` changed neither, nor
  `koblitz_fast_arith`, `koblitz_index_calculus`, `semaev_decomp` or
  `binary_ecc`, which the measured path calls. It did change other library
  modules linked into the binary (Groebner and tower code), which the example
  never calls; code layout can still move timings slightly.
- Both runs are reported whatever this run shows, side by side: it is a second
  runner allocation, so the difference between the two runs is a (single)
  sample of between-allocation variation, not a new interval. If the second run
  fails a gate or its rank-stage speedup falls under 1.2, that is reported as
  such and the PR's claim is restated to cover both runs.
- The canonical N83 row and the leaderboard section keep the first run until the
  second is analysed; no number is replaced before both are in.
