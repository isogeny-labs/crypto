# Recovery of running legacy Spot processes

- **Status:** investigating
- **Kind:** implementation/validation gap; explicit export authorization pending
- **Affected work:** [Spot runtime](../infra/spot-system/README.md), [native checkpoints](../infra/spot-system/NATIVE_CHECKPOINTS.md)
- **Next action owner:** Spot deployment task
- **Last updated:** 2026-10-18

## Problem

The five already-running factor-base processes predate native checkpoint
support. Their CRIU images exist locally, but off-host export and an actual
legacy-process restore have not yet been verified. Native recovery tests do
not discharge this requirement.

## Evidence and uncertainty

- **Observed:** CRIU 4.2.1 successfully dumped PIDs 6656, 6657, 6665, 6696 and
  6704 with `--leave-running`; all five continued. Measured freezes and source
  identities are in [the redacted report](../infra/spot-system/validation/live-criu-snapshots.json).
- **Observed:** a credential-free disposable counter was dumped, killed and
  restored on the same host. Packaged CRIU 3.17.1 had failed with `sys_rseq -22`.
- **Observed:** new native solver checkpoints survived a real worker termination
  and restore on its automatic replacement with the reference answer unchanged.
- **Observed:** automatic approval review blocked export of full memory images
  to S3 because they may contain inherited credentials. Explicit permission for
  that destination has been requested; images remain root-private locally.
- **Missing capability:** validated CPU/kernel/backing-file compatibility and
  coordinated automatic restore of the old live-program images. A dump alone
  does not prove a usable cross-host recovery point.
- **Not established:** transparent migration of arbitrary experiments, clock
  accounting across boots, exactly-once outputs, or safe CPU-family switching
  for CPU/process images. No such claim is made.

## What would unblock it

- [ ] Resolve explicit authorization for private S3 memory-image export.
- [ ] Verify image integrity and all required backing files on an isolated
  compatible worker, without replacing the original running process.
- [ ] Restore a copy, verify CPU progress/thread health and isolate its outputs;
  stop only that validation copy after the check.
- [ ] Validate fenced ownership and original-host failure detection before
  enabling automatic legacy restore or moving the fixed IP.

## Next discriminating step

Use a private snapshot of one known factor-base PID for a compatible-worker
restore probe once export authorization is resolved. Failure should identify
the exact missing resource or CPU/kernel condition; success permits testing
the remaining four. Keep every original process running throughout validation.
Current legacy snapshots are not registered as native supervisor jobs.

## Resolution and history

Unresolved. The original host remains outside the managed Auto Scaling group,
and fixed-IP cutover stays disabled. The local producer refreshes only the
explicitly registered processes and never restores, restarts or terminates them.
