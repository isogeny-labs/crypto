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
  with 8/16/32 vCPUs, excluding Flex families, aggregate limits of 64 vCPUs
  and 256 GiB RAM. CPU 0's whole physical core is reserved for housekeeping.
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

For the versioned request/result schemas, portable AWS launcher and GitHub
Actions trigger, see [the end-to-end harness contract](HARNESS.md).

### Native MCP connector

`mcp/` is a Rust stdio MCP server for this deployment. It uses the same Job
manifest as the shell launcher and exposes seven tools: `isolab_k8s_overview`,
`isolab_k8s_run`, `isolab_k8s_benchmark`, `isolab_k8s_status`, `isolab_k8s_logs`, `isolab_k8s_cancel`,
and `isolab_k8s_results`. Results lists S3 keys for each attempt; retrieve their
contents with `aws s3 cp`. The server never launches a workload on the harness
host. It requires Bash, GNU timeout, jq, kubectl and AWS CLI on PATH, a reachable
Kubernetes endpoint, and an authorized AWS identity. Building requires Rust.

From this deployment directory, install and register on the current host:

```sh
./install-mcp.sh /home/ec2-user/ops/eks-isolab/mcp
mkdir -p /home/ec2-user/ops/eks-isolab/tmp
codex mcp add isolab-eks \
  --env KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig \
  --env ISOLAB_KUBE_CONTEXT=i-01f85a6c905181498@isolab.us-east-2.eksctl.io \
  --env TMPDIR=/home/ec2-user/ops/eks-isolab/tmp \
  --env PATH=/home/ec2-user/.local/bin:/usr/local/bin:/usr/bin:/bin \
  -- /home/ec2-user/ops/eks-isolab/mcp/isolab-eks-mcp
```

