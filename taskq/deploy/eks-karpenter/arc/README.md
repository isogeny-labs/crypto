# GitHub Actions runners on isolab EKS

This installs the GitHub-supported Actions Runner Controller (ARC) chart version
`0.15.0` for `aburan28/crypto`. ARC listens for Actions jobs, starts one ephemeral
runner pod per job, and Karpenter supplies a Spot EC2 node when a pod cannot
fit. Empty runner nodes are consolidated after five minutes. The runner NodePools are
separate from the `cpu-spot` Iso Lab measurement pool.
Both deployment scripts install the cluster-wide `spot-only-policy.yaml`, a
ValidatingAdmissionPolicy that rejects creation or updates of Karpenter
NodePools unless they explicitly require only `spot` capacity. This guards
against an accidental On-Demand fallback when a new pool is added. It does
not control standalone EC2 instances or Auto Scaling groups outside EKS.

| Scale set / `runs-on` | Pod request | Pod memory limit | Maximum runners |
| --- | --- | --- | --- |
| `crypto-eks-linux` | 4 CPU, 24 GiB plus Docker sidecar | 28 GiB runner | 11 |
| `crypto-eks-highmem` | 16 CPU, 112 GiB plus Docker sidecar | 120 GiB runner | 1 |

The base NodePool admits x86-64 R-family instances with 8 or 16 vCPUs and caps
aggregate provisioning at 16 vCPUs and 128 GiB. An additional R-family pool
admits 16-vCPU nodes and caps at 16 vCPUs and 128 GiB. The pool names and the
`gha.dev/pool=on-demand` scheduling label are legacy; the Karpenter
`karpenter.sh/capacity-type=spot` requirement controls the actual EC2 capacity.
Both ARC scale sets have `minRunners: 0`. With no queued or running jobs, ARC
removes its ephemeral runner pods and Karpenter removes empty runner nodes after
five minutes. The managed system node group has a two-node floor for the ARC and
cluster controllers. NodePool limits are upper bounds, not spending limits or
guaranteed capacity; the regional Spot vCPU quota can prevent new nodes from
launching. Worker EC2, EBS and NAT traffic are billed. Linux x64 jobs are the
only migration candidates; ARM, macOS, Windows, GPU and hardware-isolated
ecbench jobs remain on their current runners.

## Required access and authentication

The deploying IAM role needs `eks:DescribeCluster` and Kubernetes cluster-admin
access on `isolab` while installing cluster-scoped resources. Remove broad
administration access after deployment if ongoing administration uses a narrower
role.

ARC needs a fine-grained GitHub PAT restricted to `aburan28/crypto` with
Repository permissions → Administration: Read and write. Save it in a local
0600 file and create a Kubernetes Secret named `arc-github-auth` in
`arc-runners` with the key `github_token`. Keep the token out of Git, shell
history and Helm values. A GitHub App with the permissions in GitHub's ARC
authentication guide can also be used; the Secret then needs the keys
`github_app_id`, `github_app_installation_id` and `github_app_private_key`.

## Deploy and verify

Use an authorized kubeconfig for `isolab` and an administrator network path to
its API endpoint. The cluster's public endpoint currently allows only the
original harness host IP, so other hosts need an approved endpoint allowlist
change or private connectivity.

```sh
export KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig
kubectl apply -f taskq/deploy/eks-karpenter/arc/namespaces.yaml
kubectl -n arc-runners create secret generic arc-github-auth \
  --from-file=github_token=/home/ec2-user/ops/eks-isolab/arc/github-token
taskq/deploy/eks-karpenter/arc/deploy.sh
```

The Secret creation step is done once; for rotation, update the Secret and
restart the listener/controller as directed by ARC. Do not copy a personal
`gh` OAuth token into the cluster. The script is idempotent and pins both Helm
charts to `0.15.0`.

