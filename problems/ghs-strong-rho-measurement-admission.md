# GHS strong-rho frontier remeasurement lacks execution admission

- **Status:** open
- **Kind:** access/tool limit; experimental admission gap
- **Affected work:** [strong-rho protocol](../research/quartic_ghs_strong_rho_20261114/PROTOCOL.md), [round-A amendment](../research/quartic_ghs_strong_rho_20261114/ROUND_A.md), [frontier report](../research/ecbench_e2e_frontier_20261031/REPORT.md)
- **Next action owner:** authorized IsoLab budget/pool operator; experiment executor after every preflight gate passes
- **Last updated:** 2026-11-15

## Problem

The preregistered matched strong-rho panel for the restricted GHS-weak
`E0/F_{q^3}` family cannot yet be submitted because the current managed
runtime has no AWS/checkpoint identity, live account-budget headroom is
unknown, and the available GitHub/launcher roles cannot perform the mandatory
all-tenant pool check.

The unchanged scientific workload covers q = 4,051 through 4,194,271, three
instance seeds, five primary repetitions per seed through q = 262,111 and
three thereafter, with `strong-b8` primary plus the frozen width and old-rho
diagnostics. Round A changes only the high/top active deadlines. The original
acceptance criterion remains a complete, admitted, same-target, cold
IC/strong-rho comparison across all eleven sizes and all three native-work
weights.

## Evidence and uncertainty

- **Observed:** on 2026-11-14, `aws sts get-caller-identity` in the selected
  cloud runtime returned `Unable to locate credentials`; no checkpoint upload
  token or configured checkpoint profile was present. No build or Job was
  submitted. [pcc:unverified]
- **Observed:** the deployed GitHub OIDC trigger policy in
  `taskq/deploy/eks-karpenter/remote/bridge.yaml` permits the publisher,
  CodeBuild request bridge, and selected S3 receipts, but not AWS Budgets
  readback. The launcher EKS group is a namespace submitter and cannot list
  all Nodes and all-namespace Pods required by `scripts/isolab_pool_check.sh`.
  [pcc:unverified]
- **Derived or proved:** no mathematical obstruction and no performance result
  is established by these access failures.
- **Suspected explanation:** the selected runtime intentionally exposes no
  account identity or cluster-wide operator capability. This is an access
  boundary, not a failed experiment.
- **Missing capability:** an approved operator receipt with current daily and
  monthly budget balances, all-tenant pool/quota state, and a usable
  checkpoint transport; followed by a digest-pinned custom image and raw-result
  read access.
- **What this evidence does not establish:** it does not overturn or strengthen
  the historical N4/N4-v2 frontier, measure the new rho implementation, prove
  capacity absence, or authorize a different backend or accounting rule.

## What would unblock it

- [ ] An authorized identity records account `104186811093`, region
  `us-east-2`, both USD 200 daily-budget readbacks, the USD 1,000 monthly
  readback, their telemetry timestamps, charged/reserved spend, and at least
  the frozen USD 5 unreserved experiment headroom.
- [ ] The same launch window records a successful
  `scripts/isolab_pool_check.sh --cpu 8 --memory 8Gi --pool cpu-spot`
  receipt using all Nodes and all-namespace Pods, plus namespace quota and
  active Job inventory. Repeat it before each two-Job wave.
- [ ] `scripts/checkpoint.sh gate` passes for the pushed result branch, and
  `check-experiment-git` accepts every launch input at the exact GitHub
  revision.
- [ ] A custom image is published by digest with the pinned scientific-source
  hashes, and an authorized result reader can retrieve every raw object and
  verify `SHA256SUMS`.

These conditions admit a launch; they do not admit a performance conclusion.
Correctness, repetition, calibration, rho-step, replay, independent-validator,
PCC, benchmark-record, and leaderboard gates remain downstream.

## Next discriminating step

- **Action or proof obligation:** run the read-only identity, budget, quota,
  pool, and active-Job preflight from an approved IsoLab operator context.
- **Inputs and controls:** the exact pushed round-A revision, the next wave's
  8-vCPU/8-GiB concurrent request (16-vCPU/16-GiB total across four Jobs),
  current cost-control policy, and the existing all-tenant pool-check script.
- **Execution limits and stop condition:** read-only; stop before image build or
  Job creation on the first absent permission, stale/unknown budget balance,
  insufficient headroom, pool conflict, source mismatch, or checkpoint
  failure.
- **Outcomes and next actions:** a complete passing receipt permits the
  separately reviewed image/request round; a permission failure keeps this
  access blocker open; insufficient headroom waits for a later window; a
  later capacity or Job failure is retained as infrastructure evidence and
  consumes the frozen all-attempt ceiling.
- **Recheck trigger:** an authorized operator identity or complete signed
  preflight receipt becomes available.

## Resolution and history

Unresolved. Opened 2026-11-14 after the implementation/protocol PR merged and
the selected runtime's read-only access checks failed. This record neither
launches the panel nor changes the historical frontier.
