# Mixed extension helpers and partial-relation elimination

**Current status, 2026-10-23: partial / blocked / unfunded.** Corrected v2 panel has 336 evaluated profiles and 20,976 unique arm witnesses freshly replayed. Paired row/recovery controls match. V1 cross-arity witness overwrite is preserved as incomplete evidence. Complete helper/construction costs remain missing.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/E23.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). Local phase receipts do not complete the original experiment.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E23: proposed, blocked, not started.** Route 1; priority P1.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e23).
[Parent route card](../XP-20261021-twist-trace-factor-base/README.md).

## Question and decision value

Can general extension-point helpers reduce collection cost after every helper is eliminated, compared with the identical rational partial-relation graph?

A heldout complete-cost improvement supports this collector. A larger partial graph with unchanged useful span does not. Pure-twist-only output must agree with the E29 separation certificate.

## Inputs and method

The ten ordinary fixtures, E21 bases of sizes 8,12,16, and helper caps 16,32,64. Use relation arities 2 and 3. Admit an r-primary projector only after checking the exact extension-group order and required coprimality; otherwise emit a typed unsupported cell.

Verify each proposed relation in the actual extension group. Build unknown columns only for nonzero r-primary projections of general helpers; pure twist helpers are zero in that source component. Compare zero-helper, pure-twist, general-helper and direct-rational-helper arms. Apply the identical graph/Schur elimination policy, stopping on target-row span with recoverable scalar, and replay every final row in E. Neither known scalar witnesses nor hidden helper logs may guide collection.

## Controls and evidence owed

A deliberately fake twist-log column must be rejected. Compare the same projected helper identities, same graph, same target and same stopping rule; include graph cycles yielding only zero rows and a standard rational large-prime-style control.

Partial rows and their exact group witnesses, projector certificates, helper identities, graph components, elimination transforms, nonzero independent final rows, scalar recovery and all collection/storage/elimination costs.

## Prerequisites

Certified projectors, exact helper identities, replayable graph elimination and full counters are missing.

Dependent designs: E21, E29.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB; cap each cell at 2^20 group operations, 2^16 stored partial rows and 120 seconds. Record whichever cap fires and the unresolved target span.

These are proposed process caps, not calibrated runtime/disk estimates. Full
core-hour and wall-hour estimates remain null until a pilot and runnable
protocol exist. Funding is zero, below the existing funded seed cards; no
allocation, lab publication or spending is performed. The card retains an
intentional failing command and is not runnable.

## Before ready and before done

- [ ] Implement native Rust driver and an independent checker.
- [ ] Freeze exact inputs, source, controls, seeds, counters, complete cost intervals and resource estimates.
- [ ] Commit and hash PROTOCOL.md; pin merged code; fill checkpoints and real verifier/run argv.
- [ ] Validate the ready card and preserve every failed or inconclusive cell.
- [ ] Land the bounded result, independent replay, report, diagram and inspected PDF.
- [ ] Meet the stated decision criterion; distinguish construction, stage and pipeline results.

Novelty and any IC speedup remain unestablished. The original E00–E17
requirements are preserved. This card records a proposal, not a completed run.
