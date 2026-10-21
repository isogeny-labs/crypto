---
name: isolab-eks
description: Submit crypto experiments to Iso Lab on EKS Spot workers from Claude, Codex, another harness, or GitHub Actions; verify the tested commit, earned L2 isolation, private S3 results, and checkpoint/resume readiness. Use for Kubernetes lab execution, not the separate legacy NATS Iso Lab service.
---

# Iso Lab on EKS

Use the repository's maintained harness contract in
[`taskq/deploy/eks-karpenter/HARNESS.md`](../../../taskq/deploy/eks-karpenter/HARNESS.md).
This skill is shared with Claude through `.claude/skills/isolab-eks`.
Work from the repository root. It does not grant AWS or GitHub access; use the
caller's configured identity without copying credentials into requests or Git.

The lab's recovery policy is **CRIU plus EBS volume snapshots**. Preserve
computation progress after a Spot interruption, invalidate the interrupted
timing interval, and start a new benchmark epoch with fresh warmup, L2 checks
and A/A controls. Read [recovery requirements](references/recovery.md) before
promising resumability: the ordinary v1 submission paths below do not yet
automate this policy. A successful manual recovery test is not that integration.

## Select the submission path

- **GitHub identity, callers anywhere:** dispatch `isolab-experiment.yml` on
  `main` with an explicit 40-character `source_revision`. It builds that commit,
  publishes a digest-pinned image, launches EKS and verifies S3 receipts. Read
  [submission details](references/submission.md). Confirm the selected workflow
  revision includes commit-image support before calling it.
- **AWS identity, callers anywhere:** validate an `isolab.eks.request/v1` document
  and run `taskq/deploy/eks-karpenter/remote/launch-remote request.json`. The
  CodeBuild launcher supplies the private EKS network route.
- **An installed EKS MCP connection:** start with `isolab_k8s_overview`, then
  `isolab_k8s_benchmark` for ecbench measurements or `isolab_k8s_run` for an
  authorized arbitrary command. Read the live tool schema first. Benchmark
  takes the inner `job` object, not the portable request envelope.

Prefer the commit-building GitHub path for new code when no per-run image has
already been built and verified. A caller provenance SHA on its own does not
change the executable in the default image. Keep the tested source SHA,
workflow SHA, binary SHA-256 and OCI digest distinct.

## Before launch

1. Retain the user's workload and freeze the source commit, spec, seeds,
   resources, original deadline and acceptance criteria. Use public/known-answer
   fixtures for crypto validation. A smoke test cannot replace the requested run.
2. Check existing jobs and receipts for this request before creating a unique
   job name. Capture the request hash, GitHub/CodeBuild run ID, Job UID and Pod
   UID. A timeout after submission is ambiguous: look up the original request,
   not a newly named duplicate. A GitHub rerun is a new experiment, not resume.
3. Verify live capacity. The deployed x86 CPU pool spans compute, general-purpose
   and memory families; instance availability and CPU compatibility still
   matter. The worker cap is 64 vCPUs and the account's Standard Spot quota is
   currently 96 in `us-east-2`; other consumers share it. Do not request a quota
   increase or raise pool caps implicitly. Count default instance vCPUs even
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
whole-core NUMA placement and zero automatic measurement retries. Keep these
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
