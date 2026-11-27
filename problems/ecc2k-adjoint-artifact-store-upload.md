# ECC2K-130 adjoint study artifact upload

- **Status:** open
- **Kind:** observed authenticated S3 access failure; cause unknown
- **Affected work:** [PR5015](https://github.com/isogeny-labs/crypto/pull/5015), [adjoint projector study](../research/trace_zero_projection_setup_20261123/README.md)
- **Next action owner:** configured artifact-store infrastructure owner / next upload-capable continuation
- **Last updated:** 2026-11-23

## Problem

The complete raw study cannot yet finish the required artifact-store migration because the original authenticated upload/replay workflow fails HeadObject with HTTP403. The scoped index-calculus implementation and fixed instruction panel are published; remote custody and normal merge/admission gates remain open. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** [artifact-autofix run45213255631](https://github.com/isogeny-labs/crypto/actions/runs/45213255631), source2e3e496b, failed in its original upload/replay step. [Full compressed log](../research/trace_zero_projection_setup_20261123/checks/artifact-autofix-45213255631.txt.xz), [run API](../research/trace_zero_projection_setup_20261123/checks/artifact-autofix-45213255631.json) and [failed step](../research/trace_zero_projection_setup_20261123/checks/artifact-autofix-failed-step.txt) preserve the failure. Five failed requests identify four distinct hashes; one appears twice. [pcc:unverified]
- **Retained:** complete raw rows, all counter/native/profile outputs in the deterministic whole archive, exact frozen executable and all source/export hashes remain in the PR. No payload was replaced by a partial archive or marked uploaded. [pcc:unverified]
- **Derived or proved:** no cause or object-existence conclusion is established. A403 may represent denied access or an absent object hidden without ListBucket permission; it is not a404.
- **Missing capability:** successful original authenticated upload and full byte read-back under the configured identity/store.
- **Limits:** this infrastructure failure does not alter the measured instructions or constitute independent validation. Full-root release compilation also timed out before tests and remains a separate integration gate. [pcc:unverified]

This is consistent in symptom with the existing [shared store obstruction](chained-transfer-current-store-and-startup.md); no common root cause is asserted.

## What would unblock it

- [ ] Original-policy upload of every selected complete object succeeds, with exact whole-byte SHA256/length read-back and valid generated store manifests.
- [ ] Final-head artifact/size/PCC/validation/integration gates and review finish without bypass. The full-root test obligation remains independent of storage success.

Successful storage would close remote custody for this study, not confer independent scientific admission.

## Next discriminating step

Use the original artifact tool on the preserved source only after evidence that the configured authenticated store operation is available. Scope is file transfer/integrity checks, with zero scientific reruns, paid resources, remote deletion or history rewrite. Stop on authentication/HTTP failure or any hash mismatch. Successful full remote replay permits the actual generated manifests; another failure preserves the new complete log and leaves this record open.

No direct Put around ambiguous Head403, alternative identity/provider, permission exemption, empty push or unchanged failing-suite retry is proposed. Recheck when store access changes or a substantive new source has its own automatic CI evidence.

## Resolution and history

Unresolved. The ready PR retains normal auto-merge subject to gates; it is not a waiver of any failed check. Completing this record does not complete migration or root-library validation.

The preliminary unsigned statement PCC-20261123-e89520 mistakenly counted five requests as five distinct objects. The correction receipt retains that claim and identifies the actual four distinct keys; it is not relied on. [pcc:unverified]


### 2026-11-23 — successor half-trace constructor, PR5070

The separately frozen half-trace study hits the same HTTP403 symptom in original artifact-autofix run45247559161, while uploading source/protocol head8453eb6263. The exact failed step is [preserved](../research/trace_zero_halftrace_elimination_20261123/checks/artifact-autofix-source-freeze.txt); no common cause or object absence is inferred. [pcc:unverified]

The successor retains complete [raw/executable archives and member hashes](../research/trace_zero_halftrace_elimination_20261123/README.md), with byte-exact local extraction checks. Its numerical panel and focused checks are complete; broad integration/admission and remote custody remain distinct. Original authenticated whole-object migration/read-back is still required; no direct-Put, alternate identity, policy waiver or unchanged retry. [pcc:unverified]
