# Missing constraint and solver comparison for twist-assisted IC

- **Status:** open
- **Kind:** construction gap, proof-verification gap, implementation gap and missing measurements; these are not a universal impossibility theorem.
- **Affected work:** [routes 1–3 tracker](../research/notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md), [trace card](../backlog/XP-20261021-twist-trace-factor-base/README.md), [cover card](../backlog/XP-20261021-twist-cover-transfer/README.md), [solver card](../backlog/XP-20261021-twist-symmetry-solver/README.md).
- **Next action owner:** unassigned research/implementation lane.
- **Last updated:** 2026-10-21

## Problem

A twist-assisted IC improvement for the NIST prime-field P-curve setting cannot yet be established because no specific useful restriction on trace fibers, verified cover transfer, or equivalent solver encoding has been implemented and compared against a matched rational-only IC baseline.

The source setting is E/F_p in characteristic p > 3 with the stated odd prime-order rational group; exact toy fixtures and representations must be registered and frozen before curve runs. The expanded deliverable is a concrete experiment program and its bounded arithmetic preflight. The useful-relation and performance experiments remain unperformed.

## Evidence and uncertainty

- **Observed:** [prior PR #2368](https://github.com/aburan28/crypto/pull/2368) reports mixed-component/rational-only controls and projection separation. It was open when this record was written. Its evidence is not independently replayed, admitted or promoted here.
- **Derived or checked:** E00 checks five trace/cover polynomial preconditions by exact coefficient expansion and evaluated identity certificates, including false controls. The rational/anti-invariant split, trace-fiber contract, group interpretation and exceptional charts still require their native group replay and faithful Lean bridges.
- **Suspected explanation:** an apparent gain from additional extension solutions may consist only of multiple lifts of the same projected relation. Whether a carefully restricted family reduces actual decomposition cost is unknown.
- **Missing capability:** native implementations of the designed restrictions, an equivalent-system builder, relevant cover/Jacobian maps, admitted fixtures and baseline/cost protocol, independent replay, resource estimates and paired complete-cost measurements.
- **What this evidence does not establish:** it neither proves that all auxiliary twist geometry is useless nor supplies an efficient NIST algorithm, a novel construction, a scaling result or a security conclusion.

## What would unblock it

- [ ] Supply one explicit restriction or cover with exact domain, field, subgroup, exceptional cases and reproducible native construction.
- [ ] Prove or independently verify its projection/transport, relation equivalence and subgroup recovery; record formalization limits without assuming the desired result.
- [ ] Freeze a same-projected-base rational-only IC control and the strongest applicable solver baseline, exact fixtures and cost definitions.
- [ ] Complete paired correctness and cost comparisons, including failed attempts, verification and duplicate removal; report baseline/candidate > 1 only for a genuinely measured gain.
- [ ] Deliver independent replay and the required result artifacts before any claim promotion.

These conditions would resolve a particular candidate's comparison gap. A null or negative result is a valid candidate closeout, not a proof that no other restriction exists. A stage improvement leaves whole-pipeline and scaling obligations open.

## Next discriminating step

- **Action:** implement the route-1 trace/fiber and projection-equivalence verifier, then supply a concrete membership restriction; use its exact projected relation set to define the route-3 control.
- **Inputs and controls:** registered public synthetic ordinary prime-field fixtures, exact curve/twist representations, identity and exceptional branches, same projected factor base, duplicate-lift and rational-only controls.
- **Execution limits:** no route performance run is scheduled, and its cost remains unestimated. Each blocked card must freeze its admitted fixtures/cost protocol, explicit resource envelope, checkpoint/restart and stop conditions before readiness.
- **Outcomes:** equal projected rows without cheaper computation close the tested restriction as null; a verified stage reduction advances to full-pipeline costing; a mismatch blocks performance interpretation until fixed; unsupported maps remain construction gaps.
- **Recheck trigger:** a linked implementation or proof PR, a concrete restriction, a verified cover/solver adapter, or new replay evidence.

## Resolution and history

- 2026-10-21: created with a planning tracker and three blocked, unfunded backlog cards. No experiment was launched.
- Unresolved. Completing or merging this record does not establish an IC improvement.

### 2026-10-21: experiment designs expanded

The [18-experiment program](../research/twist_routes_1_3_20261021/EXPERIMENTS.md) now specifies concrete trace-coordinate/slope/norm families, divisor controls, equivalent membership and solver systems, explicit cover complements, subgroup/class-map checks, representation ablations, full-pipeline combinations and conditional pairing/lift ingress. The three backlog cards contain hashed design protocols. E00 supplies a small native polynomial preflight; it does not resolve the missing useful restricted set, actual group/divisor map, solver comparison or full cost measurement. The blocker stays open.

The [E00 result](../research/twist_routes_1_3_20261021/reports/RESULTS.md) subsequently passed five polynomial preconditions with exact coefficient/evaluated checks and rejected five false variants. Formal Lean checks, group/map bridges, an independently replayed useful restriction and any solver/complete-cost improvement remain open.