If the tools do not appear after registration, start a new Codex session to load
the connection. This registers with the Codex installation on this host; another
harness needs its own MCP configuration and network/authentication access. See the
[official Codex MCP configuration documentation](https://developers.openai.com/codex/mcp).
For other stdio clients use the same executable and environment. No key is
stored in the MCP configuration. `ISOLAB_KUBE_CONTEXT` is required explicitly
so changing the kubeconfig's current context cannot redirect a submission.

Example `isolab_k8s_run` arguments (the default image is pinned in `image.json`):

```json
{
  "job_name": "my-ecbench-plan-001",
  "cpu": 2,
  "memory": "1Gi",
  "seconds": 900,
  "command": ["/bin/sh", "-ec", "mkdir -p \"$ISOLAB_OUTPUT_DIR\"; ecbench plan --spec /opt/isolab/specs/smoke-prime.json --json > \"$ISOLAB_OUTPUT_DIR/plan.json\"; aws s3 cp \"$ISOLAB_OUTPUT_DIR/\" \"$ISOLAB_RESULTS_URI\" --recursive"]
}
```

Run returns promptly; use status until the Job is terminal, then logs/results.
Use a unique Job name for each intended run. If a submission times out, inspect
that name before retrying: Kubernetes rejects duplicate names while the Job
exists, but names become reusable after deletion or TTL cleanup. The tool does
not retry submissions automatically. Commands are argv arrays; a shell only
runs when requested explicitly, as above. Custom images must use SHA-256 digests.
Cancel deletes the Job and its pods, including temporary data; uploaded S3
artifacts survive. Retries use separate pod result prefixes.

For diagnostics without an MCP client, set the same environment and run
`isolab-eks-mcp tools` or
`isolab-eks-mcp call isolab_k8s_overview '{}'`. Each external command has a
25-second deadline; status/logs can make two such calls. The MCP server handles
requests sequentially and does not offer a blocking wait or streaming logs.
The implementation follows the
[MCP stdio protocol](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports).

Connector validation: `cargo test --locked --manifest-path mcp/Cargo.toml` and
`cargo clippy --locked --manifest-path mcp/Cargo.toml --all-targets -- -D warnings`.
For a live check (creates short Spot Jobs), set the two connection variables
above plus `ISOLAB_MCP_BIN` to the installed executable, then run
`mcp/smoke.sh /absolute/path/to/evidence`. It verifies submission over stdio,
duplicate rejection, success/failure, logs, a downloaded S3 result hash, and
cancellation. It retains successful and intentionally failed Job records until
the normal one-day TTL; idle workers are removed by Karpenter.

### Shell launcher

The thin shell launcher can be called by Codex, another harness, or a CI runner:

```sh
export KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig
./submit-job --image YOUR_IMAGE_DIGEST --cpu 6 --memory 12Gi --seconds 14400 \
  -- /usr/local/bin/ecbench run --spec /inputs/spec.json --out /work/results --cpus auto
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
for those constraints. CPU values are 2..28; the largest node is 32 vCPUs and
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
waiting for capacity. Finished Job objects are retained for one day. Pods and
their logs can disappear when Karpenter removes the empty worker after two
minutes; S3 results remain until deliberately removed. Persist important
logs/results before the pod exits. Namespace Jobs quota includes finished Jobs
until TTL cleanup.

## Iso Lab fidelity and integration boundary

### L2 benchmark jobs

Use `isolab_k8s_benchmark` for crypto measurements:

```json
{"job_name":"crypto-l2-001","seconds":900,"instance_type":"c8i.2xlarge"}
```

For a worker with **no SMT/hyperthreading**, use the existing instance selector:

```json
{"job_name":"crypto-nosmt-001","seconds":900,"instance_type":"c8a.2xlarge"}
```

The following x86-64 sizes report eight physical cores and one thread per core
in the regional EC2 API. They fit the current pool and the existing amd64 image:

| Instance sizes | Physical cores / vCPUs | Memory |
| --- | --- | --- |
| `c7a.2xlarge`, `c8a.2xlarge` | 8 / 8 | 16 GiB |
| `m7a.2xlarge`, `m8a.2xlarge` | 8 / 8 | 32 GiB |
| `r7a.2xlarge`, `r8a.2xlarge` | 8 / 8 | 64 GiB |

All six sizes are offered in us-east-2a/b/c as checked on 2026-10-19; this does
not guarantee available Spot capacity. `c8a.4xlarge` supplies 16 physical cores
and 32 GiB when a larger job needs them. AWS also explicitly documents
[C8a's one-vCPU-per-physical-core design](https://aws.amazon.com/ec2/instance-types/c8a/).
An unpinned request still permits SMT instances; pin the type when no SMT is a
requirement. The private lab results retain the actual topology, L2 audit and
S3 checks; use the results tool to locate the desired Job's artifacts.

An EKS managed node group can instead use a custom EC2 launch template with
`CpuOptions.ThreadsPerCore: 1`. For example, a `c8i.4xlarge` template can set
`CpuOptions: {CoreCount: 8, ThreadsPerCore: 1}` to expose eight logical CPUs
instead of its default sixteen. This is an alternative for selecting an Intel
host without SMT; it is documented here, not provisioned. See
[EKS custom launch templates](https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html)
and [EC2 CPU options](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-specify-cpu-options.html).
The installed Karpenter CRD exposes only nested virtualization under cpuOptions,
not threadsPerCore; do not patch Karpenter-generated launch templates. A managed
group would need separate scaling and integration with the shared capacity
budget. For these Linux instances, disabling SMT does not lower the instance
price or EC2 vCPU quota accounting, per [AWS CPU-option pricing and limits](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html).

Omitting `spec` uses `l2-validation.json`: the registered public 20-bit fixture,
nine arms, two targets, one warmup and five measured rounds. Supply an ecbench
spec object to measure another workload. It must require L2, include a reference
and matching A/A control, and use at least five rounds and one warmup. The
default fixture validates isolation and correctness; it establishes no scaling
or algorithm speed claim. Larger workloads need a suitable timeout.
The optional `instance_type` pins hardware within the existing Spot pool;
omitting it lets Karpenter choose a permitted instance. There is no fallback
to another type when a specific type is requested and unavailable.

The [GitHub Actions entry point](HARNESS.md#github-actions) builds a new native
image from the requested commit and submits its immutable digest automatically.
The optional `build` object binds the image, source revision and executable
SHA-256; both the pod and the launcher verify that identity. Callers that omit
`build` retain the deployed default image.

The tool requests four exclusive vCPUs, selects bootstrap profile `v4`, and runs
`run-ecbench` with `--cpus auto`. One whole physical core measures the child;
the remaining allocated CPUs accommodate the parent shell and harness. `--cpus inherit` would
reserve the entire pod allocation and leave those processes nowhere to go.
The pod remains restricted and unprivileged. Each lab job has its own node.

Worker bootstrap confines OS services to CPU 0's physical core, reserves it with
Kubernetes' static/full-core/strict CPU manager, and routes movable IRQs and
unbound workqueues to housekeeping. This includes AL2023's separate
`runtime.slice` for kubelet/containerd. It discovers sibling topology at boot.
The kernel also boots with managed-IRQ isolation, full tickless
CPUs and offloaded RCU callbacks on workload CPUs. Fresh workers reboot once
before joining; a persistent service checks the kernel arguments and reapplies
runtime affinity before every kubelet start. Kubelet refuses to start if this
service fails. The AL2023 reserved-cgroup defaults
are cleared because they conflict with `reservedSystemCPUs`; pod resource
accounting and reservations remain enforced. DaemonSets share the remaining
unallocated CPUs. No measurement thresholds or native grader rules change.
Scheduler-domain load balancing remains enabled so ordinary multithreaded
jobs can use their allocated CPUs. Each measured ecbench child is separately
pinned by the native harness. Live validation and retained private failure
artifacts include 108/108 L2 records on the final v4 profile, complete S3 replay, and
4,405 passing library tests on a separate multithreaded job.

After a 60-second startup settle, the native quiet preflight must pass. Every
record, including warmups, must earn L2 or L3, and all measured executions must
replay identically. The tool also records an admitted A/A comparison. Any
below-L2 record fails the Job with exit 42; missing admitted controls fail with
43. All available raw records, audits, rejection reasons, exit status and a
SHA256SUMS manifest upload to the Pod UID's S3 prefix, including on failure.
There are no automatic retries. An interrupted node can still lose unuploaded
data; missing artifacts never count as a successful benchmark.

L2 is earned by the recorded execution, not guaranteed by a node label. Spot
VMs can experience host contention or interruptions; preserve and investigate
rejections instead of rerunning until green. Compare methods within a session
on the same host, retain A/A intervals, and do not pool wall times across
different instance types. This does not establish bare-metal L3 or the legacy
Iso Lab strict tier.

Configuration references: [Kubernetes CPU policies](https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/),
[reserved system CPUs](https://kubernetes.io/docs/tasks/administer-cluster/reserve-compute-resources/),
[AL2023 dynamic nodeadm configuration](https://docs.aws.amazon.com/eks/latest/userguide/al2023.html#al2023-nodeadm),
[Karpenter 1.15 user data](https://karpenter.sh/v1.15/concepts/nodeclasses/#specuserdata).
Kernel details: [Linux CPU isolation](https://www.kernel.org/doc/html/latest/admin-guide/cpu-isolation.html).

### Legacy worker boundary

Existing `isolab` uses NATS JetStream plus host-level workers. Its strict policy
requires mechanisms such as host cgroup partitions, IRQ placement, hardware
counters and bare-metal checks. Ordinary restricted Kubernetes pods on EC2 VMs
do not establish that tier. Static CPU allocation and one lab job per node do
not prove absence of host noise or hypervisor contention.

The native connector provides the `isolab_k8s_*` pod workflow. It does not
migrate the existing NATS hub or change the legacy `isolab_run` contract. Native
tools such as ecbench run as pod commands, with their recorded fidelity kept
honest. Compatibility with the NATS worker contract would additionally require
its leases, fencing, result commits, retry semantics and fidelity checks;
privileged host workers need a separate design. Existing deployments and
running experiments are unchanged.

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
