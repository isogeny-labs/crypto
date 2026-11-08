#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#align(center)[#text(size: 15pt, weight: "bold")[P3: cross-trace covariate test of the split-prime statistic]
#v(3pt) research/split_prime_ecdlp_20261107/followups/p3_cross_trace · run-20261107T2220Z · 2026-11-08]

#block(inset: 8pt, stroke: 0.5pt, radius: 3pt)[*Outcome (frozen rule 2): null at the tested sizes* [pcc:unverified]. Across 2,000 random prime-order curves at each of $p_24$ = 16,777,213 and $p_28$ = 268,435,399, neither exact $m = 2$ decomposition coverage nor the measured rho cost depends on $z_691$ (split primes $ell <= 691$), the partial $L(1, chi)$ ratio, or the $psi_3$/$psi_5$ root counts: none of 14 tests is Bonferroni-significant ($alpha$ = 0.003125), positive controls informative (planted +2 % recovered, power $>= 0.995$). The split-prime seed is closed for the coordinate track at these sizes. The A/A control exposed a defect in this study's rho walk, which limits the rho half. Card `CARD-p3-cross-trace-20261107-2`: executed as specified (director).]

= Method
Protocol frozen 535d9da826 with Addendum A (merged in \#3657): $K$ = 2,000 curves per prime, $T$ = 256 rho targets per run, runs A and B with disjoint seeds, exact $m = 2$ coverage of the first $N$ prime-abscissa points ($N$ = 502 and 2,007), OLS of each outcome on each standardized covariate with $log n$ as nuisance; $psi_5$ dropped at $p_28$ ($p_28 equiv -1 mod 5$ makes it identically zero for prime order). Instrument `p3census` (Rust). Card 1 was killed by the executor shell's 30-minute background limit after its $p_24$ steps; card 2 ran detached. Replay check: card 1's $p_24$ curves, covariates, rho A, rho B and analysis files are byte-identical to card 2's (yield identical apart from wall time).

= Results
#table(columns: 6, stroke: 0.4pt, inset: 3pt, align: left,
  [*prime*], [*outcome*], [*$z_691$*], [*logL1*], [*$psi_3$*], [*$psi_5$*],
  [$p_24$], [rho/√n], [+0.17 [−0.37, +0.71]], [+0.37 [−0.17, +0.91]], [−0.07 [−0.62, +0.47]], [+0.70 [+0.16, +1.25]],
  [$p_24$], [coverage], [−0.002 [−0.003, 0.000]], [0.000 [±0.002]], [−0.001 [±0.002]], [0.000 [±0.002]],
  [$p_28$], [rho/√n], [+0.26 [−0.45, +0.97]], [+0.43 [−0.29, +1.15]], [+0.61 [−0.11, +1.34]], [not tested],
  [$p_28$], [coverage], [0.000], [0.000], [−0.001 [−0.001, 0.000]], [not tested],
)
Effects per covariate SD, % of the outcome mean, 95 % CI; none Bonferroni-significant. Three rho intervals reach past +1 %, so the rho half cannot exclude a +1 % effect everywhere; no covariate is significant and $>= 1$ % at both primes.

#figure(image("../figures/p3_summary.svg", width: 100%), caption: [(a) the 14 tests with the ±1 % decision line; (b) A/A: per-curve rho cost in runs A and B; (c) census $z_691$ with the published curves. Data from run-20261107T2220Z.])

= Instrument finding: the rho walk is defective
Runs A and B correlate at $r$ = 0.974 ($p_24$) and 0.986 ($p_28$); between-curve variance is 41 and 72 times the within-curve expectation. Per-curve cost is bimodal, about 1.6 or 2.6 · √n at $p_24$ and 2.7 or 5.1 · √n at $p_28$, while a sound negation-map walk costs about 0.9 · √n, flat in $n$. Outcome Y1 therefore measures this walk, not rho in general. The jump table is seeded per target; the clusters are not separated by trace parity, $t mod 4$, $n mod 3$, $n mod 4$ or $a mod 2$. Suspected ingredient: 32 small-coefficient jumps under the negation map. Problem record `problems/p3census-rho-walk-bimodal.md`. The coverage outcome is exact and rho-free.

= Descriptive (decides nothing)
Percentile of each published curve's $z_691$ in the census, $p_24$ / $p_28$: prime239v1 0.05 % / 0.00 % (below all 2,000 at $p_28$), secp224r1 4.00 % / 2.75 %, secp521r1 5.50 % / 5.45 %, secp384r1 13.95 % / 14.50 %, others 27 % to 54 %.

= Next steps
Re-run Y1 with the lab's `rho_negation` or a corrected walk under the same frozen covariates and seeds; diagnose the bimodality on 50 curves at $p <= 2^20$.
