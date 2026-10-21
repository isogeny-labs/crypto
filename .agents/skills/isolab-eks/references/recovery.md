# Checkpoint and resume admission

The owner selected CRIU process checkpoints plus EBS volume snapshots, with
computation resumed and a fresh benchmark epoch after interruption. The current
deployed EKS v1 job path uses temporary `/work` storage and does not automate
this policy. Its normal Job retry is not resume.

An operator-controlled EKS test on 2026-10-21 did restore a UID-1000 Rust SHA-256
process from CRIU 4.2.1 and an encrypted EBS CSI snapshot onto a new c8a.2xlarge
Spot instance in another availability zone. It continued from iteration 48,128
through 1,048,576, with one process start and native replay of every recorded
hash. Read the [tested procedure and evidence](../../../../taskq/deploy/eks-karpenter/resume/README.md).
This qualifies that fixture and configuration, not arbitrary commands, ordinary
MCP/GitHub jobs, or unattended Spot recovery. Track the remaining integration in
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

Use the live recovery request/status schema once implemented. Do not add an
unrecognized `resume` field to `isolab.eks.request/v1`; its validator deliberately
rejects unknown fields. A requested resumable experiment must fail admission when
its checkpoint/restore capability is missing, not silently run without recovery.
