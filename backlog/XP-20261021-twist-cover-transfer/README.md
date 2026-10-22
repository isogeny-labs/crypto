# Twist route 2: explicit cover, Jacobian subgroup maps and destination solver

- **Status:** blocked for the full campaign; bounded local panels completed on 2026-10-21–22.
- **Tracker:** [routes 1–3](../../research/notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md).
- **Blocker:** [missing constraint and solver comparison](../../problems/twist-trace-constraint-and-solver-gap.md).
- **Prior work:** [PR #2368](https://github.com/aburan28/crypto/pull/2368), unmerged when this card was prepared.

## Question and decision value

Can a verified same-field cover involving the curve and its twist lower projected relation or complete IC cost once construction and subgroup transport are charged?

A constructed curve with checked divisor maps and subgroup recovery would make the route executable. A Frobenius-only match or unsupported infinity model would leave a construction gap; a complete matched-cost comparison would determine whether this particular transfer improves the IC baseline.

## Prerequisites and next step

Ten selected covers now have checked complements and executable divisor-class transfer. A costed Jacobian decomposition solver, complete accounting and independent-host class-map replay remain open.

Select a bounded construction family, freeze its candidates, and certify a smooth curve, correct infinity model, induced pullback/norm, subgroup kernel and scalar recovery before benchmarking.

Controls: Necessary Frobenius filter separated from existence; product/direct-E control; ramified and identity cases; exact subgroup order and degree coprimality; same public target and complete transport costs.

## Cost and resources

Bounded local runs and native drivers now exist, with receipts and process limits in the execution report. Full-campaign runtime estimates remain null. The nominal card resource fields and failing sentinel describe no completed run and must be replaced before future readiness.

Proposed funding is zero, below the existing funded seed cards. Nothing is posted to cairn and no spending or execution priority is changed.

## What done means

- [ ] Exact fixtures, curve/twist identities, subgroup, baseline and candidate frozen in a committed protocol.
- [ ] Native driver and independent checker implemented and pinned to a merged commit.
- [ ] Exceptional cases, controls, heldouts, resumable execution, resources, stop conditions and verification defined.
- [ ] Code/protocol pins, checkpoint fields and output schema filled; the failing sentinel replaced; native backlog validation passed before ready.
- [ ] Preregistered run completed or its bounded failure preserved, with correctness and complete accounting for every compared arm.
- [ ] Result, independent replay and required report/visual/PDF artifacts landed by linked PR; tracker and problem record updated.

A null result closes only the frozen candidate/question. A stage gain does not establish a complete IC improvement or applicability at NIST size. Merging this card does not mark its experiment done.

## Expanded experiment design

[Route protocol](PROTOCOL.md) now defines the owned experiments, controls, falsifiers, dependencies and readiness conditions. The [18-experiment program](../../research/twist_routes_1_3_20261021/EXPERIMENTS.md) also contains the coupled trace/solver and cover/pipeline panels. No broader experiment has been launched merely by recording its design.

## Bounded execution update, 2026-10-22

Ten selected covers have executable class pullback/norm checks, independent PARI orders and bounded image controls. Three explicit isogenies now have algebraic duals. Positive three-gluing quotient/polarization construction, Jacobian decomposition, formal/independent-host map replay and complete costs remain open. See the [result and requirement table](../../research/twist_routes_1_3_20261021/reports/EXECUTION_RESULTS.md), exact receipts and source/protocol freezes. All original acceptance items above remain requirements; local correctness evidence is not a complete-cost improvement. No funded or remote full campaign is running.
