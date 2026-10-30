# 28-bit pencil single-target replay, begun 2026-10-21 17:06 UTC

The first frozen target for the p=48,383 model was solved by the point-driven
pencil collector and same-process source-negation rho, with both answers
checked by source-curve multiplication. The pencil recorded 23,639 verified
three-point relations, 20,814,919,424 modular row updates, rank 23,638 and
9,658,483 stored nonzero coefficients. A fresh archive-input process started
at 17:06 UTC and matched every non-wall field in the two-row setup/target
record. The separate native validator recomputed both scalars against the
archived source target, five summary values and all 36 pencil/rho pricing cells.

At `M+0.5S+10I` and 0.1 GAE per native logical operation, the complete
setup-charged pencil cost is 6,245,360,741.028572 GAE; matched rho is
64,413.628571 GAE; pencil/rho is 96,957.132823. Across the 36 cells, 0/36
are below rho, with ratios from 26.422722 to 113,636.290374. The
[36-cell grid](../../results/pencil-pilot/f48383-one-target-price-grid.csv)
and [raw JSONL](../../results/pencil-pilot/f48383-one-target.jsonl) retain
all counts. The [validator report](../../results/pencil-pilot/f48383-one-target-validator-report.json)
and [replay certificate](../../results/pencil-pilot/f48383-one-target-replay-certificate.json)
record the independent checks. The first and replay L0 wall diagnostics were
1,723.90 and 1,668.79 seconds, and neither enters the operation-count prices.

The command from the repository root was
`PENCIL_MAX_TARGETS=1 research/four_followups_20261021/target/release/pencil-full --cohort 28 48383`
for both runs, followed by `validate-pencil --single` on the primary file.
The host was an Intel Xeon 6975P-C with 32 visible CPUs. The
[figure source](plot_one_target.py), [36-cell graph](pencil-one-target-grid.svg)
and [PDF](report.pdf) document this one target. It is separate from the nine
complete smaller-curve slices and is not a 60-target 28-bit estimate or a
full-cohort result.
