# Graph-IC exact augmented-row basis — 2026-11-23 UTC

## What was done (commands, files, features)
- Read current AGENTS, ACTIVE, current graph catalog SOURCES/README, the canonical session-log reference (large GitHub file fetch returned empty despite nonempty blob SHA), lane handoff and current PR #5043. Confirmed PR #4988 merged, #5043 draft/open. Read exact original Rust `projected_rank.rs`, evaluator, source-bound native test and the original binary collector.
- Queried the runtime tool registry for KB/Conductor/independent workers; none exposed. Fallback: current repository source, cross-repository GitHub code search (`cryptanalysis` prime-orbit/large-prime research and `crypto-autoresearcher` IC goal notes), Diem 2006 and Cavallar 2000.
- Created preregistered mathematical/control protocol `research/lp_graph_algorithms_20261123/AUGMENTED_BASIS_SELECTION_20261123.md` at `7a28163b`.
- Implemented additive original-Rust `select_augmented_basis` in `src/cryptanalysis/large_prime_graph/projected_rank.rs` at `eaf222be`: keep original small-rank pivot rows plus at most one row with nonzero residual known RHS. Count retained sparse and source terms, keep ordered source-index selectors, avoid changes to production filters. Added serde defaults for old exact-rank receipts at `e24fe011`.
- Added native original-module tests at `b071cbfa`: fixed F101 contradictory RHS example, zero/incompatible controls, 160 deterministic canonical sparse rows over F3/F7/F101/F2147446831 against independent dense original small/augmented ranks, source witness and order-preservation, atomic invalid-input refusal.
- Native validation is requested through the existing source-bound graph workflow on PR #5043. At last inspection after authored test commit `45236388569` was pending; do not call this revision tested until terminal. Prior 37/37 PASS #45233736133 belongs to predecessor CLI source; not to the new augmented-basis source.

## Decisions made (and why)
- Selected exact *augmented* row-space basis pruning instead of further inversion-count tuning: `rank(S)` alone does not preserve contradictory known RHS, but all nonzero post-elimination RHS residuals span one dimension, so pivot originals plus the first such original row preserve `rowspan([S|b])`.
- Preserve full source-bearing original rows in ascending order; do not substitute echelon combinations or promote component savings into a solver gain. The selection routine is opt-in and not wired into any ECDLP production solver. Cost includes full sparse rank audit, metadata and output copying.
- No new paid compute, merges, or parallel native builds. One coordinator with no independent worker backend. No source edits in neighboring lanes.

## Open threads
- Check latest exact-head source-bound native CI, fix compiler/test errors if any; append exact terminal receipt and current SHA.
- Qualify full unshimmed root crate and binary collector only after the unrelated `prime_root_trace.rs` parser blocker is repaired by its owner. Audit a real bounded captured binary stream as a correctness control.
- Obtain naturally generated, group-verified ordinary Prime31 two-LP source with `p=2147483647`, subgroup `ell=2147446831`, G, targets and signed relation provenance before matched forest/pivot/MST, cold IC, optimized negation-rho measurements.

## Errors (what went wrong, how it was handled)
- GitHub connector returns empty content for large canonical `docs/session-logs/2026-11-23.md`; preserved it and wrote this scoped dated log rather than overwrite concurrent sessions.
- No research-KB or Conductor/agent-spawn tools and no Cargo/local repository checkout in this runtime; recorded bounded source fallback and used existing free native CI without claiming unexecuted tests.
