# Coherent crypto library prospective source review

The final frozen recipe is accepted for one bounded library compilation after root release. Independent read-only hashing verified 1,103 source files, 506 retained dependency files and 145 compiler/runtime artifacts. Source membership matched exactly, no source symlink was present, and the prospective output directory was empty. No compiler or test was invoked by this review.

Reviewer assembler_independent_review_v2 did not author the wrapper. Prior draft familiarity and advisory corrections are disclosed. Final runner b521e4924df61ab379fc32a4e44b8db1a704437303abe89fe2e97648b55895aa is bound by closure.json together with argv, plan, inventories and freeze21aadc8e. The exact review JSON contains full hashes and conditions.

Earlier draft findings were corrected before launch: caught failures previously exited zero; a receipt-I/O signal could leave apparent success; empty disk observations could break reporting; reused counts omitted the retained graph; compiler support and source membership were not checked; HOME was reassigned. The author retained earlier drafts. Root explicitly accepted output retention within the monitored allocation rather than deletion of failure evidence.

The wrapper creates a zero-token pipe, passes both actual descriptors to rustc, and names them in MAKEFLAGS and CARGO_MAKEFLAGS. This supplies one implicit jobserver-managed slot, not a hard CPU or thread quota. The exact graph was freshly built previously; cached extern crates have byte custody, not fresh source attestation. See https://doc.rust-lang.org/rustc/jobserver.html and https://doc.rust-lang.org/stable/nightly-rustc/cargo/compiler/job_queue/index.html.

Required launch envelope is timeout -k5s295s python3 rebuild2-runner.py, one attempt, hard8GiB address space and1GiB per file. Internal work cutoff275s and success cutoff290s leave cleanup time. Aggregate output4GiB and logs16MiB are sampled thresholds and can overshoot; free space starts at least8GiB and stop threshold is4GiB. These are source properties, not runtime observations.

Acceptance of the future result requires outer exit0 plus passed final receipt, unchanged inputs, confirmed child reap and disk readback. Signals, uncertain launch/reap, interrupted receipt I/O or an uninterruptible kernel operation require an honest failed/uncertain classification. Subsequent exporter compilation and testing remain separately scoped. No scientific workload was reviewed or launched.
