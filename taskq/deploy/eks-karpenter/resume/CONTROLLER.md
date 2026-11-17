# Resumable EKS submission implementation contract

The deployed native controller implements the separate resumable contract.
Automatic worker-loss and controller-restart qualification E passed, including
native computation replay and a distinct fresh L2/S3 epoch. See the
[qualification report](validation/20261021-controller/REPORT.md) for exact runtime
pins, final-runtime retest and coverage limits. Ordinary v1 remains non-resumable.

The portable `isolab.eks.resumable/v1` request binds a logical run name,
original absolute UTC deadline, immutable source/binary/image identity, the
original L2 ecbench spec, checkpoint interval and bounded recovery count. Initially
only the qualified c8a.2xlarge CPU class is admitted. Ordinary v1 fresh jobs retain
their broader x86 Spot placement policy and zero measurement retries.

A dedicated VPC CodeBuild controller owns one durable Kubernetes ConfigMap
per logical run. A different controller may take ownership only after AWS reports
the prior CodeBuild execution terminal, using a resource-version guarded update.
An unchanged request is idempotent; changing its contents under the same run name
is rejected. Controller retries must reconcile existing resources and operation
markers, never create another live writer or restart computation from zero.

The trusted supervisor runs in `isolab-recovery-system`, with no service-account
token or AWS identity in its workload pod. It drops the computational payload to
UID/GID 1000, empty supplementary groups, no capabilities and no_new_privs. The
native progress launcher forces L0 and denies performance-counter creation; the
fresh benchmark uses the ordinary Restricted pod path. Containerd's disabled CRI
restore-via-create path remains disabled.

Each periodic generation leaves the complete process tree stopped, flushes all
mutable files to the PVC, takes a CSI EBS snapshot, and verifies a private
versioned encrypted S3 archive by downloading and hashing it before committing.
Only then does computation continue. The latest two committed generations remain
eligible for restore. Older EBS snapshots and S3 versions are physically retained
until explicit operator cleanup; this implementation does not garbage-collect them.
An interrupted or corrupt new generation cannot replace a known-good generation.
Original source/image/spec/deadline and runtime compatibility remain bound to every
attempt. Every failed checkpoint, restore and verification is retained.

On worker loss, replacement waits for EC2 to confirm the previous worker is
terminated. A missing heartbeat, an unavailable Pod, stopped EC2 instance or a
missing EC2 record cannot authorize takeover. The controller has no permission
to terminate arbitrary workers. It clones the committed volume snapshot, checks
all payload/runtime hashes and the CPU/kernel/container/CRIU class, clears the
restore supervisor's supplementary groups, and restores the original process tree.
No fallback to starting the original command is permitted. The original UTC
deadline remains in force, independent of recovered monotonic timers.

After computation succeeds and native replay verifies its results, one distinct
benchmark epoch uses the original L2 spec, warmup, A/A, all-record isolation gate,
native replay and all S3 checks. Its Job/Pod identities and prefix are separate
and linked from the logical recovery receipt. Timing that crosses a checkpoint
or restore is never admitted. Failed benchmark admission is terminal; it is not
an infrastructure retry. Both computation and benchmark artifacts remain private.

Acceptance requires a real harness request, periodic committed checkpoints, an
induced loss of only that experiment's own Spot worker, automatic restored
progress, native correctness replay, and the fresh L2/S3 epoch. It also requires
controller restart, duplicate request, incomplete generation, corrupt archive,
deadline expiry and nonterminal-old-owner tests. A successful manual qualification
does not satisfy these controller acceptance criteria.

## Bounded memory envelope

Requests may set `"memory":"8Gi"` when that per-run envelope is authorized.
Only `4Gi` and `8Gi` are accepted. Omission selects the historical `4Gi`;
explicit `4Gi` canonicalizes to omission, preserving existing request bytes,
hashes and persisted state identities. Selecting `8Gi` changes the immutable
request hash and cannot change the envelope of an existing logical run.

