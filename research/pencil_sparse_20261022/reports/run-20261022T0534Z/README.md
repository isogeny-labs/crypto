# Nine paired 60-target pencil curve slices

The [frozen protocol](../../PROTOCOL.md) specified one degree-peeling
candidate arm for the eight remaining small pilot curves after the p=739
gate. The exact same archived workload IDs, source transfer, verified
relations, and same-process source-negation-rho controls were used. Every
candidate and rho scalar in these nine slices was checked by multiplication
on the source elliptic curve.

The exact eight-curve command set is [run_peel_cohorts.sh](../../run_peel_cohorts.sh),
executed from the repository root. The p=739 gate command is in the
[earlier run record](../run-20261022T0529Z/README.md). The peel executable
SHA-256 was
`539144816c83d765da6ea5731f34b5a2d3a8bd75519d02721342f56c797ce6a8`;
the frozen input archive SHA-256 was
`550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.

The [PDF](RUN_REPORT.pdf), [full-band graph](peel-nine-curves.svg), and
[native-price graph](peel-nine-native.svg) display the
representative full cold-cost ratios and all-cell ranges. Native
`../../results/p*-peel-vs-online.csv` files retain all 36 price cells,
including the 12/36 per curve where peel costs more than online and the
36/36 per curve where it costs more than rho. The precise raw operations and
candidate/rho values are in `../../results/p*-peel.jsonl`, alongside the
original frozen pilot `f*-cohort.jsonl` records. The `../../results/price-grid.csv`
is one row per target and cell; `../../results/validation.json` records the
completed independent source, replay, rho-control, and price checks. Wall
times are L0 diagnostics because concurrent jobs shared the
host.

The p=739 candidate was an earlier gate reused in this nine-curve graph.
The run does not add nine times 60 new distinct source targets to the
original pilot; it reevaluates the same 540 targets with a second solver.
