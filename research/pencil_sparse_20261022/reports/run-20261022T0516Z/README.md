# run-20261022T0516Z: exact pencil order screen

The [protocol](../../PROTOCOL.md) predates these runs. The original binary
base was `f879244559b368a69e97e4f86988e5c0bdb7f896`; the screening binary
SHA-256 was `40820a6179e903b2482c1eefc7c088222e0d1e1b3ce260a72aa6901c220d742b`.
The source archive digest was
`550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
The host was `ip-172-31-21-97.us-east-2.compute.internal` (x86_64).
Isolation is L0; wall times are diagnostic only.

The nine commands have the form
`PENCIL_NODE_ORDER=ORDER pencil-sparse --cohort 16 P` for
`ORDER=identity,reverse,affine` and `P=739,1151,1187`. All 60 frozen
workloads per curve ran, one target per candidate/rho record. Their exact
workload IDs, seeds, setup and phase counts are in the
[raw files](../../results/). No target failed. All 540 candidate/rho
pairs source verified; the independent native validator checked 1,080
recovered scalars and all 19,440 price cells. Fresh archive-input reruns
matched all 549 setup/target records after wall fields were removed.

The [PDF](RUN_REPORT.pdf) and [graph](matrix-order.svg) display the native
[summary](../../results/summary.json). The complete per-target
[price grid](../../results/price-grid.csv) and [validation](../../results/validation.json)
support the numbers. Derived ratio in every cell is the candidate's cold GAE
divided by the matched rho GAE; no wall-time ratio is reported.
