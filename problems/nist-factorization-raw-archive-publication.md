# NIST factorization bulk archive awaits artifact-store access

- **Status:** `open`
- **Kind:** access/tool limit
- **Affected work:** [R02 evidence](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/README.md), PR #2379
- **Next action owner:** artifact-store operator after connection setup
- **Last updated:** 2026-10-22

## Problem

The complete R02 NFS raw archive exceeds GitHub's 100 MB file limit, while
the runtime has no configured AWS identity for the repository's required
`s3://crypto-autoresearcher/store` backend. AGENTS.md sections 13 and 13a
require large immutable evidence to be published through that store.

## Evidence and uncertainty

- **Observed:** current cloud runtime observations list no outbound
  identities, secrets or runtime variables. The task's AWS directory is
  empty. No credentials or identity alias have been inferred or invented.
- **Preserved:** complete raw files, deterministic local archive, byte count
  and SHA-256 in the [archive manifest](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/archive-manifest.json).
- **Available for review:** results, every individually named prime
  certificate, Lean product proofs, inspected report and compact runnable
  P-256 checkpoint are committed without the bulk archive.
- **Missing capability:** an observed-ready AWS connection authorized to
  use the configured store. No remote upload or `store:` index entry is
  claimed. This access gap does not invalidate the replayable arithmetic,
  and does not make full raw publication complete.

## What would unblock it

- [ ] Select the configured, observed-ready AWS connection and build/use
  the repository's native `crypto store` command.
- [ ] Push the unchanged checksum-bound archive with provenance, commit its
  actual `store/index.json` entry, and verify remote readback and SHA-256.

Those checks establish durable bulk publication. They do not complete the
remaining factorization or prove a historical seed-selection reason.

## Next discriminating step

Recheck runtime readiness after connection setup. If ready, use `crypto
store push` for this archive only; do not change the shared backend or
delete remote objects. If access fails, preserve the exact failure and
keep this record open. No credentials are requested in chat or copied into
evidence. Recheck trigger: an existing connection name or completed setup.
