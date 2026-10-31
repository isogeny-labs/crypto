# Give Claude, Codex or OpenCode access to Iso Lab

Use the same submission contract from all three agents. Authentication belongs
to the machine or workflow running the agent; the skill does not grant access.
There is no shared service-account key to copy into a client.

| Where the agent runs | Submission path | Required identity |
| --- | --- | --- |
| Existing harness host | Installed stdio MCP or its CLI | Host's temporary AWS instance-role credentials |
| Laptop, another cloud or arbitrary CI | `remote/launch-remote` through CodeBuild | An explicitly authorized AWS profile/role |
| GitHub Actions | `isolab-experiment.yml` on `main` | Repository Actions write access; the workflow assumes its AWS OIDC role |
| Pod inside this cluster | Kubernetes Job API | `isolab/harness` ServiceAccount with a projected API token |

The deployment is account `104186811093`, region `us-east-2`, cluster `isolab`,
namespace `isolab`, bucket `isolab-results-104186811093-us-east-2` and CodeBuild
project `isolab-experiment-launcher`. These names are identifiers, not secrets.

## 1. Connect on the existing harness host

Run these commands from the repository root, where the agent's MCP process will
execute. AWS CLI, kubectl, jq, Bash and GNU timeout must be on its PATH.

```bash
export AWS_REGION=us-east-2
export ISOLAB_CLUSTER=isolab
export ISOLAB_BUCKET=isolab-results-104186811093-us-east-2
export KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig
export ISOLAB_KUBE_CONTEXT=i-01f85a6c905181498@isolab.us-east-2.eksctl.io
export ISOLAB_MCP_BIN=/home/ec2-user/ops/eks-isolab/mcp/isolab-eks-mcp
export TMPDIR=/home/ec2-user/ops/eks-isolab/tmp
mkdir -p "$TMPDIR"

aws sts get-caller-identity
kubectl --kubeconfig "$KUBECONFIG" --context "$ISOLAB_KUBE_CONTEXT" \
  --request-timeout=15s auth can-i create jobs.batch -n isolab
taskq/deploy/eks-karpenter/remote/check-ready
"$ISOLAB_MCP_BIN" call isolab_k8s_overview '{}'
```

Confirm the authorized lab identity, `yes` from `can-i`, and `ready: true` from
`check-ready`. The overview lists existing jobs; it is not a readiness probe.
The context is a persisted kubeconfig alias, not a dependency on the old EC2
instance ID. Zero worker nodes is normal: Karpenter starts a Spot worker for a
pending pod. Readiness does not guarantee capacity.

Choose your client below. Registration writes client settings; it does not grant
IAM permissions. Keep existing unrelated MCP entries.

### Codex

This host already has `isolab-eks` registered. Check with `codex mcp get
isolab-eks`; on a new installation register it once:

```bash
codex mcp add isolab-eks \
  --env KUBECONFIG="$KUBECONFIG" \
  --env ISOLAB_KUBE_CONTEXT="$ISOLAB_KUBE_CONTEXT" \
  --env AWS_REGION="$AWS_REGION" --env TMPDIR="$TMPDIR" \
  --env PATH=/home/ec2-user/.local/bin:/usr/local/bin:/usr/bin:/bin \
  -- "$ISOLAB_MCP_BIN"
```

