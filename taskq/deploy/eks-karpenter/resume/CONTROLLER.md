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
