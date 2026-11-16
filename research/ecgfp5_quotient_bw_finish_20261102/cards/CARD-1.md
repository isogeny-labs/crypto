# CARD-1: run the three frozen cells (mechanical, no judgement)

Protocol: `PROTOCOL.md`, frozen at commit 3d3f68d70. Do not edit any file in this directory
except the outputs that `run_cell.sh` writes under `results/`.

1. `cd research/ecgfp5_quotient_bw_finish_20261102` (inside the crypto checkout).
2. Check that `bin/hnfsolve` and `bin/bwfinish` exist. If they don't, stop and report.
3. Check that no `msolve`, `hnfsolve` or `bwfinish` process is running (`pgrep -af`). If one
   is, stop and report.
4. Launch exactly this, once, detached:
   `nohup sh -c 'for p in 4111 262231 33554371; do sh run_cell.sh $p; done' > results/run_all.out 2>&1 < /dev/null &`
5. Report the PID and the first line of `results/m5_p4111/started_at.txt` after it appears.
   Do not wait for the cells to finish, and do not interpret anything.
