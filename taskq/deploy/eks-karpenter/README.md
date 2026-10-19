# Iso Lab on EKS with Karpenter

This deployment creates `isolab` in AWS account `104186811093`, `us-east-2`.
The harness submits Kubernetes Jobs; pending pods cause Karpenter to provision
Spot workers. Empty workers are removed after two minutes. This is independent
of the existing `taskq/deploy/aws` Redis/managed-worker stack.

## Resources

- EKS 1.36, private and public subnets in three AZs, one NAT gateway.
- Private Kubernetes endpoint plus a public endpoint restricted to
  `3.134.144.201/32`, the current harness host. Change this allowlist explicitly
  before connecting from another host; the kubeconfig alone does not grant
  network access.
- Two On-Demand `t3.medium` system nodes for DNS, CSI and Karpenter.
- Karpenter 1.15.0, two replicas on system nodes, scoped controller IAM,
  node access entry, SQS interruption queue and EventBridge interruption rules.
- `cpu-spot` NodePool: x86-64 Linux, Spot only, C/M/R generation 6+, instances
  with 4/8/16/32 vCPUs, aggregate limits of 64 vCPUs and 256 GiB RAM.
  No GPU or automatic On-Demand fallback is enabled.
- VPC CNI with network-policy support, CoreDNS, kube-proxy, metrics-server,
  EBS CSI, an encrypted `gp3` StorageClass and seven-day control-plane logs.
- Versioned, encrypted S3 results bucket and immutable ECR repository
  `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks`.
- Restricted `isolab` namespace, resource quota, runner IRSA role scoped to
  artifact prefixes, and job-submission RBAC for the harness.

The NodePool CPU limit excludes the four vCPUs of the two system nodes.
Karpenter's limit checks are eventually consistent and can briefly overshoot
during concurrent provisioning; this is not a hard spending limit. Namespace
quota separately caps admitted pod CPU requests and limits at 64. Spot capacity
can be unavailable or interrupted; jobs needing recovery must checkpoint to S3.

Estimated idle cost at 730 hours/month: about **$175/month** (EKS $73,
two t3.medium nodes $60.74, NAT $32.85, its IPv4 $3.65, system gp3 disks $4.80).
Spot workers, worker disks, stored artifacts/images, logs, requests and data
transfer/processing are additional. The single NAT is a cost choice and an
egress availability dependency. System nodes and EKS continue billing at idle.

## Install or resume

Requires AWS CLI v2, eksctl 0.231.0, kubectl 1.36, Helm 3.20.0, jq and ripgrep.
Use the host instance role or an AWS profile; never embed keys in manifests.
The scripts verify the account before provisioning. Infrastructure configuration
and IAM ARN values are intentionally specific to this deployment.

```sh
cd taskq/deploy/eks-karpenter
export KUBECONFIG="$PWD/runtime/kubeconfig"
./deploy.sh
./smoke.sh
```

`deploy.sh` creates the cluster if absent and installs/reconciles storage, workload
resources, Karpenter and its NodePool. An existing cluster must be ACTIVE; the
script does not reconcile changes to existing VPC, nodegroup or EKS configuration.
Review and apply those changes with eksctl/CloudFormation separately. Helm CRD
upgrades also require explicit review; do not merely change the chart version.

The upstream Karpenter CloudFormation template is vendored unmodified from
release v1.15.0; SHA-256:
`8ee49d6de4f2397797aea5c961706c90491a291b6a961268000cb2daadbd958f`.
The chart OCI digest is
`sha256:881ac708884b0aae218c696cadc62805739043dba7a6340e93059204c5933593`.
The worker AMI is pinned to `ami-082b570a48503ea93`, Amazon EKS AL2023 1.36
release `v20261012`, in us-east-2. Review and roll updates deliberately: worker
expiration and drift replacement are disabled to protect long running jobs.

## Submit from the harness

The thin shell launcher can be called by Codex, another harness, or a CI runner:

```sh
export KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig
./submit-job --image YOUR_IMAGE_DIGEST --cpu 6 --memory 12Gi --seconds 14400 \
  -- /usr/local/bin/ecbench run --spec /inputs/spec.json --out /work/results --cpus inherit
kubectl -n isolab get jobs,pods
kubectl -n isolab logs -f job/JOB_NAME
kubectl -n isolab delete job JOB_NAME    # cancel a run
```

The image must contain the executable and inputs. Use immutable image digests
for experiments and record the source revision and input hashes. The general
example above assumes you supply `/inputs/spec.json` inside your image or volume.
The accompanying `image.json` records a ready-to-use native ecbench image with
AWS CLI, a public synthetic planning fixture, the source revision and binary
hash. It is a small execution image, not a complete compiler/Sage/GPU toolchain.
Use `--dry-run` to render JSON, or `--wait` to wait for success/failure and print
logs. Without `--wait`, submission returns immediately. The supplied command is
an argument array, not interpolated into a shell.

