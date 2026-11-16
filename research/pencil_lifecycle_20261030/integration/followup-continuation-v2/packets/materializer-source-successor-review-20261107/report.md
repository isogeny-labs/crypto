# Independent materializer source review

[pcc:unverified]

The reviewed successor changes intermediate cleanup storage to a durable journal prefix while retaining the full final inline cleanup array. Source fidelity is supported for candidate `085234b563a2c766040706ed7d805b2971074efba14da022265a6aa6db33ca63`; runtime qualification remains pending. No worker, compiler, crash test or timing experiment ran.

## R1 — Append durability

First record uses exclusive creation; each buffered append checks byte count, flushes and fsyncs before pointer update. First directory entry is fsynced.

## R2 — Checkpoint publication

A copied snapshot omits only inline cleanup for intermediate checkpoints. Existing temporary write, file fsync, replace and directory fsync remain. Prefix metadata identifies committed bytes, SHA256 and record count.

## R3 — Interrupted prefix

Crash or exception before pointer update leaves preceding committed prefix authoritative. A trailing partial or complete unreferenced record is not evidence of successful completion. Missing pointer denotes no committed records, not no children.

## R4 — Failure and final receipt

Normal final checkpoint includes original full inline array plus storage metadata. Append/fsync failure propagates into failed status. Inline final records may exceed last committed prefix only on failure; a passed terminal record must agree with its complete journal prefix. Finalizer catches cleanup journaling errors and retains failure.

## R5 — Custody and scope

Cleanup helper is byte identical. Guard-through-finalization region is exact after replacing only cleanup recording call. Deadline, quota, ownership, source closure, checksums, command sequence and bounded cleanup remain unchanged.

## R6 — Consumer and execution qualification

No candidate code executed. Before admission, consumers must verify prefix length/hash/count and terminal status; actual durable filesystem behavior and bounded completion remain unqualified.

## Worked interruption cases

1. Partial append or fsync failure: previous receipt pointer remains authoritative, error exits through failed finalization; new inline item is diagnostic only.
2. Append fsynced, process interrupted before pointer assignment: complete tail remains unreferenced. No new command starts because the exception propagates.
3. Pointer changed in memory, receipt replace interrupted: old or new atomic receipt may remain; only its exact prefix may be read.
4. Normal completion: final inline array includes all cleanup records; checkpoint carries added storage label and journal prefix. Initial status is failed and status becomes passed only after unchanged input closure and checks.

## Blind byte-work derivation

For N child records, the prior per-child checkpoints serialize at least N(N+1)/2 record appearances. The successor appends N records and emits N once in the final inline array. This is a structural count, not measured bytes or speedup. The list is still retained in memory.

## Review provenance and limits

Preread plan SHA 900a05c7af183063cfb47901d992170c7c876c51a04baee1dde42f05f2e0c02c. Initial a243c87 source feedback identified nonexclusive creation; final085234 adds exclusive first creation and directory fsync. Base4e01 and helper2d506f were checked directly. The guard-to-finalization body matches exactly after only the cleanup recording call substitution. Scope/deadline/debits are unchanged; no new preparation configuration or launch permission is supplied.

KB tools were unavailable in the tool catalogue. Fallback used exact local sources and primary Python/Linux filesystem documentation. Python documents flush before fsync; Linux documents that persisting a new directory entry needs directory fsync. Sources: https://docs.python.org/3/library/os.html#os.fsync and https://man7.org/linux/man-pages/man2/fsync.2.html.

Next: independently qualify journal-prefix consumers and actual runtime only under a current reviewed execution envelope. No scientific knowledge promotion or rho comparison follows from this source review.

Reviewed source commit: 0c2f48f9a0738beeaece69ce32649684809335ae.
