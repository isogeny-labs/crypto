# EKS CRIU and EBS snapshot recovery

The native `isolab.eks.resumable/v1` path automatically checkpoints computation
and restores it on a compatible replacement worker. The
[controller qualification](validation/20261021-controller/REPORT.md) exercised
controller restart, dedicated Spot-worker loss, resumed native crypto progress
and a distinct fresh L2 epoch with complete S3 verification. **Ordinary Iso Lab
v1 jobs remain non-resumable** and use temporary `/work`. Full main-branch GitHub
Action verification remains blocked on the PR merge/checks; the
[integration record](../../../../problems/eks-resumable-l2-experiments.md) tracks
that and the remaining qualification limits.

The current runtime pin adds the periodic-generation and exited-child capture
fixes. Its [latest qualification](validation/20261022-periodic/attempt-b/REPORT.md)
verified two committed generations and resumed computation after worker loss,
but **failed overall** because three fresh observations earned L1. The gate
rejected the epoch without retry and retained its S3 output. The earlier all-L2
qualification above is separate evidence, not a pass for this later run.

The selected policy is to resume computation, retain the interrupted epoch as
invalid for timing, and start a fresh benchmark epoch with new warmup, L2
observations and A/A controls. Snapshotting or restoring RAM does not preserve
a benchmark's admission grade.

## Installed snapshot infrastructure

The lab uses EBS CSI `gp3` volumes with encryption enabled and
`WaitForFirstConsumer` binding. `install-snapshotter` installs/checks the managed
`snapshot-controller` addon and applies `snapshot-class.yaml`. The tested addon
is `v8.6.0-eksbuild.10` on EKS 1.36. It checks the AWS account and kubeconfig
endpoint before changing Kubernetes objects. Existing unhealthy/in-progress
addons require inspection instead of a second create/update operation.

```bash
KUBECONFIG=/path/to/isolab-kubeconfig \
  taskq/deploy/eks-karpenter/resume/install-snapshotter
```

The `isolab-ebs-checkpoints` VolumeSnapshotClass uses the EBS CSI driver and
`Retain`. Deleting a VolumeSnapshot object does not delete its EBS checkpoint;
retention/cleanup must explicitly remove snapshots only after their generation
is no longer required. A controller and CRDs alone are not a recovery service.

## Tested operator procedure

This is a qualification procedure for trusted CPU processes with all mutable
files on one PVC and an immutable container image. It requires operator access
to a dedicated namespace; do not relax the existing `isolab` namespace's
Restricted PodSecurity. The probe used a privileged supervisor with private
PID/mount namespaces, no host network, no service-account token, and a payload
running as UID/GID 1000 with all capabilities dropped and `no_new_privs` set.
Its setup helper temporarily read the disposable worker's CRIU binary and
libraries; it was removed before restore. This is not a security boundary for
untrusted user code.

1. Freeze a protocol, original UTC deadline, image digest, executable hash,
   request/spec identity and compatible CPU/kernel/runtime class. Pin the
   instance type for restoration. The tested class is c8a.2xlarge, AL2023 kernel
   `6.18.54-121.166.amzn2023.x86_64`, containerd `2.3.5+unknown`, CRIU `4.2.1`.
   Count capacity against the existing 96-vCPU account quota and 64-vCPU lab cap.
2. Start the process in a private PID namespace with its mutable files and
   redirected stdout/stderr on encrypted EBS CSI storage. Keep checkpoint files
   readable only by the operator. Qualify actual workload descriptors, mmap
   backing files, IPC and CPU features; the fixture proves no general support
   for network sessions, GPU state, perf descriptors or arbitrary namespaces.
3. Check the recorded PID/start identity before acting. From the same namespace,
   dump the child and leave it stopped:

   ```bash
   LD_LIBRARY_PATH=/work/tools/lib /work/tools/criu dump \
     --tree "$payload_pid" --images-dir "$generation_dir" \
     --shell-job --leave-stopped --manage-cgroups=ignore \
     --cpu-cap=all --file-locks --log-file dump.log -v4
   ```

   `--manage-cgroups=ignore` retains Kubernetes control of destination cgroups.
   It does not validate placement; validate the replacement pod's resources and
   CPU affinity separately. Confirm the process is stopped; flush all files.
   Hash the checkpoint, executable/runtime dependencies and mutable output.
