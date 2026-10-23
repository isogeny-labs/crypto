# EKS recovery qualification and GitHub integration

- **Status:** partially resolved; GitHub and broader workload qualification pending
- **Kind:** implementation gap; runtime constraint
- **Affected work:** [shared Iso Lab skill](../.agents/skills/isolab-eks/SKILL.md),
  [EKS harness](../taskq/deploy/eks-karpenter/HARNESS.md)
- **Next action owner:** Iso Lab implementation task T-70
- **Last updated:** 2026-10-23

## Problem

The separate native recovery harness now resumes a qualified Spot-interrupted
crypto computation with CRIU and EBS snapshots, then admits a distinct fresh L2
epoch. Full main-branch GitHub Action execution and broader workload/runtime
qualification remain open. Ordinary v1 fresh jobs are intentionally non-resumable.

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
- **Implemented and tested:** periodic coordinated CRIU/filesystem generations,
  immutable request/owner/deadline binding, restore fencing and reconciliation,
  compatible replacement placement, and a new harness measurement epoch. The
  [automatic controller report](../taskq/deploy/eks-karpenter/resume/validation/20261021-controller/REPORT.md)
  distinguishes live recovery tests from offline failure-path tests.
- **Timing constraint:** `ecbench` captures its host, CPU plan and preflight at
  session start and uses per-child timing/perf counters. Restoring those objects
  onto another worker cannot establish valid measurements on that worker.
  Open-descriptor compatibility for arbitrary ecbench checkpoints is untested.
- **Not established:** transparent resume of arbitrary commands,
  cross-instance-family CRIU portability, GPU/ARM recovery, or L2
  validity for an interval that spans checkpoint/restore.

## What would unblock it

- [x] A versioned submission/status contract binds source, image, command/spec,
  deadline, ownership and recovery generation without silently dropping resume.
- [x] A durable controller periodically commits coherent RAM/filesystem snapshots
  after verified private storage, retains a previous valid generation and
  handles interrupted uploads, corrupt archives and controller restarts.
- [x] Replacement is fenced against duplicate live writers; replaying an
  ambiguous submission cannot create another running copy.
- [x] A real harness-submitted crypto computation survives an induced loss of
  its own Spot worker and finishes from restored state, within the original
  deadline. Workloads that cannot checkpoint fail admission visibly.
- [x] A distinct post-restore epoch earns L2 with new warmup and A/A controls,
  native audit/replay and complete S3 hash verification. Interrupted measurements
  remain retained and excluded from admitted timing claims.
- [ ] GitHub/AWS/MCP callers use that implemented contract; the shared skill
  documents the tested route and limitations.

## Next discriminating step

Qualify the periodic-tick bootstrap against the unchanged 600-round crypto
workload, then finish PR #2342 after exact-head CI/review and dispatch the
main-only GitHub Action. Require its built image,
source/binary hashes, actual EKS identity and complete L2/S3 receipts to agree.
Keep the existing 96-vCPU account quota and 64-vCPU worker cap. Interrupted
generation and corrupt-archive cases have offline tests; a live multi-generation
fallback fault injection remains additional qualification, not a completed test.

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
Automatic zero-restore submission passed its separate 36-record L2/30-replay/
10-A-A/19-S3-hash epoch. Test E then passed worker loss plus controller restart:
1,140 checkpointed records continued to 1,806, with the exact prefix retained.
Its fresh epoch passed 1,806 L2/1,800 native replays/600 A-A pairs/19 S3 hashes.
Duplicate terminal submission preserved its attempt and benchmark UID. Final
runtime qualification is recorded in the linked report. Sixteen native tests
include incomplete-generation retention, corrupt/truncated archive rejection,
original deadline and nonterminal-owner refusal. These offline tests do not
claim an actual live corrupt-EBS fallback experiment.

The shared validation-gate fix has landed on main; parent PR #2342 is being
refreshed against it. Pending exact-head CI/review and full main-only Action
execution keep this record open. The lab host MCP/client are upgraded; a fresh
connection exposed all nine tools and read F as succeeded. Existing connections
must reconnect to discover the recovery tools; other processes were left running.

