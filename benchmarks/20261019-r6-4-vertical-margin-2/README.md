# R6.4 — vertical margin, phase two

This directory is the browsing copy for the measured and projected cost study in [`research/isogeny_fibers_20261010/round6/r6_4_vertical_margin_2/`](../../research/isogeny_fibers_20261010/round6/r6_4_vertical_margin_2/). The research directory is the source of truth for raw JSON, text logs, analysis, plot source, and the complete report.

## Figures

- [Generic kernel-finding costs](fig1_generic_find.svg)
- [Projected ladder local exponents](fig2_local_exponent.svg)
- [Ascending/descending cost ratios](fig3_asc_desc_ratio.svg)
- [Extrapolated crossover table](fig4_crossover.svg)
- [Full report](report.pdf)

## Reading the result

The study uses toy/synthetic vertical instances at a 61-bit prime and reports counted arithmetic costs. It does not establish a sub-rho attack or a result for deployed curves. The measured ladder fit is exponent 2.582 [2.544, 2.620] over 20 primes, ℓ = 37–863. The projected NTT ladder fit above ℓ = 2^13 is 1.919 [1.724, 2.115], based on measured per-step costs multiplied by exact step counts; full kernel finding was not run at those sizes. That interval contains values below 2, so it is insufficient evidence of an exponent below 2 (not evidence of ≥ 2). The displayed crossover shifts are extrapolations. Additions are uncounted in every arm and in the rho reference, so any ranking of routes or crossovers holds only under the restricted cost model (M + W + 91 I); the complete-cost ordering is unresolved. On the ladder the ascending/descending paired ratio 1.001 [1.001, 1.002] excludes 1: a small difference was detected, and equality is not claimed.

The ψ route is capped at ℓ = 131. The Φ route's counted evaluation/root cost excludes generation of Φℓ mod p; the library generation wall-time fit is ℓ^4.6, so the Φ result is only a lower bound. NTT word-product cost W = 1 M is assumed, not measured. Wall times were contended and are not used to claim a speedup.

Reproduce the figures from the committed records with `python research/isogeny_fibers_20261010/round6/r6_4_vertical_margin_2/plot.py`.
