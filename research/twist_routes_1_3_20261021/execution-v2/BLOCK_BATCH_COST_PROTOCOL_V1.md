# E26 follow-up: one worker per candidate instead of per helper branch

The separate-worker instruction probe is preserved. Its first fixed toy case
records 5,784,840 native guest instructions for eight helper workers versus
2,423,541 for the direct worker, despite fewer row multiplications. Test
whether eliminating repeated startup changes that comparison. This is a
follow-up motivated by training evidence, not a new heldout result.

Reuse the same p=59, B=8, m=2 target-label-0 systems. Run all seven native
encodings and the direct A/A repeat through the same new batch-worker
interface. Deserialize a list of the existing branch jobs, run the unchanged
F4 kernel sequentially, and serialize all resulting bases. GM=1, grevlex,
degree bound 32. No solution, branch or failed process can be omitted.

Require every branch basis and row-multiplication count to equal its old
independently certified output. The shared branch budget is 250 seconds;
outer process cap 300 seconds, global checkpoint cap 900 seconds, 4 GiB,
one process/thread at a time. Repeat the identical direct input; retain the
5% A/A spread gate. Freeze source and this protocol before measurement.

Use the same Valgrind/Callgrind package, Ir event and cache/branch settings as
the separate-worker protocol. The interval is one process startup, batch
input parsing, every symbolic/F4 solve and batch output. Encoder construction,
parent orchestration, certification, sign/root lifting, kernel work and DLP
recovery remain excluded. Record branch counts separately from process
counts. Stock msolve is retained in the separate-worker reference; no batched
msolve library adapter or fastest-external claim is supplied here.

Recheck raw Ir counters independently. Native projected-row ranks remain
separate from affine solver costs. No candidate selection or new target is
generated. Whether the worker-stage result improves, ties or loses, full E27
accounting, broader replication and E28 final labels remain open.
