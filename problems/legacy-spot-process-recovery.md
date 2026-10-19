# Recovery of running legacy Spot processes

- **Status:** investigating
- **Kind:** automatic recovery/ownership implementation and validation gap
- **Affected work:** [Spot runtime](../infra/spot-system/README.md), [native checkpoints](../infra/spot-system/NATIVE_CHECKPOINTS.md)
- **Next action owner:** Spot deployment task
- **Last updated:** 2026-10-19

## Problem

The five already-running factor-base processes predate native checkpoint
support. Their five current CRIU images have now been exported to authorized
private S3 and restored successfully as copies on a compatible separate worker.
Automatic fenced recovery and fixed-IP handoff remain unimplemented for those
legacy processes. Native recovery tests do not discharge this requirement.

## Evidence and uncertainty

- **Observed:** CRIU 4.2.1 successfully dumped PIDs 6656, 6657, 6665, 6696 and
  6704 with `--leave-running`; all five continued. Measured freezes and source
  identities are in [the redacted report](../infra/spot-system/validation/live-criu-snapshots.json).
- **Observed:** a credential-free disposable counter was dumped, killed and
  restored on the same host. Packaged CRIU 3.17.1 had failed with `sys_rseq -22`.
- **Observed:** new native solver checkpoints survived a real worker termination
  and restore on its automatic replacement with the reference answer unchanged.
- **Observed:** after automatic approval review initially blocked export, the
  user explicitly authorized these five memory snapshots in the private bucket.
  All five archives passed AES256, size, metadata and full-download SHA-256
  checks; [receipt](../infra/spot-system/validation/private-criu-export.json).
- **Observed:** all five restored on a separate `m8i.8xlarge`, with matching
  kernel/libraries and `--cpu-cap=all`. Each copy had two healthy threads and
  advancing CPU ticks during a 10-second check. Only copies were stopped;
  [evidence](../infra/spot-system/validation/legacy-criu-cross-host-restore.json).
- **Authorization gap:** review separately rejected recurring future uploads as
  outside the five-current-image grant. A separate recurring grant is pending;
  recurring export is disabled and local snapshots continue.
- **Missing capability:** source failure detection, fenced ownership and
  coordinated automatic restore of the old live-program images. A successful
  manual restore alone does not establish automatic recovery.
- **Not established:** transparent migration of arbitrary experiments, clock
  accounting across boots, exactly-once outputs, or safe CPU-family switching
  for CPU/process images. No such claim is made.

## What would unblock it

- [x] Resolve explicit authorization and verify private S3 export of these five images.
- [x] Verify image integrity and all required backing files on an isolated
  compatible worker, without replacing the original running process.
- [x] Restore all five copies, verify CPU progress/thread health and isolate outputs;
  stop only that validation copy after the check.
- [ ] Obtain separate authorization before enabling recurring future memory exports.
- [ ] Validate fenced ownership and original-host failure detection before
  enabling automatic legacy restore or moving the fixed IP.
- [ ] Validate monotonic-clock handling, original launcher deadlines and durable
  completion receipts before replaying saved processes automatically. The
  10-second CPU-progress probes do not establish correct timing accounting or
  prevent replay of a job that completed after its snapshot.

## Next discriminating step

Build and validate source-failure fencing and restoration ownership with
disposable replicas before enabling automatic legacy restore. Treat CPU/kernel
and backing-file equality as admission requirements; the successful m8i probes
do not establish cross-family memory-image portability. Preserve original
processes and IP throughout validation. Current legacy snapshots are not
registered as native supervisor jobs.

## Resolution and history

Unresolved. The original host remains outside the managed Auto Scaling group,
and fixed-IP cutover stays disabled. The local producer refreshes only the
explicitly registered processes and never restores, restarts or terminates them.
