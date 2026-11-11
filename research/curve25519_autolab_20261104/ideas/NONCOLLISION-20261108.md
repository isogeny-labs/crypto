# Curve25519-family: relations from a source other than random collisions (Idea Generator, 2026-11-08)

Status: **proposals only**. Nothing here is a measurement, a result or a status change. A number is one of:
quoted from a committed file (path given, tagged `[pcc:unverified]`); *derived* (arithmetic or argument in this
file, not run); or *predicted* (to be frozen by the Coordinator before any run). I had read, search and web
tools only: no run, no ecdb query, no Conductor check, no edit outside this file.

## 0. The target, in the units that decide how every record reads

Target (owner): counted S at most 0.75 x matched rho-neg. Matched rho-neg S at w = 0: 1.447 at k = 20
(STRUCTURE-R4 section 0 item 2, `ledger.json` tracks.algebraic) and 1.08 / 1.04 / 1.01 at k = 24 / 28 / 32
(OBSTRUCTION.md "How the measurements sit") [pcc:unverified]. Derived arithmetic:

| k | matched rho-neg S | 0.75 x rho (target) | K=1 floor 0.886 | K=2 floor 1.329 (C=1) | K=3 floor 1.662 (C=2) |
|--:|--:|--:|--:|--:|--:|
| 20 | 1.447 | **1.085** | below target: rho overhead 63% | target is below it | below it |
| 24 | 1.08 | **0.81** | target below floor | below | below |
| 28 | 1.04 | **0.78** | target below floor | below | below |
| 32 | 1.01 | **0.76** | target below floor | below | below |

The matched rho varies by curve: `../c1/README.md` ratios imply about 1.59 / 1.13 / 0.98 / 0.91 on its fresh curves
(S divided by ratio), so the targets there are about 1.19 / 0.85 / 0.74 / 0.69. The conclusions below hold either way.

Consequences, within the random-collision model (OBSTRUCTION.md):
- With random relations, no arm with an unknown seed (C >= 1) can reach the target at any k. C = 0 is rho or BSGS.
- At k >= 24 the target is below the K = 1 floor, so rho itself would have to beat its own floor.
- At k = 20 the target (1.085) is above the floor (0.886) only because measured rho carries 0.56 sqrt(r) of
  overhead. Calibrated `bsgs.negation` already measures S about 1.0 on unrelated primes (0.979, 0.765, 1.032, 0.901,
  1.205, 0.966 at 16..26 bits, `research/ecbench_calibration_20261002/table-round1.md` lines 33-38)
  [pcc:unverified]: about 0.69 of 1.447 at k = 20. ecbench README lines 952-957 call that "the known time-memory
  trade, never a finding", and SYNTHESIS section 3 rules "Interleaved BSGS and grumpy giants" "no" for the same
  reason. **A k = 20 pass of the 0.75 line is therefore not evidence of a new relation source.**
- Section 5a band: every S below is also priced at w = 0, 0.1, 1. Matched rho-neg at k = 20 is 1.447 / 1.547 /
  2.448 [pcc:unverified]. A zero-GAE-trial algebraic arm can look good at w = 0 and lose by orders at w >= 0.1
  (SYNTHESIS 4a "unpriced shifting"; Table 2 budget c*).

**One correction to the frame, derived here.** OBSTRUCTION.md's floors are Poisson floors: they hold for random
pair sets. For a d-independent (oblivious) design the pair events are not Poisson, and incidence counting caps the
success probability more tightly than the Poisson law at saturation. NC-1 and NC-2 test whether the floors are floors
of the problem or only of random sets.

## 1. Levers I did not turn into records, and why (dead or closed; one line each)

