# BenchLock prospective source review

The proposed owner-PID release policy has no blocking discrepancy identified at the inspected Linux source scope. The first control helper has a compilation blocker: helper.rs invokes `<BenchLock as Drop>::drop(g)` directly in C4. Rust prohibits explicit destructor invocation (E0040). This is a source finding, not an observed compiler run. Preserve the helper and correct it additively before native admission.

## Scope and custody

Independent review of the proposed patch and control helper; reviewer authored the earlier close-only pipe-barrier diagnostic, not this patch. All 13 producer manifest entries match, and the helper's extracted primitive is byte-identical to the declared proposed-source interval. No compilation, execution, repository edit or scientific work was performed. Current materialized call sites in gpu.rs and runner.rs acquire/drop guards; no Clone or exposed raw-descriptor API was found.

## Ownership and error behavior

The original acquirer PID is saved on successful acquisition. Owner Drop attempts LOCK_UN before File close; inherited child Drop omits explicit unlock. Linux flock attaches to the open file description, so explicit unlock can release the shared lock while child descriptors remain open. Same-process thread movement retains the PID. This does not implement intentional cross-process ownership transfer. Explicit unlock failure is ignored and close remains a fallback; release is not guaranteed if other descriptors survive. Process termination without Drop retains kernel descriptor lifetime behavior.

The acquisition error is captured immediately following failed flock, before formatting or further syscalls. Only WouldBlock enters contention or waiting. Other errors are returned with their captured diagnostic. This changes prior classification of arbitrary flock errors as contention. Six proposed controls do not exercise a non-WouldBlock acquisition error; that branch currently has source review only. wait=true and signal behavior are excluded by the helper's stub and calls.

## Control design

The helper orders all forks before its first created thread. Pipe barriers order child readiness, parent release/probe and child termination. Child paths avoid formatting and normal unwinding. Parent RAII attempts kill/reap on ordinary failure; parent and child alarms bound abnormal paths. Outer timeout remains required. The helper intentionally excludes full String-containing guard destruction in C4, because that would deallocate after fork. Its attempted direct destructor-body invocation is prohibited by Rust. A corrected shared release method or explicitly revised destruction contract needs a new source binding and review. No silent substitution should be admitted.

The prospective checks separately cover raw close-only inheritance, parent owner release, child nonowner behavior, early return, and same-process thread movement. They do not determine the cause of the two original GPU failures, whose errno/holder identities were not retained. Isolated primitive tests are not whole-crate integration qualification.

## Native admission prerequisites

Correct and review the helper/source relationship; freeze exact existing rustc and matching libc rlib paths and hashes; preserve one compilation bound of 120 seconds plus 5-second kill grace. Only after successful compiler receipt, freeze compiled helper hash and unique absent output directory. Admit one 60-second invocation plus 5-second grace, internal 15-second alarm, with all stdout/stderr/native receipts retained. Require six control PASS markers and ALL_SIX_CONTROLS_PASS plus exit zero. No retries inferred. Root separately owns typed integration and applicable software regression checks.

Sources: Linux flock(2), https://man7.org/linux/man-pages/man2/flock.2.html ; Rust E0040, https://doc.rust-lang.org/error_codes/E0040.html ; Rust CommandExt after-fork constraints, https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html .
