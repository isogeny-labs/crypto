# Actual-parent 6420ed5c integration validation

This is development validation for PR #2684, not a pilot, confirmation, new
parameter search, curve/map construction or solver campaign. This record covers pre-publication development
validation. All historical receipts remain retained.

## Source and conflict preservation

- Previously published head: `ad8c52e0ddac1643ba3ee68b255bd461fc6d285a`.
- Fetched actual parent: `6420ed5c3a26ad2825e437097ecfdc2f8f61d8d3`,
  from still-open #2430, following its newer merge of main.
- Local merge: `df3b0ec17a324a14e2f474a97e82021fac9a6cfa`, with those
  child/base parents in that order. No rebase, force-push or parent PR merge.
- Copilot request [6451458252](https://github.com/isogeny-labs/crypto/pull/2684#issuecomment-6451458252)
  had no response or resolution after about one hour. Takeover comment
  [6452700333](https://github.com/isogeny-labs/crypto/pull/2684#issuecomment-6452700333)
  supersedes it using the current AGENTS.md fallback.
- Exactly two conflicts were resolved: the shared session log and problem
  index. All eight child and 33 parent log entry bodies are preserved
  verbatim; seven distinct child versions are appended to the intact parent
  log. All original problem-index links survive, including the child's
  current allocation-status row and the parent's new entries.
- Library, Cargo, workflows, gate and artifact tools equal the actual parent.
  At the merge, all three conductor study trees equal the published child's
  trees; the four `INPUTS.sha256` pins pass. New follow-up records only add
  documentation and do not alter frozen inputs or measured archives.

See [byte/source preservation receipt](merge-preservation.json).

## Development validation status

| Check | Observed result |
|---|---|
| Release cargo check, library plus native example | Pass |
| Release native example tests/build | 12 pass, zero failed/ignored/filtered; release build passes |
| Separate required full release library attempt | Started 20:33:58 UTC; still compiling at the retained snapshot; no final result |
| Artifact tool tests | 27 pass |
| Gate suite | 61/62 pass; committed-tree case fails |
| Full gate diagnostic comparison | Exactly the same 44 prior diagnostics; none names this study |
| Site suite | Blocked on absent artifact-backed backend leaderboard |
| Global artifact workflow wiring | Fails: the same two parent conductor workflows lack pulls |
| Scoped artifact policy / large-file / whitespace | Pass |
| Fresh plan/audit/preflight/backlog | Byte-identical retained allocation; zero solver calls; expected preflight 3/blocked; backlog valid |
| Pinned source/protocol/CSV/allocation / rustfmt | Pass |

[Development traces](development-traces.md) and
[command receipts](python-command-receipts.json) retain actual exits.
The 44 gate errors are the same diagnostic strings as the
[earlier retained diagnostics](../current-parent-20261027/gate-errors.json):
13 leaderboard, seven benchmark and 24 archive diagnostics; 20 explicitly
name missing files. The JSON list is 57,845 bytes, SHA-256
`8f922f0c9671f8409d2ebd91bc0248976df7df10ab0f2a26f8aea533a7a7cb40`.
No gate/index or unrelated parent integration is patched around.

## Artifact input contract

The current parent manifest pins the backend leaderboard to 8,451,778 bytes,
SHA-256 `074eafcf56e144fae0668391f2115be4c5d40cd101c1fb1e9191521391cb06be`,
but its reproduction lock names output digest
`16b0fd0e9a68483556b6d2e714de96e25004dfd5c423398bf74b3171d30ce6b5`.
These pins differ. The local file is unavailable, and no remote read-back
is independently verified here. The files are parent-identical; no new
artifact bytes or manifest pointer are fabricated. See the
[exact contract receipt](backend-artifact-contract.json). Normal credential
configuration is unchanged.

## Historical versus current evidence

The previous 543bbd-based full library result remains failed: 4,902 passed,
13 failed and 122 repository-ignored, exit 101. Nine network cases pass its
separate focused reruns; four parent-code failures remained on that source.
The new parent changes those ecbench paths, so their earlier failure cannot
be called freshly reproduced here, and source inspection is not a new pass.
The old raw log, progress snapshot and incomplete original-base receipts are
retained unchanged. Old PIDs and unified handle are retired.

The new full command started once at 20:33:58 UTC under the normal permitted
network context, including local TCP fixtures. The changed source justifies
this separate development-validation attempt; no old command is resumed or
rerun. Its [progress/ownership receipt](library-progress.json) has no final
exit or declared test count at the compilation snapshot. Keep the raw log and
explicit exit marker; missing processes alone never establish completion.
Do not relaunch this attempt merely because a session handle is unavailable.
The default sandbox denied OS identity reads of the new worker; this is not
process absence or a result, and no signal was attempted.

This integration still requires its actual full result, exact published-head
CI and reviews. Publication does not establish those remaining checks or
merge authorization. The old full aggregate stays failed.

## Scientific scope

The 128-class allocation, 4,096 class/log pairs, 32,768 primary slots/seeds
and 512 blocks are unchanged. Allocation validity does not construct exact
models/maps or run solvers. The pilot and provisional 1,024,000-solve
confirmation remain unlaunched, with controls/full replays additional work.
Crater equivalence, complete CM accounting and the prime-field algebraic
oracle remain open. No hardness or full-cost speed claim is added.
