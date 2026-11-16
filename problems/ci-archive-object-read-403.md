# Shared CI archive objects return HeadObject403

- **Status:** open
- **Kind:** access/object-availability limit; cause not established
- **Affected work:** [PR4147](https://github.com/isogeny-labs/crypto/pull/4147),
  shared backend/IC leaderboard checks
- **Next action owner:** repository artifact/CI maintainers
- **Last updated:** 2026-11-16

## Problem

The `current` and `recompute` jobs cannot evaluate their shared archived
inputs because the artifact client receives403 Forbidden from HeadObject.
Their workflow definitions and artifact tool are unchanged between PR4147
code head3f57265af7deee978f7040afb65ba276c7181c50 and basea1c98c8e8.
This is separate from the auxiliary study's successful native/offline
replay checks and its corrected draft-tier registration.

## Evidence and uncertainty

- [current job](https://github.com/isogeny-labs/crypto/actions/runs/43941812873/job/133561011109):
  artifact pull for docs/backend-leaderboard and the historical claim-n13-v2
  session fails before its study computations.
- [recompute job](https://github.com/isogeny-labs/crypto/actions/runs/43941812941/job/133561009743):
  pull for ecbench_accounting_v2_20261023 and ic_tool_program/rule fails on
  archived SHA-256 object keys.
- [Exact excerpts](../research/p256_aux_smooth_20261114/continuation_20261116/evidence/ci-first-head-excerpts.json)
  retain the source job URLs and object keys. PCC-20261116-b9df82 is unsigned
  [pcc:unverified].
- Unchanged Git blobs: backend-leaderboard.yml6ed15e880c402b67163d6592f54725550f0f6166,
  ic-leaderboard-recompute.yml9fc9783ce0caa2ed0fc7caf7c266815b159a8e4d,
  artifact toolcb9aadf9fe7f33848148f8b75b9900bb5fefbb96.
- The403 alone does not distinguish missing objects from role/bucket-policy
  authorization. The CI identity/policy cause has not been established.
  No mathematical failure, primary benchmark failure or main-branch
  reproduction is inferred. No credentials are requested or included here.

## What would unblock it

- [ ] Restore availability and authorized reads for the named canonical
  object digests in the actual CI identity/context, without replacing hashes.
- [ ] Run the existing artifact pull commands and verify the returned bytes
  against their SHA-256 records, then pass the original jobs on the actual
  current PR head. Do not omit inputs, change recorded conclusions or waive
  the checks to make the PR mergeable.

These conditions restore those CI checks, not the missing EKS performance
panel,2x result, regularity proof or independent scientific standing.

## Next discriminating step

An authorized maintainer checks one failing object and its read policy under
the same identity/context, then reruns the unchanged command only after the
condition changes. If bytes are missing, recover the exact archived bytes;
if access is denied, repair the scoped read authorization. Recheck on that
change. No repeated solver experiment or credential workaround is needed.

## Resolution and history

2026-11-16: recorded from two exact-head failures. Unresolved. PR4147's
mathematical inputs and these shared archive paths were not substituted.
