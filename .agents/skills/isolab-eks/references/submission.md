# Portable submission

For first-time setup, read [agent access and authentication](../../../../taskq/deploy/eks-karpenter/ACCESS.md).
It includes Claude/Codex/OpenCode configuration, the current host's preflight,
remote caller grants and the distinction between Kubernetes tokens and AWS roles.

The maintained schemas and executable validators are under
[`taskq/deploy/eks-karpenter/contracts`](../../../../taskq/deploy/eks-karpenter/contracts)
and [`remote`](../../../../taskq/deploy/eks-karpenter/remote). Use those definitions;
do not invent request fields from this example.

## GitHub Action

Resolve the desired commit before dispatch. The spec must exist at that commit.
Select an explicit repository when running from a different checkout:

```bash
revision=$(git rev-parse HEAD)
gh workflow run isolab-experiment.yml --repo aburan28/crypto --ref main \
  -f source_revision="$revision" \
  -f spec_path=taskq/deploy/eks-karpenter/l2-validation.json \
  -f instance_type=m8a.2xlarge -f seconds=900
```

The default spec is an integration fixture. Replace it with the frozen requested
workload for a real experiment. Record the dispatch time and matching run ID.
The build runs without AWS identity; the trusted main-branch publisher uses OIDC.
A feature-branch dispatch skips AWS publication and the experiment. Do not loosen
OIDC trust to run it before merge. PR contract checks do not run measurements.

A failed or ambiguous dispatch must be reconciled against existing workflow runs
before another dispatch. Check `gh-RUN_ID-RUN_ATTEMPT` in EKS/launcher receipts.
The current Action's original duration bound includes Kubernetes provisioning;
build/publisher setup is accounted separately. Preserve both in the report.

## AWS launcher

Prepare a private JSON file with schema `isolab.eks.request/v1`, `job.job_name`,
`job.spec`, instance/resources/deadline and caller provenance. For a new executable,
include the validator-supported `job.build` object with the configured ECR image
URI by digest, exact source SHA and binary SHA-256. Bind
`provenance.source_revision` to that same source SHA.

```bash
export AWS_REGION=us-east-2
export ISOLAB_BUCKET=isolab-results-104186811093-us-east-2
export ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher
export ISOLAB_CLIENT_OUTPUT=/absolute/path/to/unique-request-receipts
taskq/deploy/eks-karpenter/remote/validate-request request.json
taskq/deploy/eks-karpenter/remote/launch-remote request.json
```

These deployment names identify the current lab; confirm access and readiness
instead of assuming every environment is authorized. The caller needs scoped S3
and CodeBuild permissions. Do not use permanent AWS keys in GitHub workflows.

## MCP and command jobs

Use `isolab_k8s_benchmark` for the audited ecbench contract. For custom commands,
`isolab_k8s_run` accepts an argv array and digest-pinned image; it inserts no shell.
Custom images must support UID 1000. Outputs under `/work` are temporary unless
uploaded to `ISOLAB_RESULTS_URI` or protected by a validated recovery path.

Retain job identity, then use `isolab_k8s_status`, `isolab_k8s_logs` and
`isolab_k8s_results`. Inspect the returned receipts and objects; an S3 prefix
existing does not prove its data are complete, valid or from the selected pod.
