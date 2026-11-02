# Managed owner: reconcile before continuing

The service `twist-collision-certificate-panel-v1.service` accepted its one
launch with exit 0 at 2026-10-25T02:52:22Z. Its observed main PID is
3540174, active/running, with `Restart=no`. The registration/source freeze
is commit `221e34d57e5e052e0c3c0320e1748019d35235c1`, before launch.
The copied runner matches the registered SHA-256. Both controller and
native descendants have been inspected in the actual unit cgroup.

- Frozen workload and code: [FREEZE.json](FREEZE.json).
- Raw output: `../runs/collision-certificate-panel-v1`.
- Actual owner output/launch receipts: `../results/collision-certificate-v1/launch/`.
- Queued commands and exits: raw `metadata/`; actual launch UTC, epoch,
  launcher PID and outer budget are recorded after reservation acquisition.
- Native durable pending/degree/result events: `cases/<id>/<first|aa>/systems.jsonl`.
- Terminal native summary: raw `analysis/summary.json`.
- Controller terminal marker: raw `metadata/terminal-exit`.

Never infer a stopped job from an observation timeout. Inspect the managed
unit, actual PID/cgroup, exact command, completed phase receipts and pending
native journals first. Never rerun a completed or interrupted phase with a
fresh deadline. The driver refuses an existing run/phase; its managed owner
has no restart loop. Preserve partial results and original limits.

Completion requires every registered case-arm, both producer executions,
independent audit and deterministic parity, including all failed cases.
A partial matrix/certificate diagnostic does not close the complete-cost
or broader twist-symmetry objective. Diem, the old cancelled waiting/freeze
lane, existing EKS jobs and the original 85-configuration panel remain
separate owners and are not restarted here.

At the 2026-10-25T03:10:18Z checkpoint, the first p37 phi case has first,
independent audit, repeat and parity exits 0. The paired doubling first
execution exits 0, with its remaining gate phases pending. These are
partial execution receipts; the 42-case experiment is not complete. Raw
model numbers remain diagnostic, and final full-panel reduction/report and
separate archive verification/publication remain pending.
