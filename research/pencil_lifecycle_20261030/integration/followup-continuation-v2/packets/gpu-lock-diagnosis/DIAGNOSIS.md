# GPU capsule lock failure: read-only diagnosis

The full library run failed at gpu.rs:1270:47, the immediate nonblocking reacquisition of BenchLock after `run` returned. Its native terminal result is 5,792 passed, one failed and 128 ignored. The separately retained isolated replay passed its single test with exit code zero. The latter does not replace or turn the failed full-suite result green.

All preceding capsule value assertions were independently recomputed from the retained bytes and passed: expected selftest_failed, exact host SHA256, environment prefix, exact lock path and CUDA elapsed-scope text. The selftest executable exited zero with empty output, as this fixture intends; its selftest_failed status is not the test failure. At the later inspection, no /proc/locks entry matched the retained lock inode. There is no lock-owner snapshot from the failing instant.

## Source and operating-system mechanism

BenchLock in source/isolation.rs:400–444 owns a File, acquires nonblocking flock, and relies on File closure for release. There is no explicit LOCK_UN destructor. Its error message labels every unsuccessful flock call as another benchmark holding the lock; it does not retain errno. Therefore the panic proves a nonzero flock result, not by itself EWOULDBLOCK or the identity of another holder.

The Linux flock manual documents that locks attach to open file descriptions: fork/dup duplicates share one lock, and closing a descriptor releases it only after all references are closed, unless explicitly unlocked. Independently opened descriptors are treated separately. [Linux flock(2)](https://man7.org/linux/man-pages/man2/flock.2.html)

Fork inherits the parent's open descriptors referring to the same open file descriptions, including flock locks. [Linux fork(2)](https://man7.org/linux/man-pages/man2/fork.2.html)

Rust documents pre_exec callbacks as running after fork and before exec; standard-library descriptors have CLOEXEC, which closes them at exec rather than at fork. [Rust CommandExt](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html)

GPU source/gpu.rs:116 and source/runner.rs:719 install pre_exec callbacks. In a concurrent test process, an unrelated spawn can consequently inherit a held BenchLock descriptor temporarily. If the parent closes its last descriptor before that child execs/exits, the shared open description can remain locked, and the immediate independent acquisition can fail. This is a source-supported hypothesis; no trace identifies the actual fork, descriptor, errno or child at the failing instant.

The GPU fixture's own child is reaped and watchdog joined before run returns (gpu.rs:175–185), so that child does not explain the proposed surviving reference. The capsule fixture path is unique in the inspected test source. The host recompute helper preserves its prior version dispatch, and this test checks the serialized byte hash rather than calling that helper.

## Minimal validation path

1. Preserve the full failed run and the one passing isolated replay exactly as retained here; neither establish that the full current suite passes.
2. Before changing BenchLock semantics, use a small isolated software-only diagnostic with deterministic pipe barriers: acquire a temporary lock, fork a helper that retains its inherited descriptor without exec, close the parent descriptor, attempt an independent nonblocking acquisition, then close/reap the helper and reacquire. Record errno and bounded cleanup. This tests the proposed mechanism without rerunning the scientific workload. It has not been executed here.
3. If needed to establish relevance to this test, run the existing fixture with explicit bounded concurrency diagnostics or syscall tracing of flock/close/fork/exec, retaining errno and inode/PID identity on failure. Agree a finite number/time bound before running; do not loop until a pass and discard failures.
4. Choose a repair only after deciding the intended ownership contract. A test-only bounded wait could mask a real release bug; a production explicit LOCK_UN changes inherited-lock lifetime and needs its own reviewed ownership test. Serializing a diagnostic run can characterize concurrency but does not prove default parallel-suite correctness.

No code edits, test/build execution, lock acquisition, scientific run or cloud action was performed by this audit. `source-bindings.json` binds all source and native output copies to their original paths; `observations.json` distinguishes observed outcomes from the hypothesis.
