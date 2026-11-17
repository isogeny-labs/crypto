# Route 2: experiment-design protocol

Date: 2026-10-21. Status: committed design, **not a ready benchmark protocol**. The exact executable drivers, admitted fixtures, baseline pin, cost calibration and resource estimates for E01–E17 are still missing. E00 has its separate frozen preflight protocol.

[Canonical program](../../research/twist_routes_1_3_20261021/EXPERIMENTS.md), [machine matrix](../../research/twist_routes_1_3_20261021/experiments.json), and [shared blocker](../../problems/twist-trace-constraint-and-solver-gap.md).

## Experiments owned or consumed

### E10 — Explicit degree-two covers

Question: When is the complementary elliptic quotient actually the required twist?

Inputs/arms: C_c: v^2=f(u^2+c), f(c)!=0; quotient E and D_c: z^2=t f(t+c); use the explicit short complement coefficients.

Controls: Reject f(c)=0; same-j only control; trace-opposite only control; explicit Fp isomorphism check.

Acceptance: Smoothness/genus, complete quotient maps including infinity, complement trace/j, isomorphism and image size.

Stop and interpretation: Neither matching j nor opposite trace alone certifies the requested isomorphism or subgroup transfer.

Missing prerequisites: E00; native smoothness and map checker.

### E11 — Gluing and alternative complements

Question: Do other gluing orders or complementary-factor constructions provide a better executable route?

Inputs/arms: Galois-equivariant 2/3-torsion gluing where applicable; recover the complement from a supplied cover; bounded typed candidate families.

Controls: Degenerate/product polarization control; incompatible Frobenius control; identical-field checks.

Acceptance: Actual curve/map construction, indecomposable polarization, subgroup kernel, factored evaluation cost and failures.

Stop and interpretation: A rejected family or failed bounded search does not establish universal nonexistence.

Missing prerequisites: E10; gluing/complement implementation.

### E12 — Jacobian arithmetic and recovery

Question: Does the proposed transfer work on the divisor classes the collector actually produces?

Inputs/arms: Pullback/norm on the chosen infinity model; arbitrary valid reduced classes; image subgroup order and recovery.

Controls: phi_*phi^*=[degree]; origin and ramified fibers; independent CAS; direct-E solver.

Acceptance: Exact group-law and kernel/recovery checks; separate setup, maps, arithmetic and recovery costs.

Stop and interpretation: Unsupported two-infinity or arbitrary-class arithmetic remains unsupported, not approximated by rational point-class tests.

Missing prerequisites: E10 or E11; native divisor-class arithmetic and norm.

### E13 — Restricted cover images

Question: Can restricting cover points give a small base without a worse membership or map problem?

Inputs/arms: All-C(Fp) control; restricted u/trace/norm supports; degree/genus ladder only with frozen factored maps.

Controls: Same projected base on E; fixed-genus image bound; independent point counts.

Acceptance: Actual image size, g/degree/representation size, construction and membership cost, projected rows.

Stop and interpretation: A fixed bounded-degree cover with order-p image does not supply a small base merely by changing the ambient variety.

Missing prerequisites: E12.

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
