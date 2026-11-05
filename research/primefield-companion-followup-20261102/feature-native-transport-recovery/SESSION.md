## 2026-11-05 — Adaptive normalized transport recovery (T-1722)

### What was done (commands, files, features)

Added consuming transport/consumer cursors retaining the last two column vectors, middle coordinates and projective geometry tail. Bound the preserved quartic decoder to this source with explicit lane/total-window limits. Native suite passes207 tests, including100 split windows,1375 cursor advances,450 skipped residuals and20 frozen-agreeing forward-checked public recoveries. Recovery retains16 extra signs,4 skipped signs,20 extra geometry steps and480 extra conversions. Actual phase/value counters record33 direct initializations across20 recovery calls and none in their extensions. Separate checker accepts the complete479-input identity and20 records; scalar and initialization corruptions fail. [pcc:unverified]

### Decisions made (and why)

Advance all state on a skip; omit only residual evaluation. Length1 initialization may still require a direct phase/value call in its first extension. Reject exhausted/overflowed budgets before advancing. Claims PCC-20261105-456c87 and PCC-20261105-70349e have local verdict not_covered, no signed standing. Sources checked (KB): unavailable MCP/wrapper; read multiseed source, adaptive geometry Tail contract, preserved quartic/batch adapters and separate checker. Prior addition/normalization assumptions unchanged. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Instrument the complete new path against the frozen batch-reciprocal source, including setup/compiler, initial phase/value work, per-column recurrence, residual powers/resultants, allocations, decoder, skips and verification. Investigate combined transport of the monomial after accounting identifies the relevant costs. Original4563-case cohort unchanged. No comparative panel or cloud job. PR3498 merged during this round; publish this increment on its own branch. Current-head CI/review pending.

### Errors (what went wrong, how it was handled)

Initial build failed because moving the consumer into source.rs changed Rust's implicit submodule lookup. Preserved build log, receipt, dependency lock and exact failing source snapshot; added an explicit transport.rs path. No native test failure. Thirty compiler warnings remain, including inherited reference-module warnings. Published predecessor artifacts and pinned proofs unchanged.