| lever | status | where it is closed |
|:--|:--|:--|
| Pohlig-Hellman with cofactor 8 | dead: r prime and the target lies in `<G>` (assumption: confirm ecbench draws Q in `<G>`; one grep of the target generator). Cofactor 8 acts only through projection, already in every IC arm (trials per relation 7.66-8.38 vs 1.00) | STRUCTURE-R4 section 3 row "cofactor 8" |
| Montgomery/Edwards birational structure, x-only, T0 on the Kummer line, `xi = x + 1/x` | dead for S: field-level (ratio 1.00 in GAE) or acts outside `<G>`; the linear-2-torsion quotient moves native cost only (see NC-3) | STRUCTURE-R4 rows "x-only", "cofactor 8", "linear 2-torsion quotient"; `../r4-i2/README.md` |
| Equivariant walk over the Montgomery automorphism group | dead: the group is {+-1} on `<G>`; no automorphism of order above 2 in the isogeny class | STRUCTURE-R4 section 2 "No extra automorphisms", row "endomorphisms" |
| Quadratic twist, mixed relations | dead: mixed S3 roots are Frobenius conjugates, rational only at 2-torsion | STRUCTURE-R4 row "quadratic twist" |
| Frobenius structure, Weil descent (Gaudry/Diem) | dead: prime field, no subfield. Extra derived fact: `E(F_{p^2})` has r-part equal to `E(F_p)[r]` because r' != r, so a descent to `F_{p^2}` adds no point of `<G>` | STRUCTURE-R4 row "Frobenius"; SYNTHESIS section 2 |
| Relations from endomorphisms or the class-group action | dead, derived here: any alpha in End(E) acts on `<G>` (the pi = 1 eigenline of E[r]) as a scalar lambda computable from alpha's expansion, so `alpha(P) = [lambda]P` is a tautology with no unknown, and an isogeny to another curve only relocates the same r points: the class has h(O) times r states but r equivalence classes, and canonicalising costs isogeny degree | STRUCTURE-R4 rows "endomorphisms", "isogenies" (Theorem 15.1) |
| xedni calculus (lift to Q, hope for dependence) | dead in the literature: Jacobson, Koblitz, Silverman, Stein, Teske, "Analysis of the xedni calculus attack" (Des. Codes Cryptogr. 20, 2000). Cited from memory, not re-fetched | none in repo |
| Summation polynomials over an unstructured base, MITM k-SUM decomposition | dead at the price band. Derived from SYNTHESIS Table 2: a MITM 2+2 decomposition at m = 4 costs about N^2/2 to N^2 lookups per trial, 0.9k to 1.8k at k = 24 (N about 42) and 13k to 26k at k = 32 (N about 161), against c* about 36 and 88: 25x to 50x over at k = 24 and about 300x at k = 32, growing like sqrt(r). Measured: the pair, triple and MITM oracles are 162x and 566x behind the tuned walk (RESIDUAL_WALKS 10.4-10.5); the arity sweep's best arity at 32 bits is 11.5x rho in its own unit (S3 root solves, MITM table included, so native work is counted there, not a w = 0 reading) | SYNTHESIS Table 2 and R4; `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` sections 10.4-10.5; `research/ic_arity_sweep_20261027/README.md` lines 56-70 |

I found no other lever that is new to the repository and consistent with the obstruction note. The three records
that follow: NC-1 (K = 1 design, the exact bound and control), NC-2 (the same idea at C = 1 and C = 2, which is
index calculus), NC-3 (the one algebraic variant r4-i2 left unbuilt; dead for the 0.75 target).

## 2. Idea records

