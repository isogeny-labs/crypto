# Required root-library validation and fixture correction

## Completion update

The full command has now completed with exit 101: 5,404 passed, 26 failed and
123 default ignored. The previously pending legacy isogeny-walk case passed.
Every failure was a missing tracked compile/runtime fixture; all 26 exact-name
replays pass after materialization, against the identical native executable.
No root-library or test source was edited. There is no remaining local validation job.

The complete raw output is archived as `root-cargo-attempt3-final.log.gz`, with
its hash/size fixed in the stored-artifact binding file. The earlier incomplete
snapshot below remains historical evidence. This is not a fresh single green
full-suite run: preserve both the original nonzero exit and the separate 26 passes.
The combined coverage is all 5,430 default non-ignored cases, with 123 existing
ignored cases outside that original scope. No wall-clock performance conclusion
is drawn from the test harness's printed duration.

## Historical snapshot

Snapshot: 2026-10-30 05:11 UTC. This is development regression validation,
not a scientific experiment or a measured algorithm comparison. The full
library check is **not complete and not reported green**.

Root source revision: `7923f05fcb96bdf653545b09cba2a5d05cd3c744`.
This study adds independent research/proof projects; it does not edit the root
Rust library. The actual compiler was official Rust 1.99.1, preserved alongside
the previously installed 1.78 toolchain. The root lockfile was not changed.

## Attempts and actual corrections

1. `cargo test --release --lib --locked` under 1.78 exited 101 because a locked
   dependency uses the2024edition. Retained `root-cargo-attempt1.log`.
2. `cargo +stable test --release --lib --locked` compiled dependencies under
   Rust 1.99.1, then exited 101 on 49 `include_*` references to 40 tracked fixture
   paths outside the sparse checkout. Retained `root-cargo-attempt2.log.gz`.
   Materialized the exact tracked paths; none were classified as deleted.
3. The same current-toolchain command compiled the library and started all
   5,553 default-scope cases. At this snapshot, 5,403 passed, 26 failed on runtime
   fixture paths, 123 were pre-existing ignored cases, and one case was still
   running. The immutable snapshot is
   `root-cargo-attempt3-snapshot-20261030T0511Z.log.gz`.
4. Restored the relevant tracked emulator source, archived sessions, parameter
   files, cited reports and frozen chain constants without editing any case.
   Replayed all 26 failed cases by their exact names against the SAME native
   test executable, with one harness thread. All 26 pass, 0 fail, 5,527 filtered;
   retained `root-fixture-rerun.log.gz`. No failed case was dropped or replaced.

The running executable and the on-disk executable have the same SHA256:
`ddfd4f95408b128db7d5bb9c1effed5f2100999cc3fc3332488f9e2de02869ad`.
The binary itself is not vendored; its source/lock/toolchain and hash are
receipts. The native inventory command also completed successfully.

## The unresolved case at the historical snapshot

`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`
has not returned. It invokes the existing `run_end_to_end(101,1,8,
&[3,5,7],3,1.3,1.25,Some(64))`, including its paired route. It is an
unmodified legacy extension-field validation case, not the new prime-field
candidate and not a recorded timing result. No timeout, failure or mathematical
obstruction is inferred from its duration, and it has not been cancelled.

The actual local process at the snapshot is PID 36657; tool session 31956 owns
the full command. Its live log is `/workspace/audit/prime-root-cargo-attempt3.log`.
Its eventual status must be collected and appended as a versioned receipt.
Do not convert this snapshot or the 26 focused passes into a full-suite pass.

The default library's 123 ignored cases were not enabled; they include isolated
benchmarks and separately tracked investigations. This is the original default
scope, not a new exclusion rule. No legacy test or measurement protocol changed.

The study's own native 16-record replay, 22 certificate/hash tests and all three
Lean projects are separate completed checks. Their success does not discharge
the unresolved full-library status, global contraction formalization, novelty
or the original ECDLP goal.
