# GitHub Actions runners on isolab EKS

This installs the GitHub-supported Actions Runner Controller (ARC) chart version
`0.15.0` for `aburan28/crypto`. ARC listens for Actions jobs, starts one ephemeral
runner pod per job, and Karpenter supplies a Spot EC2 node when a pod cannot
fit. Empty runner nodes are consolidated after five minutes. The runner NodePools are
separate from the `cpu-spot` Iso Lab measurement pool.

| Scale set / `runs-on` | Pod request | Pod memory limit | Maximum runners |
| --- | --- | --- | --- |
| `crypto-eks-linux` | 2.8 CPU, 24 GiB plus Docker sidecar | 28 GiB runner | 8 |
| `crypto-eks-highmem` | 14 CPU, 112 GiB plus Docker sidecar | 120 GiB runner | 1 |

The two general CI pools each cap provisioning at 16 vCPUs and 128 GiB,
using x86-64 R instances (8/16 vCPUs in the base pool, 16 in the extra pool).
The `gha-gates` pool reserves another 16 vCPUs and 128 GiB for the high-memory
queue through `gha.dev/pool=gates`. Its runner requests 14 CPUs plus the
0.5-CPU Docker sidecar, fitting a 16-vCPU R node after node reservations.
The general pool names and `gha.dev/pool=on-demand` label are legacy;
all three pools require `karpenter.sh/capacity-type=spot`.

Together these CI pools cap at 48 vCPUs. With the current 32-vCPU harness host,
4-vCPU system group, and one 8-vCPU benchmark worker, the planned allocation
is 92 of the account's 96 Standard Spot vCPUs in us-east-2. Recalculate this
budget when other Spot consumers change. The benchmark pool retains its own
64-vCPU cap; account-wide capacity still limits concurrent launches.

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
access on `isolab`. The EKS access entry for this session's role,
`arn:aws:iam::104186811093:role/spot-experiments-WorkerRole-G1FZ84yk3gS6`,
has the cluster-scoped `AmazonEKSClusterAdminPolicy`. Remove this broad access
after deployment if ongoing administration uses a narrower role.

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

## Iso Lab build queue

`isolab-build-values.yaml` defines `crypto-isolab-build` for contract checks and
source compilation, without a Docker daemon or privileged sidecar.
`isolab-values.yaml` defines `crypto-isolab` for publishing. Each is a one-job ARC queue on
the existing runner nodes. Its runner requests 1.5 CPUs and 12 GiB (limits
2 CPUs/16 GiB); the publisher Docker sidecar uses the namespace defaults. This can use
spare CPU on the memory-rich runner nodes without joining the general CI
backlog. `minRunners: 0` allows it to idle at zero. It does not reserve EC2
capacity or change the benchmark pool's 64-vCPU cap.

Install this scale set separately, without resetting other live runner settings:

```sh
helm upgrade --install crypto-isolab-build \
  oci://ghcr.io/actions/actions-runner-controller-charts/gha-runner-scale-set \
  --version 0.15.0 --namespace arc-runners --atomic --wait --timeout 10m \
  -f taskq/deploy/eks-karpenter/arc/isolab-build-values.yaml
helm upgrade --install crypto-isolab \
  oci://ghcr.io/actions/actions-runner-controller-charts/gha-runner-scale-set \
  --version 0.15.0 --namespace arc-runners --atomic --wait --timeout 10m \
  -f taskq/deploy/eks-karpenter/arc/isolab-values.yaml
gh variable set ISOLAB_BUILD_RUNNER --repo aburan28/crypto --body crypto-isolab-build
gh variable set ISOLAB_ACTIONS_RUNNER --repo aburan28/crypto --body crypto-isolab
```

The Iso Lab workflow uses these queues for manual runs and same-repository PRs;
fork PRs retain the hosted runner. Delete the variables to restore hosted
routing. Existing queued jobs retain their original routing. The builder has
no AWS OIDC permission; only the main-branch publishing job obtains AWS
credentials. Benchmark pods still use the dedicated `cpu-spot` nodes and
the same L2, replay, A/A and S3 verification gates.

A regional Spot quota can block both general CI and benchmark nodes even when
a NodePool is below its own cap. Account for other Spot hosts and the managed
system group before dispatching; a larger quota alone does not launch instances.

The ordinary Linux scale set requests 2.8 CPUs per runner (its 4-CPU limit
is unchanged). With the 0.5-CPU Docker sidecar, two runner pods request 6.6
CPUs and 56 GiB, fitting an 8-vCPU R node after kube/system reservations and
DaemonSets. The previous 4-CPU request allowed only one runner per node,
even when more than half of its RAM was free. The 8-runner limit is
recorded in `linux-values.yaml`. Existing running pods keep their original
resources; replacement pods adopt the new requests. This changes CI capacity
only, never the dedicated benchmark pod's Guaranteed resources.

## CI gates within the 96-vCPU quota

`gate-workflows.txt` records ordinary CI workflows routed to the separate
high-memory queue so research jobs cannot fill its only runner slot. Apply
these names to `CRYPTO_EKS_HIGHMEM_WORKFLOWS`, preserving any existing entries.
The existing routing expressions keep fork PRs and `pull_request_target` on
GitHub-hosted runners. CodeQL's ARM and language matrix jobs also retain their
existing runners. No checks, path filters, or review gates are disabled.

When migrating from the old M-family nodes, cordon them and let active jobs
finish. Lowering a NodePool limit does not remove its existing nodes. Verify
that the old nodes have become empty and terminated before counting their
vCPUs as available; do not drain active experiment jobs. Reduce the general
scale set to eight runners after its active count falls to eight or fewer.
The checked-in values describe the final allocation. All queues idle at zero
and all empty CI nodes consolidate after five minutes.
