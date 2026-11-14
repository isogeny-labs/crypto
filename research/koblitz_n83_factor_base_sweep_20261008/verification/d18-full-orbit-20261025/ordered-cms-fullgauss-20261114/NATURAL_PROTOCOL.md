# D18 first natural query (frozen 2026-11-14 UTC)

The full-Gauss constructed positive solved in 2.04 seconds. Its complete
assignment passed every original XCNF clause and XOR row and an independent
curve, subgroup, and relation-row replay. This releases one first natural
query on the same primary N83 `a=0` curve and D18 base.

Use the public target from `pilot-01/probe-corpus.json`, fixed as
`[1834913121178197242856238, 8797964190732846213981377]`. The exact
source commit, worker binary, D18 object, corpus, export mode, and caps are
in `NATURAL_LOCK.json`. Construct a six-file source snapshot directly from
that Git commit. The source-bound worker verifies its compiled bytes against
this snapshot. Export the natural XCNF with no solver search and a 300-second
internal cap plus 310-second process-group watchdog. Keep the command,
stdout/stderr, receipt, phase log, all output and hashes, and failure status.
The export is an L0 shared-host diagnostic; require three 15-second host
samples with one-minute load at most 24, unchanged swapouts, and at least
2 GiB scratch.

After a successful export, pin the natural XCNF SHA-256, exact clause/XOR
counts, and worker receipt in a committed second-stage lock **before**
solver execution. Run the frozen full-Gauss solver flags from the lock with
one million conflicts, 600 solver seconds and 660-second outer watchdog.
Preserve every result. A SAT assignment must pass the original XCNF and
independent field/curve, subgroup and relation-row replay before it counts
as one natural relation. One relation does not establish natural yield,
rank, subgroup logs, unseen-target descent, or an end-to-end factor-base
choice on either N83 curve.

The first natural-solver host sample failed the load gate before launch.
Retain that file unchanged and pass a fresh, uniquely named sample file with
`--host-preflight-file` on a later retry. The runner records its name and hash
in the solver receipt; the load, swapout, scratch and solver caps are unchanged.
