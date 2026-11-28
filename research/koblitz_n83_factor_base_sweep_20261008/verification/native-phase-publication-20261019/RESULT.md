# Approved native evidence publication

The user explicitly approved the new 49 MB bundle on 19 October 2026. The
native store push succeeded; a fresh S3 GET matched SHA-256 and length, and
all 308 payload files passed their SHA256SUMS checks after fresh extraction.
The manifest itself matched its frozen SHA-256. HEAD and GET receipts agree
on ETag and length; S3 reports AES256 encryption.

- Logical name: `store:evidence/n83/native-phase-20261019/runs.tar.xz`.
- SHA-256: `bef8a03c355b2238282638e326dd571bd91dd6873f5ed1df5fe12db2f0dcd5c3`.
- Bytes: 49,012,484.
- Manifest SHA-256: `30b56a3960e48b164dc354e779d32a5c6fc251a2eafb938d17af380bf2084004`.
- Remote: `s3://setec-astronomy-artifacts/store`, region `us-east-2`.
- Key: `store/sha256/be/bef8a03c355b2238282638e326dd571bd91dd6873f5ed1df5fe12db2f0dcd5c3`.
- Index and provenance: [store/index.json](store/index.json).
- Transport receipts: [push](push.json), [HEAD](s3-head.json), [GET](s3-get.json).
- Download checks: [archive](download.sha256), [manifest](download-manifest.sha256),
  [308 file checks](download-file-checks.log).

This addendum supersedes only the publication state in the frozen
[rejected plan](../native-phase-20261019-v3/raw-artifact-plan.json); that plan
and its compact review manifest remain unchanged. The archive preserves all
seven container attempts, including failed and censored attempts, original
binaries, three source archives, exact public inputs, native cards, relays,
assessments and original validation. Later integration validation remains
separately committed; publication does not enlarge any scientific claim.

To resolve the logical name, set `AWS_DEFAULT_REGION=us-east-2`,
`CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store` and
`CRYPTO_STORE_INDEX` to this directory's absolute `store/index.json` path.
Extract with `tar -xJf runs.tar.xz`; from `raw-bundle/`, run
`sha256sum -c SHA256SUMS`. The deterministic repack recipe remains in the
archive's `collect-evidence.sh` provenance. No archived bytes were changed.
