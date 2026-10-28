# Work log

## What was done (commands, files, features)

- Read the current dated session log, repository instructions and pinned native implementation. Compared native sources d79..c14: no diff in the three relevant existing files.
- Created isolated worktree /home/ec2-user/crypto-ml-f4-trace-20261028 and branch codex/ml-f4-trace-20261028 at c14d4552e513f6e28bda0e0e045f5e64beb492db.
- Claimed Conductor T-756 and froze the narrowed design before implementation.
- Committed the design at25a46f07530 and initial wire schema atfd0ece16169. The complete schema clarifications and named admission model are recorded in SCHEMA.md and ADMISSION.md.
- Implemented the optional native observer in koblitz_groebner.rs and its child algebra_trace.rs. Existing untraced wrappers use the same native kernels with no observer. No adapter, solver dispatcher, consumer or cloud file changed.
- Added focused local software tests and an explicitly ignored fixture emitter. Scoped stable rustfmt and git diff --check passed. Cargo, native test execution and emitted fixture verification have not yet run; the parent owns the shared build slot and separate checker.
- Validation of clean source1e22241dab67a236eb954f93c75ab38d228a8101 subsequently passed:14 focused tests,12 existing native RREF regressions, and strict release Clippy for lib/tests/crypto bin. Native test binary SHA256190b4d5ed14d9c2c6cfb35e7dac7981e77f2407cb12dd80b4d022328b3bc384c is retained outside the shared target. The two explicitly invoked helpers and four opt-in measurement tests were ignored by those ordinary filters.
- Checker review requested a native row-cap0 censored fixture. Extended only the existing isolated test helper to optionally export its acknowledged lines and returned failure receipt. Frozen schema now states limit minima and the legacy cap+1 candidate ordering. No native runtime code changed in this extension; its final test/lint/fixture gates are pending at this commit.

## Decisions made (and why)

- Require input degree <= D so the 163-column bound is valid without omitting original generators.
- Keep producer independent-rank and complete-execution flags false until the separate checker exists.
- Preserve source ownership: no adapter or consumer edits. Use this scoped handoff rather than editing the shared session log outside the assigned territory; parent can carry the entry into its integration log.

## Open threads (unfinished, pick up next time)

- All source-bound local gates and independent native replay now passed, as detailed in VALIDATION.md. Hosted exact-head CI/review and bottom-up delivery remain pending. Producer claims remain false even on a completed stream. No launch or performance claim follows.
- Fixture emitter: run the exact ignored test cryptanalysis::koblitz_groebner::algebra_trace::tests::write_checker_fixtures with F4_TRACE_FIXTURE_DIR naming a new absolute external directory, F4_TRACE_SOURCE_COMMIT naming the exact40-digit source commit, and F4_TRACE_PRODUCER_SHA256 naming the64-digit executing test-binary SHA256. It writes five bounded JSONL streams plus prefix receipts. These metadata values remain caller declarations.
- Censored fixture emitter: invoke the exact ignored selected_profile_child test with F4_TRACE_TEST_EXPECTED=native_size, F4_F2_MAX_ROWS=0, F4_TRACE_CENSORED_FIXTURE_DIR naming a new absolute directory, and the same source/binary metadata variables. It writes native_size.jsonl plus native_size.failure.json. Environment changes are confined to that child process.

## Errors (what went wrong, how it was handled)

- Previous local PR status note suggested the wrong later check cadence. Additive external CADENCE-CORRECTION.md changes next unchanged PR2756 check to about20:17UTC, preserving the immutable S3 design archive. No extra poll occurred.
- The first background heartbeat did not survive its shell and the original lease expired. Edits paused, exact scopes were checked and T-756 was reclaimed as attempt2/fencing_epoch3 before continuing. The retained heartbeat session now renews the current lease; external receipts are under /home/ec2-user/ml-dataset-resume-20261028/f4-trace-implementation.
- Source review identified that elimination can densify output rows. Workspace admission now separately reserves the actual output popcount before readback rather than assuming source-row sparsity.

## 18:07 UTC final validation and preservation

- Final source 29973950752 passed 14 focused tests and strict release lint. Parent ran the immutable e368482f executable once for the full library gate: 5,197 passed, zero failed, 125 ignored. The earlier 12 native RREF regressions remain applicable by exact runtime-source equality. Earlier pending statements above describe their historical points only.
- Separate checker PR2800 passed actual native interoperability: ranks 0/1/1/2/92, 68 genuine nonzero start-word XORs, four rehashed corruption rejections, and a four-event cap-zero prefix with no final rank. All six CLI reports matched library reports. Parent source review found no actionable producer defect.
- Accepted source archive53f2f989 and terminal full-gate archivee5dbfcf4 passed fresh readback, content hashes and incremental source-bundle verification. The preceding c6fb086c archive is quarantined because its exclusion assertion found a copied live full log. It remains immutable; the accepted correction used a finite completed-file allowlist. Exact URIs and claim boundaries are in VALIDATION.md/VALIDATION.json.
- Native runtime, tests, fixtures and shared Cargo output were not changed or re-executed during final delivery. Scope remains a bounded native Boolean F4 single-call API. Hosted PR checks and integration remain separate. Source metadata is not execution attestation; no scientific dataset or fall-degree result was generated.

## 18:10 UTC PR delivery

- Opened regular ready PR [2802](https://github.com/isogeny-labs/crypto/pull/2802) at 2026-10-28T18:10:13Z. Creation head was 07d4e817ac45da9ca1587e95e3ed5c8019f39bfd; this handoff-only follow-up does not change runtime, build or tests.
- Target remains codex/ml-binary-root-trace-20261028 at c14d4552e513f6e28bda0e0e045f5e64beb492db, parent PR2782. Independent checker is PR2800. The PR is open; hosted exact-current-head CI/review and main integration are pending, and feature-base auto-merge is unavailable.
- The child app attachment request hung without a result and its cell was terminated. Parent will reconcile attachments before retrying; no successful attachment or duplicate request is claimed here.
- Final delivery/source checkpoint and exact post-push head are retained externally under /home/ec2-user/ml-dataset-resume-20261028/f4-trace-delivery. Check CI about 50 minutes after the final push/activity, later unchanged checks about four hours apart; do not busy poll or merge into a temporary review base.
