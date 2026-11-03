---
name: isolab-eks
description: Default execution path for authorized x86 CPU experiments and crypto benchmarks in this repository. Submit to Iso Lab on EKS Spot from Claude, Codex, OpenCode or a harness; verify the tested commit, L2, S3 results and checkpoint/resume requirements. Explicit legacy NATS or bare-metal requests use their own runner.
---

# Iso Lab on EKS

When the user asks to run a supported experiment, use this skill and execute
the submission with the available authenticated client. Do not stop at giving
the user a launch command. Loading the skill alone does not request a new run.
Honor an explicitly selected backend, frozen workload, budget and stop condition.
Local builds, unit tests and static checks remain development work.

Use the repository's maintained harness contract in
[`taskq/deploy/eks-karpenter/HARNESS.md`](../../../taskq/deploy/eks-karpenter/HARNESS.md).
This skill is shared with Claude through `.claude/skills/isolab-eks`; OpenCode
can discover the same `.agents/skills` and `.claude/skills` locations.
Work from the repository root. It does not grant AWS or GitHub access; use the
caller's configured identity without copying credentials into requests or Git.
For a missing connection or a new machine, read
[agent access and setup](../../../taskq/deploy/eks-karpenter/ACCESS.md): choose
the host MCP, remote AWS bridge, GitHub or in-cluster route, configure the client,
and verify the actual caller's permissions before submitting. No shared static
service-account key is required. Creating new identities or broadening access
is a separate administrative action, not an implicit effect of loading this skill.

The lab's recovery policy is **CRIU plus EBS volume snapshots**. Preserve
computation progress after a Spot interruption, invalidate the interrupted
timing interval, and start a new benchmark epoch with fresh warmup, L2 checks
and A/A controls. Use the separate `isolab.eks.resumable/v1` contract for this
policy; ordinary v1 jobs are fresh, non-resumable jobs. Read
[recovery requirements and commands](references/recovery.md) before launch.

## Select the submission path

- **Existing AWS credentials:** use the installed EKS MCP when available;
  otherwise use the portable AWS launcher below. Run `aws sts get-caller-identity`
  in the executor's environment and confirm account `104186811093`. Keep its
  existing role/profile; do not initiate SSO or ask for another key when that
  identity already has the required permissions. STS identifies the caller;
  it does not prove permission to submit. Follow the access guide for actual
  authorization checks. A missing MCP connection does not block the AWS bridge.
- **GitHub identity, callers anywhere:** dispatch `isolab-experiment.yml` on
  `main` with an explicit 40-character `source_revision`. It builds that commit,
  publishes a digest-pinned image, launches EKS and verifies S3 receipts. Read
  [submission details](references/submission.md). Confirm the selected workflow
  revision includes commit-image support before calling it.
- **AWS identity, callers anywhere:** validate an `isolab.eks.request/v1` document
  and run `taskq/deploy/eks-karpenter/remote/launch-remote request.json`. The
  CodeBuild launcher supplies the private EKS network route.
- **Resumable native CPU computation:** use the native AWS-only recovery client
  with `isolab.eks.resumable/v1`, a digest-pinned recovery image and an original
  absolute deadline. Worker-loss recovery and a separate L2/S3 epoch have prior
  c8a.2xlarge evidence; the latest periodic test recovered computation but failed
  its fresh L2 epoch. Read the exact runtime qualification and recovery reference;
  this does not qualify arbitrary commands or cross-family restoration.
- **An installed EKS MCP connection:** start with `isolab_k8s_overview`, then
  `isolab_k8s_benchmark` for ecbench measurements or `isolab_k8s_run` for an
  authorized arbitrary command. Read the live tool schema first. Benchmark
  takes the inner `job` object, not the portable request envelope.

For a new source revision, use a verified image built from that commit. Prefer
the commit-building GitHub path when the reviewed workflow is available on
`main` and its OIDC trust matches the current repository. A pending merge or
repository transfer can block that route; use an already verified per-run image
through the AWS bridge when available, or report the missing build explicitly.
A caller provenance SHA does not change the executable in the default image.
Never replace the requested experiment with the deployed integration fixture.
Keep source SHA, workflow SHA, binary SHA-256 and OCI digest distinct.

For a Claude director/executor card, freeze the request, launcher argv,
non-secret environment, receipt directory and timeout before delegation. Follow
[the card instructions](references/submission.md#mechanical-task-cards).

## Before launch

1. Retain the user's workload and freeze the source commit, spec, seeds,
   resources, original deadline and acceptance criteria. Use public/known-answer
   fixtures for crypto validation. A smoke test cannot replace the requested run.
2. Check existing jobs and receipts for this request before creating a unique
   job name. Capture the request hash, GitHub/CodeBuild run ID, Job UID and Pod
   UID. A timeout after submission is ambiguous: look up the original request,
   not a newly named duplicate. A GitHub rerun is a new experiment, not resume.
3. Verify live capacity. Ordinary jobs use `cpu-spot`; native L2 jobs select
   `cpu-l2-spot` and its benchmark taint automatically. Both span x86 compute,
   general-purpose and memory families. The desired ordinary ceiling is 16 vCPUs /
   128 GiB; L2 is 8 vCPUs / 32 GiB (24 vCPUs / 160 GiB combined). Verify actual
   NodePool limits and the namespace quota; the MCP overview's compiled-in
   numbers are not a live readback. Follow
   [COST_CONTROLS.md](../../../taskq/deploy/eks-karpenter/COST_CONTROLS.md).
   Commit and push source, protocol and launch configuration before builds or
   trials; run `check-experiment-git` as documented there and retain its receipt.
   Record a finite dollar ceiling and charge all attempts, including failures.
   Stop new launches when the budget is exhausted or its remaining balance is
   unknown. Queue work instead of raising quotas, changing providers or reducing
   the frozen experiment. Do not request a quota increase or raise pool caps implicitly. Count default instance vCPUs even
   when SMT is disabled. `m8a.2xlarge` and the contract default `c8a.2xlarge`
   have verified no-SMT L2 evidence. GPU and ARM builds are separate work.
4. For resumable work, check the implemented recovery contract and a real
   restore receipt, using the recovery reference. If only the ordinary v1
   submission path is available, state the integration gap before launching
   requested resumable work; never silently submit a non-resumable substitute.
   Neither a Kubernetes retry nor an EBS snapshot alone establishes recovery.

## Admit and verify measurements

The ecbench path requires L2, at least five measured rounds, a warmup, a reference
and matching A/A control. It uses four CPUs, Guaranteed requests/limits,
whole-core NUMA placement and zero automatic measurement retries. Profile v6
uses scheduler-domain isolation on benchmark workers; ordinary multithreaded
workers retain load balancing. Benchmark workers retain mandatory host PSI but
disable per-cgroup pressure accounting, so per-container PSI is unavailable. Keep these
checks intact; do not lower isolation thresholds or selectively discard failed
records to make a run pass.

Wait using the harness's bounded status mechanism. Preserve failures, timeouts
and all measurement records. A successful Pod is not sufficient: verify the
launcher result, request/spec identity, actual task image digest, native binary
hash, ecbench audit and replay, per-execution L2 gate and A/A admission. Download
S3 outputs and verify every manifest payload; require the receipt's Job and Pod
UIDs to match. Follow pagination when listing objects.

Report the source, image, execution outcome, measured L2/replay counts, controls,
private S3 receipt/results locations and any remaining gap. Keep process-memory
checkpoints in private encrypted storage, never GitHub artifacts or Git. An
infrastructure pass is not an algorithm speedup. For performance claims, follow
the repository's measurement, independent replay and report-evidence rules.
