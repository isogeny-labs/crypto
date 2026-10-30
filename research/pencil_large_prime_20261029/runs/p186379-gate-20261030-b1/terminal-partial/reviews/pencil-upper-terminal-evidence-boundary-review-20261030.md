# Upper-gate terminal evidence boundary

This is a read-only source review, recorded at 2026-10-30 03:27:26 UTC. It does not assert the live gate has failed or completed.

The computation deadline remains **03:28:20 UTC**. The original **05:05:50 UTC** authorization expiry and **10,800 active-worker-second** cumulative budget remain unchanged. The unused outer window does not grant a fresh computation deadline, rerun, restore or resume.

At the supervisor loop deadline, the implementation kills the recorded process group, saves failed status and returns an error (supervisor.rs:628–635). That branch does not update the status timestamp or record a newly reaped exit code, so these fields cannot establish a precise observed termination instant. Controller failure handling saves durable state, attempts stop and publishes state plus available diagnostics (cloud.rs:783,1012–1027). The bootstrap EXIT trap uploads top-level JSON receipts, conditional on successful S3 operations (run-controller:15–26).

Previously committed checkpoint archives retain the compute directory and generation image/manifest (main.rs:14; cloud.rs:355–391,514–536). The manifest hashes the retained files (supervisor.rs:397–412). This can preserve partial session records, spec, stdout and stderr as of checkpoint capture. It does not establish completed computation, independent replay, fresh measurement or a qualified cost comparison.

There is no automatic final partial-computation upload on failure. The successful computation archive path runs only after completion and verification (cloud.rs:923–958). Latest writes remaining on EBS must not be described as already present in S3. The last-two-generation state policy removes state inventory entries, not archive objects (model.rs:164–168).

A neutral terminal report should bind the observed terminal/controller receipts and exact versioned checkpoint archive hashes, separate complete checkpoint-time rows from missing or incomplete rows, and retain the full 18-slot /15-measured-slot gate denominator. Incomplete or unverified records cannot admit gate completion, a complete gate ratio, or remaining-cohort execution. The companion JSON records exact source hashes and references.

No cloud calls, solver executions, source changes or launch/resume actions were performed for this review.
