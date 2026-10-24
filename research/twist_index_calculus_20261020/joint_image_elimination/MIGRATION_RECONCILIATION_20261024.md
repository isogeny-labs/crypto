# Reconciliation after the 01:36 UTC app recovery

Observed again on 2026-10-24 at 01:41–01:43 UTC, before resuming work.

- The approved migration receipt remains
  `/srv/experiments/agent-migration/handoff-receipt.json`.
- `/run/spot-legacy/diem.json` reports `deadline-expired`, 50 durable records
  of 52, `pending=false`, and no launcher PID. Its original deadline is
  1792521741.993407. No generator was launched or restarted.
- EKS overview lists only terminal jobs. The study's capability probe
  `twist-joint-capability-20261024-v1` failed its scheduling deadline. No
  command ran in that probe; it is not a scientific failure. No EKS job was
  resubmitted and no role or node-pool policy was changed.
- Process inspection finds no native study runner, proof gate, msolve, or
  study orchestration process. Docker lists no running study container.
  Missing exit receipts are recorded as interruptions, not successful or
  failed mathematical results.
- The old source-side waiting freeze lane belongs to the retained migration
  history and has not been restarted by this study.

## Retained work and exact continuation

| Lane | Saved evidence | Reconciliation | Continuation |
|---|---|---|---|
| Historical 189 cases and 21 doubling controls | `../endomorphism_image_base/` | Complete, independently audited and priced | Do not rerun |
| Joint construction v1 | `runs/algebra-v1/`, `results/algebra-v1-validation.json` | 297 constructions and 694 system files validated | Reuse validation |
| Native global S5 regression | `runs/regression-v1/metadata/*.exit` | Six solves terminal: five allocation failures and one cooperative incomplete result | Do not rerun |
| Native source-block unions | Same metadata; three `*-orbit-union.started.txt` files | No exit receipt, solve directory, or surviving process | New recovery record, existing frozen runner and inputs only |
| Polynomial Lean gate v2 | `runs/lean-gate-v2/` | Interrupted; no gate verdict retained | Fresh receipt, unchanged theorem source, 900 s watchdog |
| Repair verification v7 | Scratch launcher log; no `runs/verification-v7/` directory | Guarded verification never produced its first run artifact | Execute once in a fresh v8 directory |
| Uniform-mass Lean gate v1 | `runs/uniform-lean-v1/` | Empty stdout/stderr, no verdict or process | Fresh receipt; preserve v1 |
| External regression v1 | Metadata plus first input/command | No basis, exit receipt, or surviving container | Fresh continuation; never infer a complete basis from an exit code |

All new scientific phases retain one thread, an 8 GiB address-space limit,
the repository's busy reservation, and at most the original 1,200 s phase
budget. Compilation remains separate. Recovery watchdogs may be shorter;
they do not replace the original outstanding experiments or establish a
negative mathematical result. Original scripts and partial artifacts remain
unchanged. No completed scientific phase is repeated.
