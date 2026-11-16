#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= Pollard rho across 57 isogeny classes of $FF_(2^31 - 1)$: cost depends on $r$ alone

Phase 2 of `research/fp31_whole_field_census_20261111`, 2026-11-11. Protocol `phase2/PROTOCOL.md`, frozen before data:
sha256 `3d81d9fe…19c7ba`, commit `53adef0ef5`. Task card `CARD-fp31-phase2-rho-20261111-1`: compiled by the experiment director,
run by the mechanical executor, receipt in `phase2/cards/`. Instrument `isoclass-rho`, binary sha256 `4f089730…7424d3`,
selftest 0 failures. Host: 4-vCPU cloud container at isolation L0. The route deviation is recorded in PROTOCOL §7.
The metric is the exact iteration count, so wall time is descriptive only. Classification: a null-hypothesis characterization,
rho against rho. No claim here has a pcc claim id: all are *[pcc:unverified]*.

== What ran

- *Selection.* 400,000 curves were drawn from seed 20261111, and their orders found by BSGS. The frozen rule picked 57 classes in draw order:
  - 16 prime-order classes with one endomorphism ring (S1), 16 with two (S2) and 16 with three or more (S3);
  - 8 classes with cofactor 4–64 (R, the $sqrt(r)$-scaling control);
  - the $j = 0$ class with $r = 165,188,041$ (J0, automorphisms of order 6).

  The sample also checks phase 1: 2.657 % of the drawn curves have prime order, against the census prediction of 2.638 % (0.75 SE).
- *Per class.* An isogeny walk with $ell in {2, 3, 5, 7}$ from the root curve, with every edge checked by the tool, carries one ECDLP instance.
  Rho then ran 200 times per curve on up to 16 curves of the class, plus 2 isomorphic models, an A/A replica, a planted negation fold
  and a planted 4-adding walk. A generator sweep ran 40 times per generator. In total: 394 curves, 136,000 class-mode rho runs
  and 145,240 generator rho runs, all 57 classes complete (0 failed), 1,000 s.
- *Metric.* $c = "iters" slash sqrt(pi r slash 2)$ per solve, where $r$ is the prime subgroup rho runs in.

== Result: null supported (frozen decision rule, PROTOCOL §6)

#table(columns: (auto, auto, auto), stroke: 0.4pt,
  [*test (48 main classes; Bonferroni α = 0.05/7 = 0.0071)*], [*statistic*], [*p*],
  [T1 Kruskal–Wallis of $c$ across classes], [H = 61.5, df 47], [0.076],
  [T2 Spearman of class mean $c$ against $log_2 H$ (class size)], [ρ = −0.111], [0.45],
  [T3 … against $log_2 f$ (Frobenius conductor)], [ρ = −0.039], [0.79],
  [T4 … against number of endomorphism rings], [ρ = −0.032], [0.83],
  [T5 … against $log_2 |D_K|$], [ρ = +0.032], [0.83],
  [T6 within-class KW p-values uniform (45 classes with ≥ 2 curves)], [KS D = 0.158], [0.20],
  [T7 generator KW p-values uniform (48 classes)], [KS D = 0.152], [0.20],
)

Every test is above α, and the detector control (planted negation fold) reads 0.741, inside the frozen window [0.65, 0.76].
Class means of $c$ span 0.970–1.087 (mean 1.026). Their between-class SD is 0.0222, against 0.0178 expected from sampling noise
alone; T1 does not find that difference significant. Stratum means: S1 1.029, S2 1.024, S3 1.025, R 1.030.

#figure(image("figures/class_means.svg", width: 100%),
  caption: [Mean $c$ per class with ±2 SE, all 57 classes. Recorded values: `phase2/analysis.json`, `classes`.])

#figure(image("figures/invariants.svg", width: 100%),
  caption: [Class mean $c$ against class size and number of endomorphism rings (48 prime-order classes). No association (T2, T4).])

== Controls

#figure(image("figures/controls.svg", width: 100%),
  caption: [Pooled mean ratios to each class's root curve, ±2 SE, with theoretical expectations in red. `analysis.json`, `controls`.])

- *Detector.* The planted negation fold reads 0.741 ± 0.005 against the ideal $1 slash sqrt(2) = 0.707$. The excess matches the earlier
  readings of this tool (0.732 in `research/isogeny-class-rho-n45`) and is the fruitless-cycle overhead of the negation map.
- *Null controls.* The A/A replica reads 1.003 ± 0.007 and the isomorphic models 0.998 ± 0.006.
- *$sqrt(r)$ scaling.* The R classes against the main classes, on normalized $c$, read 1.009 ± 0.005. Raw cost therefore scales as $sqrt(r)$.
- *j = 0 automorphism fold.* It reads 0.449 ± 0.024 against $1 slash sqrt(6) = 0.408$, from 200 rho runs on one class.
- *Deviation from a stated expectation.* The planted 4-adding walk reads 1.317 ± 0.009. PROTOCOL §5 wrote "about 1.12 (Teske)",
  but earlier runs of the same tool read 1.27–1.36 (`research/isogeny-class-rho`) and 1.291 (`-n45`). The arm is not in the decision rule.
  The §5 expectation was mis-stated; the reading agrees with every earlier measurement.

#figure(image("figures/kw_pvalues.svg", width: 100%),
  caption: [Per-class Kruskal–Wallis p-values across the curves of a class (left) and across generators (right). Under the null they are uniform (red line).])

== Reading and limits

- Across 48 prime-order classes, class size, conductor, number of endomorphism rings and $|D_K|$ do not predict rho cost.
  Neither does the curve within a class, at any volcano level, nor the generator. This holds at a resolution of about 1 % per class
  (class-mean SE 0.9 %). The census's one cross-class effect, the Lenstra bias, changes $r$. It does not change the cost at a fixed $r$.
- Combined with phase 1: over $FF_(2^31 - 1)$, the generic ECDLP cost of a curve is fixed by the factorization of $\#E$ and by $"Aut"(E)$.
  The isogeny class, and the curve inside it, do not move it. This is the theorem's prediction, now measured at whole-field scale.
- *Limits.*
  - The decision rests on 48 classes out of 8,646 prime-order classes, with 200 rho runs per curve.
  - The S1 walks reach 1–4 curves, because a class with one endomorphism ring has only horizontal neighbours.
  - The rho runs of different arms may share seeds. The earlier reviewers' shared-seed covariance concern applies to this tool too.
  - Rho only; no index-calculus arm. Within-class IC yield was measured in `research/isogeny_class_ic_yield_20261102`.
  - Isolation L0, and the raw data are not uploaded to S3: there was no AWS identity in this session.
- This leaves the index-calculus baseline unchanged and adds no leaderboard or `benchmarks/README.md` row.
