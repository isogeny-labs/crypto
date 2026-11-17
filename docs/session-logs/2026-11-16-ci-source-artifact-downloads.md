# CI source and artifact download repair — 2026-11-16

## What was done (commands, files, features)

- Reproduced CI source materialization failure for `examples/n83_d18_root_inventory.rs`.
- Restored its exact dependency `research/n83_d18_shared_bits_20261020/root_inventory.rs` from original commit `4e1d311d87571aa3309ed4bfac51d06539ef2297`, blob `c464dfc051747463c92d7fa6af999053c44ca743`.
- The next materialization attempt exposed `examples/pdp_lin_cdcl.rs`, an explicit Cargo example target absent from HEAD. Restored the exact first-parent source of deletion merge `f1cc1c6e786`, blob `4d011bc52328f649c3ae2073bd63205a44ded4a2`. Its standalone library metadata compilation with `rustc --edition=2021 --crate-type lib --emit=metadata` passed.
- Switched content-addressed artifact downloads from high-level `aws s3 cp` to direct `s3api get-object`, preserving SHA-256/size verification and partial-file cleanup. Added regression tests for request arguments, endpoint forwarding, disabled access, denied access, missing objects and corrupt downloaded bytes.
- Artifact suite: 52 tests passed. CI scope suite: 14 tests passed. The full `node .github/scripts/ci-scope.mjs --materialize` check passed and materialized 280 compile-time inputs after both recoveries.
- Sources reviewed: `tools/artifacts/artifacts.py`, `tools/artifacts/test_artifacts.py`, `.github/scripts/ci-scope.mjs`, `.github/scripts/test-ci-scope.mjs`, `docs/artifacts.md`, the original source commits, and Amazon S3 API error responses: https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html.

## Decisions made (and why)

- Restore original Rust bytes rather than remove Cargo targets or suppress source-discovery errors.
- Preserve the configured remote and all content-addressed manifests. Do not turn a storage-access failure into a skipped or passing check.
- Record this handoff in a task-specific dated log because the shared `2026-11-16.md` path is reserved by Conductor task T-6720.

## Open threads (unfinished, pick up next time)

- Live S3 downloads remain blocked: `GetObject`, `GetBucketLocation` and `ListObjectsV2` returned `AllAccessDisabled` for `setec-astronomy-artifacts`, including with local account credentials. The bucket was not among buckets listed by that account. This does not establish who owns the bucket or why access was disabled.
- Owner action required: restore bucket access or identify an approved replacement store containing the exact manifest objects. The user was asked for a replacement URI/endpoint. Existing `ARTIFACTS_REMOTE` and `ARTIFACTS_ENDPOINT` overrides can select one; no substitute remote was guessed.

## Errors (what went wrong, how it was handled)

- The original CI `HeadObject` 403 hid the direct API's `AllAccessDisabled` error. Direct GetObject now retains the specific error and reports the corresponding recovery requirement.
- First local source-materialization retry had not staged the restored file outside sparse patterns; staged it with `git add --sparse` and retried.
