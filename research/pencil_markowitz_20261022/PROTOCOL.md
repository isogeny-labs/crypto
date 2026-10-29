# Frozen exact sparse-pivot follow-up, 2026-10-22 07:21 UTC

The source input is the immutable 21-curve, 1,260-target archive
`research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`
(SHA-256 `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`).
The primary diagnostic is the original 32-bit p=186,379 target 0, workload
`Wa0fc1e72d7a0`. The independently replayed peeling control reached the
25,000,000 stored-coefficient cap with a null candidate scalar and ratio;
its raw file has SHA-256
`1a1e59e22ea23c1824f57bcf626c7270c685039732ad31d164e6f576df616873`.

The candidate changes only the exact sparse-elimination pivot priority.
The same transfer, point-driven pencil sieve, accepted relation checks,
walk-label seed, target, and same-process source-negation-rho control remain
in the `pencil-sparse` binary. At the end of each completed walk, degree-one
columns still remove their sole incident row. In the remaining live matrix,
choose a column by the minimum tuple
`((degree-1)*(shortest_incident_row_length-1), degree,
shortest_incident_row_length, column_index)`. Choose its shortest incident
row, breaking ties by row index. Row length includes factor-base, target,
and known-generator coefficients. Pivot normalization and row elimination
remain exact over the archived prime subgroup order. Refresh the dynamic
priority after each touched column changes; stale heap entries are skipped.
Every row-length inspection and heap event is charged to logical index work.
The implementation must not use a probabilistic rank shortcut or drop an
accepted relation beyond the proved degree-one rule.

First run all 60 frozen p=739 targets with this fixed rule, a 10,000,000
coefficient cap and 3,600-second external wall limit. Independently check
every recovered scalar on the source curve and exact non-wall replay. If
the small gate fails, preserve that failure and stop before the 32-bit run.
Otherwise run p=186,379 target 0 with `PENCIL_SOLVER=markowitz`, identity
node order, a 25,000,000 live-coefficient cap, 8 GiB virtual-memory limit,
and 3,600-second external wall limit. Preserve every process exit and
source-verified rho result. Repeat the exact 32-bit run in an independent
process for non-wall equality. A candidate/rho cost ratio is defined only
for a recovered, source-verified candidate scalar; otherwise it is null.

Use the native operation-price grid `S/M in {0,0.5,1}`,
`I/M in {1,10,100,1000}`, logical native work `{0,0.1,1}` GAE; the
representative cell is `M+0.5S+10I` and 0.1 GAE per native operation.
Use the native p186379 source-negation-rho from the same process. Run under
the recorded binary digest; if p186379 solves, only then extend the fixed
method to the other frozen targets, retaining failed targets and paired
rho controls. Do not extrapolate a one-target solve or failure to a 60-target
curve slice or the full 1,260-target cohort.

Python may render PDF and graphs from native result records; collection,
solving, source verification, replay checks, pricing, and result summaries
must be native Rust. Wall time is L0 diagnostic on this shared host.
