# NIST study artifact delivery

- **Status:** `resolved`
- **Kind:** access/tool limit
- **Affected work:** [criterion study](../research/nist_skip_criterion_20261025/README.md), [PR 2378](https://github.com/isogeny-labs/crypto/pull/2378)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-26

## Problem

The initial runtime lacked upload credentials. The six existing PDF,
evidence and image payloads have now been uploaded unchanged to the
repository's S3 store, remotely verified and downloaded for SHA-256 checking.
Their exact hashes and sizes are in the
study's [delivery record](../research/nist_skip_criterion_20261025/ARTIFACT_DELIVERY.json).

## Evidence and uncertainty

- **Observed:** the initial repository artifact-tool attempt exited 2 with
  `artifacts: aws not found; install the AWS CLI or set ARTIFACTS_AWS`.
  The [recorded check](../research/nist_skip_criterion_20261025/validation/artifact-store-checks.txt)
  retains the actual command and result. A temporary AWS CLI was subsequently
  installed; the identity check exited 255 because it could not locate
  credentials. The artifact-tool attempt with an explicit remote also failed.
  Those checks describe the initial runtime. The user later supplied access
  outside the repository, and authenticated checkpoint and artifact uploads
  passed. No credential values are included in these records.
- **Observed:** the artifact tool uploaded all six payloads, created six
  manifests and preserved hydrated copies. The three earlier frozen PDF/ZIP
  additions were removed from the Git index; their local bytes are unchanged.
- **Derived or proved:** remote verification reports six hydrated entries and
  zero problems. Full GET read-back of all six objects matches SHA-256.
  [Delivery receipts](../research/nist_skip_criterion_20261025/validation/artifact-delivery-20261026/)
  retain object mappings, upload and verification outputs. The PR artifact
  policy still runs independently against each published head.
- **Suspected explanation:** none needed; the missing local capability is observed.
- **Missing capability:** none for these six artifact transfers.
- **What this does not establish:** this delivery resolution does not resolve the historical selection or
  vulnerability questions.

## What would unblock it

- [x] Use `tools/artifacts/artifacts.py push` for the three files named in
      the new delivery record and the three earlier frozen PDF/ZIP files;
      preserve their bytes and commit the resulting manifests and ignore rules.
- [x] Verify the remote objects, hydrate and recheck the payload hashes,
      and update the delivery record.

These checks establish artifact availability and policy compliance. The
scientific conclusions and their remaining evidence obligations are unchanged.

## Next discriminating step

- **Action:** run the existing artifact tool from an upload-capable runtime;
  migrate the six named files under the policy in [docs/artifacts.md](../docs/artifacts.md).
- **Inputs and controls:** existing frozen files and their committed digests;
  reject any byte mismatch.
- **Execution limits and stop condition:** file transfer and integrity checks
  only; no research rerun, remote deletion or history rewrite. Stop on a
  missing credential, failed upload, checksum mismatch or failed gate.
- **Outcomes:** successful read-back permits committing valid store manifests;
  any infrastructure failure leaves delivery unresolved.
- **Recheck trigger:** availability of a configured upload runtime.

## Resolution and history

2026-10-26: repository transfer resolved to `isogeny-labs/crypto`. Installing
the temporary AWS client first exposed unavailable credentials. After the
user supplied access, all six unchanged payloads were uploaded and their
full remote bytes verified. Store manifests replace Git payload tracking.
The original failed checks remain preserved. Resolved for these transfers.
