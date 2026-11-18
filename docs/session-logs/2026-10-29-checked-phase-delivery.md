# Checked phase delivery — 2026-10-29

This scoped entry accompanies the checked-phase PR. The shared daily log is
owned by another task; its existing entries are not changed here.

## 05:14 UTC — terminal software validation and verified durability

### What was done (commands, files, features)

- Reconciled clean producer source `b7c11d1b5f8a1f37b769453d3ce0413696b9fc83`
  and immutable library SHA256 `1e7496533221060ea6202ff4fc78c80b0d5a11cb1bdabf1bfb68c430551eec8b`.
  The parent-owned unfiltered serial full library gate completed 5231 passed,
  0 failed, 123 ignored, 0 filtered; strict release Clippy and two exact retained
  mock tests passed. Earlier overlapping source-27679 subset and CLI gates remain
  separately identified in the final validation document.
- Verified the final 98,682,039-byte archive in canonical S3 under
  `ml-dataset/20261028/software/checked-phase-telemetry/dc0a2447aa6da09536c2f37b5326043907bfaca21190047008f3027bbdde85ad/validation.tar.gz`.
  Conditional PUT, ETag-bound fresh GET, original-byte comparison, contained
  hashes, restored source bundle and all 49 retained fixture hashes passed at
  05:12:02 UTC. Source and fixture checkpoints remain immutable.
- Added source-bound validation, precise receipts and delivery handoff. T766 was
  released to ready with its heartbeat stopped; new task T784 reserves only the
  dedicated research directory and this scoped session entry.

### Decisions made (and why)

- Keep the regular PR stacked on the original native collector parent PR2801;
  do not enable feature-parent auto-merge or retarget. Its later documentation
  does not require an unrelated merge. No source edit or rebuild follows the
  validated test-only retention derivative.
- Preserve the source-27679 CLI identity and runtime-equivalence proof rather
  than relabeling it as a source-b7c11 executable. Keep software fixtures and
  native lifecycle consistency separate from hardware/platform or scientific
  admission. The USD200 total cap and original external deadlines are unchanged.

### Open threads (unfinished, pick up next time)

- Publish the regular stacked PR and final source/delivery checkpoint; record
  the exact PR head and new checkpoint readback. Hosted review/CI and later
  stack integration remain parent-owned. Independent consumer validation has
  its own source, scope and receipts. Broader corpus/worker publication and
  partial-adapter coverage are not discharged by these local tests.

### Errors (what went wrong, how it was handled)

- Retained the original source-27679 parallel result 5230/1/123, exit101, with
  an unchanged live-writer lock failure whose cause remains unconfirmed;
  isolated retries passed. Its superseded serial attempt exited143 without a
  verdict. The final derivative serial run passed with no exclusions.
- Retained the earlier missing-borrow compile failure and the retention
  runner's exit127 after both exact tests passed: its narrowed PATH lacked rg.
  Only metadata hashing/readback and Clippy resumed; native tests were not
  repeated. No evidence was overwritten or promoted into a scientific result.

## 05:23 UTC — regular stacked PR and source checkpoint

### What was done (commands, files, features)

- Pushed documentation head `aca42cc2aaa908e7fa62e1dfc8ebf86911955230` and
  opened regular [PR #2825](https://github.com/isogeny-labs/crypto/pull/2825)
  at 05:22:55 UTC, based on `codex/ml-pmu-collector-20261028` at
  `843a31522de11e4d12f9a38990c1e2ec4a585fdf`. The parent app attachment
  succeeded at 05:23:08 UTC. No automatic merge or retarget was performed.
- Standard checkpoint `20261029T051857Z-aca42cc2aaa9` was freshly downloaded
  and verified: 97511 bytes, SHA256
  `0c8f92456fc881c46dc9484777afd6c055657741eb777c3b1cc86abaaafd403a`.
  Restored metadata/HEAD, bundle prerequisites, RESUME and specification bytes
  match. The utility removed its original temporary tarball, so the receipt
  explicitly records the fresh-download hash without claiming original-tar
  byte comparison.

### Decisions made (and why)

- The complete diff from tested b7c11 to the delivery head touches only reserved
  documentation/receipt paths; all recorded source/dependency/build-input hashes
  match. No tests or binaries are rebuilt for these documentation updates.
- Keep the initial creation response's mergeability uncertain until GitHub has
  computed it. A PR being open or attached does not establish current-head CI,
  independent hardware admission or main delivery.

### Open threads (unfinished, pick up next time)

- Preserve and freshly read back this publication-handoff checkpoint, then push
  the final documentation head and report its exact identity. The parent owns
  subsequent CI cadence, review reconciliation and stack integration. No source
  ownership is retained for that read-only follow-up.

### Errors (what went wrong, how it was handled)

- The Git remote reports the repository moved from aburan28/crypto to
  isogeny-labs/crypto and accepted the normal fast-forward/new-branch push.
  No force push or remote configuration change was needed. Earlier failures and
  interrupted validation remain preserved in their original evidence packets.
