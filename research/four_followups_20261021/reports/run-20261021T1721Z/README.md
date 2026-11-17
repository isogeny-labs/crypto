# Two additional pencil curve cohorts, 2026-10-21 17:21 UTC

The point-driven `pencil-full` binary ran the two remaining frozen 16-bit
Scholten models, p=1,151 and p=1,187, with 60 separate targets each. All
120 candidate and 120 same-process source-negation-rho scalars passed source
multiplication. A fresh process reproduced every non-wall record; the native
validator independently checked all 120 scalar pairs against the archived
targets and recomputed their representative costs. Across the resulting nine
complete models, it also checked all 324 price-grid cells from raw rows.

At `M+0.5S+10I` and 0.1 GAE per native logical operation, complete cold
pencil/rho ratios were 28.666756 at p=1,151 and 31.080921 at p=1,187.
The 36-cell per-model prices and phase counters are in the
[raw cohort files and grid](../../results/pencil-pilot/). The full small-size
pilot is now nine models and 540/1,260 target pairs; 720 targets remain.

Each run used `target/release/pencil-full --cohort 16 P` from the repository
root, followed by an independent identical archive-input invocation and
`cargo run --release --locked --bin validate-pencil --` on all nine complete
files. The [figure source](plot_pencil_pilot.py) uses the native
[summary](../../results/pencil-pilot/summary.json); the
[SVG](pencil-pilot-cost.svg) and [PDF](report.pdf) show all nine completed
curves. The host was an Intel Xeon 6975P-C with 32 visible CPUs; `/usr/bin/time`
diagnostics are retained in the study, but cost claims use complete operation
counts at L0 only. One deterministic seed per target was used; no confidence
interval was measured.
