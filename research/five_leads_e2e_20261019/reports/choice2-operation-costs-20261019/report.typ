#set page(paper: "a4", margin: 20mm)
#set text(font: "Liberation Sans", size: 10pt)
= Genus-2 two-choice collector: frozen operation costs

*Evidence:* 21 exact-order curves, 60 targets per curve, seven sizes from 16 to 40 bits; 1,260 matched one-target workloads. The candidate, incumbent affine collector and source negation rho all recovered and source-verified every scalar. All 6,300 records in the five-arm archive replayed with zero audit problems. The native algorithm and frozen inputs are linked from the study `REPORT.md`.

At the representative price M+0.5S+10I and 0.1M per declared native logical operation, two-choice/incumbent is *1.000857×* and two-choice/source-rho is *4.483247×* pooled. The candidate processed 221,270,948 divisors against 223,343,235 for the incumbent. Across the full 12×3 price/native grid, two-choice/incumbent ranges from 0.989481× to 1.006720×. The candidate has both wins and losses to the incumbent and no rho crossover.

#figure(image("choice2-operation-cost-ratios.svg", width: 100%), caption: [Recorded pooled and per-size two-choice operation-cost ratios on the original 1,260 targets. The dashed line marks equal cost.])

*Source of truth:* `research/five_leads_e2e_20261019/results/genus2-choice2-price-grid.csv`, `results/genus2-choice2-summary.json`, `sessions.tar.zst`, and `replay/`. This is L0 operation-count evidence on a busy shared host. Wall time and external implementation speed are unqualified. The report's original TeX source remains retained; this Typst rendering supplies the inspected PDF.
