#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show link: underline

= H2 settled: genus-2 IC on glued h = 1 instances against negation rho

*Run* `run-20261019T1730Z` (decision run, seed tag 1), `run-20261019T1700Z` (run 1, failed control, retained). Protocol: `PROTOCOL.md` plus Amendment 1. Harness: `examples/h2_glue_cold.rs` (fork of #2170's `n2_routes.rs`, blob `df0f097b…`). Host: shared 4-core cloud VM, no PMU; wall times are descriptive only.

== Answer (measured)

With every cost charged cold and all 320 answers verified, glued genus-2 IC at $h = 1$ is *not cheaper than negation rho in any of the 12 price cells at any native price*. The ratio IC/rho ranges 1.00 to 2.20 (figure 1). By the frozen rule, 6 cells are _rho-wins_, 6 are _unresolved_ (only because at $nu = 0$, free native work, the ratio is 0.996 to 1.08 with an interval containing 1) and 0 are _IC-wins_. The frozen rule's branch is therefore "scope the 22M + I addition", not "close H2". The scoping (scoping section) shows that this addition would not change the verdict outside $nu = 0$.

== Design (frozen before runs)

16 glued instances with exact $h = 1$ (8 at $log_2 N approx 36$, 8 at 40), 20 targets each = 320 targets, all arms on every target. Arms: J (#2170 `arm_glue`, double-large-prime IC, 41M + I per Jacobian addition on its sextic model), R (negation rho, default DP), RM (equal-memory rho: DP density chosen by a fixed memory rule), plus controls Rp (independent-seed A/A), Rr (identical-seed replay), Pl (plain rho, positive control). J is charged construction, both transfers, the sqrt-table build ($p$ native ops), one Legendre test per degree-2 divisor priced at $I$, and all failed work. Prices: $sigma = S\/M in {0, 0.5, 1}$, $iota = I\/M in {1, 10, 100, 1000}$; native ops at $nu in {0, 0.1, 1}$ GAE each. Ratios are sums over matched targets; intervals are two-level (instance, target) bootstrap, 4000 resamples.

== Controls (run 2)

#table(columns: (auto, 1fr), stroke: 0.4pt,
[Exact replay (Rr = R)], [identical counters on all 320 targets],
[Replay pass], [192 re-run rows identical in steps, M, S, I, native, words, Legendre],
[Independent A/A Rp/R], [1.0225 to 1.0228 over 36 combinations; all 36 intervals contain 1],
[Positive control Pl/R], [1.394 to 1.410; all intervals above 1],
[Rho calibration], [steps / (0.886 √ℓ) = 1.06 (36 bit), 1.02 (40 bit)],
[Verification], [0 unverified rows of 4,032 (incl. replays)],
[Wall A/A], [Rp/R seconds = 1.083: wall not qualified, none claimed],
)
Run 1 (160 targets) passed all of these except my own point tolerance on the independent A/A (0.938, interval contains 1; tolerance $|r-1| < 0.03$ was tighter than the sampling error of 160 rho walks). Under the frozen rule that run decides nothing; Amendment 1 replaced the tolerance by interval containment and fixed a fresh-seed replicate before it ran. Run 1's cell ratios agree with run 2 within 0.06.

== Result: cost ratio J / comparator

#figure(image("figures/ratio_grid.svg", width: 100%), caption: [J / cheaper-of(R, RM) in each of 12 price cells, three native prices; ratio [95% interval]. Source: `results/run-20261019T1730Z/analysis.json`.])

The backend's measured cost per Jacobian addition is $34 M + 7 S + 1 I$ (= 41 M + I at $S = M$), against $9 M + 4 S + I$ for an $E(F_(p^2))$ addition (= 12 M + I at $S = 0.75 M$), identical on all 16 instances. J takes 0.44 to 0.45 √ℓ steps (rho 0.91 to 0.94 √ℓ): the count advantage is real (about 2.0×) but each J step is dearer: all charged work per step is 38.6 M + 8.3 S + 1.04 I + 7.4 native ops, against 9.2 M + 4.1 S + 1.02 I + 2.0 native for rho. Distance to the matched rho at $nu = 0.1$: 1.20 (at $iota = 1000$) to 1.99 (at $iota = 1$, $sigma = 0$).

== Time × memory

#figure(image("figures/time_memory.svg", width: 100%), caption: [Steps/√ℓ against stored 64-bit words/p, per target (dots) and mean (markers). J stores 1.5 p words; negation rho at DP density $2^(-d)$, from 0.001 p to 3.5 p words, stays at 0.88 to 1.0 √ℓ steps.])

Memory does not buy rho speed beyond removing DP overhead, so rho at J's memory (RM, $d^* = 2$ on all 16 instances, 0.89 p words) is about as cheap as default rho and J's Θ(p) memory is a cost with no counterpart: J holds about 100 to 200× more words than default rho for a 2× step advantage that its per-step cost erases. A memory-capped J degenerates into distinguished-point rho (#2170 H4); not rerun.

== Scoping the ≤ 22M + I addition (frozen rule's branch)

#figure(image("figures/breakeven.svg", width: 78%), caption: [Per-addition cost at which J would equal the comparator (other J costs measured). Bars at $nu = 1$ are negative.])

- At $nu = 0$ break-even is about 22 M + I (15 to 22 M-equivalents at $iota = 1$, i.e. #2170's figure), so #2170's number is the free-native break-even.
- At $nu = 0.1$ break-even is 10 to 65 M-equivalents, i.e. below 22 M + I in every cell (e.g. 14.6 at $iota = 10, sigma = 0$ against 32). Even a hypothetical 22 M + I addition leaves J at 1.20 to 1.51 × the comparator.
- At $nu = 1$ break-even is negative in every cell: J's native work (7.4 ops/step) alone exceeds rho's total cost; no addition formula can win.
- Hypothetical column (22 M + I, not built): rho-wins 3, unresolved 9, IC-wins 0.
- The Legendre pricing does not carry the result: pricing it at 0 or 30 M gives the same tally (6 / 0 / 6), although at $nu = 0.1$ the ratio dips to 0.81 under the Legendre = 0 variant in at least one cell; IC-wins stays 0 of 12.

*Not built.* The frozen rule said "scope and build". The data above show that the build target cannot flip any cell outside $nu = 0$, so building it would change no verdict; this is a judgement the owner can override. The literature bound for real-model genus-2 addition (Erickson–Jacobson–Stein 2011, ePrint 2011/105) was not retrieved here, so no claimed operation count for it is made.

== Verdict and limits

- H2 as stated (#2170: IC beats negation rho on h = 1 glued instances) is *not supported*: 0 of 12 cells favour IC; 6 are rho-wins, 6 are unresolved (ties at $nu = 0$ only). The rule's literal branch is "scope", done; the data say the next lever is native work and memory, not the addition formula.
- Next improvement to try: cut J's native work and Θ(p) memory (7.4 native ops and 1.6 p words per step), which dominates at $nu >= 0.1$; only then does the addition cost matter.
- Scope: 36 and 40 bit group sizes, one construction (#2170's gluing), cost-model counts (no hardware speed). #2229's ecbench genus adapter accepts only the Scholten family and a 70M + 3S + I backend, so its adapters were not run; this study re-implements #2229's protocol rules in a standalone driver. No ecbench session, leaderboard row or retired-instruction metric exists (no PMU).
- No improvement over the IC baseline is claimed (null result); leaderboard unchanged.
