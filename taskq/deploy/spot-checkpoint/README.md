# Protocol-bound experiments on a queue over Kubernetes Spot nodes, with checkpoint and restore

**This design makes no performance claim.** It moves a frozen protocol's
jobs to the cheapest capacity that can run them, brings their rows back in
one shape, and fixes in advance what an interruption and a restore do to a
measurement. No benchmark row moves because of it.

Status of each part is in the table at the end: what runs today, what this
document designs, and what nobody has tested. The owner's request
(2026-10-22) is the design; the gaps are tracked in
[`problems/taskq-spot-checkpoint-restore-not-implemented.md`](../../../problems/taskq-spot-checkpoint-restore-not-implemented.md).
The skills that use it are `queue-experiment` (submit), `experiment-protocol`
(the protocol every job binds) and `review-run` (another runner's reading of
the export); the record shapes are in aburan28/crypto-autoresearcher,
`docs/experiment-protocol.md`.

## 1. What exists, and what each piece already guarantees

| piece | where | guarantees today | gap for this design |
|:--|:--|:--|:--|
| taskq | `taskq/` | Redis stream queue; a worker pod checks out a pinned commit, runs argv without a shell, measures with `wait4`, writes a write-once result under a fence; only infrastructure failures retry; a SIGTERM (eviction) hands the task back and the next attempt starts **from scratch**; Helm pools with Guaranteed QoS; KEDA scaling | no checkpoint directory, no resume, no `resumed` count in the result; `attempts` counts every start |
| EKS Iso Lab | `taskq/deploy/eks-karpenter/` | Karpenter `cpu-spot` NodePool (Spot only, C/M/R gen 6+, 64 vCPU cap), SQS interruption queue and EventBridge rules, static whole-core CPU manager, one lab job per node, `terminationGracePeriod: 1h` on the pool, S3 results, `isolab_k8s_*` MCP; L2 earned by observation (`research/eks_crypto_validation_20261019`: 72 rows, all L0 on a c8in.xlarge VM) | no checkpointing; a job interrupted by a Spot reclaim is lost unless it uploaded |
| spotlab | `spotlab/` | cheapest Spot offer across AWS and GCP; the agent uploads `$SPOTLAB_CHECKPOINT_DIR` every `interval_s` and on the interruption notice, SIGTERMs the job with `grace_s`, and the controller relaunches and restores the newest checkpoint; fencing; cost ceilings | not Kubernetes; CRIU listed as not built (a 30 s GCP notice rarely fits a large dump) |
| spot-system | `infra/spot-system/` | one Spot host; `experiment.run/v1` records; the `manifest-sha256-v1` checkpoint contract (immutable generations, manifest written last, bound to job id and spec hash, newest valid generation selected at resume, no silent restart from zero); CRIU 4.2.1 built from source for the legacy processes, with a tested kill/restore on a disposable host and the pool pinned to one instance family while CRIU images are live | one host, not a queue; CRIU enrolment is per process and explicit |
| kubelet checkpoint API | the EKS 1.36 nodes | `ContainerCheckpoint` is beta since Kubernetes v1.30 and on by default; `POST /checkpoint/{ns}/{pod}/{container}` on the node's kubelet writes a tar under `/var/lib/kubelet/checkpoints` through the CRI, which calls CRIU | the archive stays on the node unless something uploads it; restore is not a kubelet operation (section 4, route C) |

## 2. The job envelope and the two queues

A submitter writes one `experiment-job/v1` envelope
([`experiment-job.v1.json`](experiment-job.v1.json)) per (arm, instance) or
shard of a **frozen** `experiment.protocol/v1`. The envelope carries the
protocol hash, the arm and instance, the `measurement_class`, the pinned
source, the argv, placement, limits, the checkpoint block and where the
export lands. The protocol's class decides the queue:

| class | queue | pool | resume |
|:--|:--|:--|:--|
| `measurement` (a clock or counter is read against an A/A interval) | `cpu-exclusive` on taskq, or `isolab_k8s_benchmark` | Guaranteed QoS, integer CPUs, static CPU manager, one job per node; Spot allowed because the level is earned by observation, never granted by the node | **never**: an interrupted row is abandoned and a new row runs; the export carries both with their status |
| `throughput`, `search`, `training` | `cpu-spot-checkpoint` on taskq, spotlab, or the spot-system host | the Spot pool, `terminationGracePeriodSeconds` inside the provider's notice | yes, from the newest valid generation; the row's `resumed` count and `wall_valid: false` say so |

