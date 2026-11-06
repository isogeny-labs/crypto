# Generic lifecycle runtime implementation

T-950 source-only preparation. No build, test execution, solver, cohort or job was performed by this author. Root registers and validates the module on the reconciled tree. Current planning deadline22:08:40 UTC unchanged.

Sources reviewed before implementation: private all-arm-lifecycle-and-cost-patch-spec.md; existing ecbench diagnostics.rs/record.rs/runner.rs; prior isolated progress writer review; Rust1.99.1 primary docs [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html), [hard_link](https://doc.rust-lang.org/std/fs/fn.hard_link.html). Explicit ownership avoids ambient activation; no-replacement hard-link publication preserves failed files and separates visibility from synchronized durability.

API in src/cryptanalysis/ecbench/diagnostic_runtime.rs:

- DiagnosticScope::install(&Path, diagnostics::Identity) -> Result<DiagnosticScope,String>: exclusive new lifecycle-only leaf, Unix0700; entered.json publication. Identity validation reuses existing Context. Parent must be stable runner-owned and no symlink; absolute root transport remains launch adapter responsibility.
- scope.with_identity(callback) -> Result<T,String>: explicitly owned, reentrancy-rejected callback. No solver or prefix sink is invoked by runtime. Ownership token is thread-local and scope is !Send/!Sync. Drop clears ownership and performs no IO, including panic unwinding.
- scope.finish(FinishStatus::Returned|Error) -> Result<LifecycleSummary,String>: bounded finished.json, never overwrites entered.json; same implementation for every arm. Status means callback return/error only, never correctness. IO failures remain totals.failure and disable further writes; they do not fabricate mathematical output.
- read_lifecycle(path,&identity) -> Result<LifecycleRead,String>: max4 known entries,16KiB each, regular files/no Linux symlink-follow, duplicate/unknown JSON field rejection, exact identity/stage/prior IO checks, valid partial prefix and linked temporary accounting. Finished presence is not mathematical success or confirmed durability. Caller must quiesce the child first.

Numeric pencil prefix files must remain in a different directory. This module does not own or interpret them. It does not provide an automatic adapter dispatch or enable any producer. Integration must transport exact existing identity and keep candidate-only prefix overhead separate; identical generic lifecycle does not imply equal overhead.

Clock accounting: writer_ns measures bounded serialization/publication after identity/record cloning, not total instrumentation. Caller must start the full child instrumentation interval BEFORE install and end AFTER finish. Parent ingestion and manifest publication are separate overhead. A SIGKILL may leave no final summary or IO duration: absence is unknown, never zero. File and directory sync are local durability requests, not cloud persistence proof.

Fixture source (not executed here) covers nested ownership/reentrancy, panic unwind and reinstall, handled post-link failures, non-clobber, identity/duplicate-key/symlink rejection and Unix permissions. Root must run named fixtures and strict lint after module registration. Further integration fixtures and source/protocol/binary admission remain separate.
