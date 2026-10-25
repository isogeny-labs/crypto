# Algebraic IC evidence publication is blocked

- **Status:** open
- **Kind:** access and publication gate; no mathematical obstruction
- **Affected work:** [IC campaign](../research/ic_algebraic_crossover_20261023/README.md), D2
- **Last updated:** 2026-10-25

## Problem and observed evidence

The campaign has additional local results, native validators and inspected
reports that are not yet published to PR 2550. Automatic approval review
rejected updating that PR with the prepared research results and IAM details,
citing lack of approval for that specific payload. A concrete description
was prepared and explicit approval requested; no answer is recorded yet.
This restriction is separate from the user's USD 100 compute authorization.
The repository was subsequently verified private. A narrower correction of
the already-published pricing statement was accepted; that does not approve
publishing the additional results or their evidence.

The merged base also adds the artifact-store policy: non-code files over
1 MiB, and data-suffixed files over 64 KiB, must be uploaded unchanged to
the configured store before manifests replace Git blobs. A scoped plan
selects 34 earlier tracked campaign files, approximately 3.6 MB. The final
explicit-file dry run includes those plus all 19 study and six benchmark
pending files: 59 files, 8.9 MB, including unchanged constructor binaries,
complete raw validation bundles, logs, plots and inspected PDFs. The
[concrete packet](../research/ic_algebraic_crossover_20261023/PUBLISH_REVIEW.md)
lists the exact proposed publication scope and current checks.
The managed environment currently has no configured secrets, runtime
credential bindings or outbound identity. The earlier CI identity's
ephemeral configuration read was explicitly denied. No artifact upload
or successful remote readback is claimed. Lack of a local credential does
not prove that the separate artifact-store identity lacks permission.

This blocks durable delivery under current policy, not the validity of the
retained local finite results. The full 4,634-test library gate, qualified
query/closure and wide-core archive replays, and six-theorem Lean gate pass;
the browser checks pass. Known base-site and remote CI failures remain
separate, and no green merge is claimed. Never fabricate manifests, split or rename
data to avoid thresholds, change the policy, or rewrite frozen evidence.
The related [NIST publication problem](nist-factorization-raw-archive-publication.md)
records a different evidence set with the same need for verified remote bytes.

## What would unblock it

- Explicit approval to publish the prepared results and corresponding
  evidence updates to the existing PR, following the automatic review refusal.
- An observed-ready identity allowed to upload and read the configured
  artifact objects; unchanged bytes verified by SHA-256 on remote readback.
- Commit the generated manifests and ignore blocks, update reproduction and
  hydration instructions, then pass the artifact gate against the final base.
- Complete applicable source/replay/site validation, retaining unrelated base
  failures separately. This does not authorize bypassing review or CI.

## Next action

Keep completed evidence committed locally. Prepare the exact scoped
artifact plan and reviewable PR description. After approval and credential
readiness, use tools/artifacts for upload and remote verification, publish
with a normal forward-only push, and check the resulting PR head. Cloud
experiments still require ephemeral and the separate total budget ceiling.

## Resolution

Unresolved. A local merge, prepared upload plan or green native test does
not satisfy publication or complete the broader research campaign.
