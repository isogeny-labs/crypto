# APS project handoff

The shared daily-log update was blocked by automatic publication review. This file preserves only the APS-specific additions; all unrelated baseline entries are excluded.

## 2026-10-28T16:33:00Z — APS four-lead native follow-up

### What was done (commands, files, features)

- Read current instructions and the latest session log; based the new branch
  on `6187036c049b6de2b1521b014f575952e4f1705c`. Checked the sibling lead
  atlas before the local survey, re-read APS §§3.1–3.4 and Milne I §3,
  and preserved design 11's prior negative cost results.
- Added exact-root streaming with informative-relation verification, exact
  coordinate-domain substitution with deadline-aware reuse of native rational
  factoring, a two-parameter line-divisor encoding, and label-certified j=0
  horizontal-component pruning. Kept exhaustive/legacy controls and explicit
  partial-enumeration status. Added fixed public-fixture tests and CLI modes.
- All 20 APS tests have passed inside the in-progress full library suite.
  Strict library/touched-example Clippy passed. An earlier separate draft
  binary passed 63 F4-related tests. The corrected full suite and final
  touched-example execution are still pending at this entry.
- Seven new core-only Lean lemmas passed both the repository proof gate and
  the shared CryptoLean gate at `148f5dc389b341401e06d76abe7783c4ccf42f2e`.
  Gate/auditor pins and the partial formalization boundary are recorded in
  `research/aps_followup_20261028/`. No speedup was measured.

### Decisions made (and why)

- Keep finite coordinate-domain tags off auxiliaries, line parameters and
  elementary-symmetric variables. Keep full-field fallback explicit.
- Refuse pruning after independent relabeling even when geometric orbits
  match: the horizontal relation can then be informative. Pruning adds an
  inverse variable and does not imply easier elimination.
- Preserve the earlier complete-cost/replay/protocol obligations and
  historical evidence. This follow-up establishes bounded correctness only.

### Open threads (unfinished, pick up next time)

- Collect `/workspace/aps-followup-setup/full-library.log` and
  `example-tests.log` and their exit receipts. Finish the native identity
  certificate, final report/diagram/PDF inspection, review, commit and PR.
- Performance remains blocked on the complete-cost prerequisites and the
  rejected required checkpoint upload. No performance job was launched.

### Errors (what went wrong, how it was handled)

- The first build lacked sparse compile-time inputs; the maintained CI
  materializer fetched 170 tracked inputs. The initial draft APS check then
  found a positive-test fixture with no partitioned relation (18 passed,
  one failed). Replaced that fixture construction with the independent point
  oracle; all 20 final APS tests now pass. Preserved the draft failure log.
- Stopped a superseded queued compilation and retained its cancellation
  separately from completed checks. Development builds and tests are not
  performance evidence.
- Automatic approval review rejected the checkpoint gate's source/worktree
  upload to S3 because that source-code egress lacked explicit user approval.
  The upload was not retried or bypassed; local checks and requested PR
  preparation continued. No secret or chat transcript is in the task records.

## 2026-10-28T17:59:33Z — APS final verification and publication preparation

### What was done (commands, files, features)

- The corrected full release suite completed with exit 0: 5,002 passed,
  0 failed, 122 ignored. Full stdout SHA-256:
  `f7214e37fcfbe1967ebd793807d9f5a7368c00ecc5c4fc0a07755b44c11f01ca`.
  Its compact receipt, all focused outputs and source hashes are in the
  follow-up study's `validation/` directory.
- Final focused checks passed 20 APS, 7 example, 63 F4-related and 15 GLV
  tests. Strict test-target lint passed on CI's Rust 1.98 and Rust 1.99.
  Rust 1.98 formatting, actionlint and native backlog validation passed.
- Generated native line certificate `IDC1h0abc57c4d1e81eb0`; added a fixture
  test reading the shipped file, checking its statement fingerprint and
  independently seeded points. Updated the existing problem/index and
  blocked panel with the four comparisons and exact remaining blockers.
