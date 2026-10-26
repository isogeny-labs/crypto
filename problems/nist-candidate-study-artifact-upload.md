# NIST study artifacts lack an upload-capable runtime

- **Status:** `open`
- **Kind:** access/tool limit
- **Affected work:** [criterion study](../research/nist_skip_criterion_20261025/README.md), [PR 2378](https://github.com/isogeny-labs/crypto/pull/2378)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-26

## Problem

The PDF and raw-evidence files cannot yet be delivered to the repository's
S3 artifact store because this runtime has neither the AWS CLI nor configured
upload credentials. The requested PDF and complete evidence bundle exist and
are verified in the local workspace. Their exact hashes and sizes are in the
study's [delivery record](../research/nist_skip_criterion_20261025/ARTIFACT_DELIVERY.json).

## Evidence and uncertainty

- **Observed:** the repository artifact tool exits 2 with
  `artifacts: aws not found; install the AWS CLI or set ARTIFACTS_AWS`.
  The [recorded check](../research/nist_skip_criterion_20261025/validation/artifact-store-checks.txt)
  retains the actual command and result. No AWS credential profile, ambient
  credential or checkpoint upload token is configured here.
- **Observed:** the follow-up commit passes the artifact policy. Three
  earlier frozen PDF/ZIP additions in this PR exceed the new policy and
  still require migration. They remain byte-identical to the prior PR head.
- **Derived or proved:** local SHA-256, archive integrity and PDF attachment
  checks pass. Remote upload and read-back verification have not occurred.
- **Suspected explanation:** none needed; the missing local capability is observed.
- **Missing capability:** a configured, upload-capable runtime for
  `s3://setec-astronomy-artifacts/store`.
- **What this does not establish:** no evidence object is claimed uploaded.
  This delivery obstruction does not resolve the historical selection or
  vulnerability questions.

## What would unblock it

- [ ] Use `tools/artifacts/artifacts.py push` for the three files named in
      the new delivery record and the three earlier frozen PDF/ZIP files;
      preserve their bytes and commit the resulting manifests and ignore rules.
- [ ] Verify the remote objects, hydrate and recheck the payload hashes,
      update the delivery record, and pass the PR's artifact-policy check.

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

2026-10-26: repository transfer resolved to `isogeny-labs/crypto`. Source and
summary publication is possible; the attempted artifact upload cannot start.
Local PDF and evidence delivery is verified. Unresolved.