```yaml
id: NC-1-slope-exclusive-K1-design
title: "At K = 1 the collision floor 0.886 is the value for random pair sets: an oblivious point sequence with distinct pair slopes has first-collision cost down to 0.667 sqrt(r) (the control and bound for NC-2)"
track: generic collision search, K = 1 (no unknown seed). Class: BSGS-family constant-factor result. NOT index calculus. ecbench README 952-957 and SYNTHESIS section 3 (row "Interleaved BSGS and grumpy giants": "no ... never a finding") already rule such rows out of the IC ratchet. Recorded because it is the exact bound and the control that NC-2 needs, and the owner must rule on classification before any ratchet use.
obstruction_attacked: >
  OBSTRUCTION.md derives E[t_K] = sqrt(r) Gamma(K+1/2)/Gamma(K), assuming pair collisions form a Poisson process.
  At K = 1 that is the birthday law for random points, 0.886 sqrt(r) with the negation fold. The first-moment
  floor in AGENTS.md (relations <= gamma P^2/2n, RESIDUAL_WALKS) only gives success probability <= gamma T^2/(2r)
  = T^2/r at gamma = 2, which integrates to a mean cost of at least 2/3 sqrt(r) = 0.667 sqrt(r). Ratio of the two
  constants: 0.752 (a derived number, not evidence of anything by itself).
claim: >
  Let v_t = (a_t, b_t) in Z_r^2 be any d-independent sequence, P_t = a_t G + b_t Q = (a_t + b_t d) G, keyed by x
  (negation fold). A pair (s, t) with b_s != +-b_t collides iff d is one of its two slopes,
  d = -(a_s - a_t)/(b_s - b_t) or d = -(a_s + a_t)/(b_s + b_t) mod r. For d uniform in Z_r^*,
  P(first collision by T) = |Sigma(T)|/(r - 1) EXACTLY, with Sigma(T) the set of distinct slope values of the first
  T points (events for distinct slopes are mutually exclusive in d). So E[cost] = sum_T (1 - |Sigma(T)|/(r-1)), and
  |Sigma(T)| <= T(T-1) gives E >= 0.667 sqrt(r). Random jump vectors give slopes that behave like random draws,
  so E = 0.886 sqrt(r) (control). Claim to test: some design with c <= 4 jump types and at most 4 scalar
  multiplications of set-up has exact E <= 0.84 sqrt(r) at k = 20 and k = 28.
mechanism: >
  d is one uniform element of Z_r. After T oblivious points "success" is the event d in Sigma(T), a deterministic
  function of the design. Repeated slopes waste pairs; random walks repeat at the coupon rate. BSGS is the degenerate
  design: baby steps (j, 0), giants (-m i, 1), cross-pair slopes all distinct, but baby-baby and giant-giant pairs have
  b_s = b_t and are wasted, which is why BSGS-neg sits at 1.0 and not 0.667. Bernstein-Lange, "Two grumpy giants and a
  baby" (eprint 2012/294, abstract read), is a partial derandomisation: at (1.5 + o(1)) sqrt(l) additions, without
  negation, success probability 0.5625 (BSGS), 0.6753 (random walk), 0.71875 (new) against a cap of 1.
prediction:
  stage0_exact_E_in_sqrt_r: {random_jump_walk_control: "0.886 [0.85, 0.92]", bsgs_negation_layout: "1.00 [0.95, 1.05]", best_designed_k20: "0.84 [0.78, 0.90]", best_designed_k28: "0.84 [0.78, 0.90]"}
  stage0_probabilities: {beats_0.886_at_k20_and_k28: 0.55, E_le_0.80: 0.15, E_le_0.72: 0.03}
  stage1_S_w0: {k20: "1.00 [0.88, 1.15]", k32: "0.88 [0.80, 0.97]"}  # E + set-up + verification (about 30 ops = 0.08 sqrt(r) at k = 20) + jump scalar multiplications
  stage1_S_over_rho_w0: {k20: "0.69 [0.61, 0.80]", k24: "0.80 [0.72, 0.92]", k28: "0.85 [0.77, 0.95]", k32: "0.87 [0.79, 0.96]"}
  band: "unpriced about 1 hash insert/lookup per stored point (T about 0.85 sqrt(r)). k = 20: S about 1.00 / 1.09 / 1.85 at w = 0 / 0.1 / 1 against rho-neg 1.447 / 1.547 / 2.448, ratios 0.69 / 0.70 / 0.76. k = 32 ratios about 0.87 at every weight if rho's unpriced work is also about 1 per step (true at k = 20: 2.448 - 1.447)."
  reaches_0.75: "k = 20: on its central estimate it clears 0.75 at w = 0 and 0.1 and misses it at w = 1 (0.76), so no pass across the band; and the same region is bsgs.negation's (0.69). k >= 24: needs S <= 0.76 to 0.81, hence E <= about 0.70 to 0.72, within 8% of the 0.667 bound: probability about 0.03."
discriminating_test: >
  Stage 0 (exact, zero group operations, no ecbench): a native Rust subcommand (for example `curve25519_family
  slope-cover <k> <design>`) takes the curve's r, generates the vectors (a_t, b_t) mod r of a design, marks slopes in a
  bitset of size r (67 MB at k = 32), and reports E = sum_T (1 - |Sigma(T)|/(r-1)) / sqrt(r) plus the design's counted
  set-up in GAE. Designs, fixed before the run: D0 random 16-jump walk (control), D1 bsgs.negation layout (control),
  D2 Bernstein-Lange grumpy giants (published reference, no negation), D3 multi-class stride tiling with c in {2, 3, 4, 8}
  b-classes and stride vectors chosen by a pre-registered budget of hill-climbing on |Sigma| at T = sqrt(r) (costs no group
  operation; exact for every uniform d, so no overfit to a target set). Sizes k = 20, 24, 28 (about 2.6e5, 2.1e6, 7e7
  pairs); k = 32 (4.5e8 pairs) if D3 passes. Cross-check: the Monte Carlo method of NC-2 Stage 0 must agree at K = 1.
  Stage 1 only if D3 beats 0.886: an oblivious `slope-design` arm in ecbench's generic family, panel {D3,
  bsgs.negation_v2, bsgs.interleaved_v2, rho.negation, rho A/A}, 32 fresh targets x 2 rounds, `ecbench verify --replay`,
  band at w = 0, 0.1, 1, k = 20 and 28.
falsification: >
  Dead at Stage 0 if the best designed E at k = 20 and k = 28 is >= 0.886, or the D0 control is outside [0.85, 0.92]
  (the exclusive formula is wrong). Dead at Stage 1 if measured S exceeds Stage 0 E + set-up by more than 5%, if any
  recovered log fails [d]G = Q, or if S/rho at k = 32 is >= 0.95. Not a ratchet row at any stage if the owner
  classifies it as BSGS-family.
cost_band: >
  Counted: one addition per stored point; set-up scalar multiplications (about 1.5 log2 r each: 25.5 at k = 20, so four
  jumps of two multiplications cost 204 ops = 0.56 sqrt(r): hence c <= 4 and small-coefficient jumps; the design
  objective is E + set-up/sqrt(r), not E alone); one final [d]G = Q check. Unpriced: one hash insert plus lookup per
  point; memory about 0.85 sqrt(r) entries (23k at k = 32; bytes are not charged under the band rule). Priced at
  w = 0, 0.1, 1 identically for D3, bsgs.negation_v2, bsgs.interleaved_v2 and rho.negation.
controls: [D0 random walk (must give 0.886), D1 BSGS-neg layout (must give 1.0), D2 grumpy giants, rho.negation, rho A/A, "randomised-b control: replace the b values of D3's vectors by random ones; E must return to the Poisson value"]
risks:
  - "Novelty is limited. Bernstein-Lange (partial derandomisation without negation) and Galbraith-Wang-Zhang 2015/605 (interleaved BSGS with negation, 'average case below rho in the fully-optimised context' as quoted in SYNTHESIS section 3; PDF unreadable there, so their constants are unchecked) may already put a K = 1 design below rho-neg. What NC-1 adds is the exact slope-coverage figure of merit (zero-cost Stage 0) and its transfer to K >= 2 in NC-2, not the K = 1 idea itself."
  - "Classification: BSGS family, memory about sqrt(r)."
  - "Set-up at k = 20 can eat the gain (0.56 sqrt(r) for four full-size jumps)."
  - "A perfect Sidon-in-Z_r^2 design may not be generable at one addition per element; the prediction assumes partial derandomisation like Bernstein-Lange's."
closest_baseline: "Bernstein-Lange 2012 (eprint 2012/294; abstract only, body unread); Galbraith-Wang-Zhang 2015/605 (not read); ecbench `bsgs.negation` (1.0) and `bsgs.interleaved` (4/3), SYNTHESIS section 3 row 'Interleaved BSGS and grumpy giants'; Shoup 1997 and Nechaev 1994 for the T^2/r bound (from memory, not re-fetched)"
code_needed: {files: ["a new native Rust subcommand in src/bin/curve25519_family.rs (Stage 0)", "an `ecbench` generic-family method plus families.json entry (Stage 1 only)"], language: "Rust only; no Python"}
lean_obligations:
  - "slope_union_card: for r prime and a family of pairwise non-proportional difference vectors, #{d in Z_r : exists a pair with delta . (1, d) = 0} equals the number of distinct slopes (finite combinatorics; makes E exact)"
  - "first_moment_bound: P(success by T) <= gamma T^2/(2r) for an oblivious design. The statement for adaptive algorithms (Shoup/Nechaev) is a heuristic here, not a Lean claim"
parity_statement: "If Stage 0 passes at 0.84, k = 32 lands near 0.87 of matched rho-neg, still above the 0.75 line."
reduces_to: "BSGS family; Shoup-Nechaev first-moment bound"
```