The envelope compiles to the backend's spec by this table; the compiler is a
submitter-side step (today by hand or by the `queue-experiment` skill, later
a `taskq submit --job envelope.json` that the problem record names):

| envelope | taskq `task-spec/v1` | isolab `job/v1` / `isolab_k8s_benchmark` | spotlab `job/v1` |
|:--|:--|:--|:--|
| `job_id` | `idempotency_key` | `name` (unique per run) | `name` |
| `protocol.*`, `arm`, `instance` | `labels.protocol_sha256`, `labels.arm`, `labels.instance` | `labels` | `labels` |
| `source.commit`, `sparse_paths` | `source.commit`, `source.sparse_paths` | `inputs[].git` at the commit, or `source.image_digest` | `--input` blobs by hash |
| `command.argv`, `setup`, `env` | `command.argv`, `command.setup`, `command.env` | `command.argv`, `build[]` | the command after `--` |
| `command.verify` | `verify.builtin: certificate` | `verify.builtin: taskq-certificate` | `outputs.verify` in the backlog card |
| `placement.cpus`, `memory_mb`, `isolation_required`, `require_labels` | `placement.cpus`, `limits.memory_mb`, `placement.require_labels` (`pool`) | `resources.cpus`, `memory_mb`, `fidelity.policy` from the level (`strict` for A, `standard` for B/C) | `--vcpus`, `--memory`, `--flags` |
| `limits.timeout_seconds`, `max_attempts` | `limits.timeout_seconds`, `retry.max_attempts` | `limits.timeout_s`, `retry` | `limits.max_total_hours`, `max_attempts`, `max_cost_usd` |
| `checkpoint.*` | the `checkpoint` block of section 3 (proposed) | not applicable (measurement jobs are never resumed) | `checkpoint.interval_s`, `checkpoint.grace_s` |
| `export.out` | `experiment_protocol.py from-taskq` on the result | `from-ecbench` on the fetched session | `from-rows` on the fetched output |

## 3. Route A: application checkpoints (the contract that exists)

The contract is spot-system's `manifest-sha256-v1` and spotlab's job-side
rules, restated once for every backend. A job that resumes must:

1. keep every file it needs to continue under `$CHECKPOINT_DIR/gen-<n>/`,
   each generation complete on its own (never incremental across
   generations);
2. write every file, fsync it, then write `manifest.json` **last** (temp
   file, fsync, rename, directory fsync) listing `job_id`, the protocol or
   spec sha256, `generation`, `committed: true` and every file's relative
   path, sha256 and size; keep the previous valid generation;
3. on start with `CHECKPOINT_RESUMED=1`, restore exactly the generation named
   by `CHECKPOINT_MANIFEST`; never pick another and never start from zero
   when restore fails (exit non-zero and say so);
4. checkpoint every `interval_seconds` and on SIGTERM, and exit within
   `grace_seconds` of SIGTERM; a timer checkpoint is mandatory because the
   interruption notice is best effort.

[`checkpoint-hook.sh`](checkpoint-hook.sh) is the thin supervisor that runs
a command under this contract on any Linux host or pod: it restores the
newest valid generation from the store (rejecting incomplete or mismatched
ones), starts the command in its own process group, syncs committed
generations to S3 on a timer and on SIGTERM, forwards SIGTERM and waits
`grace_seconds`, and exits 75 when the run was interrupted so the queue
resumes it. The same contract is what spot-system's `checkpoint-discovery`
validates natively; the hook is the portable shell form for pods.

**Proposed additive `checkpoint` block in `taskq.task-spec/v1`** (a worker
that does not know it hands the task back, which is the documented safe
failure for version skew):

```json
"checkpoint": {"policy": "application", "interval_seconds": 300, "grace_seconds": 90,
               "store": "s3://<bucket>/taskq/ckpt/<job_id>/", "restore_validation": "manifest_sha256",
               "keep_generations": 2}
```

Worker semantics to implement (`taskq/taskq/worker.py`, `execute.py`):

- provide `TASKQ_CHECKPOINT_DIR` (a ReadWriteMany PVC path or a node-local
  directory synced to `store`), and on a resumed attempt `TASKQ_RESUMED=1`
  and `TASKQ_CHECKPOINT_MANIFEST`;
