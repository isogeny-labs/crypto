# Native twist/factor-base experiment handoff

The request is to check the ideas into code, open a PR, and make experiments
usable by repository runners. The native implementation and manual workflow
are in [tools/twist-relations](../../tools/twist-relations/README.md).

This card is a **draft scheduler record**, not a claim of a dispatched run.
The user can run bounded diagnostics through the checkpoint-gated workflow
after merge. Promotion to a mechanical ready card also requires a merged
source pin, a real recovery contract, and an independent verifier/admission
protocol. The current executable's replay is same-implementation verification.

## Staged leads and decision value

| ID | State | Discriminating experiment |
|---|---|---|
| TR-01 | Implemented | Charged known-answer solves: random/progression/twist-run bases, equal B, identical G,Q and tracked random-walk streams; direct BSGS comparator. |
| TR-02 | Implemented | Exhaust every three-x base over the fixed small fields, average all maximum-score ties, compare canonical and both-sign coverage. |
| TR-03 | Implemented | Exhaust all finite S3 triples and cross-check complete zero sets using separate curve/twist group additions. |
| TR-04 | Implementation in progress; execution pending | Exact bounded-sampler and relation-rank census under the new [finite protocol](../../research/twist_relations_tr04_20261029/PROTOCOL.md): random/progression/collision-avoiding controls, L=1..4, B=2,3; retain all failed mass. |
| TR-05 | Planned | Three/four-point decomposition algorithms checked against exact tiny-field solution inventories, including root filtering and all elimination work. No efficient twist-assisted elimination is assumed. |
| TR-06 | Planned | Target-adaptive bases and target-identifiability stopping: compare independent information gained with newly introduced logarithm unknowns. |
| TR-07 | Planned | Frozen algorithms on held-out generic curves; cold single-target and genuinely shared-setup workloads, symmetric reuse opportunities for references. |

For TR-04, freeze thresholds before evaluation and impose a finite rejection
budget. A larger sumset alone does not establish lower complete solve cost.
The current TR-04 implementation effort has its own machine-readable protocol
and checkpoint-gated field matrix in the linked study. Its draft is not a
completed experiment. The seven selector algorithms remain separate from the
two auxiliary instrument-control fixtures; the canonical export uses nine
arms by three field instances, with both base sizes retained separately.
An exact subset-state DP represents every finite IID candidate word, while
the audit's acceleration is never treated as selector runtime. No free oracle
selector enters the executed search; an oracle inventory, if used later,
is explicitly audit-only.
For TR-05, a complete oracle solution inventory is a correctness tool, not an
efficient large-field decomposition. Mixed original/twist component labels must
remain explicit. For TR-06, solve `Ax=c` for k only when the unit row selecting
k lies in the row span; full joint rank is sufficient but not necessary. A
new relation with an unconstrained new base logarithm may add no information.

The existing [quartic-twist image-base card](../XP-20261023-quartic-twist-image-base/README.md)
has different source instances, image factors, algebraic systems and frozen
evidence. Its results are preserved and are not rerun or relabelled here.

## Prerequisites, caps and completion

Use a native Rust toolchain and the committed lockfile; every stage emits raw
JSON and source/binary hashes. Exact field/seed scopes and accounting are in
[PROTOCOL.md](PROTOCOL.md). The one-CPU, 2-GiB, 1800-second envelope is a proposed
cap, not a resource estimate derived from Python timings. Scheduler funding is
zero and no cairn task has been posted. No cloud spend is asserted by this card.

A diagnostic run is complete when its declared cases and all native algebraic
checks finish, with failed/censored cases retained. A performance conclusion
additionally needs the repository's A/A and isolation admission, independent
replay and validator, strongest matched rho, external same-computation reference,
and complete symmetric phase/GAE accounting. Those are open obligations, not
implied by a passing manifest or the historical correctness fixtures.

The implementation PR makes no leaderboard/benchmark promotion. Any later
result lands separately with its visual reports, immutable raw manifest,
validation artifacts and appropriate canonical updates.
