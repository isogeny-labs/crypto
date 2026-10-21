# Checkpoint and resume admission

The current deployed EKS v1 job path uses temporary `/work` storage and has no
verified CRIU restore contract. The standalone Spot host's CRIU recovery in
`infra/spot-system/CRIU_LOCAL.md` is a different execution path. Until the EKS
recovery implementation has passed a replacement-worker restore, describe EKS
process resume as unavailable and preserve that requirement as open. Do not
claim retries resume work or silently substitute the standalone host.

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

Use the live recovery request/status schema once implemented. Do not add an
unrecognized `resume` field to `isolab.eks.request/v1`; its validator deliberately
rejects unknown fields. A requested resumable experiment must fail admission when
its checkpoint/restore capability is missing, not silently run without recovery.