```yaml
id: NC-2-designed-rows-C1-C2
title: "Designed (oblivious) point sequences at C = 1 and C = 2: the Gamma floors 1.329 and 1.662 are Poisson floors, and the incidence bound allows about 1.01 and 1.26"
track: generic decomposition with unknown seeds, C = 1 (owner-approved boundary track, `../c1/README.md`) and C = 2 (main track). This IS index calculus: the rows carry seed columns and the guard (rows with zero seed part are never used) holds. Any measured gain over the current IC baseline counts under CLAUDE.md, even short of 0.75.
obstruction_attacked: >
  The collision floor of OBSTRUCTION.md for K = C + 1 unknown logs. Derived here, with negation: M = T^2 pair
  relations (two per pair), lambda = T^2/r, the same lambda as OBSTRUCTION's count. The true point (d, s_1..s_C) is
  uniform on r^K points. Each pair relation is an affine hyperplane of r^(K-1) points, and rank K needs the true point
  on K independent ones. For an oblivious design the number of points on at least K hyperplanes is at most C(M,K)
  (K hyperplanes meet in at most one point) and at most M r^(K-1)/K (incidence count). Dividing by r^K:
    K = 2: P(rank 2 by T) <= min(lambda^2/2, lambda/2, 1)
    K = 3: P(rank 3 by T) <= min(lambda^3/6, lambda/3, 1)
  and E = integral of (1 - P) dT gives E >= 1.01 sqrt(r) at K = 2 and E >= 1.26 sqrt(r) at K = 3 (integrals
  recomputed by me: 0.900 + 0.1095 = 1.0095 and 1.109 + 0.152 = 1.261), against the Poisson floors 1.329 and 1.662.
  At small lambda designed and Poisson agree; the gap is entirely in saturation, where random sets waste incidences on
  points covered three or more times. A design that covers each parameter point about K times (no K+1 pair-hyperplanes
  concurrent) wastes none. This reverses my first draft of this file, which wrongly said exclusivity gives nothing at
  K >= 2; the reviewer's check, recomputed here, shows it does at the bound level.
claim: >
  There is an oblivious design v_t = (a_t, b_t, e_t) in Z^(2+C) (jumps independent of the point) whose exact expected
  collection cost to rank K, measured in stored points, is at most 0.92 of the Poisson value at C = 1 (E <= 1.22 sqrt(r))
  and at C = 2 (E <= 1.53 sqrt(r)), including the guard (every counted pair difference has a nonzero seed part), at
  r = 131203 and r about 2^28. The bound is 0.76 of Poisson, so the claim asks for about a third of the possible gain.
mechanism: >
  Pair-differences (Delta_a : Delta_b : Delta_e) are points of P^2(Z_r) at C = 1; each is a line in the (d, s) plane.
  The number of points with two or more lines is what rank 2 needs. A random set puts Poisson(lambda) lines on each
  point; a set of dual points with no three collinear (an arc; a conic has r + 1 points) puts at most two. Whether an
  oblivious sequence at one addition per point has near-arc pair-differences is open, as at K = 1. A polynomial design
  v_t = (t^3, t^2, t) makes all pair-lines distinct automatically (the line of a pair is determined by (s + t, s t)),
  but costs three additions per point by finite differences, which multiplies the op count by 3 and kills the gain; so
  the design search must be cheap-to-generate. Prior remark: SYNTHESIS section 1.1 says box tiling at C >= 2 is "random
  sampling to second order" because the kernel lattice is unknown. That concerns tiling the group by box translates.
  Here the figure of merit is the incidence geometry of pair-hyperplanes in parameter space, which depends only on the
  known design vectors, not on the unknown logs, so the objection does not transfer; it is untested whether any cheap
  design realises it.
prediction:
  stage0_exact_E_in_sqrt_r: {random_jump_control_C1: "1.33 [1.28, 1.38]", random_jump_control_C2: "1.66 [1.60, 1.72]", best_designed_C1: "1.22 [1.05, 1.33]", best_designed_C2: "1.53 [1.30, 1.66]"}
  stage0_probabilities: {C1_E_le_1.22: 0.2, C1_E_le_1.15: 0.07, C2_E_le_1.53: 0.15}
  stage1_S_over_rho_w0_if_stage0_passes: {C1_k20: "1.30 [1.15, 1.45] (from 1.40, `../c1/README.md`: set-up unchanged)", C1_k32: "1.55 [1.35, 1.75] (from 1.69)", C2_k32: "1.7 [1.5, 1.9] (from 1.8-2.0, `../multicurve/README.md`)"}
  reaches_0.75: "no, by a wide margin: even the bound (1.01 at C = 1) is above 0.78 to 0.81 at k >= 24. The value is a measured improvement over the C = 1 and C = 2 baselines, and a test of the floor."
discriminating_test: >
  Stage 0 (no group operations, no bitset over r^K): Monte Carlo. Sample (d, s_1..s_C) uniformly; run the design on
  scalars c_t = a_t + b_t d + sum e_tc s_c mod r; detect x-class collisions (c_t = +-c_u) with a hash; maintain the
  row matrix over (d, s) and stop at rank K, discarding rows with zero seed part. Exact in distribution for an
  oblivious design, O(T) per sample, 2,000 samples give +-0.01 on the mean. Designs fixed before the run: D0 random
  jumps (control, must give 1.33 and 1.66), D1 the existing `wwalk` C = 1 jump set (control), D2 BSGS-style stride along
  the seed axis (SYNTHESIS 1.1's tiling, control), D3 designs from a pre-registered search over cheap-to-generate
  jump sets (at most 16 jump types, one addition per point) scoring Monte Carlo E. Sizes k = 20, 24, 28. The same
  harness at K = 1 cross-checks NC-1's bitset. Stage 1 only if D3 is at least 0.92 of D0 at two sizes: a `wwalk`
  option with the designed jump set, one panel per size on the C = 1 frozen protocol (fresh curves, pre-registered arm).
falsification: >
  Dead at Stage 0 if the best D3 is within 3% of D0 at C = 1 and C = 2 at k = 20 and 28 (random-like), or if D0 is
  outside the stated control intervals (Gamma floors wrong), or if the designed gain exists only with more than 1
  addition per stored point. Dead at Stage 1 if S/rho does not fall by at least 5% against the same-panel baseline arm
  on fresh curves, or any row has zero seed part and is used, or any unverified solve.
cost_band: >
  Counted: one addition per stored point (a design needing finite differences counts them), jump set-up, row checks,
  [d]G = Q. Unpriced: hash insert/lookup per point, 3 to 4 column row operations; priced at w = 0, 0.1, 1 on the design,
  the baseline `wwalk` arm, rho.negation and rho A/A alike.
controls: [D0, D1, D2, inc-aa, rho.negation on the same targets, "randomised-e control: replace the seed coefficients e_t of D3's vectors by random ones; E must return to the Poisson value"]
risks:
  - "Achievability: a near-arc of about r dual points from one-addition jumps may not exist; the gain may be zero (prior probability of E <= 1.22 at C = 1: 0.2)."
  - "The seed-part guard removes pairs with e_s = e_t; a design with few distinct e values wastes a fraction 1/c of pairs."
  - "The bound is on the probability of rank K by T, not on a stopping rule's expectation with set-up; Stage 0 reports E and set-up separately."
  - "The table-and-probe arms are special cases of an oblivious sequence; a designed all-pairs sequence is SYNTHESIS R1 (`cwalk`) with a chosen jump set, so R1's classification question (all-pairs rows) applies and the owner rules."
closest_baseline: "SYNTHESIS R1 `cwalk` (all-pairs random walk, Poisson floor); `../c1/README.md` C = 1 arms (S 2.227 / 1.814 / 1.711 / 1.545, 68% above the floor at k = 20, 16% at k = 32); Bernstein-Lange 2012 for the K = 1 analogue; Kuhn-Struik (SAC 2001) multi-collision rho"
code_needed: {files: ["a native Rust Monte Carlo subcommand (Stage 0), shared with NC-1", "a designed-jump option in the wwalk oracle, `src/cryptanalysis/ic_framework/plugins.rs` (Stage 1 only)"], language: "Rust only; no Python"}
lean_obligations:
  - "incidence_bound_K2: for M lines in an affine plane over a field F, #{points on at least two lines} <= min(C(M,2), M |F| / 2) (finite geometry; gives P <= min(lambda^2/2, lambda/2))"
  - "the K = 3 analogue and the integral values 1.01 and 1.26 are arithmetic on these bounds, not Lean claims; the Poisson/Gamma comparison stays a heuristic"
parity_statement: "Cannot reach matched rho: the bound at C = 1 (1.01 sqrt(r)) is above the 0.75 line at k >= 24, and a cheap design realises only part of it. It is the one identified lever inside the IC tracks that could lower a floor."
reduces_to: "Incidence geometry of pair-hyperplanes; SYNTHESIS R1 with a designed jump set; NC-1 at K = 1"
```

