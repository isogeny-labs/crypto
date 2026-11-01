# GitHub Actions runners on isolab EKS

This installs the GitHub-supported Actions Runner Controller (ARC) chart version
`0.15.0` for `isogeny-labs/crypto`. ARC listens for Actions jobs, starts one ephemeral
runner pod per job, and Karpenter supplies a Spot EC2 node when a pod cannot
fit. Empty runner nodes are consolidated after five minutes. The runner NodePools are
separate from the `cpu-spot` Iso Lab measurement pool.

| Scale set / `runs-on` | Pod request | Pod memory limit | Maximum runners |
| --- | --- | --- | --- |
| `crypto-eks-linux` | 2.8 CPU, 24 GiB plus Docker sidecar | 28 GiB runner | 7 |
| `crypto-eks-highmem` | 14 CPU, 112 GiB plus Docker sidecar | 120 GiB runner | 1 |
| `crypto-eks-xmem` | 14 CPU, 440 GiB plus Docker sidecar | 480 GiB runner (x2iedn/x2iezn.4xlarge, 512 GiB) | 1 |
| `crypto-eks-xmem1t` | 30 CPU, 900 GiB plus Docker sidecar | 960 GiB runner (x2iedn/x2iezn.8xlarge, 1 TiB) | 1 |
| `crypto-isolab-build` | 1.5 CPU, 12 GiB | 16 GiB runner | 1 |
| `crypto-isolab` | Iso Lab publisher with Docker sidecar | See `isolab-values.yaml` | 1 |

The base NodePool admits x86-64 M/R instances with 8 or 16 vCPUs and caps
aggregate provisioning at 16 vCPUs and 128 GiB. An additional R-family pool
admits 16-vCPU nodes and caps at 16 vCPUs and 128 GiB. A separate
`gha-gates` R-family Spot pool reserves one 16-vCPU / 128-GiB node for
the high-memory gate runner through `gha.dev/pool=gates`. The pool names and the
`gha.dev/pool=on-demand` scheduling label are legacy; the Karpenter
`karpenter.sh/capacity-type=spot` requirement controls the actual EC2 capacity.
All ARC scale sets have `minRunners: 0`. With no queued or running jobs, ARC
removes its ephemeral runner pods and Karpenter removes empty runner nodes after
five minutes. The managed system node group has a two-node floor for the ARC and
cluster controllers. NodePool limits are upper bounds, not spending limits or
guaranteed capacity; the regional Spot vCPU quota can prevent new nodes from
launching. Worker EC2, EBS and NAT traffic are billed. Linux x64 jobs are the
only migration candidates; ARM, macOS, Windows, GPU and hardware-isolated
ecbench jobs remain on their current runners.

The `gha-xmem` NodePool (added 2026-10-24) is for single processes larger than
the 120 GiB highmem pod: msolve on the PKM N=21 system needs at least 256 GB in
one process and the dense degree-5 Weil-descent closure about 768 GB. It admits
only Spot x2iedn and x2iezn nodes with 16 or 32 vCPUs and carries its own
`gha.dev/pool=xmem` taint, so only the two xmem scale sets land there. Its limit
is 32 vCPUs and 1100 GiB, which is one 1 TiB node or two 512 GiB nodes. The
runner NodePool caps then total 80 vCPUs (16 + 16 + 16 + 32). These are upper
bounds, not a concurrent allocation: the account also runs the harness and Iso
Lab worker pools under a 96-vCPU regional Spot quota. Karpenter leaves a
runner pending when quota or capacity is unavailable.
The two scale sets differ only in their request, so the request decides the
node size. Spot prices for these families were not checked when the pool was
written. Read them before dispatching a long job. Every runner pod template sets
`terminationGracePeriodSeconds: 110`. Within the two-minute Spot notice this
gives a job's SIGTERM trap time to push its finished records. It does not save
process memory.

## Required access and authentication

The deploying IAM role needs `eks:DescribeCluster` and Kubernetes cluster-admin
access on `isolab`. The EKS access entry for this session's role,
`arn:aws:iam::104186811093:role/spot-experiments-WorkerRole-G1FZ84yk3gS6`,
has the cluster-scoped `AmazonEKSClusterAdminPolicy`. Remove this broad access
after deployment if ongoing administration uses a narrower role.

ARC needs a fine-grained GitHub PAT restricted to `isogeny-labs/crypto` with
Repository permissions → Administration: Read and write. Save it in a local
0600 file and create a Kubernetes Secret named `arc-github-auth` in
`arc-runners` with the key `github_token`. Keep the token out of Git, shell
history and Helm values. A GitHub App with the permissions in GitHub's ARC
authentication guide can also be used; the Secret then needs the keys
`github_app_id`, `github_app_installation_id` and `github_app_private_key`.

