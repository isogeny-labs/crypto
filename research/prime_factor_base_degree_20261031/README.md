# Prime-field factor bases: certificates and a secant degree reduction

## Checked Lean continuation, 2026-10-31

The [formalization](proof/FORMALIZATION.md) now proves the polynomial-ring
degree reduction using actual canonical highest-degree forms: d+1 versus
2d+1, including the nonmonomial quadratic case. The exact F17 fixture is
checked at degree 5→3 for every target and all four sign branches. All eleven
modules build; the transitive axiom audit has zero failures and no native
evaluation. The full elliptic-curve group-law bridge remains a written proof.
Earlier proof-scope statements and sealed records below describe their
historical stages. The measured continuation is in
[the completed three-track study](parallel_20261031/RESULTS.md).

## Positive secant/graph continuation, 2026-10-31

A subsequent [split-graph theorem](SECANT_GRAPH.md) derives a strict
presentation-specific homogeneous degree reduction: a degree-d completely
split graph gives degree d+1 versus 2d+1 in the exact dense two-summand
system. The explicit four-abscissa fixture has degree **5 to 3**, while its
matched nongraph control has secant degree **4**. All four sign branches are
retained. Exact upper identities and lower witnesses certify the quadratic
top ideal; the secant/group polynomial identity is also checked over Z.
External novelty, full formalization, F4 solving degree, natural yield and
performance remain open. The original round below and its verification
records remain historical. See [the continuation artifacts](secant/README.md).

Date: 2026-10-31. The requested result is a genuinely novel factor-base degree
reduction for a prime-field elliptic curve. **This round has not established
that result.** It delivers a checked exclusion for one proposed comparison,
an ordinary-prime certificate checker, and an exact candidate/reference exporter.
No new factor-base experiment has run and no new degree win is claimed.

| Requirement | Status and evidence |
| --- | --- |
| Prime fields, rather than binary extensions | Implemented: ordinary Fp bivariate systems, coefficients modulo a checked odd prime; no Boolean power reduction |
| A new factor-base construction | Hypothesis: intersect two shifted power fibers, then keep rational lifts; novelty not established |
| Genuine degree reduction | Not established; [dense m=2 exclusion](THEORY.md) rules out a homogeneous-degree win from set choice alone in that presentation |
| Complete systems and exact references | Development exporter emits both membership polynomials plus S3 and exact group-sum witnesses for matched-size controls |
| Proof certificates | Native checker verifies upper identities and lower dual witnesses for bounded-Macaulay refutation degree; this is not a homogeneous-regularity or iterative-F4 claim |
| Formal proof | Lean kernel checked the universal monomial-coverage theorem; the polynomial-ring bridge has a written proof and is not formalized |
| Experiments and PR | Await launch admission and working GitHub authentication; no background job |

## What changed in the search

The tempting elementary-symmetric Kummer construction is already implemented
in `examples/pkm_symmetric_tower.rs` and `examples/pkm_symmetric_complete.rs`.
The 2026-10-26 session log already reports lower step degrees for that
presentation on some cells. Repeating it would not establish the requested
novelty. The earlier 24-design register and the PKM note also cover many
Kummer, Dickson, norm, sumset and auxiliary-isogeny constructions.

The new candidate to investigate is

`V = {x : x^d = c1, (x+h)^d = c2, x lifts to E(Fp)}`.

It changes the point set and supplies two membership constraints. Test both
its dense exact membership polynomial and its complete compact presentation.
Include lift conditions, every auxiliary variable and all branches. A claim
from the dense system's affine falls does not fill the homogeneous degree field.

The pilot exporter currently supplies the dense presentation only. The compact
shifted-power-chain exporter, its exact rational-lift constraint and a matched
iterative F4 trace remain implementation obligations before that arm can run.
The catalogue search found no explicit shifted-fiber experiment in the narrow
repository and paper sources inspected; that is not proof of novelty.

## Literature and prior results read

- Petit–Kosters–Messeng, [*Algebraic approaches for the ECDLP over prime fields*](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  especially factor-base construction and the experimental-degree definitions.
  Its composed-map and auxiliary-isogeny methods are prior work.
- Kosters–Yeo, [*Notes on summation polynomials*](https://arxiv.org/abs/1503.08001),
  distinction between first fall and observed solving behavior. Its trace
  improvement is in characteristic two and does not transfer to this pilot.
- Repository [factor-base encyclopedia](../../docs/ic/factor-base-encyclopedia/README.md),
  prime-field entries and the current PKM section.
- `research/factor_base_designs_20261015/README.md`: all 24 registered designs,
  shape-null controls, and the distinction between an initial degree shift and
  growth in counted work.
- [PKM tower note](../notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md),
  section 3.4: known degeneration of Kummer/Dickson top forms.
- `examples/pkm_symmetric_tower.rs`, `examples/pkm_symmetric_complete.rs`, and
  `docs/session-logs/2026-10-26.md`, entry at 18:25Z: existing symmetric result.

Web searches included prime-field factor-base/symmetric Kummer degree,
shifted multiplicative-coset intersections, and dense Semaev regularity.
Search results do not establish absence of prior art. A novelty claim remains
subject to a broader full-text audit and independent mathematical review.

## Checks and next action

The standalone native crate has eight release tests: certificate replay over
odd primes, corruption/invalid-field rejection, capped-UNSAT versus SAT,
all 625 pairs of univariate linear generators over F5, ordinary-power
semantics, curve-group arithmetic, exact membership, and S3 on a point sum.
These are development checks, not factor-base measurements.

Lean 4.19.0 checked `proof/TopDegree.lean`. Its four declarations use only
standard Lean axioms, with no `sorry`, custom axiom or native evaluation.
See [theory/proof scope](THEORY.md) and [pilot protocol](PROTOCOL.md).

The current repository checkpoint rule says to run `scripts/checkpoint.sh gate`
and not launch when it fails. The worker has no S3 upload token or ready AWS/EKS
identity, and its injected GitHub token is invalid. The existing
[access problem](../../problems/n83-factor-base-screen-execution-access.md)
also applies to this pilot. A local run requires an explicit owner override
of those launch rules; local compilation, tests and proof checking were completed.

Numerical dashboards are unchanged: this round has no new admitted relation,
degree comparison, curve registration or canonical FB1 object. The report's
diagram shows the proof distinction and proposed test, not measured gains.
