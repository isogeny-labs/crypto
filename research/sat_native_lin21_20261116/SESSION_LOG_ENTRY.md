## 2026-11-16T02:15Z — Pilot: 2+1 linear leaf in the field-product-gate CDCL (Claude, subagent)

### What was done (commands, files, features)
- New directory `research/sat_native_lin21_20261116/`: `lin21.rs` (standalone, `rustc -O`, includes `examples/pdp_lin_cdcl.rs` unchanged, sha256 996770f4…0f47), `summary.rs`, `run.sh`, `PROTOCOL.md` (commit ec75e74b1a before measured rows), README, tables, figure.
- Measured l = 5..12, n = 3l + 2, 16 non-decomposable + 4 planted targets per cell, seed 20261116: 642 rows, 0 disagreements with the pair oracle, 0 censored, 150 of 150 SAT models lifted. `[pcc:unverified]`
- `lin21` median conflicts 1.42–1.55 × 2^l (l = 5..12); `lin` 1.23–1.53 × C(2^l,2) (l = 5..9); at l = 9 median lin / lin21 conflicts = 254, wall 418; `lin21` wall is 1 400–1 900 × the plain native scan. `[pcc:unverified]`

### Decisions made (and why)
- Pre-registered rule outcome: "lead holds". Reading: the encoding reproduces the linear 2+1 scan inside the solver (D ≥ 0 on all 128 targets: no x1 value pruned by learning); it is not faster than the scan.
- Substitution t = k·u with k = B/A2 so that the two roots of S3(x1, t, xR) differ in bit 0 of u; needed because a gate fires only on a fully assigned block.

### Open threads (unfinished, pick up next time)
- Not explained: excess D on a few targets (max 136 at l = 12); cell dependence of consistent-leaf counts.
- Any clause-learning gain below 2^l is untested; this pilot shows none arises by itself.

### Errors (what went wrong, how it was handled)
- Measured rows were launched through a Python driver (`run.py`) before the no-Python rule was relayed; replaced by `run.sh` + `summary.rs`, kept and declared under Deviations; l = 5 cell reproduced exactly through the shell path.
- Prediction 2 upper bound (D ≤ 16) and prediction 4 lower bound (ratio ≥ 256, measured 254) missed; reported.
