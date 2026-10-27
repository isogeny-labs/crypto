# Quotient-calibration raw validation archive publication

- **Status:** `resolved`
- **Kind:** tool/access limits; approved publication and readback verified
- **Affected work:** [relocation validation](../research/quotient_calibration_20261025/reports/run-20261025T1845Z/VALIDATION.md)
- **Next action owner:** none for archive publication; PR CI/review remain separate
- **Last updated:** 2026-10-26

## Problem

The complete raw validation-log archive for the relocated source preflight
initially had no durable published copy. It is 67,540 bytes, above AGENTS.md section
13's 64 KiB data-file limit, so it cannot be added to Git under that policy.

## Evidence and uncertainty

- **Observed:** the local `runs.tar.xz` contains all native tests/lints,
  provenance checks, the full root test log and the focused failure/rerun
  logs. Its exact SHA-256, size, contents and intended destination are in
  [ARCHIVE_PUBLICATION.json](../research/quotient_calibration_20261025/reports/run-20261025T1845Z/ARCHIVE_PUBLICATION.json).
- **Observed initially:** automatic approval review rejected the attempted
  `tools/artifacts/artifacts.py push` before execution because it would send
  internal research/test logs to S3 without explicit authorization for the
  specific payload/destination. Explicit user approval subsequently resolved
  that requirement; credential-free retries failed before uploading.
- **Observed on 2026-10-26:** the repository artifact tool uploaded the
  unchanged archive, verified the remote object, downloaded it into an empty
  cache, and verified it again. All four operations exited 0. Fresh readback
  has SHA-256 `ae5e8b6973db769db3ccef01fb81ba5837702a26221206134091a8bdff3bad2a`
  and size 67,540 bytes. See the
  [publication receipt](../research/quotient_calibration_20261025/reports/publication-20261026T1437Z/STATUS.md).
- **Derived:** the raw archive now has verified durable custody under its
  immutable hash key. This establishes no mathematical or performance result.

## What would unblock it

- [x] Obtain explicit approval for the exact archived payload and
  `s3://setec-astronomy-artifacts/store` destination.
- [x] Configure AWS access for this operation and the artifact-store CLI.
- [x] Upload the unchanged archive through `tools/artifacts`, commit its
  generated `.artifacts.json` manifest and verify fresh remote readback by
  SHA-256. Do not change bucket permissions or archive contents.

These steps establish durable raw-log custody. They do not repair the
unrelated ecbench test or execute the target-calibration experiment.

## Next discriminating step

With separately configured AWS access, readers can hydrate the exact archive
through the tracked `.artifacts.json` manifest. No additional upload or
permission change is needed. Target calibration and the unrelated baseline
test failure remain separate work.

## Resolution and history

2026-10-26 14:37 UTC: uploaded exactly one approved archive to
`s3://setec-astronomy-artifacts/store/sha256/ae/ae5e8b6973db769db3ccef01fb81ba5837702a26221206134091a8bdff3bad2a`.
The write used the repository tool's conditional SHA-256-checked upload.
Remote verification, fresh-cache hydration and final verification passed.
The generated manifest is committed with the sanitized publication receipt;
credentials were used only by the temporary upload process.

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

Resolved for this exact raw-log archive. Historical reports retain the
initial unpublished state. The source-preflight result, hosted CI and GitHub
merge remain separately scoped.
