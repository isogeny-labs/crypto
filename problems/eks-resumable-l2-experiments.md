# EKS recovery is not integrated with the benchmark harness

- **Status:** investigating
- **Kind:** implementation gap; runtime constraint
- **Affected work:** [shared Iso Lab skill](../.agents/skills/isolab-eks/SKILL.md),
  [EKS harness](../taskq/deploy/eks-karpenter/HARNESS.md)
- **Next action owner:** Iso Lab implementation task T-70
- **Last updated:** 2026-10-21

## Problem

The ordinary EKS harness cannot yet resume a Spot-interrupted experiment with
CRIU and an EBS snapshot, then automatically admit a fresh L2 benchmark epoch,
because it has no durable recovery controller or epoch transition contract.

The owner's selected policy preserves computation progress, rejects timing that
crosses the interruption, and requires new warmup, host/affinity checks, L2
observations and A/A controls before accepting new benchmark measurements.
Existing v1 jobs instead use `emptyDir` and zero automatic measurement retries.

## Evidence and uncertainty

- **Observed:** [The operator probe](../taskq/deploy/eks-karpenter/resume/validation/20261021/REPORT.md)
  restored a native SHA-256 chain at iteration 48,128 onto a replacement
  c8a.2xlarge Spot worker in another availability zone. CRIU 4.2.1 plus an
  encrypted CSI snapshot preserved RAM and files; all 1,048,576 steps completed
  with exactly one process start and native replay. Both worker identities and
  the original worker's termination fence are recorded. This proves the scoped
  mechanism, not arbitrary workload compatibility.
- **Observed runtime constraint:** containerd 2.3.5 defaults restore-via-create
  off. [Upstream](https://github.com/containerd/containerd/security/advisories/GHSA-p7v4-vr35-mj6f)
  states that enabling it leaves a destination-security-context bypass present;
  the feature is removed in 2.4. The test did not enable it.
- **Missing capability:** periodic coordinated CRIU/filesystem generations,
  immutable request/owner/deadline binding, restore fencing and reconciliation,
  compatible replacement placement, and a new harness measurement epoch.
- **Timing constraint:** `ecbench` captures its host, CPU plan and preflight at
  session start and uses per-child timing/perf counters. Restoring those objects
  onto another worker cannot establish valid measurements on that worker.
  Open-descriptor compatibility for arbitrary ecbench checkpoints is untested.
- **Not established:** transparent resume of arbitrary commands, automatic Spot
  failover, cross-instance-family CRIU portability, GPU/ARM recovery, or L2
  validity for an interval that spans checkpoint/restore.

## What would unblock it

- [ ] A versioned submission/status contract binds source, image, command/spec,
  deadline, ownership and recovery generation without silently dropping resume.
- [ ] A durable controller periodically commits coherent RAM/filesystem snapshots
  after verified private storage, retains a previous valid generation and
  handles interrupted uploads, corrupt archives and controller restarts.
- [ ] Replacement is fenced against duplicate live writers; replaying an
  ambiguous submission cannot create another running copy.
- [ ] A real harness-submitted crypto computation survives an induced loss of
  its own Spot worker and finishes from restored state, within the original
  deadline. Workloads that cannot checkpoint fail admission visibly.
- [ ] A distinct post-restore epoch earns L2 with new warmup and A/A controls,
  native audit/replay and complete S3 hash verification. Interrupted measurements
  remain retained and excluded from admitted timing claims.
- [ ] GitHub/AWS/MCP callers use that implemented contract; the shared skill
  documents the tested route and limitations.

## Next discriminating step

Implement the trusted supervisor/controller boundary around the tested process
mechanism, then run a frozen public workload through the real harness and kill
only its own Spot worker. Keep the existing 96-vCPU account quota and 64-vCPU
worker cap. Qualification must include an interrupted checkpoint, checksum
failure and duplicate recovery request, not just a clean manual restore.

A passed test would establish that workload's automatic recovery and epoch
transition. A failed descriptor/compatibility check would narrow the supported
workload class. A failed L2 or A/A gate is retained as a failed epoch, not retried
selectively as an infrastructure failure. Recheck after the controller/API
implementation or a change in the tested runtime class.

## Resolution and history

Unresolved. 2026-10-21: installed the EBS snapshot controller and class, validated
the manual CRIU/CSI replacement-worker mechanism, and made the shared skill
distinguish that proof from unavailable automatic harness recovery. Merging the
skill or this record does not close the integration requirement.

2026-10-21 implementation update: the real native ecbench qualification restored
36 records and replayed all 30 measured records after clearing inherited
supplementary groups; a normal perf-enabled process failed CRIU dump, so the
new native computational launcher deliberately disables perf and forces L0.
The [report](../taskq/deploy/eks-karpenter/resume/validation/20261021-ecbench/REPORT.md)
retains both failures and the fresh L2/S3 epoch.

A native controller, separate request/status contract, CodeBuild deployment,
MCP adapters and optional GitHub Action recovery input are now implemented.
Automatic zero-restore submission has passed computation replay and its separate
36-record L2/30-replay/10-A-A/19-S3-hash epoch. Worker-loss and controller-restart
qualification is running; this record remains unresolved until that evidence
and the failure-path checks pass. The shared inherited validation gate still
blocks the parent commit-image PR and full main-branch Action verification.
