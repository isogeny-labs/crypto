# Relation-yield and factor-base pruning

This package adds a native, write-once checkpoint analyzer for the existing
index-calculus tournament. It measures the parts of "yield" that matter
separately and permits only conservative hard stops:

- relation-hit probability and total verified-row yield;
- novel modular rank and conditional rank utility;
- factor-base coverage, dead columns, and normalized hit entropy;
- independently verified full-rank success across episodes; and
- complete charged work, retained as a diagnostic in v1.

The implementation is in
`src/cryptanalysis/relation_yield_pruning.rs`; the interface is
`icprog relation-prune --input INPUT --out REPORT`. See [PROTOCOL.md](PROTOCOL.md)
for the frozen design and [decision-flow.svg](decision-flow.svg) for the two
decision levels. The illustrated technical report is available as
[REPORT.pdf](REPORT.pdf), with [REPORT.md](REPORT.md) as its editable source.
Rebuild its embedded preview and PDF with:

```
inkscape decision-flow.svg --export-type=png --export-filename=/tmp/relation-pruning-flow.png --export-width=1200
convert /tmp/relation-pruning-flow.png -resize 520x -quality 58 decision-flow-report.jpg
pandoc REPORT.md --from markdown --pdf-engine=xelatex --resource-path=. -o REPORT.pdf
```

## Why this form

Petit, Kosters and Messeng's prime-field analysis separates relation search from
linear algebra and gives the familiar heuristic decomposition probability
`(deg L)^m / (m! p)` in the stated regime. That heuristic is valuable for
planning, but a tournament needs observed, verified yield and a rank-closure
gate. Gaudry's extension-field construction likewise makes factor-base shape and
the relation oracle inseparable from the cost model. Neither source justifies
selecting the largest raw hit count after an adaptive number of looks.

The repository's earlier factor-base evaluation already preserves per-case
decomposition, fixed-column rank, and historical source custody. The residual-
walk study separately records fixed-budget relation yield. This change turns
those distinctions into one strict native schema; it does not reinterpret or
promote either historical study.

References:

- C. Petit, M. Kosters, A. Messeng, *Algebraic approaches for the Elliptic
  Curve Discrete Logarithm Problem over prime fields*, PKC 2016, sections
  3.1--3.4, [primary PDF](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  reviewed copy SHA-256
  `a53030a6cde720c354aebfb65b1eb01e07766ce2d26e543d77f2516c0a49ef5b`.
- P. Gaudry, *Index calculus for abelian varieties of small dimension and the
  elliptic curve discrete logarithm problem*, J. Symbolic Computation 44
  (2009), [DOI](https://doi.org/10.1016/j.jsc.2008.08.006). Direct HAL retrieval
  returned HTTP 403 in this session; the repository's source-linked summary was
  used and this access gap is retained rather than silently omitted.
- Existing repository evidence:
  [factor-base evaluation](../factor_base_evaluation_20261015/README.md),
  [residual-walk review](../notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md), and
  [tournament measurement contract](../ic_candidate_tournament_20260915/MEASUREMENT.md).

## Controls and example outcome

`example-input.json` contains three synthetic candidates at the first of two
frozen looks. It is intended to exercise the interface:

| candidate | episode result | candidate result | reason |
| --- | --- | --- | --- |
| `incumbent` | `full_rank` | `keep` | two independently verified terminal successes |
| `bad_support` | `stop_structural` | `prune_structural` | deterministic rank ceiling is below target |
| `low_yield` | `stop_futile` | `insufficient_evidence` | capped optimistic future rows cannot close rank; no terminal candidate verdict yet |

The generated `example-report.json` is checked in as a schema/control receipt.
It contains no timing sample and must not enter a performance benchmark table.

## Requirements and evidence

| Requirement | Evidence in this change | Status |
| --- | --- | --- |
| Empirical relation yield | hit trials and verified rows per trial with bounded intervals | implemented |
| Factor-base success | coverage, entropy, dead columns, novel rank, terminal full-rank probability | implemented |
| Sampling-safe pruning | fixed looks, familywise allocation, Hoeffding and exact binomial bounds | implemented |
| Correctness custody | invalid/inconsistent/verification counters fail closed; reports are write-once | implemented |
| Tournament integration | native CLI and strict JSON schema | implemented; producer wiring remains follow-on |
| Fresh performance evidence | EKS run on public inputs with matched rho | not run |
| End-to-end speedup | complete IC/rho ratio and crossover | not established |

No benchmark copy, leaderboard entry, progress-timeline entry, or browser result is
created because this package reports no new performance measurement. The
canonical graphs were checked for applicability and remain unchanged.
