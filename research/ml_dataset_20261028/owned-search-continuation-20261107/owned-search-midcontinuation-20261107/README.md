# Checkpoint retained search between classified exclusions

This source overlay checkpoints the live continuation after each successfully classified and applied model exclusion, before the next search. The Session remains alive and exclusively owns its original owner reference, source, limits and absolute deadline. A sealed callback lends immutable retained state and the same mutable owner; there is no suspension-token deserialization, reconstruction, unsafe pointer binding, new grant or retry API.

The original owning integration uses a private disjoint split of authenticated fields, original budget, records, publication and sink. The live attempt and current lift records are borrowed into the existing binary encoder. The publication helper is extracted from the existing checkpoint implementation; both checkpoint paths use that helper. Failed snapshot receipts and pending bytes remain in the same Publication; helper failure stops the original budget. The loop retains the live attempt before interpreting errors. A post-publication deadline or owner refusal preserves callback output and makes continuation failure sticky.

Cadence is exactly once after each successful exclusion, bounded by the original partition limit. There are no checkpoints within the native scheduler/search or inside an unfinished exclusion. The initial pre-Matrix checkpoint and final worker checkpoint remain. Encoding is a complete retained-state snapshot, so its growing byte/work cost may consume the remaining original budget; this change does not promise every mutation is durable. Callback publication charges belong to the original parent ledger, not the local continuation charge. The checkpoint entry itself adds one local work unit and performs original-owner/deadline pre- and postchecks. No stopped owner gets new publication supply.

The owner overlay also registers sibling owned_search_report.rs and adds report_taken=false, by coordination with that packet. Its file is supplied separately by the root composition.

## Checks

- 30 continuation checks pass (4 new checkpoint controls plus 26 donor controls). They compile the actual modified continuation implementation with actual retained scheduler, search and proof kernels and a structural Ledger. New controls cover unique exhaustive model traversal across checkpoints, same original ledger/deadline and cumulative charge, retained callback receipts on postcheck refusal with no retry, expired-clock callback suppression, and no local-cap reset.
- 7 existing publication bridge checks pass against the refactored implementation. CaptureEmitter, budget and encoder are explicitly structural fixtures. They cover receipts/accounting/head failures, incomplete publication, stopped budgets and successful distinct snapshots; they are not genuine CaptureEmitter execution.
- 2 new split checks compile mechanically extracted exact encoding types and methods, including with_parts, with_continuation_parts and checkpoint_active. Structural owner, expansion, budget and publication types verify genuine disjoint borrowing, current lift records, same budget/source addresses/deadline, and stopped-budget refusal. They do not authenticate an original map or execute the real serializer in this harness.
- All four source overlays parse with installed rustfmt 1.10.0-stable skip_children. The initial explicit +1.98.0 rustfmt request failed because that toolchain has no rustfmt component; the failure is retained. Full original owning loop, complete library/CLI typecheck, real capture I/O and authenticated end-to-end run are not performed.

## Replay

Use rustc +1.98.0 --edition=2021 --test checks/harness.rs -o checks/tests, then checks/tests --test-threads=1. Bridge and borrow harness files use absolute packet paths recorded in their source; relocate these paths when copying. Regenerate borrow_encoding.rs with python3 prepare_borrow_checks.py, then compile borrow_harness.rs similarly. No external dependencies or Cargo build are used.

Patch files apply to the hash-bound source in owned-search-capture-composed-20261107/assembled. The immutable base was not edited. No cloud work, Git mutation, generator restart, or scientific measurements were performed by this packet.