The runner image is built from the official `2.338.0` release with the tools
used by ordinary crypto workflows, then pinned by its ECR digest in both scale
set value files. Build and update it using the
[runner image instructions](../../../../.github/docker/arc-runner/README.md).
Check [the current Actions runner release](https://github.com/actions/runner/releases)
regularly; GitHub rejected the older `2.333.1` image as `Outdated` during the
live deployment test.

After deployment, dispatch `eks-arc-smoke.yml` once per pool.
Confirm both jobs pass, `kubectl get nodeclaims` shows a `gha-on-demand` node
during a run, and GitHub's job log identifies the expected runner name. Check
that an empty node disappears after the consolidation interval. Linux x64 jobs
in the existing workflows have opt-in routing expressions; the fallback is the
original GitHub-hosted label. Add only validated workflow filenames to the
repository variable `CRYPTO_EKS_WORKFLOWS`, delimited by vertical bars, for
example `|spotlab.yml|rust-lint.yml|`. Use
`CRYPTO_EKS_HIGHMEM_WORKFLOWS=|some-large-job.yml|` for the high-memory pool.
The high-memory list takes precedence. Remove a filename to roll that workflow
back. The routing script is `route_workflows.py`; it prepares new direct x64
`runs-on` lines and is idempotent.

`first-wave-workflows.txt` records the 392 workflow filenames enabled in the
initial rollout. It excludes workloads requiring specialized GPU, Sage, Lean,
perf, release or measurement hosts; the 38 deferred filenames are recorded in
`deferred-workflows.txt` for individual validation. To restore the rollout
variable after an accidental reset:

```sh
python3 -c 'from pathlib import Path; names=Path("taskq/deploy/eks-karpenter/arc/first-wave-workflows.txt").read_text().splitlines(); print("|"+"|".join(names)+"|", end="")' \
  | gh variable set CRYPTO_EKS_WORKFLOWS --repo aburan28/crypto
```

The routing expression admits `push`, `schedule`, `release`, manual dispatch and
same-repository `pull_request` events. Fork PRs, `pull_request_target`,
`workflow_run` and other events retain their original GitHub-hosted labels.
This routing rule does not prevent a fork from editing a workflow file to name
the self-hosted scale set directly.

Existing queued runs keep the workflow revision they started with. They will
not migrate automatically. For runs whose revision already has the routing
expression, cancel and rerun the same run after updating the repository
variable, then verify its new job labels. An older revision cannot gain the
expression through rerun, so a fresh event is necessary. Check for duplicate
experiments before replaying.

## Security and compatibility

This repository is public. A fork can propose workflow changes that target a
self-hosted scale set directly, regardless of the routing expression in the
base workflow. ARC's Docker sidecar is privileged. Dedicated tainted nodes,
ephemeral runner pods, a service account without Kubernetes API permissions,
and IMDS hop limit 1 reduce exposure; they do not make untrusted fork PRs safe
on a cluster that also runs Iso Lab work. Do not enable untrusted PR workflows
on this scale set without stronger isolation, such as a separate cluster or
organization runner-group controls.

The repo-specific image adds Node, Rust, C/C++, Python and common build
libraries missing from the stock ARC image. The workflow parser, Rust/Clippy
pilot and both pool smoke tests passed on EKS. The image still does not match
every GitHub-hosted tool; Sage, perf, nested virtualization, macOS, ARM and GPU
assumptions require their own runners or workflow changes. Spot interruption can
fail a running Actions job, which then needs a new run.

Sources: [GitHub ARC deployment](https://docs.github.com/en/actions/how-tos/manage-runners/use-actions-runner-controller/deploy-runner-scale-sets),
[ARC authentication](https://docs.github.com/en/actions/how-tos/manage-runners/use-actions-runner-controller/authenticate-to-the-api),
[ARC routing](https://docs.github.com/en/actions/how-tos/manage-runners/use-actions-runner-controller/use-arc-in-a-workflow),
[GitHub self-hosted runner security](https://docs.github.com/en/actions/reference/security/secure-use),
[Karpenter NodePools](https://karpenter.sh/docs/concepts/nodepools/).
