# Checkpoint and resume admission

The owner selected CRIU process checkpoints plus EBS volume snapshots, with
computation resumed and a fresh benchmark epoch after interruption. The native
`isolab.eks.resumable/v1` controller implements this policy on c8a.2xlarge. The
ordinary EKS v1 job path uses temporary `/work` and remains non-resumable.

## Submit from Claude, Codex or another harness

Use a Linux x86-64 caller with AWS CLI, jq and the configured Iso Lab identity.
The caller needs AWS API access; CodeBuild supplies the private EKS route.
From the repository root:

```bash
install_dir="$HOME/.local/bin"
taskq/deploy/eks-karpenter/resume/fetch-client "$install_dir"
client="$install_dir/isolab-eks-recovery"
"$client" validate /absolute/path/recovery-request.json
"$client" submit /absolute/path/recovery-request.json /absolute/path/receipts
"$client" remote-status /absolute/path/recovery-request.json /absolute/path/receipts
"$client" wait /absolute/path/recovery-request.json /absolute/path/receipts
```

Build locally instead if needed, using the controller README. The strict
[JSON schema](../../../../taskq/deploy/eks-karpenter/resume/request.schema.json)
and [request example](../../../../taskq/deploy/eks-karpenter/resume/request.example.json)
define all fields. Replace the example's placeholders before validation. Use
`publish-runtime BASE-IMAGE.json OUTPUT.json` to compose the pinned recovery
tools over the tested digest; record that returned runtime image in the request.
Never substitute an unrelated default image for the requested source commit.
The pinned client is a Linux x86-64 binary. On another operating system, use
the GitHub route once merged, or invoke the client on the configured Linux lab
host; do not try to execute the Linux binary directly on macOS or Windows.

Retain the identical request file and original deadline on any retry. A timeout
after submit is ambiguous: inspect the original run/status before retrying.
The controller currently admits one active CodeBuild execution. A definitive
concurrency rejection started no controller; retry that same request after the
slot becomes free. Do not extend its deadline or rename it to bypass a failure.

`wait` succeeds only after computation replay and the distinct fresh benchmark's
L2, native replay, A/A and complete downloaded S3 manifest checks pass. Preserve
`result.json`, the request hash, build ID, both epoch identities and S3 locations.
Computation records are always L0/timing-invalid, even when no restore occurred.

Read the qualification named in `resume/runtime.json` for the exact pinned
runtime. The latest periodic test restored generation 2 and passed computation
replay, but its fresh epoch failed L2 (3,603 L2 / 3 L1; A/A not evaluated).
All 18 emitted S3 payload hashes matched. This remains a failed run, with no
measurement retry; earlier successful epochs do not turn it into a pass.

The maintained MCP binary also exposes `isolab_k8s_resumable_submit` and
`isolab_k8s_recovery_status` when this native client is installed on its PATH.
Inspect the live schema: older installed servers do not have these tools.
The GitHub workflow has optional `resumable`, `checkpoint_seconds`, and
`max_restores` inputs in this branch. Its full main-branch OIDC execution remains
pending the parent and recovery PR merges; do not claim it was verified by the
AWS-only qualification.

An operator-controlled EKS test on 2026-10-21 did restore a UID-1000 Rust SHA-256
process from CRIU 4.2.1 and an encrypted EBS CSI snapshot onto a new c8a.2xlarge
Spot instance in another availability zone. It continued from iteration 48,128
through 1,048,576, with one process start and native replay of every recorded
hash. Read the [tested procedure and evidence](../../../../taskq/deploy/eks-karpenter/resume/README.md).
This earlier test qualifies that fixture and configuration. The automatic native
qualification below adds the controller path. Track remaining coverage in
[the problem record](../../../../problems/eks-resumable-l2-experiments.md).

The standalone Spot host's CRIU recovery in `infra/spot-system/CRIU_LOCAL.md`
is a different execution path. Do not substitute it silently. Keep containerd's
`enable_experimental_restore_via_create` disabled: the upstream CRI restore path
does not enforce the destination security context. The tested path uses a
dedicated operator pod and restores only a trusted child process; normal
`isolab` experiment pods retain their Restricted admission policy.

The requested EKS recovery path must bind a coherent CRIU process checkpoint
and filesystem/volume snapshot into one committed generation. Checkpoint memory
alone does not capture changing backing files; a volume snapshot alone does not
capture RAM. A generation is recoverable only after its complete artifacts,
hashes and restore compatibility metadata are durable and independently checked.

Before allowing resume, require evidence of all of the following:

- Same immutable executable image, source/spec identity, original logical run
  and original UTC deadline; retain every attempt and checkpoint generation.
- Compatible architecture, CPU features, kernel, container runtime and CRIU
  version. Broad Spot placement for a new run does not imply cross-family CRIU
  portability. Check CPU capabilities and pin the tested compatibility class.