Start a new session if the connection is not loaded. See the
[official Codex MCP configuration](https://developers.openai.com/codex/mcp/).

### Claude Code

```bash
claude mcp add --scope user \
  --env KUBECONFIG="$KUBECONFIG" \
  --env ISOLAB_KUBE_CONTEXT="$ISOLAB_KUBE_CONTEXT" \
  --env AWS_REGION="$AWS_REGION" --env TMPDIR="$TMPDIR" \
  --env PATH=/home/ec2-user/.local/bin:/usr/local/bin:/usr/bin:/bin \
  --transport stdio isolab-eks -- "$ISOLAB_MCP_BIN"
claude mcp get isolab-eks
```

Use `/mcp` inside Claude Code to check the connection. This uses
[Claude Code's user-scoped stdio configuration](https://code.claude.com/docs/en/mcp).

### OpenCode

Merge this entry into your OpenCode configuration, such as
`~/.config/opencode/opencode.json`; preserve the rest of that file.

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "isolab-eks": {
      "type": "local",
      "command": ["/home/ec2-user/ops/eks-isolab/mcp/isolab-eks-mcp"],
      "enabled": true,
      "environment": {
        "KUBECONFIG": "/home/ec2-user/ops/eks-isolab/kubeconfig",
        "ISOLAB_KUBE_CONTEXT": "i-01f85a6c905181498@isolab.us-east-2.eksctl.io",
        "AWS_REGION": "us-east-2",
        "TMPDIR": "/home/ec2-user/ops/eks-isolab/tmp",
        "PATH": "/home/ec2-user/.local/bin:/usr/local/bin:/usr/bin:/bin"
      }
    }
  }
}
```

This is OpenCode's [local MCP format](https://opencode.ai/docs/mcp-servers/).
Restart the client and ask it to call `isolab_k8s_overview` before creating a job.

### Load the shared instructions and submit

From a checkout containing this change, read
[`.agents/skills/isolab-eks/SKILL.md`](../../../.agents/skills/isolab-eks/SKILL.md).
Claude sees the same file through `.claude/skills/isolab-eks`.
[OpenCode discovers these shared locations too](https://opencode.ai/docs/skills/).
If discovery fails, explicitly ask the client to read that file.

Example instruction for any of the three agents:

> Read the isolab-eks skill. Check readiness and existing jobs, then submit my
> specified experiment to EKS Spot. Preserve the exact source, image and spec;
> require native L2, replay and A/A admission, and verify S3 payload hashes.
> Return the request, Job/Pod identities and results URI. If I request resume,
> use the CRIU/EBS recovery contract and a separate fresh measurement epoch.

For MCP, the benchmark input is the **inner job object**:

```json
{"job_name":"my-unique-crypto-check","instance_type":"c8a.2xlarge","seconds":900,"memory":"4Gi"}
```

Call `isolab_k8s_benchmark`, retain its Job UID, then use `isolab_k8s_status`,
`isolab_k8s_logs` and `isolab_k8s_results`. This minimal example runs the deployed
public validation fixture. For your code, supply the supported `spec` and
digest-pinned `build` fields from [HARNESS.md](HARNESS.md#the-request); a caller
commit label alone does not change the executable. Read the live tool schema.

## 2. Submit from another machine with AWS access

Use the CodeBuild bridge when you cannot reach the private Kubernetes API. It
requires no local kubectl or Kubernetes service-account token. Install AWS CLI,
Bash, jq, GNU coreutils and OpenSSL, and use a checkout with `remote/launch-remote`.

Obtain an authorized role/profile from the account administrator. If your
organization uses IAM Identity Center, configure its assigned account and role:

```bash
aws configure sso --profile isolab
aws sso login --profile isolab
export AWS_PROFILE=isolab
export AWS_REGION=us-east-2
aws sts get-caller-identity
```

The SSO start URL, SSO region and role assignment must come from your organization;
this deployment has not provisioned them. An existing temporary role/credential
provider works too. On the existing EC2 host, use its instance role and omit
these SSO/profile commands. See [AWS CLI SSO setup](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html).

The administrator grants the **caller role** these permissions. Authorizing
the caller to assume that role is a separate step. The existing GitHub trigger
role trusts GitHub `main` OIDC only; it cannot be reused as a laptop login.

| Actions | Resource |
| --- | --- |
| `s3:PutObject` | `arn:aws:s3:::isolab-results-104186811093-us-east-2/inputs/requests/*` |
| `codebuild:StartBuild`, `codebuild:BatchGetBuilds`, `codebuild:StopBuild` | `arn:aws:codebuild:us-east-2:104186811093:project/isolab-experiment-launcher` |
| `s3:GetObject` for launcher receipts | `arn:aws:s3:::isolab-results-104186811093-us-east-2/runs/*/launcher/*` |
| `s3:GetObject` for independent raw-result download | `arn:aws:s3:::isolab-results-104186811093-us-east-2/runs/*` |
| `s3:ListBucket` for raw-result listing, with `s3:prefix` restricted to `runs/*` | `arn:aws:s3:::isolab-results-104186811093-us-east-2` |

`StartBuild` permits project overrides, so this bridge is for trusted lab
operators. The caller does not need ECR push, cluster-admin, IAM administration
or an EKS access entry. The launcher already has its own EKS submitter identity.

From the repository root, prepare a unique request in a private output directory:

```bash
export AWS_REGION=us-east-2
export ISOLAB_BUCKET=isolab-results-104186811093-us-east-2
export ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher
isolab_job="access-check-$(date -u +%Y%m%d-%H%M%S)-$(openssl rand -hex 3)"
export ISOLAB_CLIENT_OUTPUT="$HOME/isolab-receipts/$isolab_job"
install -d -m 700 "$ISOLAB_CLIENT_OUTPUT"
jq -n --arg name "$isolab_job" --arg revision "$(git rev-parse HEAD)" \
  '{schema:"isolab.eks.request/v1",
    job:{job_name:$name,instance_type:"c8a.2xlarge",seconds:900,memory:"4Gi"},
    provenance:{harness:"agent-cli",source_revision:$revision}}' \
  > "$ISOLAB_CLIENT_OUTPUT/input.json"
taskq/deploy/eks-karpenter/remote/validate-request "$ISOLAB_CLIENT_OUTPUT/input.json"
taskq/deploy/eks-karpenter/remote/launch-remote "$ISOLAB_CLIENT_OUTPUT/input.json"
```

The last command creates a real paid Spot job and waits through the launcher's
bounded completion checks. In an agent harness, run it as a background task and
retain its output directory. This example uses the deployed integration fixture;
freeze the requested workload and add its `spec`/`build` for a real experiment.
On timeout, inspect `build-start.json`, `build.json` and the existing job before
retrying. A new job name or GitHub rerun starts a new experiment.

Require `result.json` with `state: succeeded`, matching request/build identity,
Job/Pod UIDs and a results URI. The launcher checks L2, native replay, A/A and
downloaded S3 hashes. Independently fetch raw artifacts with:

```bash
isolab_uri=$(jq -er '.results_uri' "$ISOLAB_CLIENT_OUTPUT/result.json")
mkdir -p "$ISOLAB_CLIENT_OUTPUT/artifacts"
aws s3 cp "$isolab_uri" "$ISOLAB_CLIENT_OUTPUT/artifacts/" --recursive
(cd "$ISOLAB_CLIENT_OUTPUT/artifacts" && sha256sum -c SHA256SUMS)
```

Keep the receipt and native audit/replay results. A successful Pod or existing
S3 prefix alone is insufficient; [HARNESS.md](HARNESS.md) defines failure states
and exact source/build checks.

## 3. Trigger the commit-building GitHub Action

Authenticate `gh` with repository Actions write access. No AWS credentials are
needed on the calling machine. The main-branch workflow uses OIDC and configured
repository variables to publish the image and start the launcher.

As of October 24, 2026, [PR #2342](https://github.com/aburan28/crypto/pull/2342)
is unmerged and its full post-merge Action verification is pending. Use this
command after the reviewed commit-building workflow is on `main`:

```bash
gh auth status
isolab_revision=$(git rev-parse HEAD)
gh workflow run isolab-experiment.yml --repo aburan28/crypto --ref main \
  -f source_revision="$isolab_revision" \
  -f spec_path=taskq/deploy/eks-karpenter/l2-validation.json \
  -f instance_type=c8a.2xlarge -f seconds=900
```

The exact commit must be available in `aburan28/crypto`, with the spec at that
commit. Capture the matching run ID and follow the checks in
[the submission reference](../../../.agents/skills/isolab-eks/references/submission.md).
A feature-branch dispatch skips publication and the experiment. Premerge L2
evidence does not replace this separate Action verification.

## 4. Run a harness inside Kubernetes

Configure the harness Pod with `metadata.namespace: isolab`,
`spec.serviceAccountName: harness` and `spec.automountServiceAccountToken: true`.
Use the Kubernetes client's in-cluster configuration. Kubernetes supplies a
projected API token; no long-lived token Secret is created. Existing RBAC permits
Job creation/read/delete and pod/log/event reads in `isolab`.

Experiment Pods use `runner`. Its IRSA role `isolab-job` provides scoped S3
artifact access. `harness` does **not** inherit that AWS role; a harness that
reads S3 directly needs its own assigned AWS identity.
[IRSA authenticates AWS calls](https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts.html);
Kubernetes RBAC controls Job submission.

For another VPC-connected host using direct MCP, an administrator must also map
its actual IAM role to the `isolab:submitters` EKS group, grant
`eks:DescribeCluster` on `isolab` and scoped S3 result reads, and provide an API
network route. [EKS access entries](https://docs.aws.amazon.com/eks/latest/userguide/access-entries.html)
do not create network connectivity. Generate that host's kubeconfig using
`aws eks update-kubeconfig --name isolab --region us-east-2 --alias isolab`, set
`ISOLAB_KUBE_CONTEXT=isolab`, and install the connector with `install-mcp.sh`.
When using a named AWS profile, pass its `AWS_PROFILE` and any custom
`AWS_CONFIG_FILE` path in the MCP environment as well; do not embed credential
values in client configuration.
Use the portable bridge instead if you only need remote submission.

## Diagnose access failures

| Symptom | Check |
| --- | --- |
| AWS credentials missing/expired | Run `aws sts get-caller-identity` in the agent's environment; renew that environment's SSO/role session. |
| AWS works but Kubernetes says `Forbidden` | Confirm the role's EKS group and namespace RBAC; kubeconfig alone grants nothing. |
| Kubernetes connection timeout | Use the approved network route or CodeBuild bridge. A token cannot fix routing. |
| `StartBuild` or S3 `AccessDenied` | Check the caller role against the project/bucket/prefix permissions above. |
| MCP binary not found | The example paths belong to the existing Linux host. Install the connector where the MCP process runs. |
| Pod pending | Check pool readiness and regional Spot capacity; do not create duplicate submissions. |

The lab remains capped at 64 vCPUs across ordinary and L2 pools, within a shared
96-vCPU regional Standard Spot quota. Node labels do not establish L2. For
resumable experiments, follow [the CRIU/EBS recovery instructions](../../../.agents/skills/isolab-eks/references/recovery.md):
preserve computation, invalidate interrupted timings and start one separate
fresh benchmark epoch. Ordinary request-v1 jobs are non-resumable.
