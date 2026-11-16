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

## 2026-10-28T17:37:35.124186+00:00 — source review complete

### What was done (commands, files, features)

Implemented ownerless launch reservations, read-only admission, atomic owner
binding and native reserve-launch/bind-launch commands. Added eleven ledger
regressions and two CLI regressions plus the earlier canonical plan-digest test.
Independent production review found no remaining accounting defect. The direct
reserve shortcut now rejects launch IDs. Historical recovery avoids live expiry
checks while new admission and activation retain them. Formatting/diff checks
passed; compilation and runtime tests are pending.

### Decisions made (and why)

Centralize Cargo validation after a source commit. Exercise actual previous
23b3 binary for legacy compatibility, including new-reader byte preservation
and old-reader rejection of nonempty launch state. Offline CLI fixtures create
only synthetic local records, never activated grants or external jobs.

### Open threads (unfinished, pick up next time)

Collect exact-source lint, focused/full library and CLI results; retain failures
as well as successful logs and fresh S3 readbacks. No cloud generation, delivery
permission or finite resource-lifetime evidence follows from this implementation.

### Errors (what went wrong, how it was handled)

Parent review found omitted additive CLI receipt metadata required by the frozen
contract; corrected before centralized compilation. CLI follow-up was initially
sent to an idle agent without resuming it; explicitly resumed the existing agent,
preserving its assigned scope and avoiding duplicate implementation.


# 2026-10-29 dataset continuation

## What was done (commands, files, features)

Reconciled the stopped prior test process with its retained exit receipt:
source 9c3d14b passed 5,119 full-library tests, zero failed, 123 ignored, exit 0,
and matching source readback. Collected 119 focused tests, strict Rust 1.98
maintained Clippy, release build, 22 CLI checks and nonroot container preflight.
Archived the final evidence and freshly downloaded/hash-checked all 31 objects
(42,690,609 bytes) from the approved artifact store. No cloud cell was generated.
Updated coverage/launch documentation and retained the reviewed startup proposal.

## Decisions made (and why)

Keep the initial and final validation receipts separate. Preserve all failed
publication attempts alongside the successful retry. The proposed gate must
inspect the retained worker activation phase after a crash: a durably active
session advances locally without reissuing activation PUT/GET requests. Drop
temporary session leases before handing off to existing handlers. This is a
future contract, not runtime functionality validated by the current tests.

## Open threads (unfinished, pick up next time)

Publish the validated checkpoint to PR2756, then reconcile its new main-branch
conflicts under the repository Copilot-first rule and validate the resulting
source. Implement native startup gating/delivery and finite resource admission.
The pending scoped worker IAM decision remains unanswered; do not infer its
approval from T755 coordination approval. Preserve all requested solver/trace/ML
coverage in REMAINING-COVERAGE.md and the 165,150,720-cell plan. Verified cloud
cells remain zero; the full goal remains active.

## Errors (what went wrong, how it was handled)

The old process handle was missing after interruption; retained process exit and
terminal log proved completion without rerunning it. The validation collector
initially matched a nested subprocess test summary as well as the terminal suite
summary; it stopped without writing a final record, then was corrected to anchor
the terminal summary. The tests themselves passed unchanged. A Conductor check
initially omitted --scope flags; corrected read-only invocation returned allow.
The independent startup reviewer supplied the activation-recovery finding before
its subagent hit a usage limit. No completed broader review is claimed.
