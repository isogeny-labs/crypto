# Native recovery controller and trusted supervisor

One Rust executable supplies an AWS-only client (`submit`, `remote-status`,
`wait`), a CodeBuild reconciler (`controller`), and the root PID-1 supervisor in
an isolated Kubernetes Pod (`supervise`, `rpc`, `status`). See the parent
[implementation contract](../CONTROLLER.md) and [request schema](../request.schema.json).

The controller owns no payload process itself. Kubernetes ConfigMaps hold its
immutable logical request, current owner and state, guarded by resourceVersion.
Each supervisor operation is named and recorded on the EBS PVC, so a restarted
controller reconciles existing effects. `CodeBuild` status is the owner fence;
EC2 `terminated` is the separate old-writer fence. An expiring heartbeat grants
neither. API errors preserve state for bounded infrastructure retries. Failed
correctness, runtime compatibility or benchmark admission is terminal.

Before requesting a checkpoint, the controller persists the intended generation
with its `Checkpointing` phase. That intent survives committing the generation
and sending SIGCONT, until `Running` is persisted. A supervisor still reporting
the previous generation therefore cannot complete a newly scheduled checkpoint.
Restart reconciliation uses the persisted intent rather than the incremented
next-generation counter. Older running states can acquire this field on their
next checkpoint; an older state already in `Checkpointing` without an intent
fails explicitly instead of guessing whether SIGCONT or a new checkpoint was
pending.

CRIU can capture an exited ecbench child while its stopped parent has not yet
reaped it. Such a zombie is not admitted as a stopped process. The supervisor
retains that uncommitted capture, resumes the same computational process to reap
the child, and captures again, at most eight times within the original deadline.
Every admitted process and thread must be stopped and retain the original
credential checks. A completed root proceeds to native verification. This does
not retry an L2 epoch or reset a checkpoint's logical generation; no EBS/S3
generation is committed until an admissible capture exists.

The supervisor has no service-account token or AWS credentials. It drops the
payload's UID/GID, groups and capabilities before exec; the separate native
progress launcher forces L0 and denies perf descriptors. CRIU restores with
`--leave-stopped` and empty supervisor groups. Every restored process/thread is
checked before SIGCONT. The same runtime image, actual native binary, CPU
identity, kernel and container runtime class remain required. This is a trusted
process service, not a security boundary for arbitrary third-party code.

The request's original UTC deadline applies to every attempt. No fallback
launch from zero is permitted. Computation verification precedes exactly one
fresh Restricted benchmark Job. Its complete downloaded S3 manifest, original
spec, image/source/binary identity, L2 observations, native replays and A/A
controls must all pass. A failed or ambiguous benchmark submission is not given
a new Job name or another measurement attempt.

Tests exercise immutable request binding, original deadlines, ambiguous owner
and worker fences, incomplete generation retention, failed-measurement refusal,
credential restoration, completion/checkpoint races and runtime identity.
They also cover consecutive checkpoint generations and serialized controller
restarts before and after SIGCONT. These transition tests do not replace a live
multi-generation CRIU recovery qualification.
Actual qualification evidence lives beside `resume/validation/` and determines
which paths may be described as tested; unit tests alone do not prove recovery.

```bash
cargo +1.99.1 test --locked --release --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml
cargo +1.99.1 clippy --locked --release --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml --all-targets -- -D warnings
```
