# run-20261022T0529Z: p=739 degree-peeling paired gate

The exact offline peeling and minimum-degree rule was frozen in
[PROTOCOL.md](../../PROTOCOL.md) before execution. The command was
`PENCIL_SOLVER=peel PENCIL_NODE_ORDER=identity pencil-sparse --cohort 16 739`.
The baseline is the independently replayed 60-target identity-order online
collector on the identical workload IDs and seeds. The same-process rho
control also ran on every target. The source archive SHA-256 was
`550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
The exact peel-enabled binary SHA-256 was
`539144816c83d765da6ea5731f34b5a2d3a8bd75519d02721342f56c797ce6a8`.
The host was `ip-172-31-21-97.us-east-2.compute.internal` (x86_64). Wall
time is L0 diagnostic because independent work ran concurrently.

All 60 candidate, online and rho scalars were checked by the source curve.
A fresh archive-input candidate rerun matched all 61 setup/target records
after removing wall fields. The native validator recomputed the complete
candidate and rho prices and all 36 paired comparison cells. The [raw
candidate](../../results/p739-peel.jsonl), [online control](../../results/p739-identity.jsonl),
[paired grid](../../results/p739-peel-vs-online.csv), [validator](../../results/validation.json),
[graph](peel-price-grid.svg) and [PDF](RUN_REPORT.pdf) are the review sources.

At `M+0.5S+10I` and 0.1 GAE per native logical operation, complete cold
candidate/control/rho costs were 3,881,713.942857 / 4,321,445.680952 /
334,188.080952 GAE over the 60 one-target solves. Candidate/control was
0.898244 and candidate/rho 11.615357. Across the full 36-cell band,
candidate/control ranged from 0.873217 to 1.041611; 24/36 candidate
cells were lower than the online control and 0/36 below rho. The graph shows
the recorded band without extrapolation.
