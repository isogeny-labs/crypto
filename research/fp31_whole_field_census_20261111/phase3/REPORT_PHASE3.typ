#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= Index-calculus relation yield across 48 isogeny classes of $FF_(2^31 - 1)$: no class or curve effect

Phase 3 of `research/fp31_whole_field_census_20261111`, 2026-11-12.

- *Protocol:* `phase3/PROTOCOL.md`, frozen before data at commit `58e501e346`, sha256 `572f7dfb…eeea`.
- *Instrument:* the P2 instrument of `research/isogeny_class_ic_yield_20261102`, unchanged: `p2tool spec` → `ecbench run` →
  `ecbench verify --replay 4` → `p2tool analyze`. The cross-class tests are computed by `fp31-class-census ic-analyze`.
- *Execution:* task cards compiled by the experiment director and run by the mechanical executor. Attempt 1 was lost to a
  container restart after 37 of 96 cells. The loss is in `benchmarks/losses/` and described in `phase3/RESTART_NOTE.md`.
  Attempt 2 resumed with identical bound inputs, and every cell's workload and seeds are fixed.
- *Host:* 4-vCPU cloud container. The route is local, recorded in PROTOCOL §7, and ecbench graded the rows L1/L2.

Classification: a null-hypothesis characterization. It leaves the index-calculus baseline unchanged and adds no leaderboard
row. No claim here has a pcc claim id: all are *[pcc:unverified]*.

== What ran

- *Population.* The 48 prime-order classes of phase 2, 16 in each endomorphism-ring stratum (S1: one ring, S2: two,
  S3: three or more). Each class contributes every curve its phase-2 isogeny walk reached: 332 curves, each with the walk's
  instance point $P$ as generator. `ic-classes` checked $P$ on the curve, $n P = O$ and the $j$-invariant. Each class also has
  2 seeded $u$-scaled isomorphic models.
- *Measurement.* For every class and $m in {3, 2}$, the arms are `ic.pipeline`, `prime-abscissa` factor base, `mitm:m` oracle.
  The factor base has the same size on every curve: $|F| = 378$ for $m = 3$ and $|F| = 1448$ for $m = 2$, the table cap,
  identical in every class. Each curve gets 2 rounds × 2 targets of a fixed number of decomposition attempts, plus a planted
  arm with factor base $|F| dot 1.2^(1 slash m)$, whose expected yield is +20 %.
- *Totals.* 96 cells, 0 failed. For $m = 3$: 260,773 relations in 7,921,136 attempts. For $m = 2$: 265,338 relations in
  136,016,848 attempts. Every count is exact.
- *Metric.* Each curve's yield per attempt ÷ the predicted yield ($(4 slash 3) N^3 slash n$ for $m = 3$, $2 N^2 slash n$ for $m = 2$).

== Result: null supported for both $m$ (frozen rule, PROTOCOL §6; α = 0.05/14 = 0.0036)

#table(columns: (auto, auto, auto), stroke: 0.4pt,
  [*test (48 classes)*], [*m = 3: statistic, p*], [*m = 2: statistic, p*],
  [T1 Kruskal–Wallis across classes, per-curve yield ÷ predicted], [H 44.4, df 47; 0.58], [H 43.0, df 47; 0.64],
  [T2 Spearman class yield against $log_2 H$ (class size)], [ρ +0.091; 0.54], [ρ +0.144; 0.33],
  [T3 … against $log_2 f$ (Frobenius conductor)], [ρ +0.024; 0.87], [ρ +0.029; 0.84],
  [T4 … against number of endomorphism rings], [ρ +0.005; 0.97], [ρ −0.015; 0.92],
  [T5 … against $log_2 |D_K|$], [ρ +0.041; 0.78], [ρ +0.117; 0.43],
  [T6 within-class ICC p-values uniform (42 classes with ≥ 2 curves)], [KS D 0.090; 0.87], [KS D 0.137; 0.38],
  [T7 within-class binomial over-dispersion (pooled Pearson χ²)], [χ² 331.5, df 284; 0.028], [χ² 296.9, df 284; 0.29],
)

- *Thresholds.* Every test is above α. The smallest p is T7 at $m = 3$ (0.028); it is not significant at the frozen α and is
  reported as recorded.
- *Positive control.* The planted +20 % arm reads 1.198 ($m = 3$) and 1.200 ($m = 2$), averaged over the 42 classes with
  at least 2 curves. Both are inside the frozen window [1.12, 1.28]. p2tool's per-class detection rates are 0.89 and 0.92.
- *Spread.* Class means of yield ÷ predicted span 0.938–1.032 for $m = 3$ (mean 0.982, between-class SD 0.019) and
  0.944–1.023 for $m = 2$ (mean 0.997, SD 0.018).
- *Strata.* The stratum means are S1 0.986, S2 0.979, S3 0.982 for $m = 3$, and 0.992, 1.002, 0.998 for $m = 2$.
- *Isomorphic models.* The RMS log ratio is 0.048 and 0.044. This is presentation noise: a $u$-scaling changes which points have
  small abscissa.

#figure(image("figures/class_yield.svg", width: 100%),
  caption: [Pooled relation yield ÷ prediction per class, ±2 binomial SE, both $m$. Recorded values: `phase3/analysis_m{3,2}.json`, `classes`.])

#figure(image("figures/yield_vs_H.svg", width: 100%),
  caption: [Class yield against class size (T2). There is no association at either $m$.])

#figure(image("figures/controls_within.svg", width: 100%),
  caption: [Left: the planted +20 % arm per class, against the expectation 1.2 and the frozen window. Middle and right: per-class
  within-class ICC permutation p-values, which are uniform under the null.])

== Reading and limits

- Across 48 prime-order classes of $FF_(2^31 - 1)$, the yield of the prime-abscissa decomposition at $m = 2$ and $m = 3$ is
  matched by its prediction from $|F|$ and $n$ alone. It does not depend on class size, conductor, number of endomorphism
  rings or $|D_K|$. Inside a class, the curves along the isogeny walk do not differ. This extends P2's within-class null
  (2 classes at 24–32 bits) to 48 classes and every ring structure at 31 bits.
- Combined with phases 1 and 2: over this field, neither rho nor this index-calculus decomposition can tell isogenous curves
  apart, and neither depends on the class beyond $n$.
- *Limits.*
  - One factor-base family (smallest abscissa) and one oracle (`mitm`), at $m ≤ 3$.
  - About 800 relations per curve, so per-curve resolution is about 3.5 % and class resolution about 1.3–2 %.
  - 42 of the 48 classes have at least 2 curves; S1 walks are horizontal only.
  - Isolation L1/L2 on a shared host. Counts are exact; wall time is unused.
  - The raw sessions are hashed in the benchmark record but not uploaded to S3, because this session has no AWS identity.
