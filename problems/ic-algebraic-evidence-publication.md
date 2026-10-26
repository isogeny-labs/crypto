# Algebraic IC evidence publication is blocked

- **Status:** open
- **Kind:** access and publication gate; no mathematical obstruction
- **Affected work:** [IC campaign](../research/ic_algebraic_crossover_20261023/README.md), D2
- **Last updated:** 2026-10-26

## Problem and observed evidence

The owner explicitly approved publication on 2026-10-26. The results summary
and title were published to the existing ready private PR 2550 and checked
equal to the prepared text. Its repository moved to isogeny-labs/crypto with
the same repository ID. The previous approval-review refusal is resolved;
this remaining problem concerns data/source delivery and validation.

The merged base also adds the artifact-store policy: non-code files over
1 MiB, and data-suffixed files over 64 KiB, must be uploaded unchanged to
the configured store before manifests replace Git blobs. Twenty-four original
files have real remote manifests and preserved local bytes matching every
recorded SHA-256. The remaining current inventory is 41 files, 7,508,437 bytes:
ten locally tracked, 23 study pending and eight benchmark pending. The earlier
59-file dry run is historical. The remaining bytes include unchanged constructor
binaries, complete raw validation bundles, logs, plots and inspected PDFs. The
[concrete packet](../research/ic_algebraic_crossover_20261023/PUBLISH_REVIEW.md)
lists the exact proposed publication scope and current checks.
The managed environment currently has no configured secrets, runtime
credential bindings or outbound identity. The earlier CI identity's
ephemeral configuration read was explicitly denied. No artifact upload
or successful remote readback is claimed. Lack of a local credential does
not prove that the separate artifact-store identity lacks permission. A bounded
CI metadata relay was implemented and passes 16 transport/artifact-tool tests,
but GitHub's normal binary-upload endpoint rejected the configured credential
with HTTP 401. No relay was dispatched, and no S3 object was written. Its empty
temporary draft release contained no asset and was removed. The owner was asked
whether to use the documented `large-file-ok` Git exception for the exact 41
files; that required decision is pending. No exception has been applied.

This blocks durable delivery under current policy, not the validity of the
retained local finite results. Older frozen sources passed 4,634/4,635 library
tests; qualified query/closure and wide-core archive replays and six Lean
dependencies pass on their recorded inputs. The merged library gate now fails
before tests on [unchanged upstream SAT syntax](sat-progress-phase-base-syntax.md).
The fresh merged browser check passes desktop/mobile/dark/no-JS without script
errors or external requests. Full site assembly cannot start because its
8,451,778-byte migrated `docs/backend-leaderboard/leaderboard.json` is not hydrated;
the real manifest's SHA-256 is
`91d30a0f5d72dd191b5361f5c5a2699d8b0e6ac074b2f21957d8155daa75cab9`.
Remote CI remains separate and no green merge is claimed. Never fabricate manifests, split or rename
data to avoid thresholds, change the policy, or rewrite frozen evidence.
The related [NIST publication problem](nist-factorization-raw-archive-publication.md)
records a different evidence set with the same need for verified remote bytes.

## What would unblock it

- [x] Explicit owner approval; PR title/results summary published on 2026-10-26.
- An observed-ready identity allowed to upload and read the configured
  artifact objects, with unchanged bytes verified by SHA-256 on remote readback;
  or an explicit owner-approved Git storage exception, honestly recorded as such.
- Commit the generated manifests and ignore blocks, update reproduction and
  hydration instructions, then pass the artifact gate against the final base.
- Complete applicable source/replay/site validation, retaining unrelated base
  failures separately. This does not authorize bypassing review or CI.

## Next action

Keep completed evidence committed locally. The original remote wave was
migrated by commit b1ab8a4e5; preserve and verify its real manifests. Prepare
a bounded manual CI relay only if its input transport becomes usable; the
implemented relay is not a successful upload. Await the pending owner storage
decision. With ready artifact access, use tools/artifacts for unchanged upload
and remote verification; with an explicit exception, retain the exact bytes and
do not invent manifests. Publish with a normal forward-only push and check the
resulting PR head. Cloud
experiments still require ephemeral and the separate total budget ceiling.

## Resolution

Unresolved. A local merge, prepared upload plan or green native test does
not satisfy publication or complete the broader research campaign.
