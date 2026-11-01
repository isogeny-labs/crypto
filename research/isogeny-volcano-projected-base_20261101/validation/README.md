# Validation gates for the projected-volcano discriminator

No result or benchmark row may be produced until all gates below pass. This
directory is prospective; it contains no claim that a run or validation has
already happened.

## Execution gate

- Every cell in `../specs/manifest.json` has exactly one immutable EKS request,
  one successful Job UID/Pod UID pair, and matching source revision, OCI digest,
  ecbench binary SHA-256, spec SHA-256, and S3 payload manifest.
- Every session is `complete`; `records_written == executions_planned`; every
  outcome is `verified`; every measured row earns L2; the A/A admission passes.
- `ecbench verify --replay-all` is `OK` and every replay is identical. No session
  or failed attempt is deleted. Infrastructure retries retain their first
  receipts and use the frozen cell identity.
- `generate-specs.sh --check` and `sha256sum -c specs/SHA256SUMS` pass at the
  frozen source commit.

## Analysis gate

- `../analyse.sh` has the SHA-256 pinned in `../protocol.json` and runs without
  an unmatched record.
- The raw `ic-raw` positive control detects the preregistered floor deficit.
- `rho-neg` is equivalent within +/-3%, and `rho-neg`/`rho-neg-aa` passes the
  harness control criterion.
- The synthetic 0.95 floor plant is detected at the preregistered threshold.
- The 0, 0.1, and 1 GAE bands include every emitted uncharged counter. A missing
  counter or order reversal withholds any candidate/raw ranking.

## Publication gate

Before any leaderboard or `benchmarks/README.md` row, an independent validator
must add beside the evidence:

1. `replay-certificate.json`, status verified/pass, `independent: true`, scalars
   verified, and a non-empty list of independently replayed runs;
2. `validator-report.json`, status pass, `independent: true`, repository raw-data
   hashes, and recomputed headline values;
3. `VALIDATION.json` under `validation-gate/v1`, naming matched negation rho in
   GAE, the same binary/process, strongest available reference, and the complete
   0/0.1/1 band.

Then run `python3 scripts/validation_gate.py evidence <evidence-dir>`. A producer
does not self-certify independence, and native EKS replay is execution admission,
not the independent replay certificate.
