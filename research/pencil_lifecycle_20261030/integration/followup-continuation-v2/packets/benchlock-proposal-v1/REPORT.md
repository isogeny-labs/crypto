# Process-owned BenchLock proposal — not applied or executed

The private patch makes the acquiring process release its flock explicitly on guard destruction. A PID check prevents a forked child destroying an inherited guard from unlocking the parent's lock. Acquisition immediately records the operating-system error; only WouldBlock follows the contention/wait branch. Other errors now return instead of being mislabeled or retried indefinitely. This is a proposed shared primitive correction, not a test-only sleep or retry.

## Observed failure versus mechanism

The latest boxed-request full library run returned 101 with 5791 passed, 2 failed, 128 ignored. Capsule failed the independent acquisition after run returned (gpu.rs1270). Selftest failed the third run's acquisition (1129); its second deliberately repeated output path can acquire and then release on create_dir failure. Current errors discard errno. Neither failure establishes EAGAIN or a specific holder. Retained pipe-barrier evidence independently establishes EAGAIN11 while a forked child holds the shared open file description, even with FD_CLOEXEC set. It does not reconstruct the GPU failures.

## Callers and ownership

The materialized Rust tree search finds the primitive only in isolation.rs and its GPU/runner callers plus historical research copies. Three production acquisitions: gpu::run at227, gpu::run_match at636, runner::run_session at972. GPU guards remain local until return/error. run_one reaps its child and joins its watchdog; run_match joins the telemetry thread after the solve call. Runner drops its guard explicitly at1065 after restoration and session result persistence, including normal error handling. Tests acquire competing independent guards; no Clone implementation, Arc guard sharing, raw descriptor exposure or deliberate guard transfer was found. File is private; path is public. No live src/tests/examples libc::fork call was found. This is a materialized-source inventory, not proof about external library clients or unmaterialized source.

Moving a guard between threads within its process remains supported. The PID condition has no thread restriction. Intentional cloned guards are not part of the present API: do not add Clone by duplicating File with this destructor, because destroying either clone would release the shared lock. Future shared ownership must place the single guard behind a last-owner container, or explicitly design another contract. A forked child's guard is non-owning; inheritance is not transfer. Parent process termination without Drop retains ordinary kernel descriptor-lifetime semantics; the patch does not promise crash cleanup of inherited copies. PID reuse after parent exit is outside the intended live-owner contract.

## Literature and limitations

Linux flock associates locks with open file descriptions; forked descriptors share that lock, and explicit LOCK_UN releases it even when duplicate descriptors remain. https://man7.org/linux/man-pages/man2/flock.2.html

Rust documents pre_exec after fork and standard descriptors' CLOEXEC at exec. https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html

Thus owner Drop is the appropriate shared primitive boundary if its documented purpose is release when the owning guard ends. A test-only wait would weaken that boundary and leave identical production early-return windows. The candidate does not claim the actual failed interleaving was identified. Explicit unlock intentionally stops incidental children retaining the parent's lock after normal Drop; any caller intending descendant-owned protection must be redesigned instead of relying on incidental inheritance.

Drop cannot return unlock errors. The minimal candidate makes one best-effort LOCK_UN attempt and then File closes; failed unlock can still leave an inherited reference holding the lock. No logging or assertion is introduced inside Drop. A checked release API is a separate possible extension if reporting release failure is required. Acquisition error classification changes are deliberate: non-contention errors fail promptly, with their actual error text. Existing contention prefix remains.

No repository edit, patch application, compilation, native test, scientific workload or cloud operation was performed. proposal.patch and proposed source are review inputs only. Root owns reservation checks, admission and subsequent implementation.
