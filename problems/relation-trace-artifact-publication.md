# Relation trace raw archive publication

- **Status:** open
- **Kind:** artifact-store access/tool limit; exact missing-key versus permission cause unknown
- **Affected work:** [PR5102](https://github.com/isogeny-labs/crypto/pull/5102), [fixed-cohort diagnostics](../research/relation_trace_exact_20261124/REPORT.md)
- **Next action owner:** repository artifact-store administrator for access; relation lane for unchanged migration/read-back
- **Last updated:** 2026-11-24

## Problem

The raw trace archives are committed and reviewable in Git, but canonical artifact-store migration fails with HeadObject 403, preventing normal PR merge admission. The local checkpoint/S3 waiver does not replace the repository's artifact or merge gates. [pcc:unverified]

The affected payloads are the complete six-case seed7 LAST-star degree-space cohort and its retained partial ENOSPC predecessor. This publication gap is separate from mathematical replay and the still-open original 15× performance goal. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** [migration job 138214848663](https://github.com/isogeny-labs/crypto/actions/runs/45325758779/job/138214848663) on head 17e329c770 reports HeadObject 403 for partial SHA256 aa13e062592209ba8ba50ee1db098eefe86be7128cea7654bc557a573c059e84 and complete SHA256 9acbf195e594bb331b4fa2b41a4f21d336bebc7eb506811a49c312368591ecbc. [pcc:unverified]
- **Observed:** [CI_STATUS_431.json](../research/relation_trace_exact_20261124/CI_STATUS_431.json) and retained raw CI logs identify the exact jobs, hashes and publication head. The ordinary MERGE auto-merge request is enabled, with PR blocked. [pcc:unverified]
- **Derived or proved:** No store availability or permission diagnosis is proved by 403. The tool explicitly notes that an absent key without ListBucket may also return 403. [pcc:unverified]
- **Suspected explanation:** Missing object access or absent-key visibility; unknown which. No credential values are requested or inspected.
- **Missing capability:** A supported authenticated artifact writer/read-back path that can distinguish absence and publish the identical complete objects.
- **Limits:** Local native replay, complete Git archives and matching hashes do not establish durable artifact-store custody, full-root qualification, independent attestation or a speedup. [pcc:unverified]

## What would unblock it

- [ ] The maintained artifact tooling uploads or recognizes both unchanged complete objects, generates the real manifests, and reads back the recorded SHA256 values under supported access.
- [ ] The ordinary artifact migration/policy jobs pass on the final head without gate/config/key changes or fictitious pointers.

This resolves archive custody only. The unrelated legacy source assertions and original root/performance qualification remain separate obligations. See [current-main regression gates](current-main-research-regressions.md).

## Next discriminating step

- **Action:** Inspect the supported artifact writer's HeadObject/PutObject/read-back permission and absent-key handling; run the maintained migration only once access changes.
- **Inputs and controls:** The two original Git payloads and exact digests above, canonical artifact configuration and normal generated manifest contract.
- **Limits and stop:** No benchmark or cloud experiment; no credential search, dummy manifest, weaker policy or modification of frozen payloads. An unchanged 403 is retained as a publication failure.
- **Outcomes:** A successful full-object read-back permits ordinary artifact gates to decide custody. A further403 leaves the problem open and its cause unresolved.
- **Recheck trigger:** Supported artifact writer/access changes or a new canonical migration receipt.

## Resolution and history

2026-11-24: Unresolved. Both raw archives remain committed in immutable head 17e329c770 and the current branch; source 419, report 427 and all frozen claim proofs remain unchanged. [pcc:unverified]
