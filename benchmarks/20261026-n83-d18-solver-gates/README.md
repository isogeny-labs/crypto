# N83 full-D18 solver gates, 2026-10-26

Browsing copies of the two run reports and combined result. Source of truth:
[ordered run](../../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261025T2339Z-ordered-d18/REPORT.md),
[cadence-32 run](../../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261026T0121Z-cadence32/REPORT.md), and
[combined result](../../research/koblitz_n83_factor_base_sweep_20261008/reports/d18-solver-gates-20261026/RESULT.md).

| Report | PDF | Graph SVG | Graph PNG, 150 dpi |
|:--|:--|:--|:--|
| Ordered first-pair | [ordered-RUN.pdf](ordered-RUN.pdf) | [ordered-diagram.svg](ordered-diagram.svg) | [ordered-diagram.png](ordered-diagram.png) |
| Native XOR cadence 32 | [cadence32-RUN.pdf](cadence32-RUN.pdf) | [cadence32-diagram.svg](cadence32-diagram.svg) | [cadence32-diagram.png](cadence32-diagram.png) |
| Combined solver gates | [RESULT.pdf](RESULT.pdf) | [result-diagram.svg](result-diagram.svg) | [result-diagram.png](result-diagram.png) |

Ordered D18 constructed control: 66.994 s import and 1.077 s search, rank one and independently replayed. Its natural arm was censored at the 2,400 s worker wall cap. The separate cadence-32 attempt had 303.192 s supervisor wall and censored before model import on a saturated shared host. Each is n=1, L0 exploratory; they are not paired timing arms. Cold IC and matched rho runtime are missing, so no speedup ratio or factor-base ordering is reported.
