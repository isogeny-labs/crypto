# M31 algebra manual-run results

Date: 2026-11-23 America/Los_Angeles (2026-11-24 UTC).
Scope: read-only repository and primary-source review, exact output-contract audit, and bounded mathematical fallback. No new cryptographic program, experiment, benchmark, paid job, shared-state mutation, PR mutation, was launched in this review. New conclusions below are written derivations and remain [pcc:unverified].

## Source identities and retrieval

The shared source revision is crypto main `81f0308c75204f3a1c0ec52fdf78396eee0898ed`; the current reports and exact identities are cited below. Existing work in PRs #5048, #5063, #5068 and #5087 retains its ownership. The prescribed persistent rootfinding lab and complete release setup were unavailable in this runtime, so the task's bounded mathematical/source fallback was used. The knowledge-base connector was absent and its local wrapper reported exit 127. No novelty or exhaustive prior-art claim follows from that retrieval gap.

Primary reference: [Semaev 2015](https://arxiv.org/pdf/1504.01175), Section 3, Lemma 2 and Sections 4.2/4.4. Its first-fall and field qualifications are relevant to the output-contract distinctions below. [pcc:unverified]

## Canonical M31 fixture

The source STATE records:
- p = 2147483647 = 2^31-1.
- E: y^2 = x^3 + 847514846*x + 2087444724.
- n = 2147446831; G=(625037715,2100627251).
These are inherited source parameters, not a new point-count or primality certificate. Scalar/relation arithmetic uses n (or the actual subgroup modulus), not p.

## Rootfinding research cycles

### Executed result

Read the current Program B matched-F4 primary report at:
https://github.com/isogeny-labs/crypto/blob/81f0308c75204f3a1c0ec52fdf78396eee0898ed/research/rootfind_matched_f4_gate_20261123/README.md
(blob afe6f125c93bfe6276f21f94151911d8aa480604; related PR5085).

The frozen study uses p=4294967291, not M31; it records release-gate FAIL/inconclusive and no hard-regime experiment. Its F4-visit arm is domain enumeration; the genuine Gröbner arm is F4-ideal. It reports missing odd-prime F5/F6 implementations, which this worker did not independently establish across every repository.

The exact M31 transfer of the report's easy-regime criterion is:
1290^3 = 2146689000 < 2147483647 < 2151685171 = 1291^3.
Also 3*1291^2 = 5000043 < p.
Therefore M<=1290 satisfies the report's no-product-wrap condition, while M=1291 is the first integer violating it. A test above this boundary does not require a larger field: M31 itself meets f4_fp's reported p<2^32 field-width condition. This corrects an overbroad "needs larger p" reading of the README. It does not establish that a genuine F4 matrix at M=1291 fits the existing budget, nor that the instance is difficult for every algorithm. [pcc:unverified]

### Exact support guard audit

For any nonzero H in F_p[T], let Q=T^(2^31)-T^2=T(T^p-T) and G_H=monic gcd(H,Q). Let epsilon=1 if T^2 divides H, otherwise0. Then
G_H / T^epsilon = monic gcd(H,T^p-T).
Reason: Q has the same roots as T^p-T; its only extra multiplicity is the second zero root. The gcd with H keeps this extra factor exactly when T^2 divides H. This is a written elementary identity, not a new algorithmic-speed claim. It makes a future M31 shortcut exact for squarefree rational-root support. [pcc:unverified]

Counterexample to omitting the correction: H=T^2(T-1)^2 gives gcd(H,Q)=T^2(T-1), while the rational-root polynomial is T(T-1). Conversely H=T(T-1)^2 must not lose its sole zero root. Both are algebraic controls, not executed tests.

Cost warning: the Q route uses repeated squaring, but the ordinary T^p route can exploit cheap multiplication by T. A shorter generic addition chain does not alone prove a faster polynomial rootfinder. [pcc:unverified]

### Blocker and next action

The named persistent autolab checkout and full matching F4/F5/F6 release setup are not available here, and no fresh checkpoint/admission or performance reservation was obtained. Prepare an M31 boundary panel with native exact all-roots equality and a genuine F4 comparison before timing; preserve the original20-hard-instance release gate.

Impact score: 2/10 — Corrected the M31 hard-regime entry point and its rational-root support contract; both are source-backed/written preparation, without an executed solver comparison.

## Summation polynomial

### Executed result

Performed an output-contract review disjoint from the existing implementation owners. The important compatibility rule is:

1. Full Semaev projected-set equality asks for original abscissae in M31, but geometric lifts/intermediate coordinates may need Fp² and infinity.
2. Actual rational ECDLP relations require rational signed leaf points and original group replay. This is a stronger output contract.
3. M31 source arithmetic does not authorize silently replacing contract1 with contract2. An auxiliary Fp² can be represented by two M31 coefficients and an explicitly identified nonsquare; this is not an extension-field source ECDLP benchmark.
4. The existing M31 quadratic field guard T^(2^31)=T² is exact on zero sets but double at zero. Multiplicity, rank of quotient algebras, and distinct witness count must not be conflated.

The iteration6 note already records these guard counts:30 extra variables/31 quadratic equations per internal abscissa for zero-set semantics, versus59/60 for its squarefree eliminated guard. They are existing results, not new achievements of this run.

An explicit abstract ghost retained by the source suffices to reject naive equivalence: choose rational nonliftable w and rational target abscissa r with f(r)!=0 and w!=r. q(U)=S3(w,r,U) has nonsquare discriminant16*f(w)*f(r), so no M31 root, but Res_U(q,q)=0. Thus enforcing M31-valued finite auxiliaries loses a projected S4 root. This is a source-backed logical audit, not a new numerical counterexample.

The required new query primitive and native validation remain open. Neither low-degree generators, a low first-fall diagnostic, nor the source-reported bounded-degree quotient obstruction solves the target-query problem. [pcc:unverified]

Impact score: 2/10 — Completed a concrete M31 compatibility audit that prevents changing the solver's solution set; preserved active work and made no new extraction or timing claim.

## Find decomposition

### Executed result

Followed the actual current decomposition lineage rather than the stale prompt's small-prime fallback:
- PR5044 is merged, with historical M31 specialized-root oracle gates and explicit lack of genuine unspecialized GB evidence.
- PR5087 is open/draft at e4fd6931657bfc0fc1768072a69a9447448b65b5. Read VERONESE_FINDINGS.md blob88c36a7b511670cb602a2684dab93a4d15bf2913.
- PR5063 and PR5068 already own the local first-fall and signed-fibre degree barriers.

PR5087's complete projected-set gate is on small fields. Its Prime31 section is a filtered-rank diagnostic, not newly measured M31 projected-set correctness or F4 timing. The constant degree-three fall disappears when redundant local chart consequences are preinserted. The result must therefore not be treated as control of global F4 solving degree. This run checked that distinction against the primary report and avoided duplicating those experiments.

Next comparison must preserve the same original-variable constraints and complete geometric/infinity semantics in monolithic and split arms. Require at least50 complete projected-set comparisons for each M31 configuration before timing. One-dimensional slices reduce the monolithic problem to univariate factoring and cannot establish a general coupled-system advantage.

Impact score: 2/10 — Reconciled the current decomposition evidence with the new M31 mandate and identified the exact missing correctness/GB gates; no new decomposition was executed or claimed faster. [pcc:unverified]

## Three-case next-run queue (proposed; not launched)

1. **M31 rootfinding boundary/control panel.** Freeze native complete all-roots inputs at M=1290 and M=1291, with a small M31 control. Preserve caps and resource admission. Include the exact integer-cubic scan where its hypotheses apply, genuine F4-ideal as the Gröbner arm, and clearly labelled enumeration controls. A capped run is incomplete, never empty. This is one candidate panel, not a release qualification.
2. **M31 rational-root support guard control.** Test H with no zero root, a simple zero root, and a repeated zero root, including repeated nonzero roots and nonsplit factors. Compare corrected gcd(H,T^(2^31)-T²) against gcd(H,T^p-T), exact root support, and all output multiplicities. Use original native arithmetic and50 deterministic cases per case family before any component timing.
3. **M31 coupled Semaev projection gate.** After current owner handoff, compare the existing monolithic and projective/Veronese arms at m=3 and4 (extend to5 only within budget), with identical public original-variable restrictions. Use at least two free parameters before the same additional public zero-dimensional constraints, e.g. a common quadratic section; account for degenerate sections and all projective branches. Require50 complete projected-set equalities/configuration. Then instrument actual F4 maximum Macaulay degree, complete wall/work/memory, construction, projection and root recovery. Keep existing factor-base map unchanged.

These proposed tests retain their existing source, ownership and execution gates. None was launched in this review.

Impact score: 2/10 — This written audit clarifies M31 root-support and projected-set contracts, with exact source identities and explicit missing execution. It establishes no new benchmark or complete solver advantage. [pcc:unverified]