The selected value is used for both requests and limits on the resumable
computation container and its separate fresh L2 job. CPU, storage, deadlines,
restore limits, retries, placement and authorization do not change. New pencil
requests requiring 8 GiB must explicitly set the field; old requests remain
at 4 GiB. This source change does not deploy a controller or publish a runtime.

Validation and limitations are in `validation/20261029-memory-envelope/README.md`.

## Command mode

**Status: implemented and unit-tested only. Command mode has no live CRIU
qualification.** No command-mode request has been dumped, restored after worker
loss, or uploaded on EKS. Every qualification above concerns ecbench
computations and does not transfer to an arbitrary command. A first live run
is a qualification attempt, not a production run.

An optional `command` object (see `request.command.example.json`) replaces the
ecbench computation with a trusted native command:

```json
"command": {
  "argv": ["/opt/study/launch/job-prime-tower-criu.sh", "main"],
  "outputs": "/work/compute/out",
  "threads": 4,
  "binaries": {"/opt/study/bin/prime-tower": "<sha256>"},
  "pvc_gib": 32
}
```

- `argv`: 1..64 strings, each at most 4096 bytes without NUL, CR or LF.
  `argv[0]` is a normalized absolute path inside the image, not under `/work`,
  `/request`, `/proc`, `/sys`, `/dev` or `/tmp`. `build.ecbench_sha256` binds
  the SHA-256 of `argv[0]` (the field keeps its name so the `build` object and
  every ecbench request stay byte-identical).
- `binaries`: optional, at most 32 further absolute image paths (not `argv[0]`)
  with their SHA-256, for example the native program a launcher script runs.
- `outputs`: a normalized directory under `/work/compute`, not `tmp`,
  `run.stdout` or `run.stderr`. The supervisor creates it, owned by UID 1000.
- `threads`: 1..4, default 1; explicit 1 canonicalizes to omission.
- `spec` must be absent or null. `diagnostic_transport`, `worker_campaign` and
  `fresh_owner` are rejected, since each binds an ecbench spec or a fresh epoch.
- `pvc_gib`: optional computation PVC size, 8..64 GiB, default 8; explicit 8
  canonicalizes to omission. Every attempt's PVC uses it, including PVCs cloned
  from a checkpoint snapshot, so a clone is never smaller than its source; a
  parseable snapshot `restoreSize` larger than it is refused. ecbench PVCs stay 8Gi.
- `max_restores` may be 1..50 with `command`; ecbench requests keep 1..5.

The supervisor verifies `argv[0]` and every listed binary at admission of each
attempt, again immediately before the first launch, and again before CRIU
restore. Generation manifests record the listed hashes as
`runtime.payload_binaries` (omitted for ecbench, so their manifest bytes are
unchanged). It then execs `argv` directly instead of `isolab-criu-progress`
with the same `env_clear`, cwd `/work/compute`, `HOME`, `TMPDIR`, `PATH`,
`run.stdout`/`run.stderr` files and UID/GID 1000 drop, and adds
`ISOLAB_OUTPUT_DIR=<outputs>`, `ISOLAB_CRIU=1` and `RAYON_NUM_THREADS=<threads>`.
Because the progress launcher is bypassed, the supervisor installs the same
computation-only seccomp filter (perf_event_open returns EPERM) after the
privilege drop; CRIU 4.2.1 cannot dump perf descriptors and the restore
credential check requires seccomp filter mode. Checkpoint, generation commit,
fencing, restore, deadline and restore-budget logic are the ecbench code paths.

