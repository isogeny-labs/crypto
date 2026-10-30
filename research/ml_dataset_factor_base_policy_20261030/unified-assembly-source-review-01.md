# Independent unified gated assembly review01

Reviewed scratch unified-v2-integration-proposal-01 against exact base
655b690ef56d6869023a41326a127bf319cf4342:

- combined-proposal.patch:4ac2ec01b1df21ff4586883c262205b2d4cab9217e5b5d0cf4f9ed80ad95eed8
- source-inventory.json:ad2608989a04dff059a06ce76eecbbaea88261bbe945db13f02844d64ebdb2d0
- static-check-receipt.json:941f6ba094424613cb478f3bfef9dd6be4f821009653f3bbd35e3cc907c02063

No remaining blocking source finding for ISOLATED COMPILATION/SCOPED TESTING of
this GATED proposal. No build or test was executed by this reviewer. This does
not approve activation-CANDIDATE-NOT-APPROVED.patch, live installation, replacement
of owned donor source, clean committed binary generation, or corpus production.

## Composition and producer/reader trace

The16-file overlay contains12 changed files including2 new modules. Reviewed
capture/execution, startup, restore, solver03/relations and feature projection
bytes retain their component hashes. Mod/export/report differences are the
recorded contextual composition and early disabled-phase admission, not a silent
replacement of donor work. The four unchanged core API files are explicitly
bound in the inventory. CLI false export dispatch uses the historical export()
entrypoint; true uses export_with_options(). The legacy bound_run shim is now
cfg(test), and its remaining named callers are tests.

Plan/preflight: original bounded bytes enter strict PlanDocument; full typed
policy-major plan hash/global indices and protocol scope remain separate from
exact file hashes. Plan metadata truthfully declares execution disabled.

Generate/start: require_document_execution rejects V2 before protocol/work effects
in generate and before startup journal/lease/budget activation. Downstream proposed
production code retains original V2 plan bytes at root and cell, sidecar identity,
full-plan budget binding, global shard indices and exact sidecar/plan attempt
hashes. Resume checks existing cell/config/run identity rather than duplicating
unfinished work.

Worker/capture/execution: require_worker_activation rejects V2 before constructor.
Once future activation is considered, expected typed policy/config/occurrence and
parent bindings remain required. Missing both capture files is rejected based on
strict V2 plan origin, independent of deletable flags. Present capture requires
checked measurement policy. Capture checks duplicate-free V2 run/attempt bytes,
original file hashes and expected occurrence. Missing plan with V2 run/sidecar
fails in origin admission. Execution context is required for V2; an absent optional
run flag cannot remove the requirement. Constructor history remains restricted.

Verification: explicit cell-v2 rejection and require_legacy_plan_at are both
present before replay. Proposed downstream verification checks sidecar reserved
identity, expected policy/schema/history, source domain/order/actions, native
alternative preflight/domains, and both worker and final supervisor outcomes.
No empty-source negative claim is created. Empty BSGS is a separate subgroup task.

Readers: export default/opt-in share the restored early guard before output/lease;
report has the same guard. Verify/status, publish and reconcile remain gated;
publish checks before budget/lease. Restore may incur charged downloads before
schema discovery, but verify_cell blocks V2 activation. Strict full-plan/config
restore readers preserve preexisting receipts, hashes, root conflicts and replay
ordering. Opt-in subgroup artifacts remain separated from PDP features, with
known witness membership distinct from recovered scalar labels. No source/schema
or role relabeling creates a successful final claim.

## Activation is still a separate incomplete change

All gates are intentionally retained. Their mechanical removal candidate is
explicitly unapproved and leaves disabled-phase tests stale. Before activation:
reconcile T887 mod/solvers and T888 capture implementations contextually; migrate
no-effect rejection tests into enabled custody/semantic tests; validate actual
CLI producer/reader/restore paths and typed unsupported/censored profiles; add
bounded SAT producer gate-map support rather than inferring it from consumers.
No related donor source was imported in this assembly. Report/restore donor
parity remains exact; unrelated storage/S3 Result<bool> trait migration is absent.

One additional pre-activation consistency item was sent to the assembler:
bound_policy_run reads root run.json through generic read_json<Value>, whereas
V2 capture checks duplicate-free original run/attempt bytes. After gate removal,
an empty V2 export/report/status root has no cell/capture replay to enforce the
same duplicate policy. Use a V2-only duplicate-rejecting root-run parse (preserve
V1), with an empty-root rejection-before-lease test. This does not open a V2
route in the current gated proposal and is not a blocker for isolated compilation.

## Evidence boundaries

Author's static receipt records exact-context patch replay, final byte equality
and rustfmt success; this review independently checked the stated stable hashes,
source guards, relevant call sites and overlap hunks. Neither activity establishes
Rust type-checking or passing tests. Full supported native method matrix, hardware
instruction completeness, actual cloud pilot/restore, training corpus/model and
first/last-fall-degree conclusions remain outside this proposal's evidence.