2026-10-22 periodic-checkpoint follow-up: the generation-intent fix was merged
into #2342 and deployed after 18 native controller tests. Its new live attempt
failed before committing a generation: CRIU captured an exited child and the
supervisor correctly refused to call it a stopped process. The repair now retains
that uncommitted capture, lets the same computation reap the child, and recaptures
at most eight times within the original deadline. Credential checks and L2 epoch
retry policy are unchanged. Twenty controller tests and 4,474 library tests pass;
the [failed run and exact evidence](../taskq/deploy/eks-karpenter/resume/validation/20261022-periodic/attempt-a/REPORT.md)
remain recorded. Live multi-generation qualification and the main-only GitHub
Action still keep this integration record open.

2026-10-22 periodic attempt B verified the current runtime's two successive
committed encrypted generations, controller restart and worker-loss recovery
from generation 2. Its 2,486-record prefix was byte-identical after completion;
all 3,606 computational records verified and all 3,600 measured replays matched.
Both restored processes had the required stopped state and credentials. The
zombie recapture branch was not forced in this live run; native tests cover it.

**The frozen B qualification failed overall:** its distinct fresh epoch had
3,603 L2 and 3 L1 observations, all with matching native replay. The L2 gate
exited 42 before A/A; 18 emitted S3 payloads passed full readback hashes. The
controller failed the logical run without a measurement retry. No failed sample
was discarded. The three L1 records report three excess busy ticks each, zero
steal and no SMT siblings; the retained data cannot attribute those ticks to a
specific task or establish an accounting defect. No isolation threshold was
changed. The [B report](../taskq/deploy/eks-karpenter/resume/validation/20261022-periodic/attempt-b/REPORT.md)
retains its failed acceptance criteria and all public evidence. The runtime pin
now matches the deployed checkpoint repair with that limited qualification.
The original, separately frozen main-only Action acceptance criteria remain
unchanged and unverified. An identical B rerun is not scheduled to seek a pass.

2026-10-23 accounting diagnosis: a separately frozen native probe recorded every
one of 2,000 child executions on a c7i worker. Seven faulting children produced
50 ms of reported busy ticks inside approximately 25 ms wall windows, without
IRQ/softirq/steal increments. A control on the same instance type, kernel, image
and placement, with only `nohz_full` removed from bootstrap, completed all 2,000
samples with no excess above two ticks. All six emitted S3 payload hashes matched
for each probe. Both probes shared SMT siblings and claim no L2 grade. The c8a
probe A never started before its provisioning deadline; its failure is retained.
The [diagnostic report and inspected PDF](../taskq/deploy/eks-karpenter/diagnostics/validation/20261023-cpu-accounting/REPORT.md)
retain protocols, raw counters, identities and the original timestamp erratum.

Profile v5 keeps the periodic tick, rejects active full-dynticks CPUs before
kubelet admission, and retains CPU/IRQ/workqueue/RCU isolation. The native L2
grader and thresholds are unchanged. The separate 600-round c8a qualification
must still establish all L2/replay/A/A/S3 criteria before promotion; the counter
probe alone does not meet them or identify a precise upstream kernel defect.
The source merge conflict is resolved while retaining both problem-index rows.

Qualification D then exhausted its 1,800-second provisioning deadline without
starting a container. AWS rejected nominated c8a workers with
`MaxSpotInstanceCountExceeded`; other lab work occupied the available slot under
the unchanged 96-vCPU quota. It produced zero measurement payloads and no L2,
replay or A/A result. The [separate failure report](../taskq/deploy/eks-karpenter/diagnostics/validation/20261023-l2-qualification-d/REPORT.md)
retains the frozen workload, receipts, events and zero-object S3 listing. The v5
source and launcher packages are prepared but not promoted. A free 8-vCPU Spot
slot and a new, explicitly frozen provisioning attempt are needed to execute
the original full qualification; no measurement result has been retried.
