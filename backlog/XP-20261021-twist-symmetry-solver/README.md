# Twist route 3: equivalent polynomial encodings and Frobenius symmetry

- **Status:** blocked for the full campaign; bounded local panels completed on 2026-10-21–22.
- **Tracker:** [routes 1–3](../../research/notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md).
- **Blocker:** [missing constraint and solver comparison](../../problems/twist-trace-constraint-and-solver-gap.md).
- **Prior work:** [PR #2368](https://github.com/aburan28/crypto/pull/2368), unmerged when this card was prepared.

## Question and decision value

Can a native auxiliary-variable encoding lower actual solver cost per independent projected relation compared with an equivalent untwisted addition-witness formulation?

Projected solution-set equality determines whether the candidate computes the same task. Only then can a paired cost reduction count as a solver-stage improvement; lower displayed polynomial degree, extra kernel solutions or a favorable variable order alone cannot establish a pipeline gain.

## Prerequisites and next step

Restricted point sets, frozen same-base controls, native builders and independent group/ideal checks now exist. The broader B12/16/base grid, an applicable signature reference and closed full-cost/independent-host validation remain open.

Complete the remaining grid and applicable reference backend, then close native solver accounting/calibration and independent-host replay without dropping the original acceptance criteria.

Controls: Untwisted low-degree addition-witness chain; same projected base and targets; v != 0 saturation and separate v = 0/O branches; signs, multiplicities, target-row span; strongest available external CAS.

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

Equivalent native systems and three preserved 1930-cell solver attempts executed; 240 distinct projection cells match. Combined IC has 1704 verified identical replays and 216 empty-base cells. Broader grids, applicable signature baseline, full solver pricing and independent-host replay remain open. See the [result and requirement table](../../research/twist_routes_1_3_20261021/reports/EXECUTION_RESULTS.md), exact receipts and source/protocol freezes. All original acceptance items above remain requirements; local correctness evidence is not a complete-cost improvement. No funded or remote full campaign is running.

## Follow-up proposals, 2026-10-22

[E18–E29 designs](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md)
add separate blocked cards for route-3 and shared measurement panels:
[E25 applicable signature control](../XP-20261022-twist-homogeneous-solver-control/README.md),
[E26 block/hybrid elimination](../XP-20261022-twist-block-elimination/README.md),
[E27 complete cost accounting](../XP-20261022-twist-complete-cost-accounting/README.md)
and [E28 heldout selection](../XP-20261022-twist-heldout-selection/README.md).
They are proposed, unfunded and not started. This card's original frozen
protocol and acceptance criteria are unchanged.
