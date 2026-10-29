# Publishing the complete twist follow-up raw evidence

- **Status:** open; corrected remote publication verified, historical size-policy hold remains
- **Kind:** repository owner/reviewer exception requirement; the wrong-destination access diagnosis is superseded
- **Affected work:** [follow-up execution](../research/twist_routes_1_3_20261021/execution-v2/README.md), [PR #2380](https://github.com/aburan28/crypto/pull/2380)
- **Next action owner:** repository owner/reviewer for the twelve existing oversized blobs; research lane for remaining full replay and experiment requirements
- **Last updated:** 2026-10-25

## Problem

The complete 2026-10-23 twist raw run bundle exceeds Git's 10 MiB limit.
After the user corrected the destination, its immutable upload to
`s3://setec-astronomy-artifacts/store`, native study-local index registration,
fresh-cache download and remote verification all passed. Every original
member hash matches after restoring loose metadata, and the unchanged
270-cell incidence CAS input from the download passes the Rust admission
gate. The prior denials targeted the wrong `crypto-autoresearcher` bucket.
The proposed policy for that bucket is superseded; no permission change was
needed. See the [corrected report](../research/twist_routes_1_3_20261021/reports/publication-20261025-setec/REPORT.md).

The full PR also contains twelve older execution-v1 evidence blobs above
the newly merged size limit. Those files were already committed before
this follow-up. AGENTS.md §13a prohibits repacking/moving committed evidence;
§13 requires an owner/reviewer-added `large-file-ok` exception if retained.
No such exception is self-applied here.

Durable remote publication is now verified. The historical Git size hold
still prevents closing this combined record. One frozen downloaded-input
replay does not establish a complete scientific rerun or independent-host
replay; those broader requirements remain open.

## Evidence and uncertainty

- **Observed:** the deterministic bundle, exact size, hash and local path are
  recorded in [RAW_ARTIFACTS.json](../research/twist_routes_1_3_20261021/execution-v2/RAW_ARTIFACTS.json).
  [Store-status logs](../research/twist_routes_1_3_20261021/execution-v2/checks/final-20261023/store-status-v2.stdout)
  retain the initial `NoCredentials` failure. The later
  [publication report](../research/twist_routes_1_3_20261021/reports/publication-20261025/REPORT.md)
  and linked redacted receipts distinguish successful authentication from
  denied S3 calls. The conditional-write denial is not merely a lack of
  bucket-listing permission.
- **Correction:** those denied calls concerned the wrong destination. The
  [new receipt](../research/twist_routes_1_3_20261021/execution-v2/publication-20261025-setec-v1/RECEIPT.json)
  proves successful conditional PutObject, native push/index, fresh pull and
  remote verification in the user-selected bucket. No IAM or bucket policy
  was changed.
- **Verified integrity:** all 35,979 archived members and 104 loose metadata
  files match the original 36,083-member manifest; the downloaded frozen
  incidence input passes PARI replay and native Rust adjudication.
- **Not established:** complete scientific rerun, independent-host replay,
  any new IC performance conclusion, or a size-policy exception.

## What would unblock it

- [x] Select the intended bucket, `setec-astronomy-artifacts`; confirm actual
  read/write access by successful immutable upload and fresh download.
- [x] Run the native store publication using the exact immutable digest/name
  and an explicitly writable local cache.
- [x] Verify a fresh remote pull, byte count and SHA-256; commit the real
  [study-local store index](../research/twist_routes_1_3_20261021/store/index.json)
  and [extraction instructions](../research/twist_routes_1_3_20261021/store/README.md).
- [ ] Replay the archived-member hashes and original native/CAS checker
  inputs after extraction. Keep loose live metadata paths intact.
  All member hashes and the frozen E30 CAS replay pass; a full scientific
  rerun of every original checker has not been performed in this correction.
- [ ] Resolve the twelve historical oversized-file entries through the
  owner/reviewer policy exception or an explicitly authorized compliant
  resolution; preserve the failed [full-PR check](../research/twist_routes_1_3_20261021/execution-v2/checks/final-20261023/large-files.stdout).

These conditions close evidence publication only. They do not construct the
missing higher-prime quotients, supply full costs or prove a speedup.

## Next discriminating step

- **Action:** retain the verified publication, resolve the existing size-policy
  hold through the owner/reviewer, and use the downloaded bytes for further
  original-protocol replays.
- **Inputs and controls:** unchanged digest/size, normalized tar/xz recipe,
  frozen inventories, successful fresh pull and all member-hash checks.
- **Limits:** one prepared evidence object; no remote deletion, repointing,
  funding or new scientific campaign.
- **Outcomes:** complete original-protocol replay extends validation; a
  permitted size-policy resolution removes the separate merge hold. Neither
  changes the outstanding mathematical or cost requirements.
- **Recheck trigger:** owner/reviewer size-policy action or further replay
  evidence; do not request changes to the wrong bucket's policies.

## Resolution and history

2026-10-23: preserved complete local outputs, prepared the deterministic
bundle and installed the AWS CLI; credential discovery fails. Unresolved.
No existing committed raw tree was repacked or deleted. The scoped new-wave
size check passes; the full-PR check fails on the twelve older blobs. Both
outputs remain in the PR and merge stays on hold.

2026-10-25: valid ephemeral credentials were tested without persisting them.
The native upload stops on HeadObject 403. A read-only diagnostic establishes
valid authentication and denied exact-prefix listing. An independent
conditional PutObject is also denied, so no object was written and no remote
download was attempted. A policy for this exact artifact is prepared for the
owner. The access blocker remains unresolved; the reason is now permissions,
rather than absent credentials. Historical receipts and scientific results
are unchanged.

2026-10-25 destination correction: the user identified the default bucket as
the wrong destination. Read-only discovery found `setec-astronomy-artifacts`.
The same exact prepared archive was conditionally uploaded there with S3
SHA-256 verification, registered through the native store, downloaded into a
fresh cache, and verified. All 36,083 restored file hashes match, and the
frozen E30 input passes independent PARI replay plus its native Rust gate on
the same host. Earlier denied receipts and the unapplied wrong-bucket policy
remain historical evidence. The access diagnosis is superseded; the
historical Git size hold and broader scientific requirements remain open.