```yaml
id: NC-3-two-variable-jacobi-intersection-quotient
title: "Build the (c, d) two-variable Jacobi-intersection system on E' = E/<T0> that sees the whole (Z/2)^2, the one algebraic variant R4-I2 left unbuilt (likely dead for the 0.75 target)"
track: algebraic (summation-polynomial decomposition); IC-only; acts on native solver cost only
obstruction_attacked: >
  Native F4 cost of the algebraic incumbent: k = 20 `pkm-iso-t6-g4-s1093` S = 26.448 / 1.073e6 / 1.073e7 at
  w = 0 / 0.1 / 1 against rho-neg 1.447 / 1.547 / 2.448 (STRUCTURE-R4 I2 text, `ledger.json`) [pcc:unverified].
  R4-I2 Stage 0 found the single-coordinate c-tree of the JI model is an Edwards tree of a third curve
  (`../r4-i2/README.md` section 2), so qdim fell by 2^(m-1) and D equalled arm A's. A relation that sees all of
  (Z/2)^2 needs both c and d per summand (they lie on b c^2 - a d^2 = b - a).
claim: >
  At m = 3 and d' in {3, 9} (cells that finish in r4-i2) the system with two variables per summand, quotiented by the
  full sign group, has D at most 4 (arm A: 5; unquotiented: 6) and a quotient dimension strictly below arm A's 3. At
  m = 2 its widest matrix is at most 0.5 of arm A's (arm A measured 0.62 of E1's at d' = 9 and 0.68 at d' = 27,
  `../r4-i2/README.md` section 3). It cannot change the w = 0 term.
mechanism: >
  The translations by the three rational 2-torsion points of E' act on (s, c, d) as (s,-c,-d), (-s,-c,d), (-s,c,-d)
  (verified in r4-i2 self_check). One coordinate sees one sign; the pair (c, d) sees the group. Quotient variables are
  c^2, d^2 and signed products. The w = 0 cost is the trial count (about 18x rho) and does not depend on the solver.
prediction:
  stage0: {m3_D: "4 [3, 5] (claim holds if <= 4)", m3_qdim_d3: "2 [1, 3] (claim holds if < 3)", m2_widest_ratio_vs_armA: "0.5 [0.3, 0.9]", probability_stage0_passes: 0.25}
  S_over_rho_at_k20_if_stage1_runs: {w0: "18.3 [17.0, 19.5] (unchanged: trial count)", w0_1: "1.7e5 to 6.9e5 (6.9e5 = 1.073e6/1.547, at best x0.25)", w1: "5e5 to 1.8e6 (4.4e6 at best x0.25)"}
  reaches_0.75: "no: probability below 1e-4 at every weight. SYNTHESIS Table 2 puts c* below 1000 native operations per trial for m <= 6; F4 on these systems is 10^3 to 10^4 above it before any quotient."
discriminating_test: >
  Stage 0 only: a native Singular generator (not the Python `gen.py`/`run.py` of r4-i2: AGENTS.md bars extending Python
  research tools, so that harness must be replaced before extending; `../r4-i2/ji.sing` and
  `research/ic_tree_gluing_20261103/round7_edwards/gen7.sing` are the starting points), planted-point check before
  every write, msolve with the r4-i2 flags, cells m in {2, 3} x d' in {3, 9, 27} x arms {E1Q, JI-(c,d)-Q} x 2 targets,
  1,200 s, 2 GiB. No ecbench run unless Stage 0 shows D at least one below arm A and the widest matrix at most half.
falsification: "Dead if D(JI-(c,d)-Q) >= D(E1Q) in every finished cell and the m = 2 widest-matrix ratio is above 0.8. Stage 1 would be dead on arrival at the k = 20 band: the ratio to rho stays above 1e5 at w >= 0.1."
cost_band: "Stage 0: at most 24 cells x 1,200 s on one core, no GAE (diagnostic). Counted if Stage 1 ran: target and base points, isogeny images (few field operations each, native), relation trials, the final [d]G = Q check. Unpriced at w = 0: F4 multiply-adds. Priced at 0.1 and 1 by `ecbench band`; rho-neg and the unquotiented incumbent priced identically."
controls: [E1 and E1Q on the same base, the single-coordinate JI arm, the T arm, a second target per cell, inc-aa in any Stage 1]
risks: ["The two-variable system has twice the unknowns per summand; the degree may rise (r4-i2 saw cap hits at d' = 27 for m >= 3).", "No factor-base plug-in closed under the sign action exists (r4-i2 section 5), so Stage 1 needs one; no code is proposed for it here.", "The unquotiented quotient dimension of the two-variable system is not known; the claim is stated against arm A's measured 3."]
closest_baseline: "`../r4-i2/README.md` (arm A, arm B falsified); round7_edwards arms E1, E1Q (#3408); FGHR J. Cryptology 2014 (cited via R4-I2)"
code_needed: {files: ["a new Singular generator under a new research directory (Stage 0)"], language: "Singular for Stage 0 (computer algebra, not Python); Rust for any pkm-semaev model option (Stage 1 only)"}
lean_obligations: ["quotient_fibre: the quotient map by the sign group has fibres of the size of the group orbit on solutions with all coordinates nonzero (finite algebra; exact count to be fixed by the generator once the unquotiented system is built)"]
parity_statement: "Cannot reach matched rho: the w = 0 gap (18x, trial count) is untouched and the native term is 5 orders above rho at w = 0.1. At best it ratchets the algebraic track's own native cost by a factor near 2 to 4."
reduces_to: "FGHR symmetry reduction, with R4-I2's 2^(m-1) result as the baseline it must beat"
```

