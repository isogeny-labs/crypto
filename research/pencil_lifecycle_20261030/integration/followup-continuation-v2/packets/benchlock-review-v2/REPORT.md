# BenchLock additive prospective review

The v2 proposal resolves the v1 explicit-destructor source blocker. A private release_owner_lock method now contains the same PID-guarded best-effort LOCK_UN body. Actual Drop delegates to that method. Child C4 calls that method and libc close, then eventually _exit; it does not run String deallocation. The extracted primitive matches its declared source interval byte for byte. Relative to v1 helper.rs, only the prohibited method invocation and its adjacent close comment changed.

No blocking source discrepancy was identified for the proposed fixed Linux diagnostic. This review authorizes no execution by itself. There has been no compiler or native observation. The earlier helper remains preserved and its E0040 finding remains source-only. Full Rust field destruction in a fork child is excluded; the child check covers the actual shared release method plus descriptor close. Same-process parent and thread drops exercise ordinary Drop when executed.

Pipe barriers establish readiness before parent probes, all forks precede the first created thread, parent RAII handles ordinary error cleanup, and child and parent alarms bound abnormal paths. A private source-level signals stub remains unexercised because all acquisitions use wait=false. Non-WouldBlock acquisition classification is source-reviewed only. Unlock failure fallback and abnormal-exit descriptor lifetime remain documented limits. No inference about the original GPU test failures follows from this review.

Before native admission: root must freeze exact existing rustc and matching libc rlib paths/hashes, use one compile with 120-second timeout plus 5-second kill grace, retain compiler receipt, then bind the resulting executable hash and a unique absent output directory. One fixed execution may use 60 seconds plus 5-second grace, internal 15-second alarm; require all six markers and terminal marker plus exit zero. Preserve all failed outputs; do not infer retries. Root must separately validate actual integrated crate and applicable tests.

The reviewer authored the older close-only barrier diagnostic, but not this patch or helper. Review is independent of the proposal producer. No repository edits, builds, cloud actions or scientific trials occurred.

References: Linux flock open-file-description semantics https://man7.org/linux/man-pages/man2/flock.2.html ; Rust prohibited destructor call https://doc.rust-lang.org/error_codes/E0040.html ; Rust post-fork safety https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html .