Each job gets even integer CPU requests equal to limits, equal memory requests
and limits, whole-core static CPU allocation and pod anti-affinity to keep lab
jobs on separate nodes. DaemonSets still run on these nodes. Jobs use UID/GID
1000, a read-only root filesystem and writable `/work` and `/tmp`. Set images up
for those constraints. CPU values are 2..30; the largest node is 32 vCPUs and
system services reserve capacity. Memory requests must fit a permitted node.

`ISOLAB_OUTPUT_DIR=/work/results` is temporary storage; the command must create
it. `ISOLAB_RESULTS_URI` gives a unique S3 prefix containing the Job name and Pod
UID. The runner role can read `inputs/*` and `runs/*`, and write `runs/*`, but
cannot delete artifacts or administer AWS. Images that upload with `aws s3 cp`
must include AWS CLI; SDKs may use the injected web-identity credentials instead.
`smoke.sh` demonstrates the complete job → Spot node → IRSA → durable S3 path.

Validate a crypto plan and retain its artifacts using the supplied image:

```sh
IMAGE=$(jq -r .image_uri image.json)
./submit-job --image "$IMAGE" --cpu 2 --memory 1Gi --seconds 900 --wait -- /bin/sh -ec '
  mkdir -p "$ISOLAB_OUTPUT_DIR"
  ecbench plan --spec /opt/isolab/specs/smoke-prime.json --json > "$ISOLAB_OUTPUT_DIR/plan.json"
  cp /opt/isolab/build-info.json "$ISOLAB_OUTPUT_DIR/build-info.json"
  aws s3 cp "$ISOLAB_OUTPUT_DIR/" "$ISOLAB_RESULTS_URI" --recursive
'
```

`Dockerfile.ecbench` packages a prebuilt Linux amd64 binary. To rebuild, check
out the recorded source revision, run `cargo build --release --bin ecbench`, and
create an otherwise empty build context with `ecbench`, `smoke-prime.json` from
`docs/ecbench/specs/`, and `build-info.json` containing the source revision and
binary SHA-256. Build with `--build-arg SOURCE_REVISION=<revision>` and publish a
new immutable ECR tag; update `image.json` with the resulting digest. Do not
compile with `target-cpu=native` for this heterogeneous worker pool. The deployed
binary's Cargo fingerprint has no custom Rust flags.

No retries occur by default; `--retries N` permits up to N Kubernetes retries.
Every retry has a separate Pod UID/result prefix. The Job deadline includes time
waiting for capacity. Finished Jobs/logs are retained for one day, while S3
results remain until deliberately removed. Persist important logs/results before
the pod exits. Namespace Jobs quota includes finished Jobs until TTL cleanup.

## Iso Lab fidelity and integration boundary

Existing `isolab` uses NATS JetStream plus host-level workers. Its strict policy
requires mechanisms such as host cgroup partitions, IRQ placement, hardware
counters and bare-metal checks. Ordinary restricted Kubernetes pods on EC2 VMs
do not establish that tier. Static CPU allocation and one lab job per node do
not prove absence of host noise or hypervisor contention.

This deployment does not migrate the existing NATS hub or change `isolab_run`
into a Kubernetes backend. Native tools such as ecbench can run as pod commands,
with their recorded fidelity kept honest. A future native Iso Lab adapter must
preserve leases, fencing, result commits, retry semantics and fidelity checks;
privileged host workers need a separate reviewed design. Existing deployments
and running experiments are unchanged.

For an in-cluster harness, use the `harness` ServiceAccount and enable a projected
Kubernetes API token on that harness pod explicitly. External AWS roles can be
given an EKS access entry mapped to `isolab:submitters`. This group can submit,
inspect and cancel Jobs and read pod logs only in `isolab`. The current host
instance role is the cluster creator/admin; no static Kubernetes bearer tokens
were generated. These are trusted same-lab workloads, not mutually hostile tenants.

## Operations and teardown

```sh
kubectl get nodepools,ec2nodeclasses,nodeclaims
kubectl -n kube-system logs -l app.kubernetes.io/name=karpenter --tail=100
kubectl -n isolab get events --sort-by=.lastTimestamp
```

For deliberate teardown, first stop new submissions and preserve outputs. Delete
the lab Jobs, then delete `nodepool/cpu-spot` and wait for **all** NodeClaims and
their EC2 instances to terminate while Karpenter still runs. Only then uninstall
Karpenter, delete the cluster with eksctl, and delete `Karpenter-isolab`.
Do not remove the controller first: its finalizers clean up launched instances.
The `isolab-storage` stack's bucket and ECR repository use Retain, so their data
and storage charges survive stack deletion. No teardown runs automatically.

References: [Karpenter installation](https://karpenter.sh/docs/getting-started/getting-started-with-karpenter/),
[version compatibility](https://karpenter.sh/docs/upgrading/compatibility/),
[NodePool limits](https://karpenter.sh/docs/concepts/nodepools/),
[EKS pricing](https://aws.amazon.com/eks/pricing/),
[VPC pricing](https://aws.amazon.com/vpc/pricing/).