## 3. Ranking and what to do first

1. **NC-2 Stage 0, with NC-1 as its K = 1 cross-check.** One native Monte Carlo tool, zero group operations. The
   control rows (Gamma floors 1.329 and 1.662 at C = 1 and 2, 0.886 at K = 1) validate it before a designed row is
   read. If the best cheap design is within 3% of random, the question closes with a number and the Poisson floors are
   confirmed as floors for oblivious designs at the sizes tested.
2. **Not proposed: a w = 0-only algebraic arm.** Derived from SYNTHESIS Table 2: an m = 4 arm over about 25 base
   points at k = 20 would need on the order of 100 to 150 counted group operations (about 0.3 to 0.4 sqrt(r)) and would
   read about 0.2 to 0.3 of matched rho at w = 0, while its MITM lookups (about 1k per trial) price it above rho at
   w = 0.1 and 1. The Coordinator should treat any such w = 0 number as an unpriced-shifting artefact (SYNTHESIS 4a).
   This is derived and unmeasured. The arity sweep's 11.5x at 32 bits uses a native-inclusive unit and does not test it.
3. **NC-3** only as an algebraic-track ratchet, with the expected distance to rho stated above.

Recommended obstruction wording (I may not write `problems/`): "The 0.75 x matched-rho target is below the
random-collision K = 1 floor at k >= 24, and no curve structure of the family lowers it (STRUCTURE-R4 section 3). The
only identified route below the random floors is a d-independent design whose pair-hyperplanes avoid repeated coverage;
at K = 1 that is a BSGS-family result, at C >= 1 it is a bound-level gap (1.01 against 1.329 at C = 1) with no known
construction." Status: derived argument; exit criterion: NC-2 Stage 0.

