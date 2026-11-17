# Full rho and index-calculus benchmark on the three Certicom 131-bit targets

- **Status:** `open`
- **Kind:** implementation gap; measurement and hardware access gap
- **Affected work:** [the 2026-10-23 stage study](../research/certicom_131_stage_bench_20261023/REPORT.md), [generalized binary GPU rho integration](certicom-binary-gpu-rho-integration.md), and [matched GPU workload validation](ecbench-gpu-matched-workload-contract.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-23

## Problem

The requested end-to-end rho and elliptic-curve index-calculus speed comparison on ECC2K-130, ECC2-131 and ECCp-131 is not yet established. The available full-field CPU code permits state-step and S3-root stage timings, but it does not yield complete, matched solver rates. The specialized ECC2K-130 GPU walker has no matched released counterpart for the other two curves, and the attempted fresh GPU host could not be launched under this worker's current quota and policy.

## Evidence and uncertainty

- **Observed:** The [stage study](../research/certicom_131_stage_bench_20261023/REPORT.md) records all three exact-curve CPU stage rates, raw five-sample CSV, curve/subgroup and positive-S3 checks, and isolation limits. The planned `g7.2xlarge` Spot launch failed with `MaxSpotInstanceCountExceeded`; on-demand failed with an explicit `SpotOnlyEC2Launch` policy deny. Neither created an instance. The ECC2K-130 CUDA client is specialized to its Koblitz walk; `src/binary_ecc` and `src/ecc` provide generic CPU arithmetic for the other curves.
- **Derived or proved:** A rho group-state step omits distinguished-point transport, collision probability, cycle handling and recovery. An S3 root query omits factor-base yield, relation collection, sparse linear algebra and target descent. These stage rates cannot determine complete solve rates by themselves.
- **Suspected explanation:** No unmeasured performance or mathematical claim is made.
- **Missing capability:** Released, independently replayed GPU rho clients for ECC2-131 and ECCp-131; a permitted available GPU slot for a fresh ECC2K-130 run; complete, exact-target index-calculus cost and success receipts with matched rho reference.
- **What this evidence does not establish:** No conclusion about a target logarithm, index-calculus crossover, full attack cost, or fastest implementation follows from the stage study.

## What would unblock it

- [ ] Obtain a permitted ephemeral GPU launch after Spot capacity/quota is available; run the frozen ECC2K-130 correctness gate and five complete-iteration samples, preserving the host and job receipts.
- [ ] Implement and independently validate ECC2-131 and ECCp-131 GPU rho walks with exact curve IDs, arithmetic parity, coefficient invariants, cycle handling, DP records, checkpoint/restart and collision replay; time the same defined workload on the same GPU class.
- [ ] For each exact target, run a complete index-calculus protocol through factor-base build, relations, filtering, linear algebra, descent and verified logarithm, or record a fixed-budget censored outcome with all phases and the matched strongest rho reference.

These conditions would establish measured rates only for the stated implementations, hardware and workload. A full target solve remains a separate result.

## Next discriminating step

- **Action or proof obligation:** Re-run the existing ECC2K-130 GPU benchmark on the same protocol once a Spot slot is available, while separately validating a generic ECC2-131 GPU walk against the CPU reference.
- **Inputs and controls:** Exact published curve parameters, planted known-answer subgroup points, fixed seeds, source hashes, matched step/DP definitions, independent replay and same-device timing.
- **Execution limits and stop condition:** Preserve the study's five-sample GPU protocol; stop on arithmetic mismatch, dropped report, unexplained cycle, failed launch or benchmark contention.
- **Outcomes and next actions:** A successful run supplies one GPU row; a failed launch remains access evidence; arithmetic failure requires a fix before timing; no collision within budget is a censored search, not a proof of hardness.
- **Recheck trigger:** A Spot quota/capacity change or an independently validated ECC2-131/ECCp-131 GPU client.

## Resolution and history

- 2026-10-23: Opened after the exact-curve CPU stage diagnostic and two failed GPU launch routes. Full benchmark coverage remains unresolved.

Unresolved. This record does not substitute stage timings for complete solver rates.
