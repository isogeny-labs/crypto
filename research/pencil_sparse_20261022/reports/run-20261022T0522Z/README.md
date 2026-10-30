# run-20261022T0522Z: p=186,379 first-target cap diagnostic

This was added to the [frozen protocol](../../PROTOCOL.md) before execution.
The command was `PENCIL_NODE_ORDER=identity PENCIL_MAX_TARGETS=1
pencil-sparse --cohort 32 186379` with a 3,600-second external wall budget.
The binary was `40820a6179e903b2482c1eefc7c088222e0d1e1b3ce260a72aa6901c220d742b`
and the source archive was
`550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
The host was `ip-172-31-21-97.us-east-2.compute.internal` (x86_64), with
L0 wall-time isolation.

The one target produced 69,672 independently checked three-point relations
and rank 69,672, then reached 10,011,236 stored coefficients against the
10,000,000 cap. The exact modular row-update count was 1,233,576,833.
The candidate did not recover a scalar; the same-process rho did and passed
source multiplication. Candidate full cost and candidate/rho ratio are null.
The recorded process exit was 0 because this is an explicit solver failure
record, rather than a process crash. The [raw JSONL](../../results/p186379-identity-t0.jsonl)
and [PDF](RUN_REPORT.pdf) with its [graph](matrix-cap.svg) preserve the
observed stop condition. The source workload ID, algorithm seed, counters,
rho result and failure code are in the raw row. Wall time is diagnostic only.
