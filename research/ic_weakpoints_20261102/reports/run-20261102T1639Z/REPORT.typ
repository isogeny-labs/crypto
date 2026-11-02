#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Six weak points of EC index calculus, measured on Weil-descent systems

*Run* 20261102T1639Z (frozen arms A2, A3, Y, P) and Amendment-1 arm Y2 · *tool* `research/ic_weakpoints_20261102/tool` (Rust, std only), commits `1a572f0607` (arms) and `97b0965fa7` (diagnostics, summaries) · *protocol* `PROTOCOL.md` frozen before data · *host* one 4-vCPU cloud container, no isolation (exact counts only, no timing) · *seeds* base 20261102 (Y2: 20261103), derived per arm.

*Scope.* Gaudry/Diem descent: $E: y^2 = x^3 + a x + b$ over $FF_(p^n)$, random $a, b$, factor base ${P : x(P) in FF_p}$, $n in {2, 3}$. Not an ecbench session, not a leaderboard row, not admitted by the validation gate.

== Outcomes
#table(columns: (auto, 1fr), inset: 4pt, stroke: 0.4pt,
  [*Row 1* solving degree], [$n=3$: degree 10 in 88/88 systems for $p = 31 dots 2^30$; Macaulay matrix $252 times 286$, $c(3) = 1.03$–$1.07 times 10^6$ $FF_p$ mults (dense upper bound). $n=2$: degree 3, 87 mults. Flat in $p$.],
  [*Row 5* geometry], [Zero-dimensional 176/176, quotient dimension 64 = Bézout (87/88). $p <= 127$: 32 rational solutions, 5 genuine (equal to the independent point-arithmetic count), 27 non-split (84 % spurious), 2 singular, 0 positive-dimensional.],
  [*Row 3* yield], [mean/$lambda$ = 0.88–1.19 over 20 curves; dispersion CI contains 1 in 19/20; heavy tails only from subfield targets $x_R in FF_p$ (replayed: 25 decompositions, all $R + P - P$).],
  [*Row 4* rank], [log Q correct 15/15; rank shortfall 0–3 (one curve 6); cofactor and duplicate-row controls pass 15/15; sign-drop bug detected 12/15.],
  [*Row 6* cost], [IC/rho (negation-map reference) $w=0$: 0.29–3.4; $w=0.1$: $2.8 times 10^4$–$3.2 times 10^5$; $w=1$: $1.5 times 10^5$–$1.7 times 10^6$. Rho ahead 7, unresolved 8, IC ahead 0.],
  [*FB size / LP*], [One-large-prime yield matches $3 phi^2 (1 - phi)$ (0.93–1.04); full-relation deficit in Y (0.85 at $phi = 1/2$) not reproduced in Y2 (1.08), recorded as noise.],
  [*Derived*], [No large primes: best $phi$ gives IC/rho plateau 684 ($w = 0.1$), 3761 ($w = 1$) for groups $>= 2^60$, since both scale as $p^(3/2)$.],
)

#figure(image("fig1_solving_degree.svg", width: 100%), caption: [Rows 1 and 5. Solving degree, quotient dimension and dense Macaulay cost at the solving degree against $log_2 p$; 8 random curves and targets per $p$; band = min–max. Run 20261102T1639Z, `data/algebra_n{2,3}.jsonl`.])

#figure(image("fig2_yield_dispersion.svg", width: 100%), caption: [Row 3. Dispersion index and mean/$lambda$ per curve with 95 % bootstrap CIs (1000 resamples, computed natively); 2000 (Y) or 6000 (Y2) targets per curve. The two wide intervals come from single subfield targets. `data/yield*.jsonl`.])

#figure(image("fig3_phi.svg", width: 62%), caption: [Rows 2/7. Observed over exact hypergeometric expectation for relations inside the first $phi$ fraction of the factor base, pooled over $p in {251, 509}$, Poisson 95 % CI. Y: 6 curves × 2000 targets; Y2: 8 curves × 6000.])

#figure(image("fig4_cost_band.svg", width: 100%), caption: [Row 6. Left: measured complete solves (15 curves, log Q verified), IC cost = walk additions + $w$ (trials × $c(3)$ + linear algebra), against analytic negation-map rho with comparisons priced at $w$. Right: derived model from the measured constants; not a measurement.])

== Limits
Dense Macaulay elimination overstates $c(n)$ relative to F4; the decomposition oracle used for relation finding is point arithmetic (its additions are excluded and replaced by the Gröbner price); $n = 4$ and double large primes were not measured; toy sizes only; one host, no isolation, which is immaterial for exact counts but rules out any timing statement.
