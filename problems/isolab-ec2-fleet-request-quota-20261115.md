# Iso Lab EC2 Fleet request quota blocks worker provisioning

- **Status:** open
- **Kind:** resource/service limit; internal account-count cause unknown
- **Affected work:** [Four-rung single-target tournament](../research/ic_boundary_tournament_20261115/README.md)
- **Next action owner:** AWS account operator; existing Job remains queued
- **Last updated:** 2026-11-15

## Problem

The authorized n=61/71/73/83 single-target comparisons cannot start because EC2 CreateFleet refuses the fixed m8a.2xlarge worker with MaxFleetCountExceeded. Job ic-boundary-single-20261115-1912 has UID 751ec9e0-1e39-4c6e-a592-cc76c66a59a9. The requested 112 evaluation workloads, original K grids and validation gates remain outstanding.

## Evidence and uncertainty

- **Observed:** [Retained Karpenter errors](../research/ic_boundary_tournament_20261115/reports/run-20261115T1924Z/fleet-error.jsonl) and [pod snapshot](../research/ic_boundary_tournament_20261115/reports/run-20261115T1924Z/pods-snapshot.json). Live namespace/pool capacity admitted 4 CPU / 24 GiB. The regional Standard Spot vCPU quota readback was 96; only the existing system instance was running. This does not identify the fleet-count cause.
- **Observed:** The default DescribeFleets response was empty, but [AWS requires explicit IDs for instant fleets](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeFleets.html). Five historical IDs recovered from CloudTrail were already absent on explicit description. No stale record suitable for safe cleanup was established.
- **Derived or proved:** No algorithm statement. Worker provisioning has not succeeded.
- **Suspected explanation:** Unknown; AWS's internal fleet request accounting requires reconciliation.
- **Missing capability:** Provisioning an otherwise admitted worker in the authorized account/region/pool.
- **Limits:** A selected L2 pool is not an achieved isolation receipt. There are no algorithm timing, replay or comparative results from the queued pod.

## What would unblock it

- [ ] The original Job obtains its fixed worker before 2026-11-16T08:11:12.640103Z, retaining its UID and zero-retry policy.
- [ ] Account/operator evidence explains the CreateFleet refusal and establishes that ordinary provisioning is admitted within the existing resource and dollar ceilings.

Worker admission is necessary, not sufficient: compilation, all four rungs, measurement controls and replay remain downstream obligations. No quota, pool, provider or budget increase is implicit in this record.

## Next discriminating step

- **Action:** Reconcile the EC2 fleet-count service error using the retained AWS request IDs and explicit instant-fleet inventory; do not delete records associated with active/stopped instances.
- **Inputs and controls:** Frozen source, image, Job manifest and all 112 requested workloads linked from the study.
- **Limits:** USD20 additional authorization, original 13-hour deadline, one sequential measurement Job, zero experiment retries. Do not reset the deadline or relaunch after failure.
- **Outcomes:** If ordinary provisioning becomes available, the original queued Job runs unchanged. If the deadline expires, retain the terminal failure and classify the four comparisons as unattempted due to provisioning. Any later attempt requires its own authorization.
- **Recheck trigger:** Existing Job status/worker admission; the bounded watcher observes the same UID without submitting work.

## Resolution and history

Unresolved. 2026-11-15: exact launch inputs published; image smoke and local harness checks passed; one Job submitted; CreateFleet refusal retained. No fleets, instances, infrastructure limits or other workloads were changed.

2026-11-15T22:20Z: Owner selected native Mac execution of the same four rungs. The original cloud Job was UID-guarded, suspended and deleted before it executed; no associated pods remain. See [local plan](../research/ic_boundary_tournament_20261115/local-plan.json). The cloud fleet-count issue remains unresolved; local correctness and timing qualification are separate obligations.
