# Checked phase software delivery handoff

## Goal

Deliver the approved producer checked-phase integration, bounded partial-snapshot
v2 contract, compatibility checks and durable software evidence on a regular PR
stacked on the original native collector parent. Preserve the user's existing
USD200 total cap, external job ownership and deadlines. The broader dataset and
fall-degree work is separate; this lane does not claim it complete.

## Lead / hypothesis

This is an engineering contract: checked calling-thread diagnostics must bind to
their actual start, attempt, fixed policy and lifecycle; interrupted partial
snapshots must preserve valid immutable artifact prefixes. Software consistency
does not certify hardware, platform, whole-worker cost or scientific results.
The primary Linux/Serde references and independent source reviews are linked in
DESIGN.md, SOURCE_REVIEW.md and VALIDATION_20261029.md.

## Spec

The frozen implementation contract is DESIGN.md, with exact measurement-policy.json
(785 bytes including LF; SHA256 `45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50`).
Validated source is b7c11d1b5f8a1f37b769453d3ce0413696b9fc83; final production
CLI remains separately bound to `27679c4e59bc38e8c18e34dbc9687f9a2d5eb317`.
This handoff does not amend the frozen policy or scientific protocols.

## State

Regular [PR #2825](https://github.com/isogeny-labs/crypto/pull/2825) was opened at
2026-10-29 05:22:55 UTC on the original parent branch and attached by the parent
coordinator at 05:23:08 UTC. Its initial documentation head was
`aca42cc2aaa908e7fa62e1dfc8ebf86911955230`. The current PR metadata and final
checkpoint identify subsequent documentation-only handoff commits; hosted
checks remain separate from local validation and no automatic merge is enabled.

The full unfiltered serial library gate passed 5231/0/123 with 0 filtered and
exit 0 on immutable executable `1e7496533221060ea6202ff4fc78c80b0d5a11cb1bdabf1bfb68c430551eec8b`.
The final two exact mock retention tests and strict release Clippy passed.
Earlier source 27679 selected/dataset/CLI gates and their overlap are documented
separately. Every source/runtime/dependency/build/test path remains identical
between the validated source and delivery documentation except the documented
earlier test-only retention change from 27679 to b7c11.

Final canonical archive:
s3://setec-astronomy-artifacts/ml-dataset/20261028/software/checked-phase-telemetry/dc0a2447aa6da09536c2f37b5326043907bfaca21190047008f3027bbdde85ad/validation.tar.gz
(98682039 bytes). Conditional PUT, fresh ETag-bound GET, byte comparison,
contained hashes, restored source bundle and retained fixture hashes passed.
VALIDATION_RECEIPTS.json preserves the original receipts. The archive includes
historical parallel 5230/1/123 exit 101, interrupted superseded serial 143 with no
verdict, earlier compilation failure and metadata-runner exit 127 recovery.

The final archive's payload/validation-b7c11/fixtures contains sealed-worker and
sealed-registryless-prefix, 49 files total. Those are mocked software fixtures,
with original synthetic run-binding placeholders clearly separated from actual
producing source/executable provenance. The independent consumer owns its own
audit/export/restore and admission conclusions.

T766 implementation ownership was released after source/binary verification;
its heartbeat stopped normally. T784 is documentation-only delivery ownership.
No implementation scope should be reacquired without a genuine failure requiring
a fix and an available reservation. Do not restart the old released heartbeat.

## Next steps

Reconcile PR #2825 on codex/ml-pmu-collector-20261028, preserving the exact
delivered head and original parent lineage. A new session first checks
GitHub, the Git checkpoint, VALIDATION_RECEIPTS.json and the immutable archived
logs before repeating any command. Hosted CI/review and later stack integration
belong to the parent coordinator. Pending checks are not passes; no automatic
merge into the feature parent or a TEMP review branch is authorized.

The initial documentation checkpoint is verified in SOURCE_CHECKPOINT_RECEIPTS.json.
The final documentation checkpoint is recorded with the final PR-head handoff.
For any later update, preserve its source/delivery checkpoint and freshly
download it to verify its declared hash/length, exact source HEAD and RESUME
bytes. If only delivery documentation changed, demonstrate all other Git paths
unchanged from the validated source rather than rebuilding or relabeling the
existing executables. Keep the scoped session entry with the PR.

## Runs in flight

No producer build, test, fixture generator or cloud job remains running in this
lane. The final full test has a terminal exit 0. Only the owned delivery heartbeat
may remain until publication/handoff. Existing external jobs, Diem and the old
freeze lane retain their separate owners; this lane did not restart or change
them. Do not infer global job completion from this producer handoff.

## Stop and success conditions

This software-delivery lane succeeds when its source and documentation are
committed/pushed, the regular stacked PR is attached to the parent task, and all
declared durable archives/checkpoints have fresh hash-verified readback. Release
delivery ownership after handing off the exact PR head and current observed
checks. The parent retains CI cadence and budget ownership. A PR or local pass
does not complete the broader scientific corpus, worker publication gate,
independent partial adapter, hardware admission or fall-degree question.
