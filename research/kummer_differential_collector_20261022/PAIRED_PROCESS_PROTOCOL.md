# Same-process rho follow-up

Recorded 2026-10-22 07:18 UTC while the full `paired_uncached` run was in
progress, after the single-target pilot and before its 40-bit candidate
slice or any full-cohort rho output. This document records the actual run
boundary; it is not retroactively described as a pre-run freeze.

The native `paired_uncached` executable runs the uncached differential Kummer
collector over the original 1,260 target IDs, writing into
`results/paired/uncached/`, then calls the existing
`five_leads_study::three_leads::solve_rho` negation solver over those exact
IDs in the same process. Rho uses each archived record's public curve spec,
algorithm seed and trial cap. It measures the source-addition unit natively,
adds the independent source-verification phase to solver counters, checks
`kP=Q`, and writes `results/paired/rho/rho-{bits}.jsonl`.

The paired analyzer must independently replay every source scalar over Fp²,
compare every paired candidate row exactly to the previously frozen uncached
row, check all rho source results, and price the paired rho and candidate
with the same 36 cells, including native charge 0, 0.1 and 1. Outputs go
under `results/paired/analysis/`. Archived-rho prices remain separate in
`results/` and must not be silently overwritten. A `VALIDATION.json` is
emitted only after the complete paired cohort and native two-pass headline
recompute succeed.
