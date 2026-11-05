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
