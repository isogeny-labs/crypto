# 2026-11-18 — Prime31 weighted-M6 versus shared BSGS and native IC frontier

## 12:31–12:35 UTC — Track, preregister, implement and measure 1,024-Q matched frontier

### What was done (commands, files, features)

- Read `AGENTS.md`, existing `docs/ACTIVE.md` and dated session logs, #4714 existing complete weighted-M6 Rust controls, #4596 shared-BSGS source/CI, #4692 RR9 ownership and #4711/#4712 current index calculus work. Audited existing S3 quotient, S4/S3 chain failure, early negative B31 coordinate/base trials and the prime-field PKM/APS publications. KB and Conductor not attached; bounded repository/primary-source fallback.
- Recorded task in `docs/ACTIVE.md` in PR #4714's existing lane, then froze `research/prime31_ic_20261115/native/PRIME31_WEIGHTED_M6_SHARED_BSGS_FRONTIER_PROTOCOL_20261118.md` at commit `755d3b62b48d8d0fece5800a6894125407801daa`, before first executed matched run.
- Updated **existing** `research/prime31_ic_20261115/native/examples/prime31_weighted_m6_batch.rs` to accept the original #4532/#4596 1,024-Q caller-verified corpus as an optional positional path, leaving the original no-arg frozen target sequence untouched. Strictly verifies caller sidecar and Q, then passes **only Q** to the existing Rust solver. New negative input test fails closed.
- Added `.github/workflows/prime31-weighted-m6-shared-bsgs-1024q.yml` to build the unchanged original Rust and BSGS source, run the complete native mixed IC source corpus, perform exact M6/BSGS ABBA on all 1,024 identical Q, and compare per-target labels, without supplying caller-only scalars to either solver.
- Both exact-head GitHub Actions executions at `87e3f0a1607cd2b1f22af79cff5f83e614d2aff8` passed: [#44305053128](https://github.com/isogeny-labs/crypto/actions/runs/44305053128) and [#44305062836](https://github.com/isogeny-labs/crypto/actions/runs/44305062836).
- Independent downloaded raw ZIP SHA-256: `aa61631b9298b2c6acc0d677282d702a2e88489f6af53a09c28f5b0194c58879` (artifact #14427043131) and `9a3fe05ebe162198c519ca287937607bfe590cac2315977073ad081bbdf109c0` (artifact #14428240804).
- Wrote committed `PRIME31_WEIGHTED_M6_SHARED_BSGS_FRONTIER_RESULT_20261118.json` and `...RESULT_20261118.md` with all measured values, exact-source links, signed-status limitations and next experiment queue. [pcc:unverified].

### Decisions made (and why)

- M6 recovered 1,024/1,024 logs four times with rank 31 in 106 training calls and 3,660 target queries. Full cost M6 = 3359.597 / 3391.541 ms vs best shared BSGS 144.049 / 157.859 ms on runner 1, and 5301.732 / 5330.527 ms vs 250.565 / 255.661 ms on runner 2. **M6 loses by 20.85–23.32x**. Same-runner existing mixed 4+4 IC (single earlier source-generation run) = 1897.579 / 2841.305 ms; M6 is 1.77–1.88x worse. All measurements [pcc:unverified].
- Do **not** promote a leaderboard row or claim a novel non-generic ECDLP solver. The 7.368× prior scalar-to-batched M6 engineering improvement is narrower, and 109,090,357 query EC additions remain unchanged by field inversion batching.
- Keep #4692 as sole RR9 implementation lane; this run supplies actual full-solver competitive cost ceilings and a new exact-matched corpus integration instead of duplicate F4 code.
- Proposed lowest-cost subsequent controls: exact nonenumerative RR9/F4 small-base public membership/certificates; generic early-exit any-witness as a control; algebraic fibre input degree; signed expert proof audit of subexp #4698. No new paid compute, no merges.

### Open threads (unfinished, pick up next time)

- Independent signed PCC of both whole solver traces; source-specific arithmetic replay is **not** a signed independent attestation.
- Require a *real nonenumerative* public-residual algebraic witness with proper B4/B8 positive/negative and exceptional fibre coverage. Target indicative budget ~504 µs/accepted trial to beat existing native IC or ~38 µs/trial to beat fastest BSGS on runner 1 (optimistic, at same yield and excluding setup).
- Same-host optimized rho and trusted APS/PKM non-generic comparator on exact Q and honest memory-cap bands. Add an independent second verifier and cross-host controlled run.
- The shared repo `docs/session-logs/2026-11-18.md` was not overwritten; separate log avoids concurrent append collisions.

### Errors (what went wrong, how it was handled)

- Exact workflow-run-by-workflow URL was rejected by GitHub connector; used the supported runs collection to locate IDs and then read exact-job statuses/logs and archive artifacts.
- First workflows triggered twice (push and PR) on the same source; both completed successfully and are preserved, not cherry-picked.
- The central algorithmic hypothesis failed against the strongest matched solver. This is retained as a negative result; no efficiency or complexity advantage inferred.