- on SIGTERM: forward SIGTERM to the child's process group, wait up to
  `grace_seconds`, sync committed generations, then hand the task back
  **without** counting an attempt (a preemption is not an infrastructure
  failure of the task) and record `resumed += 1` on the task hash;
- on claim: run manifest validation (`manifest_sha256`) over the newest
  generation, fall back to the previous valid one, refuse when none is
  valid and generations exist;
- in the result: per run `resumed`, `checkpoint.generations_written`,
  `checkpoint.restored_from_generation`, and `wall_valid: false` whenever
  `resumed > 0` or the row was interrupted; the sum of attempts' wall time
  is kept as `wall_seconds_total` for cost accounting, never as a
  measurement.

Pod settings ([`values-spot-checkpoint.example.yaml`](values-spot-checkpoint.example.yaml)):
`terminationGracePeriodSeconds: 110` so the child's 90 s grace and the
upload fit inside AWS's 120 s notice; the Karpenter NodePool keeps its
long `terminationGracePeriod` because the node waits for the pod; KEDA
`cooldownPeriod` long enough that scale-in never evicts a pod
mid-generation.

## 4. Route B: CRIU process images (designed, untested in a pod)

For a program that cannot checkpoint itself (a long native solver whose
state is its address space), the worker dumps the process tree instead of
asking it to save. The owner's spot-system already does this for its legacy
processes on EC2 with CRIU 4.2.1 built from source, after AL2023's packaged
3.17.1 failed on `rseq` under kernel 6.18 (`infra/spot-system/CRIU_LOCAL.md`),
so the pieces are known; what is new is doing it inside a pod on a queue.

- **Pool.** A separate `cpu-spot-criu` pool whose image carries the pinned
  CRIU build and whose pod spec grants `CAP_CHECKPOINT_RESTORE`,
  `CAP_SYS_PTRACE` and (for mount and net namespace restore) `CAP_SYS_ADMIN`,
  with `seccompProfile: Unconfined`, no gVisor, and `/proc/sys/kernel/ns_last_pid`
  writable; the pod is therefore not `restricted`, so it lives in its own
  namespace with its own quota, never in `isolab`.
- **Dump schedule.** `criu pre-dump --track-mem` every `interval_seconds`
  into `gen-<n>` with `--prev-images-dir gen-<n-1>`, so memory is
  transferred incrementally; on SIGTERM a final `criu dump --tree <pid>
  --shell-job --leave-running=false --cpu-cap=<cap>` whose size is the
  dirty pages since the last pre-dump. The first run measures the dump
  rate; the worker refuses a job whose resident set would not dump inside
  `grace_seconds` (`checkpoint.criu.image_budget_mb`).
- **Restore.** The next pod on the same pool runs `criu restore
  --images-dir gen-<n> --restore-detached` and then the self-check
  (`restore_validation: criu_selfcheck`): the restored process must report
  a progress marker that the manifest recorded, and the worker compares;
  a failed self-check is an `infra_error`, never a fresh start.
- **Constraints the envelope must pin.** The same CPU feature set
  (`placement.instance_families` or `--cpu-cap=fpu`; spot-system pinned
  m8i for its live images), the same kernel major and CRIU version in the
  image, no network sockets (`network: none`; `--tcp-close` otherwise),
  no GPU (the CUDA plugin through `cuda-checkpoint` exists in CRIU's GPU
  support and is out of scope here), a private time namespace recorded in
  the manifest, and image privacy: a CRIU image holds the process's memory,
  so the store prefix is encrypted and never leaves the account.
- **Why not by default.** A dump freezes the process (spot-system measured
  0.46 to 1.29 s per dump for its processes) and a large final dump may not
  fit a notice; a program that can write its own state in seconds is
  cheaper and portable. Route B is for what cannot.

## 5. Route C: kubelet checkpoint and restore from an image (observed, not designed)

The kubelet API produces the archive; turning it into a runnable pod needs
a checkpoint image (`buildah` with the runtime's checkpoint annotation:
`io.kubernetes.cri-o.annotations.checkpoint.name` for CRI-O,
`org.criu.checkpoint.container.name` for containerd) and a runtime that
restores from it when the image is used in a pod's `image` field. CRI-O
supports that path; containerd's CRI restore was proposed in
containerd/containerd#10365 and its status on the EKS AL2023 containerd
of this cluster is unverified. A Kubernetes checkpoint/restore working
group was announced in January 2026 with pod-level checkpointing and
preemption use on its roadmap, with no committed version. Until a node AMI
of this cluster is shown to restore a checkpoint image, route C stays a
forensic tool here, and the design uses routes A and B.

