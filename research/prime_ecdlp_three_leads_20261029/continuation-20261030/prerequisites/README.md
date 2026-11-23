# Native prerequisite: local attempt records

`durable_attempt.rs` is a std-only Rust library primitive, outside the active
Cargo/Lean source bindings. It launches nothing and is not connected to the
PKM driver/full-panel card. Caller checkpoint/protocol/runtime gates remain
mandatory. The local unit log records5passing fault controls and strict clippy, with
source/binary hashes in [the component receipt](../validation/main-d30b9e0db-20261105/attempt-record-checks/receipt.json).
These are unsigned producer observations. [pcc:unverified]

The writer creates a unique attempt directory and immutable canonical binding
record. Explicit file and directory sync errors are propagated. A final outcome
is synced as pending before publishing a nonoverwriting hard link; an unpublished
pending file is unfinished. Duplicate attempts/finalizers cannot replace their
evidence. Inspection rejects malformed, oversized or mismatched records and
labels missing terminal outcomes unfinished, with work counts unavailable.

All metadata remain producer assertions: supplied request/source labels are not
authenticated here. A completed status is not witness/scalar/cost verification.
Successful sync requests depend on filesystem guarantees; power-loss tests,
hostile-directory races, remote persistence, source-attested execution, resume
and full-supervisor/driver integration remain outside this component. An error
can occur after a terminal link is visible; inspect reports bytes, not receipt
of the writer's durability acknowledgment. No timer/counter is inferred as zero.

The missing integration is precise: a qualified native supervisor must create
an attempt before launching a gated child, journal every terminal result, retain
unknown work after interruption and reconcile resumed attempts without an
uninterrupted timing claim. This leaf API supplies local record mechanics;
it does not implement or authorize that launch route.

Primary sources read before implementation:
- Rust File::sync_all and Drop error behavior: <https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all>.
- OpenOptions::create_new atomic nonoverwrite: <https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new>.
- Rust hard_link OS/filesystem limits: <https://doc.rust-lang.org/std/fs/fn.hard_link.html>.
- Linux fsync(2), including explicit directory sync and error propagation: <https://man7.org/linux/man-pages/man2/fsync.2.html>.

Native fault controls cover abandoned/pending-only attempts; all four terminal
labels; duplicate and publication collisions; mismatched/torn/oversize records;
and invalid bindings/I/O failures. No scientific panel is part of these checks.

## Local callback driver and read-only retry inventory

`attempt_driver.rs` now composes the record primitive with caller-supplied gate
and work callbacks. Attempt creation precedes gate evaluation; refusal omits
work. Gate/work errors publish Failed when possible, while retaining a second
publication error if recording also fails. A panic is not converted into a
success/failure receipt: missing terminal remains unfinished. The inventory
reads every explicitly supplied unique ID without rewriting old attempts;
counts and uninterrupted timing are unavailable. It is not an exhaustive scan
and cannot establish that the caller supplied every historical attempt.

[Native receipt](../validation/main-d30b9e0db-20261105/attempt-driver-checks/receipt.json)
records11passes (6new driver controls plus5record controls), strict standalone
clippy and rustfmt exit0. Failure injection covers gate/work errors, failed
creation before callbacks, pending publication collision preserving both errors,
mock unwind and separate retry, and invalid/duplicate/mismatched inventory.
These are unsigned producer observations. [pcc:unverified]

The callback Permit value is supplied data, not proof of checkpoint/protocol/
runtime qualification. This module starts no process, implements no deadline,
reaping, remote persistence or actual resume, and is not wired into the PKM CLI.
Inventory operates within the same trusted-local-directory assumptions as the
record primitive. Hostile-root/symlink races and process/power-loss durability
are unqualified. A Completed label still verifies no scalar, relation or count.

Sources reviewed before implementation: the existing G2 audit and durable record
contract; Rust catch_unwind notes (unwind only, not abort; not a generic exception
mechanism) <https://doc.rust-lang.org/std/panic/fn.catch_unwind.html>; Child::wait
and try_wait reaping contracts <https://doc.rust-lang.org/std/process/struct.Child.html#method.wait>.
Normal callback errors use Result; only the unit test catches a mock unwind.
Next integrate an actual gated native child under a qualified supervisor,
preserve observed reaping/deadlines and all failed/unfinished work, then qualify
interruption/resume without claiming one continuous clock.

## Root-child observation before publication

`child_observation.rs` now wraps an already-spawned `std::process::Child` and
uses the existing local attempt-record types. `poll` must collect an actual
exit before sidecar callbacks or outcome publication become available. None or
errors do not authorize them; accepted deadline stop alone is still nonterminal.
A nonzero observed exit maps to Failed. Deadline stop requested plus observed
exit maps to Capped, conservatively: successful kill can race a natural exit,
so this does not prove its causal reason. Cached observed success is not relabeled
by a later stop request. Exit0 is a process status, not scalar/count validation.

