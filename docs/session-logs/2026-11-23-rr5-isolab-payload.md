# RR5 B677 IsoLab payload integration — 2026-11-23

## 15:35 PST — Recover state and track the missing payload

### What was done
Read current AGENTS.md, ACTIVE, the newest canonical session-log entries, the registered B677 protocol/admission, current producer and consumer source, and the maintained IsoLab workflow/build/command route. Current producer PR #4974 is merged; checker continuation #5073 and PKM #5081/#5088 remain separately owned. Opened tracking PR #5099 from main 3dd555c5d9b54c5bc0f3507eb6a917a5cf64e1f6. Tracking commit 2c3831b7c6018bfe69f2b0de50d986d40f9aa04a changes ACTIVE only.

### Decisions made
Implement fixed RR5 payload support using the published native extractor and unchanged independently authored receipt checker. Preserve the frozen sequence, resource ceilings and separate build/checkpoint/budget/capacity/replay gates. Do not add another extractor study or change existing PKM work. Local compiler, unit and contract checks are development work; registered research extraction remains unrun.

### Open threads
Implement source-bound payload build/publication, the fixed runner and outcome/identity contract tests. Verify before publication. A real research run additionally needs successful checkpoint transport, authenticated EKS admission, a tested image and live bounded budget/capacity; these have not been established.

### Errors
Conductor is absent (`conductor check` exit 127); no KB MCP tools and `scripts/research-kb status` reports no executable. Fallback prior-work review uses exact repository paths above; no novelty claim. The GitHub contents endpoint returned empty content for the >1 MiB canonical log; the Git blob endpoint and fresh shallow clone recovered it. A depth-one fetch hid the tracking commit's parent; depth-two fetch restored ancestry and fast-forwarded without history rewrite.

## 16:04 PST — Implement the fixed payload and identify deployment boundaries

### What was done
Added a standalone Cargo binary target pointing to the unchanged independent receipt checker, the fixed native payload build, a data-only publisher, the fixed request/runner, transport collection, and a separate replay job. Added synthetic orchestration and receipt-custody tests. Preserved source selection guards; documented the two deliberately reviewed whole-file pin refreshes and the test-only selected-range addition in SOURCE_REVIEW.md. Source-byte comparisons remain unsigned producer assertions. [pcc:unverified]

### Decisions made
Use the maintained isolab-results bucket for job results and retain the separate setec checkpoint destination. Run selected-source binaries only in unprivileged build/scientific/replay contexts. Keep transport, per-call identity, row replay and source-bound extraction completion separate. The workflow can reuse the existing scoped checkpoint-upload secret; it must not broaden IAM or export raw records under a sanitized filename. Main advanced to e07e257b17805a68877e2c4bd7aa18776f321bf0 with unrelated NIST state-control records; merged it without editing those records or rewriting the PCC ledger.

### Open threads
Finish the exact published-source bundle qualification, required root-library gate, evidence/claim recording and PR update. The checked-in GitHub trigger policy lacks both ListBucket and full raw-record GetObject rights; live grants are unknown. The workflow checks the necessary list permission before publication/submission, and collection still requires full object-read authorization. An actual admitted run additionally requires valid checkpoint transport, finite known remaining budget, live all-tenant capacity and a published tested image. None of these deployment checks has been performed here.

### Errors
Initial native compilation stopped at a stale original-source checksum; re-extraction found the one test-only selected-range change and an unchanged selected Fp range. Sparse source hydration initially omitted src/hash/sha256.rs; the build inventory now includes src. Rust 1.100.0 uses a different Cargo build-directory layout, so source-manifest discovery supports both layouts. Read-only review caught and corrected omitted hash-map validation, misleading failure summaries, coupled evidence downloads, live Pod args/restart checks, and the results-bucket mismatch. CLI push authentication is unavailable; publication uses the existing GitHub connector. These are development/integration observations, not scientific extraction results. [pcc:unverified]
