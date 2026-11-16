# PREREGISTRATION: L2-binary-xonly-twist-filter

Frozen: 2026-11-07, before any measurement data were computed.
Lead: twist work in x-only relation search on binary curves.
Parent study (read for context only): `.../scratchpad/wt/research/twist_ecdlp_20261027` (E0 separation lemma, E1 prime-field census).

## 0. Status, scope and honesty notes

* No measurement data exist yet. The only code run before freezing was a feasibility smoke test at k = 17 in PARI/GP 2.15.4
  (`frozen_pre_data/smoke_identities_k17.gp`, ≤ 3000 random samples per identity). It checked that (i) the lift criterion
  `Tr(x + a + b/x^2) = 0` agrees with `ellordinate`, (ii) the binary Semaev S_3 below vanishes on sums of curve points,
  (iii) `x(P + T0) = sqrt(b)/x`, and (iv) the trace test of the S_3 quadratic separates E-type from twist-type x1.
  Its output was discarded and is not evidence. Consequence: the hypotheses below are theory-derived (with the toy
  sanity check above), not blind predictions. They are still falsifiable by the main run, which uses different
  sizes, seeds and sample counts.
* The hypotheses concern the **solution set** of S_{m+1} over V (solver-independent) and the **cost of an enumeration
  solver** (solver-specific). Results for Groebner/Weil-descent solvers are NOT measured here. Anything said about them is
  labelled extrapolation or open.
* Whether the repository's current index-calculus baseline already filters non-lifting x-values has NOT been checked (the
  main repo is not read or built here). Arm U below is "the same solver without a filter", not "the repository baseline".
  Any improvement claim is therefore scoped to "vs the unfiltered arm of this same instrument". It is not a claim against
  the repository baseline, and the filter may be a relabelling if the baseline already does it (open item O1).
* Rules obeyed: compute only in `gp -q` (`default(parisizemax,1000000000)`) or a tiny dependency-free Rust crate in this
  directory (`target/` deleted afterwards); Python (`.../scratchpad/venv/bin/python`) only to plot recorded values; main
  repo crates never built; everything this experiment writes stays under 150 MB; no per-target files; fixed seeds;
  operation counts are the primary metric, wall time secondary and labelled contention-affected (4 shared cores, observed load
  above 10 at freeze time).

## 1. Setting and notation

* q = 2^k, k odd. E_{a,b}: y^2 + xy = x^3 + a x^2 + b over F_q, b ≠ 0, ordinary. Primary family a = 1 (Tr(1) = 1 for odd k), so
  #E = 2n with n odd, and #E' = 2^{k+1} + 2 − #E ≡ 0 mod 4. Curves are required to have n prime.
