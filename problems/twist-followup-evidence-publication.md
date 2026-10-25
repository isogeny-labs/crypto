# Publishing the complete twist follow-up raw evidence

- **Status:** open
- **Kind:** access/tool limit and repository owner/reviewer exception requirement, separate from the mathematical and cost gaps
- **Affected work:** [follow-up execution](../research/twist_routes_1_3_20261021/execution-v2/README.md), [PR #2380](https://github.com/aburan28/crypto/pull/2380)
- **Next action owner:** artifact bucket/identity owner; native upload and replay lane after object access is granted
- **Last updated:** 2026-10-25

## Problem

The complete 2026-10-23 twist raw run bundle exceeds Git's 10 MiB limit and
cannot be uploaded to the configured `s3://crypto-autoresearcher/store`
because the supplied identity is denied object access. The 2026-10-25
credentials authenticate successfully with STS; S3 denies the native HEAD
preflight, exact-prefix listing and an atomic create-only PutObject. No
successful upload, fresh download or index entry is claimed.

The full PR also contains twelve older execution-v1 evidence blobs above
the newly merged size limit. Those files were already committed before
this follow-up. AGENTS.md §13a prohibits repacking/moving committed evidence;
§13 requires an owner/reviewer-added `large-file-ok` exception if retained.
No such exception is self-applied here.

This blocks durable remote publication of the full replay input. Small
summaries, receipts, source freezes, reports and proof logs are reviewable;
they do not replace the missing remote raw object. Local verification and
remote reproducibility are distinct statuses.

## Evidence and uncertainty

- **Observed:** the deterministic bundle, exact size, hash and local path are
  recorded in [RAW_ARTIFACTS.json](../research/twist_routes_1_3_20261021/execution-v2/RAW_ARTIFACTS.json).
  [Store-status logs](../research/twist_routes_1_3_20261021/execution-v2/checks/final-20261023/store-status-v2.stdout)
  retain the initial `NoCredentials` failure. The later
  [publication report](../research/twist_routes_1_3_20261021/reports/publication-20261025/REPORT.md)
  and linked redacted receipts distinguish successful authentication from
  denied S3 calls. The conditional-write denial is not merely a lack of
  bucket-listing permission.
- **Derived or proved:** no scientific inference follows from this failure.
- **Suspected explanation:** the particular identity/bucket policy cause is
  unknown. No IAM or bucket policy was changed.
- **Missing capability:** authorized PutObject and GetObject for the exact
  prepared content-addressed object. GetObject is still needed for replay;
  a successful read has not been established.
- **Not established:** successful upload, a resolvable store entry, a verified
  remote pull, complete remote replay, or any IC performance conclusion.

## What would unblock it

- [ ] Grant the intended identity object read/write access using the
  [narrow proposed policy](../research/twist_routes_1_3_20261021/reports/publication-20261025/REQUIRED_OBJECT_POLICY.json)
  and relevant bucket policy. This proposal is reviewable and has not been
  applied; its conditional-write path does not require bucket listing.
- [ ] Run the native store upload, using the manifest's exact immutable
  digest/name and an explicitly writable local cache.
- [ ] Verify a fresh remote pull, byte count and SHA-256; commit the actual
  `store/index.json` entry and extraction instructions in the PR.
- [ ] Replay the archived-member hashes and original native/CAS checker
  inputs after extraction. Keep loose live metadata paths intact.
- [ ] Resolve the twelve historical oversized-file entries through the
  owner/reviewer policy exception or an explicitly authorized compliant
  resolution; preserve the failed [full-PR check](../research/twist_routes_1_3_20261021/execution-v2/checks/final-20261023/large-files.stdout).

These conditions close evidence publication only. They do not construct the
missing higher-prime quotients, supply full costs or prove a speedup.

## Next discriminating step

- **Action:** publish the concrete prepared bundle once object access exists.
- **Inputs and controls:** immutable digest/size, normalized tar/xz recipe,
  frozen raw-member inventories, and a fresh verified pull.
- **Limits:** one prepared evidence object; no remote deletion, repointing,
  funding or new scientific campaign.
- **Outcomes:** a verified upload/pull permits remote replay; an access error
  remains an access failure. Do not add a fictitious store entry or split
  the bundle to evade the Git policy.
- **Recheck trigger:** an authorized store identity or owner/reviewer-approved
  publication mechanism consistent with repository policy.

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