## 6. Interruption, end to end

```text
EC2 Spot Interruption Warning (2 min, best effort)
   ├─ EventBridge → SQS → Karpenter: taint, drain, replacement node (interruption queue configured)
   │       └─ kubelet: SIGTERM to the pod, terminationGracePeriodSeconds 110
   └─ worker's own IMDS poll of spot/instance-action every 5 s (fallback when the queue is late)
              └─ worker: SIGTERM to the child's group → child commits gen-n → sync to store → hand back (resumed += 1)
KEDA/Karpenter: a new pod claims the task → validate gen-n → TASKQ_RESUMED=1 → the row continues
```

Rebalance recommendations are published as Kubernetes events and do not
drain; a worker may take a timer checkpoint on one. A hypervisor pause
past the lease is handled by the fence as before: a stale worker writes
nothing.

## 7. What a restore means for the rows (fixed, and enforced by the export lint)

- Counted units (group operations, field multiplications, relations,
  instructions) are valid across a restore.
- Wall and CPU time across a restore are not: `wall_valid: false`,
  `resumed > 0`, `checkpoint.restored_from_generation` set. The export lint
  in crypto-autoresearcher refuses a resumed row with a valid clock.
- A measurement-class row is never resumed. An interrupted one is
  `interrupted` / `not_completed`, kept in the export, and a new row runs
  under a new `run_id`.
- A retry counts infrastructure failures; a non-zero exit or a timeout is
  an outcome, never retried, never evidence about the thing measured.
- Every row carries the host it ran on and the isolation level it earned,
  so a result assembled from several Spot hosts can still be read per host
  class and never pooled across classes for wall time.

## 8. Status

| part | status | evidence or owner |
|:--|:--|:--|
| taskq queue, pools, fence, retries, result schema | implemented | `taskq/`, CI `taskq.yml` |
| EKS Spot pool with interruption queue and L2 CPU manager | implemented, measured | `taskq/deploy/eks-karpenter/`, `research/eks_crypto_validation_20261019` |
| application checkpoint contract (`manifest-sha256-v1`) | implemented on the spot-system host and in spotlab; **the pod form here is the hook script, untested on the cluster** | `infra/spot-system/CHECKPOINT_DISCOVERY.md`, `spotlab/README.md`, `checkpoint-hook.sh` |
| `experiment-job/v1` envelope and backend mapping | designed; compiler not implemented | this directory |
| taskq `checkpoint` block and resume semantics | designed; worker not implemented | section 3; problem record |
| CRIU pool (route B) | designed from spot-system's tested CRIU build; **not run in a pod** | section 4; problem record |
| kubelet checkpoint and image restore (route C) | observed; not designed | section 5 |
| export semantics after restore | implemented in the export lint (crypto-autoresearcher `tools/experiment_protocol.py`) | `tests/test_experiment_protocol.py` there |

## 9. Sources

Retrieved 2026-10-22 while writing this document (provenance per the
sibling repositories' citation rule): the Kubernetes kubelet checkpoint API
reference (`ContainerCheckpoint` beta since v1.30, endpoint and archive
location); the EC2 user guide on Spot interruption notices (two-minute,
best-effort, IMDS `spot/instance-action`, EventBridge detail type
`EC2 Spot Instance Interruption Warning`); the Karpenter disruption page
(interruption queue handling: taint, drain and replace on a warning;
rebalance recommendations evented only); containerd/containerd#10365 and
cri-o/cri-o#4199 (restore through CRI; the containerd status on this
cluster unverified); the CRIU GPU checkpointing page and NVIDIA's
`cuda-checkpoint` repository (CUDA state through a plugin; out of scope);
criu.org's Kubernetes page returned HTTP 503 and is cited through the
2022 Kubernetes blog on forensic container checkpointing and the two pull
requests above. Internal: `infra/spot-system/` (CRIU 4.2.1 on AL2023,
kernel 6.18; `rseq` failure of 3.17.1; dump freeze intervals),
`spotlab/README.md`, `taskq/README.md`.