- Confirmed main's actual required contexts are `gate`, `size`, `artifacts`.
  The implementation is committed locally; final receipt publication and the
  regular PR are the next action.

### Decisions made (and why)

- Use release optimization level 3 and codegen-units 256 for development and
  focused correctness CI, with two Cargo build jobs. This is build
  partitioning, not a timing comparison or a changed workload.
- Keep full raw library stdout outside Git because it exceeds the data-file
  threshold. Commit its checksum/summary and complete focused outputs; do
  not present the summary as an independent replay or a full raw archive.
- Preserve cancelled/draft checks separately. All observed missing-fixture
  failures now have passing reruns, with no assertion or test changed.

### Open threads (unfinished, pick up next time)

- Publish the final source/evidence tree and regular PR, enable guarded
  auto-merge under all three required contexts, and address review feedback.
- Practical speed and scaling remain unmeasured. Source checkpoint egress
  approval, complete native accounting/replay/export and a frozen controlled
  protocol are still required before the deferred panel can run.

### Errors (what went wrong, how it was handled)

- The required full suite was long-running, especially existing closure and
  isogeny-walk cases; its actual completion above supersedes pending status.
- No checkpoint upload retry, performance run, EKS submission, test skip or
  ledger promotion occurred.

## 2026-10-28T18:11:11Z — APS publication guards

### What was done (commands, files, features)

- Verified that `aburan28/crypto` and `isogeny-labs/crypto` resolve to the
  same public GitHub repository, ID 1221029056, and that the configured
  origin and earlier APS PR #2614 use that repository.
- Matched uploaded Git blobs against local object hashes. A workspace
  reconnect interrupted publication; committed sources and test receipts
  survived and publication resumed from verified files.
- Verified that the shared daily log's 88,878 baseline bytes are already
  public and unchanged, with only the APS-specific handoff appended.

### Decisions made (and why)

- Leave `problems/README.md` unchanged in the published PR after the
  repeated upload rejection. Keep the APS-specific problem record, study
  and blocked panel in scope; retain the proposed one-row index patch
  locally pending explicit publication approval.
- Publish a source tree matching the verified local Git tree through
  content-addressed GitHub blobs, avoiding unrelated promisor downloads.

### Open threads (unfinished, pick up next time)

- Finish the regular PR and check its current-head gates and review.
- The shared index row and required S3 source checkpoint remain blocked on
  explicit approval. Performance still also requires complete accounting,
  independent replay/export and a frozen controlled protocol.

### Errors (what went wrong, how it was handled)

- Automatic approval review initially rejected the GitHub destination and
  shared daily log. Read-only repository identity and byte/diff checks
  established the destination and append-only scope; both then cleared.
- Automatic review continued to reject the full shared problems index as
  unrelated research/operational disclosure despite its public baseline and
  one-row APS diff. Its change was excluded instead of bypassing review.
- A Conductor check without network permission could not reach the existing
  loopback service; the network-enabled retry showed only our own T-3/T-6
  reservations. No other agent's files were edited.

## Publication scope correction

Automatic review subsequently rejected the refreshed shared log even after its append-only/public-baseline check. The published tree leaves both shared files unchanged. The complete APS handoff is retained here; updating the canonical shared log and adding the central problem-index row remain pending explicit publication approval. No blocked upload was bypassed.

## Verification receipt publication

Automatic review rejected a Clippy receipt containing internal workspace/build
metadata. Published receipt copies now normalize local paths and omit build-lock
diagnostics; original and normalized SHA-256 hashes are recorded. Unmodified
receipts remain local. Test outcomes and failures are preserved, with no new
performance claim or test run.

A subsequent rejection covered the broad dependency/build list in the Rust
1.98 Clippy output. Detailed Clippy build receipts are therefore withheld from
publication; a concise command/status receipt and original hashes remain.
