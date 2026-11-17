#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)

= Rho step cost across isogenous curves of $FF_(2^31 - 1)$: a curve-level effect, confounded with execution order

Phase 4 of `research/fp31_whole_field_census_20261111`, 2026-11-12. Protocol `phase4/PROTOCOL.md`, frozen before data
(sha256 `cd2984ea…`). Card `CARD-fp31-phase4-rhostep-20261112-1`. The director assessed the execution as _impeded_:
45 of 48 classes completed. Every claim here is *[pcc:unverified]*.

== What ran

- *Population.* 48 prime-order classes, each with its isogenous curves plus 2 isomorphic models.
- *Design.* The same planted $k$ on every curve (`planted_shared`); 64 targets × 2 rounds per curve.
- *Arms.* `rho.plain`, a `rho.plain` A/A arm, and `rho.negation` as the positive control.
- *Metric.* Wall ns per counted group operation (GAE).
- *Volume.* 150,144 measured runs, every answer correct. 12,283 rows were graded below L2.
- *Missing classes.* c046, c047 and c048 were refused at start: the host was not quiet because of a concurrent build in this
  session. The rows of c052 were graded L0 because shell and Python processes of this session ran on the reserved CPU.

== Frozen result: effect suspected (PROTOCOL §6)

- *T1.* Within-class Friedman p-values across curves are not uniform: KS p = 3.5 × 10⁻¹⁵. 10 of the 45 classes are below
  0.05/45.
- *Size.* The between-curve SD of log ns/GAE is 0.95 %, with a 95 % upper bound of 1.07 %. The per-class max/min ratio has
  median 1.038 and maximum 1.081.
- *Controls.* The A/A arm reads 1.0009 ± 0.0005. The positive control (`rho.negation`) reads 1.094, so the detector works.

== Why the effect is not evidence of cheaper arithmetic (exploratory diagnostics, after the frozen analysis)

- *Execution order.* ecbench runs workloads in spec order within each round; `order: shuffle` permutes only the arms. Each
  curve's runs therefore fill one contiguous time window (9 curve switches in 1,920 runs of class c039). The curve
  is confounded with time.
- *Persistence.* Per-curve speed persists in the A/A arm (mean rank correlation 0.64, 43/45 positive) and in the negation arm
  (0.59, 43/45). The A/A arm replays the identical walks (same algorithm seed and the same GAE count in all 1,280 pairs).
  The negation arm takes different walks.
- *Isomorphic models.* An isomorphic model (the same group, other $a, b$) differs from its base curve as much as unrelated
  curves do: median |Δ log| 1.2 % against 1.3 %; probability that the iso pair is closer 0.49. The effect is not a property
  of the curve or its class. Popcount and bit length of $a$ and $b$ show no association (p ≥ 0.22).
- *No monotone drift.* Speed against position in the session has mean rank correlation −0.10 (19/45 positive), and the
  decile profile is flat within ±0.5 %.
- *Code.* `rho.plain`'s step is the affine addition $lambda = (y_2 - y_1)(x_2 - x_1)^(-1)$, which uses neither $a$ nor $b$.

#figure(image("figures/phase4_diagnostics.svg", width: 100%),
  caption: [Left: within-class Friedman p-values. Middle: per-curve speed agrees across the A/A and negation arms. Right: no
  monotone trend with session position. Source: `phase4/analysis.json`, `diag_*.json`.])

== Reading

Phase 4 cannot separate a representation-dependent cost (the specific coordinates) from non-monotone host fluctuation
over each curve's time window. It is *inconclusive* on the question asked. What it bounds: whatever the cause, curves
differ by about 1 % SD (at most about 8 % max/min) in measured ns per group operation, and the difference does not follow
the isomorphism class. The required follow-up (PROTOCOL §6) is a preregistered run that interleaves curves: several short
sessions per class, each listing the curves in a seeded random order with fresh targets, blocked on (session, target), with
no other process on the host.
