## 2026-11-05 — Incremental adaptive source cursor (T-1659)

### What was done (commands, files, features)

Implemented reusable preparation and consuming bounded cursors that retain the last projective grid and two homogeneous seed pairs. Saved run reports89 tests passing,320 split-window extensions and44 small-curve extensions including20 anchors and24 recurrence steps. Eight adaptive recoveries reuse one preparation; four exceptional steps report recurrence,23 lane updates,24 converted points and one batch. Test-only actual phase() counters agree with branch selections. [pcc:unverified]

### Decisions made (and why)

Preserve the existing coefficient decoder and source formulas. Keep anchor precedence, direct seeding for an insufficient one-point prefix, private cursor state, lifetime-bound provenance and consuming failure semantics. Frozen baseline replay accepts eight outputs and rejects a corrupted scalar; its metadata consistency checks are not execution proofs. Claims PCC-20261105-99d02c and PCC-20261105-cc8f6a await signed coverage. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Bind complete-cost instrumentation before any admitted performance panel. Investigate shorter adaptive signature prefixes and collapsed targets for further query reduction. Original4563-case cohort preserved; current-head CI and review pending.

### Errors (what went wrong, how it was handled)

Eleven compiler warnings retained. No native test failure; negative replay rejects intentionally. Upstream journal conflict resolved by preserving both histories. No cloud job or timing panel launched.
