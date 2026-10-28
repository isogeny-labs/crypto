# Launch allocation implementation notes

## 2026-10-28T17:26Z — frozen contract and parallel implementation

### What was done (commands, files, features)

Committed additive native plan-binding metadata in ed4b66e7ece and frozen
reservation/delivery contracts in214732d66e3. T755 released producer scopes;
Conductor T761 holds the new implementation paths. The authorized coordination
comment remains on PR2790; this work does not take over its trace changes.
Two Codex agents implement budget.rs and mod.rs; a third reviews their contract.
No new worker, real grant, cloud request or permission change was made by these
commands. Current source has not yet completed its new centralized validation.

### Decisions made (and why)

Reserve finite startup/execution resources before Job creation; bind that same
allocation to actual externally observed Job/Pod IDs afterward. Preserve every
pending/bound obligation and exact retry identity. Keep existing v1 worker grants
and direct-only ledgers compatible; reject old-binary reads of nonempty new state.
A scoped ConfigMap handoff is documented as an unapplied proposal because current
launcher permissions and mounts provide no verified post-creation delivery route.
SQLite/Kubernetes primary references are linked in the committed design.

### Open threads (unfinished, pick up next time)

Finish code and review; run focused/full library tests, lint and CLI lifecycle
checks on frozen source; archive logs and receipts to the approved S3 store and
refresh PR2756. Actual startup delivery, finite regional resource admission,
worker S3 access and first fresh cloud restore/replay remain open. Verified
native cloud dataset cells remain zero. Full solver/trace/training coverage
remains in REMAINING-COVERAGE.md.

### Errors (what went wrong, how it was handled)

Conductor task show requires the UUID in this environment; T761 shorthand returned
404. Reconciled by UUID, checked scopes clear after lease expiry and explicitly
reserved them again. The shared2026-10-28 session log is owned by T755, so this
scoped note retains the implementation handoff without conflicting writes.
Independent review identified direct-CLI retry and expired-history validation
hazards; responsible agents are addressing them before centralized validation.
