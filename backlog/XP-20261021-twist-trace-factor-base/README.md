# Twist route 1: structured trace factor base and rational-only equivalence control

- **Status:** blocked for the full campaign; bounded local panels completed on 2026-10-21–22.
- **Tracker:** [routes 1–3](../../research/notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md).
- **Blocker:** [missing constraint and solver comparison](../../problems/twist-trace-constraint-and-solver-gap.md).
- **Prior work:** [PR #2368](https://github.com/aburan28/crypto/pull/2368), unmerged when this card was prepared.

## Question and decision value

Can an explicitly restricted extension-point family reduce the cost of finding independent relations on its small trace image compared with the same projected rational factor base?

An exact projection census determines whether the restriction changes the useful relation set or only repeats trace fibers. A verified stage-cost reduction would justify a solver and pipeline comparison; equal rows at equal or higher cost close only this restriction as a null or negative result.

## Prerequisites and next step

Concrete restrictions, native trace/projection checks and same-image controls now exist. Larger-field completion, full native-work accounting/calibration and independent-host replay remain open.

Complete the remaining larger-field families and price decomposition against the frozen same-image controls before any stage or whole-pipeline improvement is claimed.

Controls: Identity and v = 0 cases; pure twist trace-zero control; duplicate lifts; the exact same projected factor base, targets, signs, arity and target-row stopping rule.

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

Bounded trace/norm/divisor/census and rational-quartic controls executed. E15 supplies same-base controls and exact same-host replay. Full larger-field families, general proof bridges, calibrated complete cost and independent-host replay remain open. See the [result and requirement table](../../research/twist_routes_1_3_20261021/reports/EXECUTION_RESULTS.md), exact receipts and source/protocol freezes. All original acceptance items above remain requirements; local correctness evidence is not a complete-cost improvement. No funded or remote full campaign is running.
