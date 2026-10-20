#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9.5pt)
= Five ECDLP follow-ups: verified operation evidence

*Evidence date:* 2026-10-20. *Pricing:* M+0.5S+10I; 0.1M per declared native logical operation except the cubic arithmetic gate, which has no native charge. All ratios below are operation counts, not qualified wall speed.

== Two complete genus-2 cohorts

The two-choice collector and the projective numerator chart each ran on all 1,260 original exact-order source targets (21 curves, 60 targets each, 16–40 bits). For the projective chart, all 3,780 arm records were source-verified and independently replayed. At the stated price, projective/affine is *0.642714×* and projective/source-rho is *2.881445×* pooled. The full 12×3 price grid and per-curve rows are in `results/genus2-projective-price-grid.csv`.

#figure(image("projective-cost-ratios.svg", width: 100%), caption: [Complete paired operation-cost ratios by order size; 180 targets at each size.])

== Mixed fibers: end-to-end pair-table variant

All 540 candidate records on nine eligible prime-order curves were source-verified; native reanalysis retained every price and curve. Homogeneous, mixed and matched-random pooled ratios to rho are 3.6435×, 3.2222× and 3.7180×. This is a pair-table oracle solver; the proposed heterogeneous polynomial/F4 solver remains untested.

#figure(image("mixed-fiber-cost-ratios.svg", width: 100%), caption: [Complete DLP cost ratios; 180 exact-order targets per arm.])

== Cubic transfer and odd-kernel escape: unresolved attacks

For Tian's published F523 cubic fixture, the 3-kernel and smooth quartic were checked. Native quartic Jacobian arithmetic verifies T=1987S and rS=0 from published classes; five optimized cubic-field source-rho seeds recover the source scalar. One quartic doubling costs 14.043× mean source rho in this implementation. A bounded linear search received the published S,T without their scalar, recovered 1987 and source-verified it; its cold cost is 51979.77× mean rho. The source-to-Jacobian map and proposed genus-3 index calculus were not executed. The 20 Smith-negative genus-3 holdouts were materialized, but odd kernels and end-to-end attacks on them remain untested.

#figure(image("tian-arithmetic-gate.svg", width: 100%), caption: [Arithmetic diagnostic only; one quartic doubling and a known-scalar multiplication divided by the mean of five complete source-rho solves.])

#figure(image("tian-linear-dlp.svg", width: 100%), caption: [Cold cost of one verified DLP on published destination classes; no source-to-Jacobian map or index calculus was run.])

*Evidence:* `REPORT.md`, `results/`, `correctness/`, `sessions_projective_chart_v2.tar.zst`, and the seven linked `reports/run-*` PDFs. Negative outcomes and unexecuted routes remain explicit in the report. Literature: #link("https://arxiv.org/html/2012.07173")[Tian's cubic transfer] and #link("https://arxiv.org/html/2007.03172")[Tian's genus-3 odd-isogeny construction].
