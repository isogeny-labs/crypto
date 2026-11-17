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

## Follow-up — proceed without S3

### What was done (commands, files, features)

- Owner instructed to proceed without S3. PR #4492 had already merged, so this continuation uses a new branch based on current main.
- Added S3-free artifact reads: verified cache, explicit recovery index, then the requested path's full Git history. The temporary automatic source for an S3-configured `pull` is Git; `--source remote` remains an explicit opt-in. File/HTTP source behavior is preserved.
- Git path-history simplification initially hid reachable historical objects. Following full history recovers the original session records. An archived report relocated without being tracked at its new path is mapped to its original historical Git blob.
- Retained the current backend report as a 684,523-byte deterministic gzip recovery bundle from the local verified cache. Its uncompressed SHA-256 and length match the existing manifest; no reproduction lock or manifest was resealed.
- Recovered and independently hash/length-checked all three artifacts from the originally failing backend pull: current report, archived report and claim-n13-v2 records, using an empty cache without S3.

### Decisions made (and why)

- Do not contact S3 in the default pull path while the bucket is disabled. Git's existing promisor remote may supply historical Git blobs for partial checkouts; full commit history is required.
- Retain the one report version missing from reachable path history as a checked-in recovery bundle. Keep all recovered content subject to its original SHA-256 and length, with bounded decompression and no skipped evidence gates.

### Open threads (unfinished, pick up next time)

- Artifacts absent from both Git and approved recovery bundles still fail with an explicit recovery requirement. This mechanism does not imply that the entire historical store is available.
- S3 remains disabled; no storage, IAM or credentials were changed. Upload/migration operations still require a functioning store when they have actual data to upload.

### Errors (what went wrong, how it was handled)

- An initial negative test incorrectly expected an already hydrated input to be absent; corrected the fixture to remove it before exercising failed recovery. Added cold-cache, same-size mismatch, merge-hidden history, recovery blob/bundle and decompression-limit controls.

- Follow-up validation: artifact suite 57 tests passed. The original backend pull command hydrated all three files (18.7 MB) from an empty cache with the AWS executable deliberately unavailable.
