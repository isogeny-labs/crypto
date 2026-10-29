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
