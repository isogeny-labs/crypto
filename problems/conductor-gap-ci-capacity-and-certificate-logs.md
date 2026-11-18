# Conductor-gap PR CI capacity and certificate logs

- **Status:** investigating
- **Kind:** resource limit; unavailable failure evidence; unknown attribution
- **Affected work:** [PR #2560](https://github.com/aburan28/crypto/pull/2560), [CI follow-up](../research/conductor_gap_replication_20261025/checks/CI_FOLLOWUP-20261025.md)
- **Next action owner:** PR #2560 authoring workflow
- **Last updated:** 2026-10-25

## Problem

The full PR merge gate is unverified because an IC framework build exhausted
runner disk and a separate certificate job failed without retrievable logs.
This does not invalidate the completed archive replay or establish a solver
defect; neither failure can be treated as a passing validation.

## Evidence and uncertainty

- **Observed:** On head `b9d5caad068edaf6262ace868f6dd857d406fb8c`,
  [IC framework job 121126868766](https://github.com/aburan28/crypto/actions/runs/40179357316/job/121126868766)
  failed building `libcrypto_lib` with `No space left on device (os error 28)`.
  Its subsequent parameter and synthetic pipeline checks did not run.
- **Observed:** [Certificate job 121126869339](https://github.com/aburan28/crypto/actions/runs/40179357503/job/121126869339)
  stopped before native validation; the first log fetch returned `BlobNotFound`.
- **Observed:** The parent's corresponding framework run was cancelled, so it
  supplies no passing baseline and does not establish regression attribution.
- **Suspected explanation:** Checkout/build disk demand may exceed that runner's
  available capacity. Occupancy evidence has not isolated the responsible inputs.
- **Missing capability:** Failure logs for the certificate job and successful
  execution of the remaining checks on the final PR head.
- **Not established:** A native algorithm failure, a passing parent baseline,
  or a cryptographic obstruction. The measured archive and scientific verdicts
  remain unchanged; the dedicated replay workflow passed on the delivered head.

## What would unblock it

- [ ] Complete the IC build and its parameter/pipeline checks on the final head
  within the existing workflow budget, retaining the actual inputs and checks.
- [ ] Obtain the certificate failure evidence and pass its applicable native
  validation, or document a confirmed external/base failure under the repository
  rules. Required checks still must be satisfied before merging.

These conditions address CI execution only. Complete CM setup accounting,
algebraic-oracle coverage and general hardness conclusions remain separate.

## Next discriminating step

- **Action:** Retrieve the failed job evidence at the next scheduled check;
  inspect checkout/cache/build occupancy before changing runner behavior.
- **Inputs and controls:** Current PR head, its exact parent, existing workflow
  input sets, and the recorded failures. A cancelled baseline is inconclusive.
- **Limits:** No new scientific measurements or paid resources; retain existing
  validation and workflow budgets. Do not disable checks or mutate frozen evidence.
- **Outcomes:** Fix a demonstrated PR regression and revalidate it; report an
  established baseline/service failure; retain unknown causes as unresolved.
- **Recheck trigger:** The next authoring-workflow check or new CI/review evidence.

## Resolution and history

2026-10-25: recorded while repairing demonstrated dashboard/registry-output
staleness. No speculative certificate or runner change is included. Unresolved.
