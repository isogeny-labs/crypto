# Publishing the complete twist follow-up raw evidence

- **Status:** open
- **Kind:** access/tool limit, separate from the mathematical and cost gaps
- **Affected work:** [follow-up execution](../research/twist_routes_1_3_20261021/execution-v2/README.md), [PR #2380](https://github.com/aburan28/crypto/pull/2380)
- **Next action owner:** environment/store identity owner; native upload and replay lane after credentials exist
- **Last updated:** 2026-10-23

## Problem

The complete 2026-10-23 twist raw run bundle exceeds Git's 10 MiB limit and
cannot be uploaded to the configured `s3://crypto-autoresearcher/store`
because this managed environment has no AWS credentials.

This blocks durable remote publication of the full replay input. Small
summaries, receipts, source freezes, reports and proof logs are reviewable;
they do not replace the missing remote raw object. Local verification and
remote reproducibility are distinct statuses.

## Evidence and uncertainty

- **Observed:** the deterministic bundle, exact size, hash and local path are
  recorded in [RAW_ARTIFACTS.json](../research/twist_routes_1_3_20261021/execution-v2/RAW_ARTIFACTS.json).
  [Store-status logs](../research/twist_routes_1_3_20261021/execution-v2/checks/final-20261023/store-status-v2.stdout)
  reach the installed CLI's `NoCredentials` error. The first attempt lacked
  the CLI; installing it does not supply an identity.
- **Derived or proved:** no scientific inference follows from this failure.
- **Suspected explanation:** no identity is configured; remote write/read
  authorization has not been tested and remains unknown.
- **Missing capability:** an environment-attached AWS identity authorized to
  list, write and read the configured store prefix.
- **Not established:** successful upload, a resolvable store entry, a verified
  remote pull, complete remote replay, or any IC performance conclusion.

## What would unblock it

- [ ] Attach the store identity through environment credentials/settings,
  without placing credential material in the repository or chat.
- [ ] Run the native store upload, using the manifest's exact immutable
  digest/name and an explicitly writable local cache.
- [ ] Verify a fresh remote pull, byte count and SHA-256; commit the actual
  `store/index.json` entry and extraction instructions in the PR.
- [ ] Replay the archived-member hashes and original native/CAS checker
  inputs after extraction. Keep loose live metadata paths intact.

These conditions close evidence publication only. They do not construct the
missing higher-prime quotients, supply full costs or prove a speedup.

## Next discriminating step

- **Action:** publish the concrete prepared bundle once the identity exists.
- **Inputs and controls:** immutable digest/size, normalized tar/xz recipe,
  frozen raw-member inventories, and a fresh verified pull.
- **Limits:** one prepared evidence object; no remote deletion, repointing,
  funding or new scientific campaign.
- **Outcomes:** a verified upload/pull permits remote replay; an access error
  remains an access failure. Do not add a fictitious store entry or split
  the bundle to evade the Git policy.
- **Recheck trigger:** a configured store identity or owner/reviewer-approved
  publication mechanism consistent with repository policy.

## Resolution and history

2026-10-23: preserved complete local outputs, prepared the deterministic
bundle and installed the AWS CLI; credential discovery fails. Unresolved.
No existing committed raw tree was repacked or deleted.
