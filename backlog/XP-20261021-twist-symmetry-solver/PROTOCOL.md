# Route 3: experiment-design protocol

Date: 2026-10-21. Status: committed design, **not a ready benchmark protocol**. The exact executable drivers, admitted fixtures, baseline pin, cost calibration and resource estimates for E01–E17 are still missing. E00 has its separate frozen preflight protocol.

[Canonical program](../../research/twist_routes_1_3_20261021/EXPERIMENTS.md), [machine matrix](../../research/twist_routes_1_3_20261021/experiments.json), and [shared blocker](../../problems/twist-trace-constraint-and-solver-gap.md).

## Experiments owned or consumed

### E00 — Polynomial preflight

Question: Do the proposed trace and complement coordinate formulas satisfy their exact cleared-denominator polynomial identities?

Inputs/arms: Five compiled statement ASTs; exact integer coefficient expansion and repository identity certificates; deliberately false +1 variants.

Controls: Exact expansion and modular evaluation are different arithmetic checks; all negative variants must reject.

Acceptance: All five exact coefficient differences zero; 32 issuer and 32 domain-separated checker points pass; five false variants reject.

Stop and interpretation: Any mismatch stops use of the affected formula; no transfer or performance claim follows from passing.

Missing prerequisites: source/protocol freeze for E00.

### E03 — Slope and real-part restrictions

Question: Does restricting u and m=z/v help beyond a rational parametrization of X=m^2-2u?

Inputs/arms: u in U and m in M, H in {2,4,8}; enforce 4dv^2=Delta_T(m); add independent v restriction as an ablation.

Controls: Same quartic witness written entirely over Fp; exact-F table; ordinary low-degree addition witnesses.

Acceptance: Projected solution-set equality first; then encoding size and counted work per independent row.

Stop and interpretation: If the candidate and rational witness systems are identical after renaming, classify the result as encoding reuse rather than new twist geometry.

Missing prerequisites: E01.

### E06 — Exact relation census

Question: Which restricted bases actually span the needed target rows?

Inputs/arms: m in {2,3,4}, B in {8,12,16}; all nonidentity targets for tiny fixtures; freeze sign/orbit conventions.

Controls: Complete enumeration or MITM oracle; identical projected base and target; raw versus canonicalized rows.

Acceptance: Exact relation set, target coverage, rank over r, target-row span and verified known-answer recovery.

Stop and interpretation: Do not silently replace unattainable full-column rank with a new stopping rule; freeze target-span stopping in advance.

Missing prerequisites: E02 or E03 or E04.

### E07 — Equivalent system builders

Question: Which membership representation avoids merely moving cost into a higher-degree constraint?

Inputs/arms: Univariate product membership, bit decomposition with proven range, selector/product chains and explicit branching.

Controls: Untwisted addition chain; exact same projected F; independently enumerated projected solutions.

Acceptance: Equality of full projected sets including multiplicity/sign policy and exceptional branches; equations/variables/degrees.

Stop and interpretation: A mismatch blocks all solver comparisons until repaired; coefficient fields and modular-wrap conditions are part of the input.

Missing prerequisites: E06.

### E08 — Solver and ordering panel

Question: Does symmetry reduce actual solving cost against the strongest equivalent encoding?

Inputs/arms: Frozen variable orders, chain/quadratized/symmetric encodings; msolve/open reference plus available applicable F4/F5 engines.

Controls: A/A runs; untwisted witness system; redundant-variable positive/negative controls; same output verifier.

Acceptance: Counted native operations, Macaulay dimensions/nnz, reductions, solving degree, memory, failures and cost per useful row.

Stop and interpretation: Lower displayed degree alone is not a win; timeouts and OOMs remain incomplete outcomes under the same envelope.

Missing prerequisites: E07; calibrated external-solver adapters.

### E09 — Torsion over extensions

Question: Can torsion symmetry survive Galois descent without false factor-base compression?

Inputs/arms: 2-torsion over Fp^3 and combined Fp^6 where applicable; Galois-invariant coordinates and descent constraints.

Controls: No-torsion encoding; [2] map on the odd subgroup; retained extension-field cost; rational torsion special-family control.

Acceptance: Equivalence, actual orbit/image size, descended system degree and complete representation cost.

Stop and interpretation: A geometric torsion orbit that leaves the rational base is not a rational factor-base quotient.

Missing prerequisites: E07; registered extension models and torsion verification.

### E14 — Model and isogeny ablations

Question: Are any savings due to cheaper constants or another isogenous representative rather than new relations?

Inputs/arms: Nonsquare d choices, x-preserving/standard twist models, fixed small-degree rational isogeny routes and verified return maps.

Controls: Equivalent direct-E representation; same source target/subgroup; conversion charged both ways.

Acceptance: Identical recovered outputs and projected relation policy; map, primitive and solver cost separated.

Stop and interpretation: Different j or field models require typed identities; do not infer an unknown-scalar speedup from faster known-scalar multiplication.

Missing prerequisites: E07; verified map adapters.

### E15 — Factorial pipeline ablation

Question: Which component or combination improves the actual IC baseline?

Inputs/arms: Trace restriction on/off, cover on/off, solver symmetry on/off; pairs use equal target/base or explicitly separate changed-base studies.

Controls: Strongest current IC baseline; strongest same-target rho; direct destination generic solver; independent holdouts.

Acceptance: Cold single-target complete counted cost and S; setup/online split; 0/0.1/1 GAE pricing band; verification and replay.

Stop and interpretation: Every claimed IC improvement needs baseline/candidate >1 with all phases; rho is a reported gap, not the IC improvement pass gate.

Missing prerequisites: E06; E08; E12 for cover cells.

## Before promoting the card to ready

- Implement and pin the actual native driver and independent verifier; a working polynomial preflight is not the missing point/solver/transfer implementation.
- Freeze exact registered ICV1/EC1 models, field bases, subgroup/generator and public targets, candidate, best applicable baseline and all input/source hashes.
- Separate complete toy censuses from uniform-target performance cells; keep planted controls separate from yield estimates.
- Freeze the H={2,4,8}, m={2,3,4}, B={8,12,16} design cells actually supported and retain empty/unsupported cells. Freeze train/holdout fixtures before solver tuning.
- Derive the resource envelope and core-hour estimate from a correctness/calibration pilot. Add resumable execution and checkpoints. No paid launch is requested by the nominal card metadata.
- Compare exact projected outputs before costs; charge construction, failed attempts, membership, lifting, deduplication, verification and matrix/target work as applicable. Use target-row span under the frozen stopping rule.
- Predeclare counted stage/pipeline acceptance as baseline/candidate >1 on matched workloads and disclose the rho gap. Apply the 0/0.1/1 GAE band and external-solver controls. A stage gain remains a stage gain.
- Retain failures and independent replay. Native backlog validation must pass after replacing the failing execution sentinel, filling the output schema/checker, pinning merged code, and re-hashing any amended protocol.

The current protocol records concrete designs and hypotheses. Committing its blob does not make its missing experiments runnable or measured; any later change to a frozen executed subsection is an explicit amendment.
