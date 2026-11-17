# Session-log entry for docs/session-logs/2026-11-16.md (for the parent session to append)

## 03:20Z lin21 wall-gap pilot (research/sat_native_lin21_fast_20261116/) [pcc:unverified]

### What was done (commands, files, features)
- Profiled the original `LinCdcl` on `lin21` (seed 777): 42 % re-elimination, 40 % eager
  explanation, 7 to 10 % matrix rebuild, everything else below 3 % each.
- Wrote `fastcdcl.rs`, a modified copy (incremental Gauss-Jordan per decision level, unit rows,
  column index, unit-cut lazy explanations, value-folding gates); original included unchanged.
- Protocol commit `dfe87cd298` (02:55:26Z, PROTOCOL.md only), then `run.sh OUT "5..12" 1` and
  `run.sh OUT 13 1`: 822 rows on 176 targets (the lin21 pilot's targets plus l = 13).
- Verified: 0 verdicts different from the pair oracle, 192 SAT models lift and sum to the
  target, conflicts equal to the original on all 134 non-decomposable targets.
- Measured (medians, shared machine): lin21-fast / scan = 20.6, 23.7, 24.5, 23.2 at l = 9..12,
  25.0 at l = 13; original lin21 / lin21-fast = 61.5 to 71.1.

### Decisions made (and why)
- Rule outcome "gap closes" (threshold 30x at l = 9..12), reported as inside the A/A band:
  per-target A/A spans [0.58, 1.51].
- Kept first-UIP learning and the branching rule; no Gray code and no learning-free leaf,
  because those change the search and the lead was the engine.
- Stretch (multi-word blocks, n = 131) not run: time.

### Open threads (unfinished, pick up next time)
- Clause-propagation share of lin21-fast grows with l (5.7 % at l = 9, 23.2 % at l = 13): no
  clause deletion; the ratio to the scan should rise beyond l = 13.
- Remaining gap is in the leaf: about 12x the scan's Gauss words, 7x its row operations.
- ECC2K-130 field (n = 131) needs multi-word gate values in `fastcdcl.rs`.

### Errors (what went wrong, how it was handled)
- First launch of the measured run died in the shell before any binary ran (directory race);
  relaunched 25 s later, no row lost.
- First `prof.sed` accumulated the clause-propagation timer inside the loop; fixed before the
  protocol.
- `summary.rs` gained 6 lines after the run to print the protocol's A/A-band statement.
