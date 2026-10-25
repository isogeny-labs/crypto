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

Use the executor's configured AWS identity. Run `aws sts get-caller-identity`
and confirm the intended account before submission; no credentials belong in
the request, card, logs or repository. Configure SSO only when this machine
actually needs a login. The existing host role and an already authorized caller
such as `coreuser` do not need a new service-account key.

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

For an authorized run, execute these commands instead of returning them as a
handoff to the user. Use a unique receipt directory and retain the original
request. Run the bounded launcher in the background for long experiments and
keep its process/build identity. A timeout or nonzero exit must be reconciled
against that original build before any resubmission. Do not add automatic
submission retries or change the job name to get a passing measurement.

## Mechanical task cards

The main session selects the backend and freezes the experiment before the
director compiles a card. For the AWS bridge, bind the request, frozen spec and
protocol, `remote/launch-remote` and its `remote/validate-request` dependency by
SHA-256. Use repository-relative paths for binds, outputs and write scopes.
The request's `job.build` pins the tested image/source/binary; the card's
`commit` pins the checkout running the client. These can be different commits.

The card runs `remote/validate-request REQUEST` followed by
`remote/launch-remote REQUEST`, using the complete repository paths above.
Set the deployment variables above and a fresh repository-relative
`ISOLAB_CLIENT_OUTPUT` in each command's `env` mapping. Inherit AWS credentials
from the process; an explicitly selected profile name can be included, secret
values cannot. Set `must_not_exist` to the receipt directory, capture stdout and
stderr outside that directory, and require `result.json` as an output. Freeze a
client timeout that covers the launcher's documented wait plus submission and
receipt retrieval, separately from the experiment's execution deadline.

The executor runs the existing card tool only. It does not choose an instance,
build an image, invent a spec, switch backends or retry submission. A requested
resumable run instead binds the separate recovery client's request and commands;
ordinary request-v1 is not a recovery fallback. A missing required value is a
`specification_error` for the main session to resolve.

The director checks the task-card receipt and the downloaded launcher result:
request/build identity, successful admission, Job/Pod UIDs, native L2/replay and
A/A results, and S3 hash verification. A local zero exit or a submitted CodeBuild
ID alone is not a completed experiment. Preserve failed results for the reasoning
tier; do not reinterpret them as permission to rerun.

## MCP and command jobs

Use `isolab_k8s_benchmark` for the audited ecbench contract. For custom commands,
`isolab_k8s_run` accepts an argv array and digest-pinned image; it inserts no shell.
Custom images must support UID 1000. Outputs under `/work` are temporary unless
uploaded to `ISOLAB_RESULTS_URI` or protected by a validated recovery path.

Retain job identity, then use `isolab_k8s_status`, `isolab_k8s_logs` and
`isolab_k8s_results`. Inspect the returned receipts and objects; an S3 prefix
existing does not prove its data are complete, valid or from the selected pod.
