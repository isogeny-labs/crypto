#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= Per-curve cost of rho and index calculus across isogenous curves, with randomised curve order

Phase 5 of `research/fp31_whole_field_census_20261111`, 2026-11-12. Protocol `phase5/PROTOCOL.md`, frozen before data.
Card `CARD-fp31-phase5-percurve-20261112-1`, run 04:22–08:10 UTC. Every claim here is *[pcc:unverified]*.

== What ran

- *Population.* The 48 prime-order classes, with every isogenous curve plus 2 isomorphic models per class.
- *Design.* Each ecbench session lists a class's curves in a seeded random order with fresh `planted_shared` targets (the
  same $k$ on every curve). This removes the order confound of phase 4.
- *Sessions per class.* Rho: 8 sessions × 8 targets. IC: 4 × 2 for each of $m = 3$ and $m = 2$.
- *Arms.* For each method: main, an A/A copy and a positive control.
- *Completed sessions.* 727 of 768.
- *Refused sessions.* 41 were refused at start because the host was not quiet (CPU pressure above 5), so they took no
  data. 23 of these fell in 04:22–04:26 while this session committed and pushed; that cluster removes class c008 from
  rho and $m = 3$. The other 18 are scattered while the session was idle. Refused sessions drop whole blocks, so the
  complete-block analysis is not biased.

== Results (frozen tests; family α = 0.05/3 for T1)

#table(columns: (auto, auto, auto, auto), stroke: 0.4pt,
  [], [*rho* (ns per GAE)], [*IC m = 3* (ns per attempt)], [*IC m = 2* (ns per attempt)],
  [classes / measured runs], [47 / 77,088], [47 / 9,972], [48 / 10,236],
  [T1 KS of within-class Friedman p], [*7.7 × 10⁻⁷*], [0.48], [0.62],
  [T2 classes with p < 0.05/48], [*1*], [0], [0],
  [between-curve SD (log), upper 95 %], [0.91 %, 1.06 %], [0.74 %, 1.36 %], [0 %, 1.55 %],
  [median / max curve max ÷ min], [1.052 / 1.103], [1.084 / 1.171], [1.142 / 1.304],
  [S1: P(iso pair closer than other pair)], [0.47], [0.47], [0.50],
  [A/A ratio], [0.9996 ± 0.0007], [0.9993 ± 0.0018], [1.0085 ± 0.0035],
  [positive control], [1.099 (rho.negation)], [1.073 per attempt], [0.865 per relation],
  [frozen verdict (§6)], [*effect suspected*], [null supported], [null supported],
)

The IC max/min ratios are larger than rho's because there are fewer runs per curve (8 against 64), so curve means are
noisier. The variance-component estimates, which correct for this, are at or below 1.4 % (upper 95 %).

#figure(image("figures/phase5_summary.svg", width: 100%),
  caption: [Within-class Friedman p-values per method (uniform under the null, red line), and the isomorphic-model test S1.
  Source: `phase5/analysis_{rho,ic3,ic2}.json`.])

== Reading

- *Index calculus.* Decomposition cost per attempt does not depend on the curve at either $m$. Phase 3 found the same for
  the relation yield. For index calculus, no isogenous curve is cheaper than another: not in relations per attempt, not
  in time per attempt.
- *Rho.* With the curve order randomised, a per-curve spread of about 0.9 % SD in ns per group operation remains, and it is
  statistically clear (KS p 7.7 × 10⁻⁷). But it does *not* follow the curve: an isomorphic model of a curve (the same
  group, a $u$-scaled equation) differs from its base as much as an unrelated curve does (S1 = 0.47, at the chance value
  0.5; the preregistered bar for "intrinsic" is 0.75). So nothing here suggests that some isogenous curve has cheaper rho
  arithmetic. The spread belongs to the particular model and coordinates. The step formula, an affine addition, uses
  neither $a$ nor $b$, and this study did not identify the mechanism.
- *Decision (frozen rule).* Rho: effect suspected, not intrinsic to the curve. Any statement that a particular model is
  cheaper needs a preregistered replication on fresh seeds: re-measure each class's fastest and slowest model, with the
  prediction that the order holds.
