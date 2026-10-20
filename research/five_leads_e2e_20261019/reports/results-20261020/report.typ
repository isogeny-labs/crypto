#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9.5pt)
= Five ECDLP follow-ups: verified operation evidence

*Evidence date:* 2026-10-20. *Pricing:* M+0.5S+10I; 0.1M per declared native logical operation except the cubic arithmetic gate, which has no native charge. All ratios below are operation counts, not qualified wall speed.

== Two complete genus-2 cohorts

The two-choice collector and the projective numerator chart each ran on all 1,260 original exact-order source targets (21 curves, 60 targets each, 16–40 bits). For the projective chart, all 3,780 arm records were source-verified and independently replayed. At the stated price, projective/affine is *0.642714×* and projective/source-rho is *2.881445×* pooled. The full 12×3 price grid and per-curve rows are in `results/genus2-projective-price-grid.csv`.

#figure(image("projective-cost-ratios.svg", width: 100%), caption: [Complete paired operation-cost ratios by order size; 180 targets at each size.])

== Mixed fibers: pair table and polynomial all-roots solvers

All 540 pair-table candidate records on nine eligible prime-order curves were source-verified; native reanalysis retained every price and curve. Homogeneous, mixed and matched-random pooled ratios to rho are 3.6435×, 3.2222× and 3.7180×.

The Semaev-S4 exhaustive-polynomial solver also recovered all 540 scalars on the same 180 targets and checked every answer on the source. Homogeneous, mixed and matched-random pooled costs are 57.8840×, 32.2975× and 34.4846× rho. All 36 pooled price cells lose to rho and the corresponding pair-table arm.

The native F4 solver additionally returned exact rational root sets on 2,290 nonidentity DLP queries; 26 identity right-hand sides needed no polynomial system. Its replay recovered and source-verified all 540 scalars. Counting F4 row-reduction multiplications and collector costs gives homogeneous/mixed/random *160899×/232110×/239616×* rho as lower bounds; all 36 pooled cells lose. Symbolic and substitution operations are uncounted, and external msolve remains unavailable.

#figure(image("mixed-fiber-cost-ratios.svg", width: 100%), caption: [Complete DLP cost ratios; 180 exact-order targets per arm.])

#figure(image("mixed-fiber-polynomial-cost-ratios.svg", width: 76%), caption: [Complete exhaustive Semaev-S4 all-roots DLP costs; 180 exact-order targets per arm.])

#figure(image("mixed-fiber-f4-lower-bound.svg", width: 76%), caption: [F4-backed DLP lower bounds from counted row-reduction field multiplications; 180 targets per arm.])

== Cubic transfer and odd-kernel escape: unresolved attacks

For Tian's published F523 cubic fixture, the 3-kernel and smooth quartic were checked. Native quartic Jacobian arithmetic verifies T=1987S and rS=0 from published classes; five optimized cubic-field source-rho seeds recover the source scalar. One quartic doubling costs 14.043× mean source rho in this implementation. A bounded linear search received the published S,T without their scalar, recovered 1987 and source-verified it; its cold cost is 51979.77× mean rho. A genuine line-sieve index calculus on those published classes also recovered 1987, verified both rational-divisor relations in the Jacobian, and source-verified the answer. At M+0.5S+10I+0.1A it costs 561.59× mean source rho and loses in all 36 price cells. The source-to-Jacobian map was not executed. The 20 Smith-negative genus-3 holdouts were materialized, but odd kernels and end-to-end attacks on them remain untested.

#figure(image("tian-arithmetic-gate.svg", width: 76%), caption: [Arithmetic diagnostic only; one quartic doubling and a known-scalar multiplication divided by the mean of five complete source-rho solves.])

#figure(image("tian-linear-dlp.svg", width: 76%), caption: [Cold cost of one verified DLP on published destination classes; no source-to-Jacobian map or index calculus was run.])

#figure(image("tian-line-sieve.svg", width: 76%), caption: [Cold cost of published-class line-sieve index calculus, including all relation and descent attempts. Source map absent; operation pricing includes modular additions.])

*Evidence:* `REPORT.md`, `results/`, `correctness/`, `sessions_projective_chart_v2.tar.zst`, and the seven linked `reports/run-*` PDFs. Negative outcomes and unexecuted routes remain explicit in the report. Literature: #link("https://arxiv.org/html/2012.07173")[Tian's cubic transfer] and #link("https://arxiv.org/html/2007.03172")[Tian's genus-3 odd-isogeny construction].