* For x ≠ 0: x is E-type (x = x(P), P in E(F_q)) iff Tr(f(x)) = 0 with f(x) = x + a + b/x^2; otherwise twist-type (x = x(P),
  P in E'(F_q), seen in E(F_{q^2}) with Frobenius pi(P) = −P). Since Tr(z^2) = Tr(z): Tr(f(x)) = Tr(x) + Tr(a) + Tr(sqrt(b)/x).
  Exact counts: #{x ≠ 0 : E-type} = (#E − 2)/2, so the per-curve E-type density is mu_E = (#E − 2) / (2(q − 1)).
* x = 0 is T0 = (0, sqrt b), the unique nonzero point of E[2], rational on both E and E' (pi(T0) = T0 = −T0): "neutral".
  Factor bases use V* = V \ {0}. 0 is never a coordinate.
* Semaev (verified at toy size, to be re-verified in the run by explicit group-law check of every sampled solution):
  S_3(x1,x2,x3) = (x1 x2 + x1 x3 + x2 x3)^2 + x1 x2 x3 + b. As a polynomial in x3: A x3^2 + B x3 + C with
  A = (x1+x2)^2, B = x1 x2, C = B^2 + b.
* Factor base V: an F_2-subspace of dimension l; 2^l = |V|. Two V types (stratified, curve index parity): "rand" (random seeded
  subspace) and "poly" (span{1, t, ..., t^{l−1}} in the polynomial basis of the lexicographically first minimal-weight irreducible
  of degree k; cross-checked in GP).
* Decomposition of a target R = a_j P + b_j Q (R in the order-n subgroup G = <P>, so R is E-type and not 2-torsion):
  R = ±P_1 ± ... ± P_m with x(P_i) in V*. m = 2 or 3. l = ceil(k/m) ("standard balancing": |V|^m ≥ q so about one solution per target).
* Solutions: x-tuples in (V*)^m solving S_{m+1}(x_1..x_m, x(R)) = 0 over F_q. A solution is a **multiset** {x_i}; "unique
  solutions" deduplicate multisets; "raw" counts keep duplicates.

## 2. Theory and derivations

### 2.1 Binary separation lemma (analogue of E0)

**Lemma B.** Let R in E(F_q), x_i in F_q^*, P_i in E(F-bar) with x(P_i) = x_i, signs e_i, and R = sum e_i P_i. Let I_E, I_T be the
index sets of E-type and twist-type x_i, A = sum over I_E, B = sum over I_T (each with its signs). Then 2B = O and
A = R − B. Hence B ∈ E[2] = {O, T0}.

*Proof.* Apply Frobenius pi to R = A + B. pi fixes R and each E-type P_i and negates each twist-type P_i, so R = A − B.
Subtract the two equations: 2B = O. QED.

**Consequences (m = 2, 3), with all x_i ≠ 0 and R not in E[2]:**
* |I_T| = 1: B = ±P_i ∈ {O, T0} forces x_i ∈ {0, ∞}. Excluded.
* m = 2, |I_T| = 2: I_E is empty, so A = O, so R = B ∈ {O, T0}. Contradiction. So **no S_3 solution over F_q has a twist-type coordinate.**
  Equivalently, for a twist-type x1 the quadratic S_3(x1, X, x(R)) has conjugate roots in F_{q^2} \ F_q, so its trace test fails.
* m = 3, |I_T| = 3: A = O, so R in {O, T0}. Contradiction.
* m = 3, |I_T| = 2 (say I_T = {i, j}): B = ±P_i ± P_j ∈ {O, T0}. Since x(P + T0) = sqrt(b)/x(P) (derived from the chord formula),
  either x_j = x_i, or x_j = sqrt(b)/x_i. The single E-type P_k then satisfies ±P_k = R − B, so x_k = x(R) (case B = O) or
  x_k = sqrt(b)/x(R) (case B = T0).
* So **the complete set of S_4 solutions with a twist-type coordinate is the degenerate family**
  D = { x_i = x_j (any x_i), x_k = x(R) } ∪ { x_j = sqrt(b)/x_i, x_k = sqrt(b)/x(R) }.
  These are trivial relations (P − P + R − R). They need x(R) in V or sqrt(b)/x(R) in V. For a generic target neither holds (probability about 2^{l−k} each).
* m = 4 (not run): D-like solutions (x_i = x_j twist, plus a genuine S_3(x_k, x_l, x(R)) solution) exist, so waste first appears at m ≥ 4 and is again degenerate.

**Comparison with the prior-study analogue.** Over a prime field with E(F_p)[2] = O the same proof gives B = O, so only the family
x_i = x_j, x_k = x(R) is exceptional. Over F_{2^k} E[2] = {O, T0} is never trivial, but T0 has x = 0, which is excluded from V*, so the
nonzero-x exceptional set is the same size as the prime case. The binary analogue differs only in the second family
(x_j = sqrt(b)/x_i), which has no prime analogue when E(F_p)[2] = O.

**Chained solver completeness (m = 3).** Of three points at most two types exist, so some pair among (1,2), (1,3), (2,3) has equal type. For
an equal-type pair X = x(P_i + P_j) is in F_q. Hence enumerating all unordered pairs {x_i, x_j} in V* x V*, solving the quadratic for X, then
the quadratic S_3(x_k, x(R), X) = 0 for x_k, finds **every** F_q solution multiset. Pairs of mixed type have a conjugate X and fail the
stage-1 trace test.

### 2.2 Numeric predictions

**H1 (solutions).** Over all unflagged targets (x(R) not in V, sqrt(b)/x(R) not in V), the number of unique S_{m+1}
solutions in (V*)^m with at least one twist-type coordinate is **exactly 0** for m = 2 and m = 3, so waste fraction w_m = 0.
Naive coin-flip model for comparison (each coordinate lifts independently with probability 1/2): the fraction of non-liftable
solutions would be 1 − 2^{−m} = 0.75 (m = 2) and 0.875 (m = 3); pure-twist 2^{−m} = 0.25 and 0.125; mixed 0.5 and 0.75. The lemma
predicts 0, 0, 0. The data discriminate by orders of magnitude.

**H2 (factor base columns).** The fraction f of twist-type elements in V* has mean 1 − mu_E (exact per curve, computed from
#E; at most 2^{−k/2} from 1/2) and standard deviation sqrt(mu(1−mu)/s) with s = 2^l − 1, up to a finite-population factor 1 − O(2^{l−k}).
For rand V this variance is exact (pairwise inclusion probabilities of a uniform random subspace match the hypergeometric ones). For poly V
with uniform b it is the pairwise-independent variance. Predicted z = (f − (1 − mu_E)) / sd has mean 0 and sd 1 (accepted range for
the pooled sd: [0.8, 1.25]). Practical size: sd of f is about 0.022 at l = 9 and 0.0020 at l = 16. So about half the columns are dead for an R in E(F_q).

**H3 (counted cost of the trace filter, enumeration solver).** Define the instrument costs, counted by instrumented primitives (section 3):
inv(k) = (k−1) S + (floor(log2(k−1)) + popcount(k−1) − 1) M (Itoh–Tsujii);
a = inv + 3M + 3S + 1T (one failed or passed quadratic attempt: leading terms, trace test);
h1 = (k−1) S + inv + 2M (half-trace and root formation), h2 = h1 + 2(k−l) (two membership tests); pre = inv + 1M + 1T (precompute of
one trace of f(x)) where S, M, T are native operations of cost 1 each (T = trace/parity/AND-popcount).
* m = 2: per attempted x1 the unfiltered arm costs a, plus h2 if x1 is E-type (probability about 1/2); the filtered arm skips twist-type x1.
  Solver calls (attempts) ratio U/F = |V*| / |V_E| → 1/(E-type fraction) ≈ 2.0. Cost ratio (many targets) R2 = (2a + h2)/(a + h2).
* m = 3: unordered pairs. Probabilities: pair types EE 1/4, TT 1/4, mixed 1/2. Stage 1 costs a for every pair. Same-type pairs pass and cost h1; each of the 2 roots X goes
  to stage 2 costing a (twist-twist: fails, no rational x3) or a + h2 (EE: passes). Mean unfiltered cost per pair 2a + h1/2 + h2/2; filtered (EE pairs only, 1/4 of
  all) per original pair (3a + h1 + 2 h2)/4. Pair-count ratio → 4.0. Cost ratio R3 = (8a + 2h1 + 2h2)/(3a + h1 + 2h2).
* Precompute of V_E: |V| evaluations of Tr(x) + Tr(sqrt b / x) (cost pre each), once per V, amortised over T targets.
* Predicted numbers (hand-derived; the generator `frozen_pre_data/prediction_model.py`, output `prediction_table.txt`):

| m | k | l | a | h1 | h2 | R (cost ratio U/F) | unique solutions per target (derivation below) |
|---|---|---|---|---|---|---|---|
| 2 | 17 | 9 | 27 | 38 | 54 | 1.333 | 1 |
| 2 | 19 | 10 | 30 | 43 | 61 | 1.330 | 1 |
| 2 | 23 | 12 | 35 | 52 | 74 | 1.321 | 1 |
| 2 | 29 | 15 | 41 | 64 | 92 | 1.308 | 1 |
| 2 | 31 | 16 | 44 | 69 | 99 | 1.308 | 1 |
| 3 | 17 | 6 | 27 | 38 | 60 | 1.724 | 0.333 |
| 3 | 19 | 7 | 30 | 43 | 67 | 1.723 | 0.667 |
| 3 | 23 | 8 | 35 | 52 | 82 | 1.707 | 0.333 |
| 3 | 29 | 10 | 41 | 64 | 102 | 1.688 | 0.333 |

  Prediction: measured per-target native-operation ratio U/F (precompute amortised, excluding lifts and linear algebra) within ±10% of the table value in each cell, and the
  unfiltered/filtered **solver-call** ratio 2.0 (m = 2) and 4.0 (m = 3) up to the E-density factor (within the sampling band of H2).
  Note the call ratio exceeds the cost ratio because a failed attempt still pays the inversion (cost a). The filter saves the attempts, not their
  inversion share.
* Unique solutions per target: m = 2: ordered solutions ≈ |V_E| · 2 roots · |V|/q = 2^{2l−k}; unordered half that → 2^{2l−k−1} = 1 for odd k, l = (k+1)/2.
  m = 3: found (pair, X, x3) triples ≈ (|V|^2/2)(1/4)(2)(2)(|V|/q) = |V|^3/(2q); each multiset is found from 3 pairs → 2^{3l−k}/6.

**H4 (solution-side trace filter, arm S).** Checking Tr(f(x_i)) on each solution coordinate before lifting avoids **0** lift attempts (all solutions lift, by H1) and adds
m T-operations per raw solution, so its cost ratio S/U ≥ 1 and ≤ 1 + 0.05 (the added operations are about m per raw solution against thousands of attempts).

**H5 (prime-field control).** Matched size (p about 2^k, p ≡ 3 mod 4, prime-order curve so E(F_p)[2] = O, same |V| = 2^l, same targets count, V a random set of x-values): the identical
statement holds. 0 non-degenerate solutions with a Legendre-nonresidue f(x_i), the family x_i = x_j, x_k = x(R) the only exception; twist density 1/2 (H2 analogue with exact mu from #E); the filter saves
solver attempts by the same call ratios (2 and 4). The matching is on structure, not on cost (the prime-field quadratic costs differ), so for the prime cost ratio the registered
formula (2a + h)/(a + h) and (8a + 2h1 + 2h2)/(3a + h1 + 2h2) is applied to the **measured** per-attempt means (consistency check, not an independent prediction).
Prior study E1 saw almost no waste at prime fields; this control repeats that with the new instrument.

**H6 (IC vs rho, distance remaining).** Prediction (qualitative, from magnitudes below): at c = 1 GAE per native operation rho is cheaper than every IC arm in every cell by a factor of at least 100;
at c = 0 group-operation counts alone may make m = 3 IC cheaper than rho for larger k (G_IC ≈ (relations) · (≈3 targets + about 4 sign-check additions) vs G_rho ≈ sqrt(pi n)/2; e.g. k = 29, m = 3: about 3.6·10^3 vs 1.45·10^4),
so such cells are expected to be **unordered across the band** and reported as such, not ranked. The derivation uses 1/(unique solutions per target) targets per relation and N_rel ≈ |V|/2 relations.
Rho (negation map, r = 16): mean group additions in [0.8, 1.6] x sqrt(pi n)/2 (derivation: random mapping rho length sqrt(pi n/2); the negation map saves sqrt 2;
DP/restart overhead and fruitless-cycle waste raise it). This is a **sanity check of the baseline implementation**, not a hypothesis about twists.

### 2.3 Extrapolation to ECC-2K130 (labelled extrapolation, not measured)

ECC2K-130 lives over F_{2^131} (k = 131, odd). If measurement confirms H1 and H2:
* Waste fraction of non-degenerate solutions: 0 (Lemma B is size-independent, as is E[2] = {O, T0}). The density of dead columns stays 1/2 with deviation about 1/(2 sqrt|V|) (below 10^-9 at l = 44 or 66).
* Under the enumeration instrument model the cost ratio is R2(131) = 1.266 (l = 66) and R3(131) = 1.625 (l = 44) from `prediction_table.txt`. This is a model value, and it is moot because enumeration needs 2^66 or 2^88 attempts per target.
* Under Groebner/Weil-descent solvers the effect of restricting x_i to V_E is not modelled and is an open question (the trace condition adds lift variables y_i^2 + y_i = x_i + a + b/x_i^2 to the descent system and V_E is not an F_2-subspace).
* Frobenius symmetry on Koblitz curves (a, b in F_2): Tr(f(x^2)) = Tr(f(x)), so V_E is stable under the factor-base orbit structure. No separate prediction.
Reporting rule: these three bullets are reported in a separate section headed "Extrapolation", with the model equations and no data-like numbers beyond the table values.

## 3. Instrument

One dependency-free Rust crate `src/main.rs` (subcommands: `census`, `arms`, `controls`, `prime`, `rho`, `e2e`) plus GP scripts for curve generation (`ellcard`) and independent verification. Field GF(2^k): carry-less
multiply with software shift-xor or `core::arch` pclmulqdq (std only), reduction by the fixed irreducible. Prime field p < 2^31 with u64 arithmetic.

**Counters** (every arm uses the same primitives; counters are incremented inside the primitives):
* G: elliptic group additions/doublings (priced 1 GAE each; their internal field operations are NOT also counted as native).
* Native operations (priced c GAE each, c in {0, 0.1, 1}): S field squaring, M field multiplication (inversion is the Itoh–Tsujii chain of S and M, no separate unit; squarings are counted 1 each, conservative),
  T trace/parity/AND-popcount/trace-of-f, X membership parity test, L table/hash lookup, LA modular multiply-adds.
* Calls: targets (decomposition attempts); stage-1 calls (m = 2: x1 attempts; m = 3: pair attempts); stage-2 calls; trace-fail attempts; half-trace root formations.
* Solutions: raw, unique; classes by independent classification: all-E, pure-twist (all coordinates twist-type), mixed, degenerate-family flagged (D, section 2.1), neutral (none expected).
* Lifts: lift attempts (half-trace for y = x z, z^2 + z = f(x)), lift failures, sign-check group additions; relations verified; rank.
* Linear algebra rows ops and entry multiply-adds (full-solve cells only).
* Wall time (labelled contention-affected).

**Independent classification.** The class of x_i is computed by `classify(x)` (trace of f(x) / Legendre), a different code path from the quadratic solver. GP cross-check on a stored sample (first 200 unique solutions per cell and per
arm U, seeds fixed): each coordinate lifted with `ellordinate`, and the relation checked by `elladd` (for some choice of signs) against R. Any mismatch between Rust labels and GP labels (or a relation that fails to verify) invalidates the cell.

**Solver (all arms).** m = 2: for each x1 in the attempt list, solve S_3(x1, X, x(R)) = 0: A, B, C as above, c = C A / B^2, trace test of c; if Tr(c) = 0, Z = half-trace(c), X = (B/A) Z and X + B/A; keep roots in V*.
m = 3: for each unordered pair {x1, x2} (x1 ≤ x2 in a fixed order): stage 1, the quadratic S_3(x1, x2, X) = 0 for X (for x1 = x2: A = 0, the roots are X = x1^2 + b/x1^2 and X = ∞); stage 2, for each F_q root X solve S_3(x3, x(R), X) = 0
for x3 (for X = ∞ the candidate is x3 = x(R)); keep x3 in V*. Raw solution {x1, x2, x3}, deduplicated per target by sorted multiset.
**Arms (binary):**
* U: attempt list = all of V* (m = 2) or all pairs of V* (m = 3). No filter.
* S: same solver as U, plus the trace filter on every raw solution before lifting (cost m T per raw solution); executed on the stored raw solution stream of U (S does not change the solver), and counted by executing it.
* F: attempt list restricted to V_E = {x in V* : Tr(f(x)) = 0}, computed once per (curve, V) (cost counted, amortised over its targets); stage 2 unfiltered root membership in V*.
* (F+S is dominated by F and is not run.)
The U and F solutions of every unflagged target must coincide as sets (section 5, F4).

**Lifts and relations.** For each unique solution: y_i for each x_i (half-trace), the sign pattern found by group additions (≤ 2^{m−1} patterns), relation `h(a_j + b_j d) ≡ sum e_i l_{x_i} (mod n)`, with h = 2 and l_x the discrete log of the canonical lift of x (unknowns l_x, d).
R_{j+1} = R_j + W_{i} from a seeded table of 64 random points a P + b Q (cost: 64 scalar multiplications and 1 G per target, counted).

**Full-solve cells (complete-solve certificate).** m = 2: k in {17, 19, 23}; m = 3: k in {17, 19, 23, 29}. Per cell 5 curves x 2 DLP instances (random d). Relation collection until N_need = |V_E| + 1 + 10 unique relations (stop rule: after collection run dense elimination mod prime n; if rank < unknowns collect 10% more targets, ≤ 5 rounds
else mark FAIL), then solve for d and check dP = Q. Linear algebra: dense zero-skipping elimination, counting row ops and entry multiply-adds. Cells k = 29, 31 (m = 2) and k = 31 (m = 3) are not run to completion: relation-phase cost per relation is measured
and multiplied by N_need ("extrapolated from measured per-relation cost"), the LA is a formula-model N_need^3/3 mult-adds, both labelled.

**Rho baseline (matched).** On the same curves (same n, same P, Q = dP with random d, 50 instances per curve; 10 curves per size): Pollard rho with negation map (canonical representative min(y, x + y) for binary, min(y, p−y) for prime), r = 16 additive branches chosen by the low 4 bits of x,
distinguished points with x low bits zero (theta = 2^{−d_dp}, d_dp = round(log2(n)/4)), walks capped at 20/theta steps (fruitless cycle), starts a P + b Q (scalar-multiply cost counted in G), DP table with lookups.
Counted: G = group additions; native per step: canonicalisation compare (1), branch index (1), DP test (1) = 3; plus 1 lookup per DP. Cost_rho(c) = G + c N. Instances run until a collision solves the log; the log is verified (dP = Q).

**GAE pricing and ranking (AGENTS.md section 5a).** Every arm's cost(c) = G + c · N for c in {0, 0.1, 1}, where N is the sum of the native operations above. Two arms are ranked only where the order holds at all three prices. "Holds" means cost_A ≤ cost_B at all three, strict at both c = 0.1 and c = 1 ("weak: tie at c = 0"
allowed when the arms have equal G). Otherwise "unordered across the band". Gap to the matched rho is reported as the ratio cost_IC/cost_rho at each c (distance remaining), never as pass/fail.

## 4. Design: arms, controls, sizes, seeds

### 4.1 Arms and controls

| Role | What | Purpose |
|---|---|---|
| Treatment | U, S, F on binary E (a = 1) | Measure waste, test the filter |
| Matched baseline | Pollard rho + negation map, same curves/n/targets | GAE-band comparison, distance remaining |
| Negative control (null structure) | Prime-field arms U, F with Legendre filter, prime-order curves, matched |V| and target counts | The lemma is not binary-specific; the instrument returns 0 waste where the prior study saw almost none |
| Null-model contrast | Coin-flip prediction 0.75 / 0.875 | Shows the data separate lemma vs independent-lift model |
| Positive control PC-A | Classifier vs explicit lift: 10^4 random x per (k, curve-type) incl. both field types, in Rust vs GP `ellordinate`; binary and prime | Instrument classifies correctly. Requires 100% agreement. |
| Positive control PC-B (twist target) | Targets with twist-type x(R') (any x with Tr(f(x)) = 1; prime: nonresidue); solver U unchanged. 1000 targets (m = 2) / 300 targets (m = 3), k in {17, 19, 23}, 10 curves, binary and prime. Verification on the twist curve E' (a' = a + gamma, Tr(gamma) = 1) | Mirror of Lemma B: every non-degenerate solution has all coordinates twist-type. The counter must report 100% "waste" (non-E-liftable) with 0 E-type coordinates. Blindness criterion: reported waste < 0.99. |
| Positive control PC-C (injection) | Take stored U solutions; replace, with a seeded RNG, a fraction rho in {0.01, 0.05, 0.25} of records by a random coordinate swapped for a random twist-type element. Feed to the counter. | Sensitivity must be exactly 1.0 (every injected record flagged) and specificity exactly 1.0 (no unmodified record flagged); observed waste fraction equals rho exactly. |
| Positive control PC-D (degenerate family, m = 3) | Targets with x(R) forced into V (V = l−1 random vectors + x(R)); k in {17, 19, 23}, 10 curves x 20 targets (binary and prime) | Theory predicts the chained solver finds exactly |V_tw| mixed multisets of the form {x, x, x(R)} with x twist-type (|V_tw| = number of twist-type elements in V*), and no other mixed solution. Count equality required. |

### 4.2 Sizes and sample counts (binary)

Curves: 20 per k for k in {17, 19, 23, 29, 31}; the m = 3, k = 29 cell uses the first 10 curves. V type alternates by curve index (even: rand, odd: poly). m = 2 uses l = ceil(k/2), m = 3 uses l = ceil(k/3).

| m | k | l | curves x targets/curve | targets | expected unique solutions (2^{ml−k}/(2 or 6) per target) | attempts, arm U |
|---|---|---|---|---|---|---|
| 2 | 17 | 9 | 20 x 1000 | 20 000 | 20 000 | 1.0e7 |
| 2 | 19 | 10 | 20 x 1000 | 20 000 | 20 000 | 2.0e7 |
| 2 | 23 | 12 | 20 x 1000 | 20 000 | 20 000 | 8.2e7 |
| 2 | 29 | 15 | 20 x 500 | 10 000 | 10 000 | 3.3e8 |
| 2 | 31 | 16 | 20 x 300 | 6 000 | 6 000 | 3.9e8 |
| 3 | 17 | 6 | 20 x 1000 | 20 000 | 6 700 | 4.2e7 pairs |
| 3 | 19 | 7 | 20 x 1000 | 20 000 | 13 300 | 1.7e8 pairs |
| 3 | 23 | 8 | 20 x 400 | 8 000 | 2 700 | 2.6e8 pairs |
| 3 | 29 | 10 | 10 x 200 | 2 000 | 670 | 1.0e9 pairs |

Power: a cell is "powered" if its attained unique solution count is ≥ 1000. Zero violations in N solutions bound the violation rate by 3/N (rule of three, 95%) so powered cells bound it below 3e-3, and the large cells (≥ 6000) below 5e-4.
m = 3, k = 29 (about 670) and k = 23 (about 2 700) are the weakest; k = 29 is labelled underpowered if below 1000. k = 31 for m = 3 is not run (about 4e9 pairs, too heavy for the shared host).
The H2 census uses 200 random V (rand, seeded) per (curve, l) in addition, to separate the sampling distribution of f from the specific V of the arms (rand strata); poly strata have one V per curve (20 curves → 10 poly V's per cell plus each curve's random b gives the randomisation).
Prime control (10 curves each): m = 2, k in {17, 19, 23, 29, 31} with 1000/1000/1000/500/300 targets per curve; m = 3, k in {17, 19, 23} with 1000/1000/400 targets per curve; V random subset of 2^l nonzero x-values with f(x) ≠ 0.
Rho: 10 curves x 50 instances per k.

### 4.3 Seeds and determinism

Master seed M = 20261107. Every stream is splitmix64 seeded from M XOR FNV-1a-64 of the ASCII string `role/k/m/curve/target` (roles: `curve`, `V`, `target`, `walk`, `inject`, `dlp`). GP uses `setrand(seed mod 2^32 + 1)`.
All curves, V's, targets and DLP instances are functions of these strings, so any cell can be replayed. The curve list (b values, #E, n) is recorded once in `results/curves.json` (about 250 lines). Same curve set for all arms and rho.

### 4.4 Storage

Output only per-cell aggregate JSON in `results/` (about 100 KB each), `curves.json`, and the 200-solution verification sample per cell (a few KB). Estimated total under 5 MB. Rust `target/` is deleted after the run. Hard cap 150 MB; if exceeded, stop.

## 5. Falsification criteria (all registered, thresholds fixed)

* **F1 (H1, exact).** One or more non-degenerate solutions (not in D, no coordinate equal to 0, x(R) not in V and sqrt(b)/x(R) not in V) containing a twist-type coordinate, in any binary cell, confirmed by the GP independent check, falsifies H1 as stated.
  The report then states the family found. If the observed waste fraction is within 3 sd of the coin-flip values (0.75, 0.875), the lemma is refuted outright.
* **F2 (H2).** For each of the 9 cells (k, m), compute pooled z over curves; reject the cell at two-sided alpha = 0.01 (|mean z| > 2.576 · sd_z/sqrt(N_z)). H2 is falsified if ≥ 2 of 9 cells reject (null probability ≈ 0.4%), or if the pooled sd of z lies outside [0.8, 1.25].
* **F3 (H3 cost).** For each cell the measured per-target native-operation ratio R̂ = cost_U/cost_F (precompute amortised) is compared with the table. H3 is falsified if R̂ is outside ±10% of the table value in ≥ 3 of the 9 cells, or if the 95% cluster-bootstrap CI of R̂ contains 1.0 in any cell at c ≥ 0.1.
  Separately, the call ratios must lie within 2 sd (H2 band) of 2.0 / 4.0.
* **F4 (set equality).** U and F solution sets differ on any unflagged target: falsifies H1 (the difference is exactly a solution with a twist coordinate) and invalidates F's relation set. The test is run on every target.
* **F5 (H4).** Any lift attempt avoided by the solution-side filter (> 0) falsifies H1 (same event). S/U cost ratio outside [1, 1.05] in any cell falsifies H4's magnitude only.
* **F6 (H5).** Any non-degenerate waste solution in a prime cell falsifies the generalisation of Lemma B to prime fields. Prime consistency check: measured prime call ratio outside the H2-style band around 2.0 / 4.0 falsifies the structural statement.
* **F7 (instrument validity: blocks all conclusions).** PC-A < 100% agreement, PC-B reported waste < 0.99 or any E-type coordinate in a non-degenerate twist-target solution counted as pass (the latter refutes Lemma B instead), PC-C sensitivity or specificity ≠ 1.0, PC-D count ≠ |V_tw|,
  or any GP/Rust label mismatch → all results of the affected cell classes are void and are reported as an instrument failure.
* **F8 (rho baseline).** Mean rho G outside [0.8, 1.6] x sqrt(pi n)/2 in any k → the rho implementation is flagged, no ranking against it is reported until fixed. This amends nothing in H1–H5.
* **F9 (verification).** Any counted relation that fails the group-law check, or a full-solve instance whose recovered d fails dP = Q, is a failure of the cell and is reported (not hidden by retry).

## 6. Analysis plan

1. Per cell: tables of counters (totals and per target), waste classification (all-E / pure-twist / mixed / D-flagged), with exact counts and Clopper–Pearson 95% intervals for the violation rate (zero case: 3/N).
2. H2: z-scores per curve, pooled mean and sd, per V-type breakdown (descriptive), cell test in F2.
3. H3: R̂ per cell with cluster bootstrap over curves (10 000 resamples, seed M), call ratios, precompute share, the S arm overhead; compare each to the registered table; the table values are not refit.
4. Prime control: the same tables; consistency check of registered formulas against measured a, h.
5. Controls: PC-A/B/C/D tables with pass/fail per F7 before any other number is interpreted. If F7 fires, nothing downstream is reported as a result.
6. GAE band: cost(c) for U, S, F, IC end-to-end (full-solve cells) and rho; ordered/unordered rule from section 3; gap ratios to rho at each c. The rank of F vs U is reported; IC vs rho is reported as distance remaining. Wall times only in a labelled secondary column.
7. Extrapolation: a separate section with the three labelled bullets of 2.3 and the model values at k = 131, no new data-like numbers.
8. Plots (python venv, values read from the recorded JSON only): (a) violation count and CI per cell vs coin-flip values, (b) z-score histogram, (c) call and cost ratio vs table per cell, (d) cost vs c for each arm and rho with the unordered cells marked. The report is `RESULTS.md` plus figures. A PDF is built only if a toolchain is available; otherwise stated.
9. Replication hooks: `VERIFICATION.json` should list an independent replay (different seed offset, k ≤ 23) and the GP cross-check results; discrepancies are reported, not reconciled silently.
10. Deviations after freezing go into an `AMENDMENTS` section appended below with date and reason. They never overwrite this text. Post-hoc analyses are labelled exploratory.

## 7. Open items and limitations

* O1: whether the repository IC baseline already restricts to V_E (unknown; not checked). If it does, the filter result is a relabelling and no improvement.
* O2: the enumeration solver has cost Θ(q) overall (total relation cost about q for the filter and 2q unfiltered for m = 2, independent of l), far above rho at all sizes. The distance remaining is large by construction. The measured filter effect is a constant factor of this phase only.
* O3: Weil-descent/Groebner solvers, m ≥ 4 and the Koblitz orbit structure are not measured.
* O4: the lift sign problem and cofactor h = 2 are handled by the relation formula above. Curves with a = 0 (h = 4) are not run.
* O5: cells with 3 or fewer expected solutions per target use many targets; k = 31 for m = 3 and the full solves at k ≥ 29 (m = 2) are not run, by cost.

## 8. Frozen artefacts

`frozen_pre_data/prediction_model.py` (generator of the prediction table), `frozen_pre_data/prediction_table.txt` (its output), `frozen_pre_data/smoke_identities_k17.gp` (feasibility check, output discarded). SHA-256 of these and of this file are in `FROZEN.sha256`.

No data have been computed. Frozen.
