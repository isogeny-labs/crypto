# APS native export and audit continuation

This continuation adds the native [export/audit tool](../../tools/aps-export/README.md)
and [producer bridge](../../tests/aps_export_bridge.rs) after PR #2920.
Implementation and development observations have producer standing until the
required signed evidence exists [pcc:unverified].

The tool binds a frozen plan and explicit execution history to every raw row;
preserves failures and missing-slot identities; reconstructs the complete
export; and checks separate arithmetic, root-set and metric contracts. It
leaves wall qualification, full operation pricing, actual execution and sampler
history outside its attestation. Exact coverage is in
[CONTRACT.md](../../tools/aps-export/CONTRACT.md).

The editable [pipeline source](pipeline.dot), [SVG](pipeline.svg) and
[report PDF](report.pdf) describe these paths.
The final synthetic-fixture boundary is pinned at
[a6630cd83e25](https://github.com/isogeny-labs/crypto/tree/a6630cd83e25ca5eed3d88d64503f12b30402094),
with [normalized receipts](validation/source-freeze-summary.json).
The [handoff](HANDOFF.md) records decisions, sources, development failures and
remaining requirements. The earlier APS studies and frozen receipts are not
rewritten by this continuation.

Main and the local native ledger independently appended entries after sequence
80. Publication review rejected the full ledger blob because its existing
baseline contains host/session metadata and a private session link. The native
additions stay intact locally, pending explicit publication approval; this PR
withholds that ledger change and must not merge. The protected histories need
the repository owner's reconciliation path; they must not be merged by choosing
a side or weakening prefix validation.
The broad base-library command has observed failures and is still running;
no full-suite pass is asserted [pcc:unverified].

The next measurement steps remain complete operation accounting, the held-out
four-size protocol/analysis and external comparison, then maintained execution
admission. The approved source checkpoint awaits configured authentication. No
experiment, performance chart or benchmark row is created by these development
checks.