## 4. Sources checked

- research-kb MCP: **unavailable in this runtime** (no tool loaded). The lead-atlas glob
  `/home/user/*/knowledge/leads/ecdlp/*` and a glob of `research/curve25519*/README.md` both timed out (ripgrep 20 s),
  so the atlas and sibling checkouts were not read and `--match` was not run. No ecdb query.
- Repo files read: `research/curve25519_autolab_20261104/ideas/OBSTRUCTION.md`, `STRUCTURE-R4.md`, `SYNTHESIS.md`, first
  80 lines of `PROPOSALS-R3.md`; `c1/README.md`; `r4-i2/README.md`; `research/curve25519_torsion_leads_20261103/LEADS.md`;
  `docs/ecbench/README.md` lines 479-480, 940-979; `research/ecbench_calibration_20261002/table-round1.md` lines 33-38
  (grep); `research/ic_arity_sweep_20261027/README.md` lines 1-80; AGENTS.md "Preserve scope", "Name the obstruction",
  "Implementation language", "The rule: boundary, table, ratio", 5a; grep hits in `RESEARCH_RESIDUAL_WALKS.md` lines 860-975.
- Novelty grep for `grumpy|anti-collision|Sidon|interleav`: `research/notes/index-calculus` (first 12 hits, all unrelated
  "interleaved A/B runs"); `docs/ic` (10 files listed, head-limited, not opened, likely the same A/B sense). Not exhaustive.
  I therefore make no "not tried" claim about slope-distinct designs beyond what SYNTHESIS section 3 says of BSGS rows.
- Web: [Bernstein-Lange, Two grumpy giants and a baby, eprint 2012/294](https://eprint.iacr.org/2012/294): abstract and
  metadata only; I used only its stated probabilities 0.5625 / 0.6753 / 0.71875. My first recollection of a "1.25 sqrt(l)"
  figure is not in the abstract and is not used.
- From memory, not re-fetched: Shoup 1997 and Nechaev 1994 (generic bound), Jacobson-Koblitz-Silverman-Stein-Teske 2000
  (xedni), Galbraith-Wang-Zhang 2015/605 (PDF not read).
- Not done: no run, no Conductor check, no session log, no lead-atlas match, no PCC claim ids (every cited number is
  [pcc:unverified] or derived here), no ecdb lookup. The incidence bounds and their integrals are my hand derivations
  (recomputed once; the K = 2 and K = 3 values 1.0095 and 1.261 agree with the reviewer's check). Section 3's m = 4
  arithmetic and the NC-1 and NC-2 predictions have about plus or minus 10% error.
