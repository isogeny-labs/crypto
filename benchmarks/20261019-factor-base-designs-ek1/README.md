# Factor-base design close-out and E-K1

This entry makes the measured stage diagnostics easier to browse. The research
folders remain the source of truth for raw records, protocols, code, and
analysis. Report PDFs and plot assets are copied into this benchmark bundle so
they render directly in the benchmark browser; the links below also point to
their canonical research copies.

**Scope:** field-operation counts for factor-base design and solver-stage
experiments. These are stage diagnostics: no pipeline cost (S), end-to-end
comparison, or speedup is reported. The recorded wall times are descriptive
and are not admitted performance evidence.

## Prime-field slope close-out

The close-out review adds no run; it re-scores committed results using
(kappa = Deltalog_2(\text{field_ops})/Delta b) against the
(m-1) break-even. Every measured series with at least three completed cells
is cost-unfavourable at the measured sizes. A0 at (m=4) remains inconclusive
with two cells. No result is extrapolated beyond the measured cells.

- [Research source and raw records](../../research/factor_base_designs_20261015/README.md)
- [Review report PDF](assets/prime-field-review.pdf) · [canonical copy](../../research/factor_base_designs_20261015/reports/review-20261019T1632Z/REVIEW.pdf)
- [κ versus break-even graph (SVG)](assets/prime-field-kappa.svg) · [PNG](assets/prime-field-kappa.png) · [canonical source](../../research/factor_base_designs_20261015/reports/review-20261019T1632Z/kappa_vs_breakeven.svg)

![Prime-field κ versus break-even](assets/prime-field-kappa.png)

## E-K1 Koblitz analogue stage run

The registered E-K1 run records 306 rows; 250 of 250 completed systems
reproduced their traces. All 14 series with at least three completed cells
are cost-unfavourable under the registered (kappa) score. The (S) arm at
(m=3) is inconclusive at both sizes (two cells); the positive control is
detected and the shape null reads mixed. Twenty-two rows are no-plant
placeholders, not completed systems. The result is limited to
(n\in\{11,23\}) and (m\in\{2,3\}).

- [Run report PDF](assets/ek1-report.pdf) · [canonical copy](../../research/factor_base_designs_20261015/reports/run-20261019T1712Z/REPORT.pdf)
- [Raw rows and exact replay analysis](../../research/factor_base_designs_20261015/runs/)
- [Field operations versus base size (SVG)](assets/ek1-field-ops.svg) · [PNG](assets/ek1-field-ops.png)
- [Solving degree versus base size (SVG)](assets/ek1-degree.svg) · [PNG](assets/ek1-degree.png)

![E-K1 field operations versus base size](assets/ek1-field-ops.png)

![E-K1 solving degree versus base size](assets/ek1-degree.png)

## A11 pipeline-pricing feasibility

No measurement was made. The feasibility note concludes that the existing
prime-curve ecbench oracles cannot consume the A11 tower systems, so neither
arm has an ecbench (S) result.

- [Feasibility report](../../research/a11_pipeline_pricing_20261019/FEASIBILITY.md)
- [Run report PDF](assets/a11-report.pdf) · [canonical copy](../../research/a11_pipeline_pricing_20261019/reports/run-20261019T1634Z/REPORT.pdf)
- [Graph (SVG)](assets/a11-graph.svg) · [PNG](assets/a11-graph.png) · [canonical source](../../research/a11_pipeline_pricing_20261019/reports/run-20261019T1634Z/graph.svg)

![A11 existing diagnostic graph](assets/a11-graph.png)
