# Quotient-calibration raw validation archive is not published

- **Status:** `open`
- **Kind:** tool/access limits; upload approval received
- **Affected work:** [relocation validation](../research/quotient_calibration_20261025/reports/run-20261025T1845Z/VALIDATION.md)
- **Next action owner:** user for restored GitHub and configured AWS access; then the repository operator
- **Last updated:** 2026-10-25

## Problem

The complete raw validation-log archive for the relocated source preflight
has no durable published copy. It is 67,540 bytes, above AGENTS.md section
13's 64 KiB data-file limit, so it cannot be added to Git under that policy.

## Evidence and uncertainty

- **Observed:** the local `runs.tar.xz` contains all native tests/lints,
  provenance checks, the full root test log and the focused failure/rerun
  logs. Its exact SHA-256, size, contents and intended destination are in
  [ARCHIVE_PUBLICATION.json](../research/quotient_calibration_20261025/reports/run-20261025T1845Z/ARCHIVE_PUBLICATION.json).
- **Observed:** automatic approval review rejected the attempted
  `tools/artifacts/artifacts.py push` before execution because it would send
  internal research/test logs to S3 without explicit authorization for the
  specific payload/destination. No retry or alternative upload occurred.
- **Observed:** the managed environment reports no configured AWS identity
  or secret bindings, and the AWS CLI is unavailable.
- **Derived:** repository source, compact receipts and reports can be
  pushed without this oversized file. That does not establish durable
  custody of the raw archive.
- **Suspected explanation:** none needed; the approval/access limits are
  concrete. No mathematical or performance limitation is inferred.

## What would unblock it

- [x] Obtain explicit approval for the exact archived payload and
  `s3://setec-astronomy-artifacts/store` destination.
- [ ] Configure supported AWS access and the artifact-store CLI.
- [ ] Upload the unchanged archive through `tools/artifacts`, commit its
  generated `.artifacts.json` manifest and verify fresh remote readback by
  SHA-256. Do not change bucket permissions or archive contents.

These steps establish durable raw-log custody. They do not repair the
unrelated ecbench test or execute the target-calibration experiment.

## Next discriminating step

After approval and configured access, hydrate the repository's tracked
`.artifacts/config.json`, confirm its exact configured remote, and publish
only this hash-pinned file. Stop on denied authorization, changed bytes,
upload failure or readback mismatch. A verified upload permits the normal
artifact manifest update; otherwise retain the explicit unpublished state.

## Resolution and history

2026-10-25 23:14 UTC: user identified `Isogeny-labs/crypto` as the current
repository; the local Git remote now uses that URL. The GitHub connection
cannot expose it (404/403), so repository identity and the current PR mapping
must be verified after organization-repository access is restored. See
[the repository follow-up](../research/quotient_calibration_20261025/reports/repository-20261025T2314Z/STATUS.md).
The existing merge/upload approval and archive hash are unchanged.

2026-10-25 21:44 UTC: explicit user approval received for the exact archive
upload and PR #2569 auto-merge. Archive hash and size rechecked unchanged.
AWS CLI installed; a normal SDK head request and repository artifact push
fail with `Unable to locate credentials`. GitHub repository/PR reads now
return 404 and Git remote access returns 403, preventing current-head
verification and publishing the manifest/status follow-up. See
[the approved follow-up](../research/quotient_calibration_20261025/reports/publication-20261025T2144Z/STATUS.md).
The approval requirement is satisfied; the remaining requirements are
supported account access, immutable upload and readback verification.

Unresolved. The local archive is retained at
`/workspace/crypto/research/quotient_calibration_20261025/reports/run-20261025T1845Z/runs.tar.xz`.
That workspace path is not a durable publication location. The source
preflight result and GitHub delivery remain separately scoped.
