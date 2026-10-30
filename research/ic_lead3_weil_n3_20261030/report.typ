#set page(margin: 1.6cm)
#set text(size: 9.5pt)
= Lead 3: index calculus on cubic-extension curves (Weil descent, n = 3)
Branch ic-lead3-weil-n3-20261030. Instrument: repo `gaudry_cubic` (unmodified) + `fit_n3.rs`; counted unit = affine additions + Fp muls/63; rho pooled S = 1.336 ± 0.104 (30 instances). Measured values; derived values are fits (OLS, bootstrap 95%).

*Literature.* Gaudry–Diem decomposes R into n factor-base points (x in Fq), Weil-restricted Semaev S(n+1). With large primes the cost is O~(q with exponent 2-2/n) (Joux–Vitse 2013 §2.1; Diem, Math. Comp. 80 (2011), Thm 3 per a search snippet; Gaudry JSC 2009 not read first-hand). At n = 3: q with exponent 4/3 (n with exponent 4/9) against rho at n with exponent 1/2. Joux–Vitse m = n-1: q squared, worse than rho for n ≤ 4.

#figure(image("fig_n3_cost.svg", width: 100%), caption: [Left: counted cost / pooled rho, per instance (points) and mean (lines). Right: phases vs rho. p = 271..8209 (log2 n 24..39), 5 seeds (arm P), 3 seeds (MitM, m=2 to p = 2083; m=2 at p = 2083 censored at the 1e6-residual cap, excluded). Source: results.json.])

#table(columns: 6, align: (left,right,right,right,right,right),
[arm],[p],[log2 n],[residuals],[ops/residual],[cost / rho],
[2LP],[271],[24.2],[1502],[1.40e4],[3535],
[2LP],[8209],[39.0],[107863],[1.49e4],[1618],
[dense LA],[8209],[39.0],[23077],[1.49e4],[397],
[Wiedemann],[8209],[39.0],[25095],[1.49e4],[398],
[MitM m=3],[2083],[33.1],[5591],[6.4e4],[≈2.9e3],
[m=2 (JV)],[1039],[30.1],[523580],[1.1e2],[≈1.3e3])

*Fitted exponents in n (p ≤ 8209):* 2LP total 0.425 [0.416, 0.434], relation phase 0.425, LA phase 0.505 [0.482, 0.525]; full-base relation 0.333, dense LA 0.894; MitM 0.697; theory 4/9, 1/3, 2/3.

*Projection (derived, 140–190 bit extrapolation, indicative):* 2LP two-phase model bottoms at ≈1.5× rho near 2\^226 and never goes below 1; theory exponents give a crossover near 2\^230; full-base arms bottom at ≈200–340× rho near 2\^42–2\^50. Pricing unpriced hash probes at 0/0.1/1 GAE does not change the order.

*Verdict.* The n = 3 exponent beats rho only for m = 3 with double large primes, and the measured cost is 1.6e3× rho at 2\^39, falling slowly; no reachable crossover. Replicates and extends RESEARCH_RESIDUAL_WALKS §11.15; no new claim.
