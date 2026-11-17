---
title: "Empirical Relation-Yield Pruning for the IC Tournament"
subtitle: "Native analyzer, statistical gates, and synthetic controls"
author: "Isogeny Labs research tooling"
date: "2026-10-31"
geometry: margin=0.75in
fontsize: 10pt
colorlinks: true
linkcolor: blue
urlcolor: blue
---

# Outcome

The existing index-calculus tournament now has a native checkpoint analyzer for
empirical relation yield and factor-base success. It separates hit probability,
verified rows, independent rank, factor-base coverage, and full-rank episode
success, then permits hard pruning only under predeclared correctness,
structural, or conservative statistical gates.

The implementation is an experiment-selection tool. No new cloud performance
panel was run, and this work does **not** establish an end-to-end ECDLP speedup
or a crossover against Pollard rho.

# Measurement model

A collector probe can produce zero, one, or several verified relation rows. A
verified row need not increase rank, and broad factor-base coverage need not
close the matrix. The analyzer therefore retains five distinct quantities:

1. relation-hit probability: hit trials divided by all trials;
2. verified rows per trial, including multiplicity;
3. novel rank per trial and novel rows per verified row;
4. factor-base coverage, dead columns, and normalized hit entropy; and
5. verified full-rank success across independent terminal episodes.

Every row is independently group-verified; rank is recomputed over the subgroup
scalar field. Invalid or inconsistent relations and verification failures have
dedicated counters and fail closed.

# Risk-controlled decisions

All candidates, episode seeds, checkpoints, thresholds, and the maximum number
of verified rows one probe can emit are frozen before the first query. For
familywise error budget $A$, $C$ candidate slots, $L$ looks, and $E$ episode
slots, v1 assigns each bound

$$a = \frac{A}{4 C L (E+1)}.$$

For an active episode with observed mean row count $\bar X$, row cap $M$, $n$
observed probes, and $r$ remaining probes, two one-sided Hoeffding bounds give

$$\mu_U = \min\left(M,\;\bar X + M\sqrt{\frac{\log(1/a)}{2n}}\right),$$

$$R_U = \left\lceil r\mu_U + M\sqrt{\frac{r\log(1/a)}{2}}\right\rceil.$$

If current rank plus $R_U$ is still below target rank, the episode stops as
futile. Without a valid frozen row cap the statistical rule fails open. This
bound deliberately treats every optimistic future row as independent, so it
does not mistake the observed novel-rank fraction for a stationary probability.

At candidate level, exact two-sided Clopper--Pearson intervals summarize
verified full-rank outcomes across independent terminal episodes. A candidate
can be pruned when its upper endpoint is below the frozen minimum, or when that
endpoint plus a frozen margin lies below the incumbent's lower endpoint. A
deterministic reachable-rank ceiling below target is a structural stop.
Coverage, entropy, novel-rank utility, and charged cost remain diagnostics in
v1.

![Two-level pruning flow. `decision-flow.svg` is the editable source.](decision-flow-report.jpg){width=95%}

# Native interface and controls

The implementation lives in
`src/cryptanalysis/relation_yield_pruning.rs` and is exposed as:

```
icprog relation-prune --input INPUT.json --out REPORT.json
```

Input uses the strict schema `ic.relation-pruning.input/v1`; output uses
`ic.relation-pruning.report/v1`. Reports are write-once, so a later checkpoint
cannot replace the earlier evidence object.

Native unit controls exercise dominated-candidate pruning, capped futility, the
uncapped fail-open route, invalid-relation rejection, factor-hit reconciliation,
and refusal to overwrite a report. The deterministic interface fixture has
three candidates:

| Candidate | Episode outcome | Candidate outcome |
|:--|:--|:--|
| `incumbent` | verified full rank | keep |
| `bad_support` | deterministic structural stop | prune structural |
| `low_yield` | bounded futility stop | insufficient terminal evidence |

The fixture is a control, not a benchmark: its work counts are synthetic and it
contains no timing sample.

# Evidence boundary

Petit, Kosters, and Messeng separate relation search from linear algebra and
derive the heuristic relation probability $(\deg L)^m/(m!p)$ for their stated
prime-field regime. Gaudry's extension-field method likewise couples
factor-base construction to the relation oracle. These models motivate the
recorded channels, but neither substitutes for verified empirical yield or
licenses adaptive winner selection.

The repository's historical factor-base evaluation and residual-walk study were
reviewed for prior measurement practice. They remain historical evidence; this
change neither reruns nor promotes them. The example report is a schema receipt.
No benchmark copy, leaderboard row, or canonical graph changes because there is
no new performance result.

To establish a speedup, a follow-on protocol must run the frozen candidates on
identical public inputs, charge relation collection, matrix construction and
linear algebra, target descent, verification, failures, and orchestration, and
compare complete successful recovery with matched Pollard rho. Independent
scalar replay must confirm $[d]G=Q$.

# Sources

- C. Petit, M. Kosters, and A. Messeng, *Algebraic approaches for the Elliptic
  Curve Discrete Logarithm Problem over prime fields*, PKC 2016, sections
  3.1--3.4. <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
  Reviewed PDF SHA-256:
  `a53030a6cde720c354aebfb65b1eb01e07766ce2d26e543d77f2516c0a49ef5b`.
- P. Gaudry, *Index calculus for abelian varieties of small dimension and the
  elliptic curve discrete logarithm problem*, Journal of Symbolic Computation
  44 (2009). <https://doi.org/10.1016/j.jsc.2008.08.006>. Direct HAL retrieval
  returned HTTP 403; the repository's source-linked review was used and the
  access gap is retained.
- Repository sources: the [factor-base evaluation](../factor_base_evaluation_20261015/README.md),
  [residual-walk review](../notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md),
  and [tournament measurement contract](../ic_candidate_tournament_20260915/MEASUREMENT.md).