**Generation pruning (command mode only).** Each generation's CRIU image lives
on the PVC under `/work/generations/gN`. Once generation `n` is fully
committed (snapshot ready, S3 archive uploaded and read back, state saved) and
the controller resumes the process, it sends a `prune` operation for `n`
carrying the committed manifest hash. The supervisor deletes `gK` for every
`K < n-1` from the live PVC and keeps `g(n-1)` and `gn`, which are the two most
recent committed generations: commit numbers are contiguous, since a commit
requires `number == next_generation`. It refuses, deleting nothing, when the
manifest hash differs from the committed one, `gn` is absent, or any newer,
uncommitted directory exists. S3 archives and EBS snapshots are never touched;
restores always clone a snapshot, which still contains the restored
generation's own directory. The controller records `prune-request-gN.json` in
its receipts and the supervisor writes `/work/control/attempt-A/prune-gN.json`
(existing, kept, pruned with bytes, errors, refusal). A failed or refused prune
never stops the computation. Because pruning follows the snapshot, the PVC
holds three generations during a dump (two otherwise) while prunes succeed;
a refused prune leaves older directories until a later one succeeds. ecbench requests never prune.

When the root process exits, its exit code is recorded whatever its value and
is never retried. There is no native replay and no fresh L2 benchmark epoch.
The controller asks the supervisor to `seal`: it kills any remaining member of
the payload's process group, then hashes every regular file under `outputs`
plus `run.stdout` and `run.stderr`, without following symlinks. Symlinks,
special files, unsupported names (control characters, backslash, non-UTF-8)
and files over 5 GiB are listed as skipped. More than 1,024 files fails the
run. The seal and `SHA256SUMS` stay in root-only `/work/control/attempt-N/command/`.
The controller copies each sealed file out with `kubectl exec cat`, checks its
hash and size against the seal, uploads it with the existing verified private
S3 put (versioned, AES256, complete readback), and writes:

```text
s3://isolab-results-104186811093-us-east-2/recovery/runs/<run_name>/
  computation/<path relative to /work/compute>   e.g. computation/out/result.json
  computation/run.stdout, computation/run.stderr
  computation/SHA256SUMS, computation/seal.json
  command-result.json        isolab.eks.command-result/v1: exit code, keys, versions,
                             hashes, skipped entries, restores; timing_valid=false, L0
  state.json                 phase "command_completed", command_exit_code
```

Only then does the run become terminal `command_completed`; the idle
supervisor Pod is then deleted (best effort, recorded on failure). The PVC and every EBS snapshot remain, as for ecbench.
The controller reports success for any recorded exit code; read
`command_exit_code`. `wait` returns `isolab.eks.command-recovery-result/v1`.
Command records are L0 computation evidence. Timing is never admitted.

What payloads must satisfy (not checked, and not qualified):

- No network connections, listening sockets or Unix sockets in the payload tree
  at any checkpoint. CRIU dumps of sockets fail or cannot be restored, and the
  workload Pod has no route that a restored connection could resume.
- The workload Pod has no service-account token and no AWS identity. Read
  inputs only from the image. Write only to the outputs directory (and `tmp`).
- Every mutable file must be a regular file or directory under
  `/work/compute`: checkpoints refuse symlinks, FIFOs and sockets there.
- Stay in the launched process tree: a daemonized or double-forked descendant
  is reparented to the supervisor, is not dumped, and is lost on restore.
- No GPU, perf, io_uring or other descriptor CRIU 4.2.1 cannot dump.
- Fit the PVC (`pvc_gib`): outputs, `tmp`, logs and up to three CRIU images.
  Each image is roughly the process tree's resident memory.
- Leave enough of the original deadline after the command exits for the
  sealed files to be copied and uploaded before the Pod's deadline. The
  deadline still applies to the completion transition.

**Shared controller slot.** The recovery CodeBuild project admits one
controller at a time (`ConcurrentBuildLimit: 1`, `QueuedTimeoutInMinutes: 10`,
`TimeoutInMinutes: 1500` in `bridge.yaml`). A 15-hour command run holds the
controller's single CodeBuild slot for its whole duration: every other
recovery submission, including other users' ecbench runs, that waits more
than 10 minutes in the queue fails to start.

This is a trusted-workload feature. The privileged supervisor is not a sandbox
for untrusted code; `argv` and `binaries` must name reviewed programs.
