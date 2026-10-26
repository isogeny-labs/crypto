# Algebraic IC evidence publication is blocked

- **Status:** open
- **Kind:** access and publication gate; no mathematical obstruction
- **Affected work:** [IC campaign](../research/ic_algebraic_crossover_20261023/README.md), D2
- **Last updated:** 2026-10-25

## Problem and observed evidence

The owner explicitly approved publication on 2026-10-26. The results summary
and title were published to the existing ready private PR 2550 and checked
equal to the prepared text. Its repository moved to isogeny-labs/crypto with
the same repository ID. The previous approval-review refusal is resolved;
this remaining problem concerns data/source delivery and validation.

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

- [x] Explicit owner approval; PR title/results summary published on 2026-10-26.
- An observed-ready identity allowed to upload and read the configured
  artifact objects; unchanged bytes verified by SHA-256 on remote readback.
- Commit the generated manifests and ignore blocks, update reproduction and
  hydration instructions, then pass the artifact gate against the final base.
- Complete applicable source/replay/site validation, retaining unrelated base
  failures separately. This does not authorize bypassing review or CI.

## Next action

Keep completed evidence committed locally. The original remote wave was
migrated by commit b1ab8a4e5; preserve and verify its real manifests. Prepare
a bounded manual CI relay for the remaining bytes using the repository’s
documented artifact credential, without assuming it is ready. On success, use tools/artifacts for upload and remote verification, publish
with a normal forward-only push, and check the resulting PR head. Cloud
experiments still require ephemeral and the separate total budget ceiling.

## Resolution

Unresolved. A local merge, prepared upload plan or green native test does
not satisfy publication or complete the broader research campaign.