### Replacing the GitHub credential

If the repository moves or the token expires, the listeners log a 404 or 401 on
`.../actions/runners/registration-token` and no runner is ever announced.
`rotate-github-token.sh` tests a new token the way ARC uses it, updates
`arc-github-auth`, restarts the controller and removes the temporary
cluster-admin it attached to the caller:

```sh
ARC_GITHUB_REPO=isogeny-labs/crypto taskq/deploy/eks-karpenter/arc/rotate-github-token.sh
```

The token needs the repository owner as its resource owner and Administration
read and write on the repository.

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

After deployment, dispatch `eks-arc-smoke.yml` once per pool (the xmem scale sets are
checked by their first real job, since a smoke run there costs an x2 node-hour).
Confirm both jobs pass, `kubectl get nodeclaims` shows a `gha-on-demand` node
during a run, and GitHub's job log identifies the expected runner name. Check
that an empty node disappears after the consolidation interval.

**Routing (owner decision, 2026-10-26).** Every Linux job's `runs-on` falls
back to GitHub-hosted `ubuntu-latest` and opts into these Spot scale sets per
workflow. Add only validated workflow filenames to the repository variable
`CRYPTO_EKS_WORKFLOWS` (Spot `crypto-eks-linux`), delimited by vertical bars,
for example `|spotlab.yml|rust-lint.yml|`. Use
`CRYPTO_EKS_HIGHMEM_WORKFLOWS=|some-large-job.yml|` for the Spot high-memory
pool; it takes precedence. Remove a filename to roll that workflow back to
`ubuntu-latest`. `route_workflows.py --apply` writes the routing expression
into any new direct `runs-on: ubuntu-latest` line and is idempotent; it skips
workflows whose bytes are pinned elsewhere (`SEALED`). Five workflows name a
scale set directly because they cannot run on a hosted runner:
`eks-arc-smoke.yml`, `dreg-a3-eks-highmem.yml`, `fb-designs-m4-highmem.yml`
and the `segment` job of `dreg-ckpt-job.yml` (xmem/xmem1t), plus the
dispatch-only `icb-aimed-2lp-compile.yml` pre-data compile gate. The latter
requires an exact expected commit and has no target-data, provider, or write
path.

### Turning the Spot runners back on (isogeny-labs/crypto)

The scale sets registered against `aburan28/crypto` before the repository
moved; their last smoke run passed on 2026-10-20. To bring them up for
`isogeny-labs/crypto`:

1. From a host with the isolab kubeconfig, rotate the ARC credential to a token
   (or GitHub App) owned by `isogeny-labs` with Administration read/write on
   the repository:
   `ARC_GITHUB_REPO=isogeny-labs/crypto taskq/deploy/eks-karpenter/arc/rotate-github-token.sh`.
   The value files already name `https://github.com/isogeny-labs/crypto`.
2. Re-run `deploy.sh` (idempotent) and confirm
   `kubectl -n arc-runners get autoscalingrunnersets` lists all six sets,
   including `crypto-isolab-build` and `crypto-isolab` for the Iso Lab workflow.
3. Dispatch `eks-arc-smoke.yml` with `pool=linux` and `pool=highmem`; confirm
   the job log names an `arc-runners` pod and `kubectl get nodeclaims` shows a
   Spot node during the run.
4. Restore the first-wave rollout (command below) and, for the high-memory
   jobs, `gh variable set CRYPTO_EKS_HIGHMEM_WORKFLOWS --repo isogeny-labs/crypto --body '|<file>.yml|'`.
5. If `vars.DREG_CKPT_WATCH` was unset while the runners were down, set it
   again only after the xmem pool has run one segment.

Self-hosted runner minutes are not billed by GitHub, so routed jobs do not
draw on the organization's Actions quota; the cost is the Spot EC2, EBS and NAT
traffic described above, inside the 96-vCPU regional Spot quota.

`first-wave-workflows.txt` records the 391 workflow filenames enabled in the
initial rollout. It excludes workloads requiring specialized GPU, Sage, Lean,
perf, release or measurement hosts, or a browser (`ic-dashboard.yml`, which
needs Chrome); the 39 deferred filenames are recorded in
`deferred-workflows.txt` for individual validation. To restore the rollout
variable after an accidental reset:

```sh
python3 -c 'from pathlib import Path; names=Path("taskq/deploy/eks-karpenter/arc/first-wave-workflows.txt").read_text().splitlines(); print("|"+"|".join(names)+"|", end="")' \
  | gh variable set CRYPTO_EKS_WORKFLOWS --repo isogeny-labs/crypto
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

`isogeny-labs/crypto` is private with forking disabled (2026-10-26), so only
collaborators can open pull requests. If it is made public again: a fork can propose workflow changes that target a
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