[Source-f543 receipt](../validation/child-observation-20261105/receipt.json)
records18native passes (7new root-child controls+11existing record/driver tests),
strict standalone clippy for wrapper/fixture and format check exit0. The exact
committed recipe checks all4source inputs and tracked cleanliness before every
phase. Dummyfixture modes are success, exit7 and blocked stdin; no crypto,
scientific panel or experiment launcher is involved. [pcc:unverified]

The library starts no process. Caller still creates the attempt before genuine
gates/launch and binds the record to that child; this association is not attested
here. Root reaping does not establish descendant cleanup, runtime isolation or
an exit-time bound. Pipe draining is not implemented. Drop does not kill/reap;
owner retains the handle on errors and must arrange cleanup. Explicit handle
transfer is for cleanup, not a resume checkpoint; it discards monitor metadata.
The pre-stop wait rejection control tests a local precondition, not an injected
OS wait failure. The record primitive persists outcome labels, not durable
reap/code/signal/stop observations. A publication error can occur after visible
terminal bytes but before a sync acknowledgment, as in its original contract.

Next connect durable observation events and exact launch identities to the
qualified native supervisor, including process-tree/deadline/interruption/resume
controls. Source/binary/command/image admission, actual checkpoint protocol and
complete costs/pricing remain open. This leaf is not the full-panel entrypoint.

Primary Rust Child docs reviewed before implementation:
<https://doc.rust-lang.org/std/process/struct.Child.html> (Drop,kill,try_wait,wait).
Repository `ReapObservation` in src/cryptanalysis/ecbench/runner.rs and the G2
accounting audit informed the no-read/no-publish-before-reap boundary. No shared
library or frozen historical source/test was modified.

## Attempt-bound declared launch and durable reap events

`recorded_child.rs` now composes the local pieces on Unix. `Prepared::begin`
creates a unique ID directory within the selected root and persists public
program/argument bytes from the same owned Command later passed to spawn.
A supplied gate callback runs afterward; refusal never calls spawn, while gate
and spawn errors retain distinct pre-child-handle records and primary/recording
errors. After a managed child exists, the Running handle stays owned by the caller
on poll/record errors; dropping it does not kill/reap the child.

Actual root PID, exit code, signal, success and stop-requested fields come from
the observed std Child and are published without overwriting before the old
outcome label. Readback rejects mismatched IDs/descriptors, inconsistent event/
outcome classes, noncanonical/torn/oversized/symlink records, and copied events
with a different attempt ID. A published reap without the final label remains
unfinished with counts/time unavailable. Pending-only records never complete it.
Successful finalization is cached; no old file is replaced on repeated polls.

[Fresh final receipt](../validation/recorded-child-20261105/final/receipt.json)
records27native passes (9new integration controls+18existing), strict wrapper/
fixture clippy and formatting all0 on sourcefc09f0f1f. The initialb9version also
passed27tests but failed strictlint1; its exact evidence is preserved separately.
The final recipe guards5committed inputs and tracked cleanliness before every
phase. The child_observation success/signal getters changed and are covered by
this fresh integrated target; oldf543receipts apply only to that old source.
These are unsigned producer observations. [pcc:unverified]

Supplied request/source/binary digests are labels, not execution attestations;
the field is explicitly `claimed_binary_sha256`. Command::get_program/get_args
capture configured public bytes, not executable/image resolution, argv0 overrides,
pre-exec behavior or environment/I/O. These classes are explicitly unbound.
No environment values are recorded. Public arguments must not contain secrets.
ID uniqueness is local to the selected root; no global namespace is proved.
The prototype can spawn its owned Command after a mock/caller gate; Permit is
not checkpoint/protocol/runtime qualification, and no production PKM entrypoint
or scientific full-panel card is wired or authorized by it.

File/directory sync requests and nonoverwriting links retain the existing
filesystem assumptions. Power-loss, hostile-root races, authenticated execution,
remote persistence and receipt of a sync acknowledgment are not established by
readback. Root reaping is not descendant cleanup or witness/scalar verification.
Stop-requested is not proof of a causal signal reason or completion-time bound.
Work counts and uninterrupted time stay unavailable. Next qualify source/binary/
image/env/protocol identities, genuine checkpoint gate, process-tree controls and
interruption/resume before a scientific route; finish counter coverage/pricing.

Sources read before implementation: Rust Command::get_program/get_args/
get_current_dir/get_envs (inherited environment is not provided by get_envs),
File::sync_all/Drop, hard_link, the earlier local primitives and G2 audit.
<https://doc.rust-lang.org/std/process/struct.Command.html>
<https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all>
<https://doc.rust-lang.org/std/fs/fn.hard_link.html>
No new ECDLP mechanism, cost law or global novelty is asserted.
