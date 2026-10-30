# PR2949 main integration: validation in progress

The reviewed merge is `4e063aa5bb048bd474c967d7c000aee741d51967`, with parents
`507316c7fc9a00afdc63f857e65b02fa3a109595` and
`af6ed463f556d26550d21a8c6c0191c172fd79ed`. Copilot request
[6516309991](https://github.com/isogeny-labs/crypto/pull/2949#issuecomment-6516309991)
had no response or visible resolver activity for over an hour; the documented
fallback explanation is
[6517345192](https://github.com/isogeny-labs/crypto/pull/2949#issuecomment-6517345192).
Auto-merge remains paused. This local integration has not been pushed yet.

Only the daily log and problem index conflicted. All 221/732 parent log
paragraphs and all 86/87 parent problem links are preserved. The prepared
publication handoff was appended after resolution. All 58 other original PR
blobs remain unchanged. No frozen evidence conflict occurred. The historical
source maps were checked at their own recorded revisions; old Cargo hashes
do not validate current dependencies. The resolution review and preservation
records here retain those checks.

Fresh scoped Lean validation passes 13 declarations, zero trusted exceptions;
the final affine module builds and kernel-replays against its imports. Local
size and artifact checks passed on the merge commit. The later handoff commit
changes only validation metadata and the append-only log.

The existing native validation is running in
`/workspace/pr2949-integration-20261030/`, tool session `49270`, shell PID
`14000`. Its exact command script is preserved here. Do not start a duplicate
while that supervisor or its children are alive. Collect each `.exit`, log and
timestamp. An absent terminal status is not a pass. The sequence covers three
example suites, strict lint/formatting, compiled CLI refusal at the unchanged
checkpoint gate, fresh backlog validation, and a fresh library build/run.
Only the library execution, after compilation, has a 2400-second cap.
`RAYON_NUM_THREADS=8`, Cargo jobs 2, serial test execution. These are development
checks, not measured scientific panels.

No fresh native or full-library pass is claimed at this checkpoint. The prior
5553-case cap remains historical: 2173 observed passes, zero observed failures,
47 ignored, one active and 3332 not reached; wrapper exit 124. Its raw log
remains NOT_UPLOADED. Keep new run logs separate, preserve failures/caps and
store large raw logs through the existing artifact policy when access exists.

Before pushing, collect actual validation outcomes and add a final receipt
without overwriting `resolution-review.json` or earlier receipts. Verify all
source-object bindings, review any remote head change, and rerun affected checks.
Then update the existing PR and inspect current-head `gate`, `size`, `artifacts`
and review state. Re-enable auto-merge only after resolution validation and use
an expected-head guard; no bypass or force push.

The full G1–G6 research goal remains incomplete: G1–G3/G6 partial, G4/G5 unrun.
There are no admitted cost, scaling or speedup results. Checkpoint/artifact/lab
access, qualified native execution/resume, complete pricing, external panels
and the Rust/polynomial/cost proof bridges remain open. The existing panel card
stays blocked and unfunded. No scientific launch, funding, lease, IAM, quota or
pool change occurred.
