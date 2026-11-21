# Restart note: the container restart during attempt 1 (2026-11-12)

This file is bound by no card and adds no protocol value. `PROTOCOL.md` (sha256 `572f7dfb…`) is unchanged.

## What happened

- Card `CARD-fp31-phase3-icyield-20261112-1` started its detached step `run-phase3` at 2026-11-12T01:04:39Z (pid 3127).
- The session's container was restarted while the step was running. `tools/task_card.py wait` then reported
  `status: lost at command 1` (exit 5). The loss is recorded in
  `benchmarks/losses/20261112-card-fp31-phase3-icyield-20261112-1-01-run-phase3.json`, cause `container_restart`.
- Before the restart, 37 of the 96 cells had finished. All were m = 3 (c008 to c044 in classes.json order), each
  printed `ok`, and none printed `FAILED`. Each finished cell is a complete `runs/<id>_m3/` directory with its
  `done` marker.

## How attempt 2 continues

- The cell running at the restart, `c045_m3`, was moved unread to `runs/_interrupted/c045_m3-attempt1/`. It is
  not pooled, analysed or cited.
- The card's stdout at the time of the loss is kept as `cards/attempt1-run-phase3.stdout.snapshot`.
- Attempt 2, card `CARD-fp31-phase3-icyield-20261112-2`, runs the same `run_phase3.sh` with the same binaries,
  input and arguments, all bound by the same hashes. The script skips every cell that has a `done` marker, so it
  reruns c045_m3 from scratch, then finishes the remaining m = 3 and all m = 2 cells.
- Every cell's workload, targets and seeds are fixed by the frozen spec arguments. A cell's counts therefore do
  not depend on the attempt in which it ran.
- No result was read before this note: only the per-cell `ok` lines, used to count finished cells.
