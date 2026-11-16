## 2026-11-16 05:20-05:55Z: native lin21 CDCL, deletion, learning-free leaf, n = 131 (subagent pilot)

For the parent session to append to `docs/session-logs/2026-11-16.md`. Local pilot, every
number `[pcc:unverified]`.

### What was done (commands, files, features)
- New directory `research/sat_native_lin21_n131_20261116/`: `fastcdcl_mw.rs` (copy of
  fastcdcl.rs with multi-word blocks, clause deletion, learning-free leaf), `mw.rs`
  (generic-width field, fast-leaf port, point-pair enumeration), `n131.rs`, `summary.rs`,
  `run.sh`, `PROTOCOL.md`, `README.md`, `tables.md`, `results.json`, three SVG figures, `raw/`.
- Protocol committed alone: `8fce46bd83` (05:38:50Z); first measured process 05:39:27Z.
- Measured (verified from `raw/`): small fields l = 5..13, 176 targets, 1,232 scan/solver rows,
  0 verdicts off the pair oracle; ECC2K-130 field d = 6..14, 90 targets, 0 verdicts off.
  Differential tests: 2,800 + 210 fresh instances, 0 verdict mismatches.

### Decisions made (and why)
- Small-field rule outcome: inconclusive. Slope of log2(solver / fast leaf) over l = 9, 11, 13
  is +0.007 to +0.017 with deletion or the leaf (merged: +0.097), ratio at l = 13 is 122 to 128
  (threshold 60).
- ECC2K-130 solver arm fixed to `delleaf` before measuring (no clause growth in development).
- Pair enumeration implemented in point form with batched additions (the faster form).

### Open threads (unfinished, pick up next time)
- Fast-leaf denominator at even n (no half-trace); truncating the over-determined leaf at
  n = 131; a strictly learning-free solver; rerun on an isolated host if any of this is cited.
- `research/scan_floor_leaf_20261116/scan_leaf.rs` does not compile with rustc 1.97.0 (type
  error at line 1314).

### Errors (what went wrong, how it was handled)
- First generic field reduction made the fast-leaf port 2.5x slower than the file it ports;
  fixed before the protocol (one-word two-fold path), disclosed there.
- `summary.rs` changed after the measured run (two added counts; `results.json` byte-identical);
  README deviation 1.
- One A/A outlier (0.36 at l = 8) outside the predicted single-target band.
