# Prime31 RR5 frozen dependency lock — 2026-11-23

## What was done (commands, files, features)

- Read current AGENTS.md, ACTIVE, Conductor guide, merged #5053 qualification report and supplemental dated log. Canonical daily log connector returned empty (large shared file); did not overwrite it. #5053 is now closed/merged at 21eb95ca16c4e5f1dbe7a02f6ae573970e8971c0; its branch no longer resolves. New isolated tracked draft #5073, baseline da7777094392d3ccd37db78dcfc761c32e7e7813.
- Downloaded both actual retained ZIP artifacts [#14851190934](https://github.com/isogeny-labs/crypto/actions/runs/45239291528/artifacts/14851190934) and [#14850584102](https://github.com/isogeny-labs/crypto/actions/runs/45239466847/artifacts/14850584102). Python zipfile/hashlib compared Cargo.lock bytes; both exactly 4198 bytes, SHA256 a6123e8bb420a0ae63745b7a9b285fea0e70930058dd6dd02fe1ccff5c4570f1, Git blob 763695554d0b0bfa91ab831a062303522ae48134. tomllib parses 19 packages. Both archived test.exit files are 0. Second artifact's test.stdout confirms 19 passed, 0 failed. No test rerun asserted here. [pcc:unverified]
- Compared exact Git blob identities for all eight files listed in the source-replay artifact at 0297bc50c3b986bec1fe3fb451fd6092d7fc5e71 and branch tracking commit 0b8374e130058534d01815f06522c3d97d726dc6: helper manifest, original checker, B8 fixture, original native manifest/library/lock, F4 and signature source. All eight identical.
- Committed the exact archived standalone lock at 35a2cd6e02b236eba20e120edc8fb2bb44953198 and fetched it back: exact Git blob 763695554d0b0bfa91ab831a062303522ae48134. Updated existing workflow at 47f9acef753b83ad84a4447ada0617b8490bb6be to require a tracked nonempty committed lock, archive it without generate-lockfile, include it in source digest replay, and fail if tracked files change. Existing cargo test --locked and resource caps retained. New workflow execution remains pending/not asserted.

## Decisions made (and why)

- Close the reproducibility dependency without repeating scientific measurements or changing producer code. Do not change the original Rust arithmetic, extraction, graph, summation, transfer, baseline, frozen evidence or PCC ledger. Prior #4974 B677 circuit remains extraction-unqualified; #5049 p+1 selector/FFT and #5061 distinct PKM map controls are components, not a cold solver. #4993 v5 rho audit did not seal a new strongest reference.
- Literature/prior-art fallback: merged native qualification report, original Prime31 cold report, #4974, #4993, #5049, #5061, and problems/prime31-exact-pair-spectrum-query.md (existing Semaev/PKM sources and open extraction obligation). No new mathematical novelty claim. KB/Conductor runtime unavailable. One actual coordinator; zero native workers; parallel GitHub source and baseline reads, sequential coordinator writes. No additional scientific timing, paid compute, or merge.

## Open threads

- Observe source-bound locked native workflow once complete; retain any canceled or failed attempt. Signed independent PCC still pending.
- The owning B677 lane must supply a genuine blind public accepted stream before full target-recovery and cold-cost comparisons; keep current extraction and benchmark reservations. No new relation extraction or full ECDLP cost result in this follow-up.

## Errors

- First local transcription failed 4198-byte guard before any write; exact base64 decoding plus remote Git blob check recovered the correct artifact bytes.
- No local Rust compiler or Conductor executable. The new native execution is not claimed; no additional workflow manually dispatched. [pcc:unverified]


## 2026-11-23 20:23 UTC — locked qualification readback

### What was done
- Exact PR head `ce568b6a07cef885986b14767e42b065a6c97237`: [native run 45249082556](https://github.com/isogeny-labs/crypto/actions/runs/45249082556), job 137953579399, completed successfully at 19:27:39 UTC. GitHub jobs API confirms every step succeeded: immutable checkout, tracked committed dependency lock, locked compilation, locked execution, final source/lock integrity, and artifact upload. Archived artifact metadata: ID 14856040426, 9056 bytes, digest sha256:75a542e52a0a9d1e2ea91e8ff89cb81f4b88509e8082c559f2e5513a0525071b, expires 2026-12-14. Raw ZIP/log retrieval is unavailable through the current connector; do not claim newly inspected per-test counts or raw receipt digests. Historical 19/19 receipt remains separate. [pcc:unverified]
- Exact head PCC gate 45249092185 and validation gate 45249082197 succeeded; PCC signing skipped. Workflow syntax 45249082080 and separate rootfinding runs 45249082687/45249082007 failed; no causal attribution without logs. No merge readiness assertion.
- Read AGENTS, ACTIVE, prior frozen-lock handoff, #4974 merged B677 report and source-exact workflow. Main snapshot 5403e9ab890e9b9dc92fa67c661ab7df0ea3c860. Canonical daily log exceeds connector limit; preserve it untouched. KB and Conductor unavailable in this runtime; no novelty assertion. Native agents unavailable: one coordinator, parallel independent GitHub reads for baseline/ownership, frozen Rust CI and artifact validation. No manual CI dispatch, experiment, paid compute or benchmark elapsed-time claim.

### Decisions made
- Stop verifier-only changes; locked CI outcome is a component qualification, not a new extraction or speed measurement. Keep this draft while signed evidence and remaining gate issues are unresolved. Do not launch duplicate extraction or change baseline/graph/transfer components.

### Open threads
- #4974's registered B677 extraction requires checkpoint transport, authenticated EKS admission and tested digest-pinned payload image; ACTIVE still records these as open. Obtain the owning lane's genuine blind relation stream, then reuse qualified Rust checker and charge complete cold costs. Raw locked artifact inspection and signed independent PCC remain open. No new accepted relations or scalar recovery.

### Errors
- Jobs logs endpoint rejected; binary artifact contents not accessible in this runtime. No local cargo, gh, Conductor or native agent tool. No unsupported claims about unrelated red workflows.
