# Managed owner: reconcile before continuing

The registered service name is `twist-collision-certificate-panel-v1.service`.
Launch is pending at this registration checkpoint. A native result or live
launch receipt, rather than this plan, establishes execution.

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