- The prior writer is fenced before restoration. An expired heartbeat or an
  unavailable node is not proof its process stopped. Never create two live
  copies of an experiment or repeat external side effects silently.
- Verified, private, encrypted checkpoint storage; preserve an older valid
  generation if the latest snapshot or checksum is incomplete/corrupt. Reject
  identity mismatches rather than starting a fresh computation under the old ID.
- Periodic checkpoints as well as interruption handling. Spot interruption
  notification cannot be the only recovery mechanism. Account for checkpoint
  freezes, dump/upload time, downtime and uncommitted lost work.
- A real kill/replacement-worker restore that advances the original in-memory
  state and matches an independently verified reference result. Retaining files
  or starting a new process from zero is insufficient.

For timed ecbench experiments, checkpoint/restore is a measurement boundary.
Record interruptions explicitly, retain and censor any affected timing interval,
recheck the new host and CPU affinity, and perform fresh warmup, L2 observation
and A/A calibration before admitting new measurements. Do not reuse the old
host's L2 grade or combine incomparable hosts into one benchmark interval.
Preserve the old epoch and give the new epoch a distinct Job/Pod identity and
S3 prefix linked to the same logical experiment. If the computation itself
resumes, label its crossing interval as computational progress only. Do not
turn a failed A/A or correctness check into an infrastructure retry.

Use the separate recovery request/status schema. Do not add an
unrecognized `resume` field to `isolab.eks.request/v1`; its validator deliberately
rejects unknown fields. A requested resumable experiment must fail admission when
its checkpoint/restore capability is missing, not silently run without recovery.

## Native implementation and current qualification

The operator qualification now also covers real native ecbench: 36 computations
continued after CRIU/EBS replacement, with 30 independent replays. Its first dump
failed on an open perf_event descriptor; the new computation launcher explicitly
forces L0 and denies perf creation. The first restore inherited group 0; clearing
the supervisor's groups fixed it. Both failures are retained in the
[native qualification report](../../../../taskq/deploy/eks-karpenter/resume/validation/20261021-ecbench/REPORT.md).
A separate fresh epoch passed 36 L2 observations, 30 replays, 10 A/A pairs and all
19 S3 payload hashes. These are scoped operator proofs, not automatic failover.

Automatic qualification E then passed a controller restart and termination of
only its dedicated Spot worker. The restored session preserved its 1,140-record
checkpoint prefix and completed 1,806 records. The distinct fresh epoch passed
1,806 L2 observations, 1,800 native replays, 600 A/A pairs and all 19 S3 payload
hashes. A duplicate terminal request created no second epoch. Final runtime F
also passed worker loss, 1,147→1,806 continued records, unprivileged replay and
the same fresh L2/replay/A-A/S3 counts. Its exact pins are recorded in the
[controller report](../../../../taskq/deploy/eks-karpenter/resume/validation/20261021-controller/REPORT.md).

This is trusted native CPU workload support. Network sessions, GPU/ARM state,
arbitrary commands and cross-family restore remain unqualified. The supervisor
is privileged and is not a sandbox for untrusted code. Private CRIU archives may
contain process memory; never copy them into Git or GitHub artifacts. Two recent
generations are eligible for automatic restore; older EBS snapshots and S3
versions are retained until explicitly cleaned up by the operator.

The longer periodic B test later recovered 3,606 computations / 3,600 replays
but failed its fresh epoch with three L1 records. That failure remains retained.
The subsequent dedicated-worker bootstrap J passed a separate, unchanged fresh
workload: 3,606 L2 records, 3,600 replays, 1,200 A/A pairs and all 19 S3 hashes.
See the [bootstrap report](../../../../taskq/deploy/eks-karpenter/diagnostics/validation/20261023-l2-domain/REPORT.md).
J did not perform another CRIU restore. The recovery controller now routes its
fresh epoch through the same `cpu-l2-spot` submission helper; computation restore
continues on ordinary `cpu-spot` workers. Keep these evidence scopes distinct.

The longer periodic B test later recovered 3,606 computations / 3,600 replays
but failed its fresh epoch with three L1 records. That failure remains retained.
The subsequent dedicated-worker bootstrap J passed a separate, unchanged fresh
workload: 3,606 L2 records, 3,600 replays, 1,200 A/A pairs and all 19 S3 hashes.
See the [bootstrap report](../../../../taskq/deploy/eks-karpenter/diagnostics/validation/20261023-l2-domain/REPORT.md).
J did not perform another CRIU restore. The recovery controller now routes its
fresh epoch through the same `cpu-l2-spot` submission helper; computation restore
continues on ordinary `cpu-spot` workers. Keep these evidence scopes distinct.