4. With every writer stopped, create a `VolumeSnapshot` of that PVC using
   `isolab-ebs-checkpoints`. Wait for `status.readyToUse=true`; verify the EBS
   snapshot is complete and encrypted. Store a private encrypted checkpoint
   archive and immutable generation manifest in S3. Download the archive again
   and compare its SHA-256 before committing the generation. A failed dump,
   empty upload, pending snapshot or mismatched readback is not recoverable.
   Keep a previous verified generation until the new one is committed.
5. For a periodic checkpoint, resume the *same* original process only after
   committing the generation. For migration, fence it. In the replacement test,
   the original pod was deleted and its own EC2 instance reached `terminated`
   **before** creating the replacement. Never use a missing heartbeat alone as
   proof that the old writer stopped. Do not terminate shared/other workers.
6. Create a new PVC with that VolumeSnapshot as `dataSource`, and a new pod on a
   compatible worker with the same image digest and original deadline. Check
   every file hash and compatibility field before restoration. Use the private
   namespace's CRIU, not containerd's disabled CRI restore-via-create path:

   ```bash
   LD_LIBRARY_PATH=/work/tools/lib /work/tools/criu restore \
     --images-dir "$generation_dir" --shell-job --restore-detached \
     --manage-cgroups=ignore --cpu-cap=all --file-locks \
     --pidfile /work/restored.pid --log-file /work/restore.log -v4
   ```

   Never fall back to running the command from zero under the resumed attempt.
   Confirm the original in-memory computation advances and verify its result.
7. Preserve its interrupted timing interval without admitting it as L2. Use a
   distinct fresh benchmark job and S3 prefix, bound to the same requested
   source/image/spec and the logical recovery receipt. Run the existing native
   audit/replay, all-record L2 gate and A/A admission unchanged. Keep old and
   new epochs separate; a cryptographic computation resuming successfully is
   not evidence about its uninterrupted wall time.

These manual steps explain the mechanism. The native controller below automates
it for the qualified workload class. Do not add unsupported `resume` fields to
`isolab.eks.request/v1`; use the separate resumable schema.

## Replay the public fixture

The native fixture starts its state at zero and never reads an output file to
initialize it. Its state transition is SHA-256(previous 32 bytes || LE64(index)).
Its pauses allow a checkpoint to be taken; this is not a speed benchmark.
The archived executed source and binary hashes identify the live probe. The
current verifier also requires the original output prefix and exact final target.

```bash
cargo test --release --manifest-path taskq/deploy/eks-karpenter/resume/fixture/Cargo.toml
cargo run --release --manifest-path taskq/deploy/eks-karpenter/resume/fixture/Cargo.toml -- \
  verify taskq/deploy/eks-karpenter/resume/validation/20261021/progress-after.log \
  1048576 taskq/deploy/eks-karpenter/resume/validation/20261021/progress-before.log
```

The verifier rejects corruption, restarted output, incomplete computation,
incorrect target counts, changed checkpoint prefixes and records after completion.
It independently recomputes every recorded chain value from the public seed;
the first step is checked against a separately generated OpenSSL reference.

## Runtime references

