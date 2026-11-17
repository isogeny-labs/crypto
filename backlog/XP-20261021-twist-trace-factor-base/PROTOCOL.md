# Route 1: experiment-design protocol

Date: 2026-10-21. Status: committed design, **not a ready benchmark protocol**. The exact executable drivers, admitted fixtures, baseline pin, cost calibration and resource estimates for E01–E17 are still missing. E00 has its separate frozen preflight protocol.

[Canonical program](../../research/twist_routes_1_3_20261021/EXPERIMENTS.md), [machine matrix](../../research/twist_routes_1_3_20261021/experiments.json), and [shared blocker](../../problems/twist-trace-constraint-and-solver-gap.md).

## Experiments owned or consumed

### E01 — Trace fibers and separation

Question: Which apparent relation gains are just multiple lifts of the same trace?

Inputs/arms: All points on the two registered odd-prime-order toy curves over the declared quadratic extension; add separate registered even-order and odd-composite controls.

Controls: Rational-only traces; pure twist trace zero; S+U with U in ker(tau); identity; rational 2-primary cases.

Acceptance: Exact trace image/fiber counts, distinct projected rows, row ranks and target-row span.

Stop and interpretation: Do not count repeated fibers as new information; a failed asserted identity is a correctness failure, not a performance loss.

Missing prerequisites: registered exact extension models and native point verifier.

### E02 — Coordinate boxes

Question: Do small coordinate boxes create a useful small trace base with cheaper membership or decomposition?

Inputs/arms: S=(u+v omega,w+z omega) with u in U and v in V; H in {2,4,8}; enumerate both y signs and branches.

Controls: Exact-image rational base; same-size hash-selected rational base; unrestricted lift control.

Acceptance: Distinct traces, nonzero coverage, fiber multiplicities, construction/encoding cost and verified independent rows.

Stop and interpretation: Record empty bases and size collisions; never normalize away construction or duplicate-removal work.

Missing prerequisites: E01.

### E03 — Slope and real-part restrictions

Question: Does restricting u and m=z/v help beyond a rational parametrization of X=m^2-2u?

Inputs/arms: u in U and m in M, H in {2,4,8}; enforce 4dv^2=Delta_T(m); add independent v restriction as an ablation.

Controls: Same quartic witness written entirely over Fp; exact-F table; ordinary low-degree addition witnesses.

Acceptance: Projected solution-set equality first; then encoding size and counted work per independent row.

Stop and interpretation: If the candidate and rational witness systems are identical after renaming, classify the result as encoding reuse rather than new twist geometry.

Missing prerequisites: E01.

### E04 — Norm and multiplicative restrictions

Question: Can norm constraints or multiplicative structure reduce solve cost after parameterization and membership are charged?

Inputs/arms: Norm(x)=u^2-dv^2 in W; norm(y) restrictions; H-th-power or subgroup conditions only when H divides p-1; intersect with a trace-box family.

Controls: Same F without norm witnesses; same-size rational base; matched subgroup-size control; nonsquare d rescaling.

Acceptance: Image size, membership degree/selector size, projection collisions and complete stage work.

Stop and interpretation: Unchanged projected sets at unchanged or higher cost close only that restriction; missing roots do not count as a win.

Missing prerequisites: E01.

### E05 — Divisors and degree-two places

Question: Can generic conjugate pairs help smooth-divisor relations when the principal pure-twist pairs are removed?

Inputs/arms: Sparse rational functions a(x)+y b(x), coefficient support frozen; distinguish x in Fp from genuinely nonrational degree-two places.

Controls: Remove divisors of x-x0 before forming rows; rational-only function family; generic degree-two-place control.

Acceptance: Factoring and lifting work, nontrivial projected divisor rows, row span and failed factorizations.

Stop and interpretation: Principal pairs and empty projected rows are not relations; retain the full degree and coefficient-height budgets.

Missing prerequisites: E01; native divisor and factorization verifier.

### E06 — Exact relation census

Question: Which restricted bases actually span the needed target rows?

Inputs/arms: m in {2,3,4}, B in {8,12,16}; all nonidentity targets for tiny fixtures; freeze sign/orbit conventions.

Controls: Complete enumeration or MITM oracle; identical projected base and target; raw versus canonicalized rows.

Acceptance: Exact relation set, target coverage, rank over r, target-row span and verified known-answer recovery.

Stop and interpretation: Do not silently replace unattainable full-column rank with a new stopping rule; freeze target-span stopping in advance.

Missing prerequisites: E02 or E03 or E04.

### E16 — Homomorphism and pairing applicability

Question: Do small twist factors or a pairing actually supply an admissible transfer on the original subgroup?

Inputs/arms: Exact gcd(r,#E_tw), embedding-degree and torsion/Frobenius tests on the declared public parameters.

Controls: Trace/norm killing pure anti-invariants; translated-coset control; known small-embedding-degree positive family kept separate.

Acceptance: Exact applicability certificates and a named nondegenerate transfer if one exists.

Stop and interpretation: A smooth twist order or shared x-coordinate relation does not supply a subgroup-preserving transfer; do not run a nonexistent destination attack.

Missing prerequisites: exact public-parameter and transfer checker.

### E17 — Arithmetic lift and twist ingress

Question: Could a characteristic-zero twist reduce to E while providing a usable small-height relation source?

Inputs/arms: Only twists whose parameter is square mod p for a same-E reduction; bounded height/rank/search families stated before a pilot.

Controls: Nonsquare reduction goes to the other finite-field component; fixed-lift and equal-height controls; charge discovery and target lifting.

Acceptance: Explicit reduction map, target lift, relation independence, heights and construction/search costs before any solver comparison.

Stop and interpretation: High rank alone is not a cheap target lift or a speedup; no unbounded rank or designer-secret assumption is treated as evidence.

Missing prerequisites: separate arithmetic-lift construction and height protocol.

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