- [Kubernetes checkpoint API](https://kubernetes.io/docs/reference/node/kubelet-checkpoint-api/)
  creates a runtime-dependent checkpoint; it is not a recovery controller.
- [containerd restore advisory](https://github.com/containerd/containerd/security/advisories/GHSA-p7v4-vr35-mj6f)
  explains why `enable_experimental_restore_via_create` must remain disabled.
  Re-enabling it retains the destination-security-context vulnerability.
- [CRIU options](https://github.com/checkpoint-restore/criu/blob/v4.2.1/Documentation/criu.txt)
  describe CPU capability and cgroup handling.
- [EKS CSI snapshot controller](https://docs.aws.amazon.com/eks/latest/userguide/csi-snapshot-controller.html)
  documents the managed addon and CSI storage requirement.

## Native controller implementation

The separate [request schema](request.schema.json), native
[controller](controller/src/cloud.rs), and [trusted supervisor](controller/src/supervisor.rs)
implement the policy in [CONTROLLER.md](CONTROLLER.md). The ordinary v1 route is
unchanged. Automatic worker-loss test E preserved a 1,140-record checkpoint prefix
and completed 1,806 computations after restoration. Its fresh epoch passed 1,806
L2 observations, 1,800 native replays, 600 A/A pairs and 19 S3 hashes. The report
records the final-runtime retest and differentiates live tests from offline
failure-path tests.

The [real ecbench operator qualification](validation/20261021-ecbench/REPORT.md)
adds actual native parent/child restore, corrected empty supplementary groups,
30 independently reproduced records and a fresh 36-record L2 epoch. Its
[PDF](validation/20261021-ecbench/report.pdf) retains the unsupported 36-bit
request, perf-descriptor dump failure and first restore's group mismatch.

Build or install the trusted native client on a Linux x86-64 host with AWS identity:

```bash
cargo +1.99.1 build --locked --release --manifest-path \
  taskq/deploy/eks-karpenter/resume/controller/Cargo.toml
client=taskq/deploy/eks-karpenter/resume/controller/target/release/isolab-eks-recovery
"$client" validate /absolute/path/request.json
"$client" submit /absolute/path/request.json /absolute/path/receipts
"$client" remote-status /absolute/path/request.json /absolute/path/receipts
"$client" wait /absolute/path/request.json /absolute/path/receipts
```

For the approved release client, `fetch-client /absolute/install/path` verifies
the private content-addressed runtime package before installation. `publish-runtime
BASE-IMAGE.json OUTPUT.json` composes those trusted tools over the tested image.
It rejects image ONBUILD hooks and executes no tested code during publication.
The [request example](request.example.json) deliberately contains invalid
placeholders: replace the source/binary/image identities and set an absolute
deadline 180..86,400 seconds in the future before validation. Preserve those
bytes on retries. For MCP, install the recovery client on the server's PATH as
well as `isolab-eks-mcp`, restart the connection and inspect its live tool list.

The request binds a logical name, exact base image/source/binary, exact recovery
image, original L2 spec, c8a.2xlarge, checkpoint interval (30..900 seconds), maximum
restores (1..5), and the original absolute UTC deadline. Computation is UID/GID
1000 without capabilities or supplementary groups. Timing is forced to L0 and
cannot pass the ordinary L2 gate. It is replayed for correctness before exactly
one distinct fresh benchmark epoch. Supported workloads are trusted native CPU
experiments; arbitrary networking, GPUs, ARM and cross-family restore are not
qualified. The private privileged supervisor is not an untrusted-code boundary.

The controller owns a resourceVersion-guarded ConfigMap and requires AWS to
confirm a previous CodeBuild owner terminal before takeover. It requires the
exact previous EC2 worker to be **terminated** before restoring; a missing Node,
expired heartbeat or stopped instance is insufficient. It never terminates
workers itself. Original deadlines and immutable requests cannot change during
recovery. A permanent S3 request marker prevents silently restarting a deleted
logical run; a crash before its initial ConfigMap exists needs explicit operator
reconciliation, not automatic restart from zero.

Each committed generation combines a frozen process tree, flushed filesystem,
encrypted CSI EBS snapshot, private versioned S3 archive and complete hash
readback. The two most recent verified generations are eligible for automatic
rollback; older S3 evidence and Retain-class EBS snapshots remain for explicit
retention management. PVCs also remain until their evidence is collected; do not
mistake Kubernetes Pod completion for storage cleanup. Interrupted generations
and failed measurements are retained and cannot replace a good measurement.

Deploy `namespace.yaml`, `rbac.yaml`, and `bridge.yaml` with `deploy-controller`
from an authorized administrator identity. It packages native binaries and binds
the CodeBuild source to an S3 version. The role has no EC2 termination permission.
The project admits one controller at a time; the combined pool cap stays 64 vCPUs and the
account quota 96. The controller can restore only after a complete generation;
loss before the first committed checkpoint fails visibly instead of starting over.

Fresh benchmark epochs select the dedicated `cpu-l2-spot` v6 pool. The ordinary
`cpu-spot` pool continues computation and restore with normal scheduler balancing.
Each pool is limited to 32 vCPUs / 128 GiB (combined 64 vCPUs / 256 GiB).
The [bootstrap qualification](../diagnostics/validation/20261023-l2-domain/REPORT.md)
passed 3,606 L2 observations, 3,600 replays, 1,200 A/A pairs and all 19 S3 hashes.
It does not replace or regrade periodic B's failed fresh epoch, and did not repeat
CRIU restoration. Runtime image/controller pins remain unchanged.
