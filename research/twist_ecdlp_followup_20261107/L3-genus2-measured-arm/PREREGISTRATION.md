# PREREGISTRATION L3-genus2-measured-arm

Lead: L3-genus2-measured-arm. Directory D = research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm
Prior study P = <scratch>/wt/research/twist_ecdlp_20261027 (read only; E6 construction, G1 negation-map rho design).
Written and frozen before any data and before any code of this study exists. Hash of this file: `PREREG_SHA256.txt`; freeze time: `FROZEN_AT.txt`.

Scope: scratchpad study. PARI/GP 2.15.4 only (`gp -q`, `default(parisizemax,1000000000)`). No Rust crate is built, and the main repository crates are not built. Python (venv, `-I`) only plots recorded values. No repository file, `benchmarks/` row or leaderboard is touched; if the result is promoted later, that PR follows the repository rules (research-visuals, benchmarks/, validation gate) separately. Per CLAUDE.md the gap to the matched rho is reported as distance remaining, not as pass/fail; no improvement over the index-calculus baseline is claimed or expected.

## 0. Disclosure of what ran before this freeze

Only inspection: I read P/README.md, P/E6-genus2-gluing/PREREGISTRATION.md, the `e6lib.gp` function list, P/G1 `PREREGISTRATION.md`, `RESULTS.md` and `g1_lib.gp`, and AGENTS.md section 5a. I ran `gp --version`, and one throw-away formula evaluation of the section 4 prediction formulas for T_S and T_F (pure arithmetic in q, no curve, no group element; its table is copied into section 4). No curve, Jacobian, relation or walk has been computed. The G1 measurement "N1 steps / sqrt(n/2) = 1.378 to 1.506 for b = 24..40" is prior data used to set the rho prediction (section 4, H5); the E6 result "#Jac = #E * #E' on 580/580 records" is prior data used as the premise of the construction. Nothing else is known.

Smoke tests that ARE allowed after the freeze: toy size k <= 8 with seeds other than the ones in section 7, outputs in `D/dev/`, never read by the analysis, to check that code runs. The numeric predictions, controls, falsifiers and analysis in this file may not change after the freeze. Implementation details this file leaves open (data layout, the exact explicit Jacobian formulas, GP idioms) are fixed at the code freeze (section 10), with file hashes, before the first confirmatory run; later script edits are deviations listed in `DEVIATIONS.md`.

## 1. Objects and the measured arm (what is built, exactly)

### 1.1 Curve pair (E, C)

q is a prime (prime fields only). E: y^2 = f(x) = x^3 + A x + B over F_q, A, B != 0, disc != 0 (so j not in {0, 1728}); E' its quadratic twist by the smallest non-residue. Following P/E6 (H2): take a non-identity Galois-equivariant permutation pi of the roots of f, the F_q-polynomial g interpolating r_i -> r_pi(i), kappa = 1/g_2, mu = -g_1/g_2, a in F_q^* with (-kappa a | q) = -1, and
  C: Y^2 = F(x) = f(a x^2 - mu)   (smooth iff f(-mu) != 0),
so that Jac(C) is (2,2)-isogenous to E x E' and #Jac(C)(F_q) = N = #E * #E'.

Admissibility filter (applied to candidate E, in this order; every rejection is counted and reported, nothing is silently resampled):
 (a) 2-division pattern [1,2] or [1,1,1] (f has a rational root). Pattern [3] is excluded because then F(x) has no F_q-root, so there is no rational Weierstrass point and no imaginary model (limit stated in section 9).
 (b) #E = h * l with l prime, h even, h <= 16, l >= q/16 (so l ~ q/h), l not dividing #E' (hence l^2 does not divide N and Jac(C)[l^inf] is cyclic of order l).
 (c) pi != id, f(-mu) != 0, and C has a rational Weierstrass point: some rational root r of f and rho in F_q with a rho^2 - mu = r (equivalently chi(a (r + mu)) = 1). Sending that point to infinity (X = 1/(x - rho), V = Y X^3, then scaling X' = c5 X, V' = c5^2 V, c5 = the leading coefficient) gives the monic quintic imaginary model C': V'^2 = X'^5 + ... ; W denotes its point at infinity. All Jacobian arithmetic is Mumford arithmetic on C' with the class of a degree-0 divisor D - 2W.
 (d) #C'(F_q) = q + 1 by direct Legendre-symbol count (this is a consistency check on the construction, not an arm cost).
 (e) N = #E * #E' annihilates 8 random Jacobian elements (random sums of two random points), using the Jacobian code under test; and the explicit Jacobian addition agrees with a reference Cantor implemented with PARI polynomial arithmetic on >= 2000 random pairs per size, including the special cases (equal operands, inverse operands, degree < 2). The reference is used for checks only and its operations are never counted.

### 1.2 The transfer map T: E(F_q) -> Jac(C')(F_q) and its cost

phi: C -> E, (x, Y) -> (a x^2 - mu, Y), is a degree-2 map over F_q, ramified over two points, so phi^*: E -> Jac(C), P -> [phi^* P - phi^* O_E] is an injective homomorphism. The fibre of O_E is the pair of points at x = infinity, i.e. a hyperelliptic fibre, linearly equivalent to 2W, so T(P) = [phi^* P - 2W]. For P = (x0, y0), s = x0 + mu, the divisor phi^* P consists of the points with a x^2 = s, Y = y0. In the coordinates of C' it is the semi-reduced divisor with
  u(X') = X'^2 + c5^2 / d,  d = rho^2 - s/a   (the roots are X'_{1,2} = c5/(x_{1,2} - rho) with x_1 = -x_2),  v(X') = (y0 / c5) X'^3 mod u(X').
(Both points have the same Y = y0; the formula is exact for d != 0 and s != 0.) Targets with d = 0 (then P = (a rho^2 - mu, 0) is a 2-torsion point, which a point of odd prime order l never is) or s = 0 (ramification, probability about 1/q) are re-drawn and counted. The transfer cost per instance is counted in field operations (M, S, I, by the instrumented code) for T(G) and T(H), in Jacobian operations (expected 0), and in verification work. The per-curve gluing construction (interpolation, rho, c5, the model change) is counted separately as `curve_setup` (field operations, root-finding calls) and reported, amortised over the instances of that curve, but not added to the per-solve total (it is added to a reported sensitivity column "total incl. amortised setup").

Transfer checks (not arm costs): T(P1 + P2) = T(P1) + T(P2) on 30 random pairs per curve; l T(G) = 0 and T(G) != 0; the order of T(H) divides l. Required 100%.

### 1.3 The ECDLP instance

G = h * (random point of E), checked of order l; x uniform in [1, l - 1]; H = x G. Per curve 4 instances with distinct x. Unknown: x. The same (E, C, G, H, x) instance is given to every arm (paired design). A solve is VERIFIED only if the returned x' satisfies x' G = H on E, computed with PARI `ellmul` (independent of every arm's code). An unverified return counts as a failure.

### 1.4 Arms

All Jacobian operations are counted as the unit "Jop" (add or double; negation and the sign of a Mumford pair are native, counted as CANON). All E operations are counted as "Eop" (add or double). 1 GAE = 1 Eop (one group addition equivalent; doubling counted the same, as in G1).

R_E (matched rho, primary baseline). Pollard rho on E restricted to the subgroup H of order l, with the negation map, written afresh in GP to the G1 design (P/G1 `g1_lib.gp`, N1): r = 32 additive table W_j = u_j G + v_j H, selector from x only, canonical point = the one with lift(y) <= (q-1)/2 (exponents negated with the point), 2-cycle test plus Brent saved-point test, escape by canonical doubling (counted as one Eop), distinguished-point (DP) trails with DP bits d = max(2, floor(log2(sqrt(l/2)) / 2)), trail cap 20 * 2^d, budget 40 * sqrt(l/2) steps, first DP collision gives candidate x (checked as in 1.3). The table construction (32 scalar multiplications each of two points, counted in Eops) and the start point of each trail are counted.

R_J (generic rho in Jac(C), same subgroup). The same walk, DP rule and budget on the Jacobian subgroup <T(G)> of order l (target T(H)), with the free negation map of the Mumford pair (v -> -v), table W_j = u_j T(G) + v_j T(H), Jops instead of Eops. This is what generic rho costs once the instance has been moved into Jac(C).

IC arms (index calculus in Jac(C); one shared relation generator, differing in acceptance, factor base size and solve schedule; section 3 has the algorithm). Notation: V = set of x-coordinates of affine points of C' (a vertex is a point up to sign; n_v = (q + w)/2, w = number of rational affine points with y = 0); n_est = q/2 (known to the code from q alone; instance-specific n_v is NOT used by any arm); theta = FB fraction of vertices; FB membership of x is `(s*x mod q) < ceil(theta*q)` for a seeded multiplier s coprime to q (no enumeration of V is ever done).
 IC-G  (Gaudry): theta = 1 (every point is in the factor base), accept every split divisor, first solve attempt at >= n_est accepted relations, growth x1.05.
 IC-S  (single large prime): theta_S = min(1, 2 q^(-2/5)), accept types FF and FL (F = in the factor base, L = large prime), first attempt at >= 0.2 n_est accepted relations, growth x1.1.
 IC-D  (double large prime): theta_D = min(1, 2 q^(-1/2)), accept FF, FL and LL, first attempt at >= 0.2 n_est, growth x1.1, elimination order large-prime columns before factor-base columns.
 IC-F  (factor base only, no large primes; used only as the positive control PC-A): theta_F = theta_D, accept FF only, first attempt at >= 0.2 * (theta_F n_est) accepted relations, growth x1.1.
Budget caps (right-censoring, section 6): trials <= 12 q for IC-G and IC-D; <= 12 * T_S(q) for IC-S; <= 8 * T_F(q) for IC-F, with T_S, T_F from section 4.

Pricing, counters and the 5a band: section 5.

## 2. Hypotheses (derivations in section 4)

H1 (transfer is exact). T is an injective homomorphism on all admissible instances, and every verified-solve check passes, i.e. 100% of the transfer checks, and the IC arms return verified logarithms.
H2 (relation-phase exponent and constant). The relation phase of IC-D costs T_D ~ n_v = q/2 decomposition attempts (Jops in the walk) per solve; IC-G costs T_G ~ q; IC-S costs T_S(q) ~ q^(6/5); IC-F costs T_F ~ q^(3/2)/4. The measured exponent of IC-D in q is 1, not below.
H3 (rho references). R_E uses R = steps / sqrt(l/2) in [1.2, 1.9] (prior G1 measurement 1.38..1.51); R_J the same R; hence rho on E costs ~ 1.45 sqrt(l/2) ~ q^(1/2).
H4 (the gap). In the primary pricing (Jop = 1 GAE, native work at w in {0, 0.1, 1}) every IC arm costs MORE than R_E at every size, the ratio IC-D / R_E growing like q^(1/2): fitted log-log slope of the ratio on q in [0.35, 0.60] on sizes k >= 10, and the ratio >= 5 at k = 8 rising to >= 30 at k = 16 (point values; derivation in section 4).
H5 (no structure exploited). The IC arms cost the same on the glued Jacobian as on a random genus-2 Jacobian of the same q: the paired-by-q ratio of IC-D relation-phase attempts, glued / random, is in [0.9, 1.1] (negative control NC-A). The E x E' structure only enters through the transfer, which costs O(1).
H6 (Jacobian arithmetic constant). One Jop costs kappa_J = 2.0 to 4.5 Eops by counted field operations (S = 0.8 M, I/M = 10; same I/M in {3, 30} reported).
H7 (relation yield). The fraction of random Jacobian elements whose Mumford polynomial splits into two distinct rational roots is p_split = 1/2 (1 - O(1/q)): measured pooled value in [0.46, 0.54] at every size.

## 3. Algorithm of the IC arms (frozen specification)

Setting. l prime, J = Jac(C')(F_q), J = H x J' with H = J[l] (cyclic of order l) and J' = l J. The projection psi: J -> H kills J'. Relations need only hold modulo J', so they are used in Z/l.

Walk. Table of 16 elements M_j = alpha_j G~ + beta_j H~ + R_j in J, where G~ = T(G), H~ = T(H), alpha_j, beta_j uniform in Z/l, R_j = l X_j with X_j a random sum of two random points of C' (so R_j in J'); scalar multiplications by double-and-add in Jops (all counted). Start D_0 = a_0 G~ + b_0 H~ + R_0 similarly constructed. Each decomposition attempt (trial) is one Jop: D <- D + M_{j(D)}, j(D) = (constant coefficient of u mod 16) + 1, and the exponent pair (a, b) <- (a + alpha_j, b + beta_j) mod l. D is a uniform-looking element of all of J (not only of H), because of the J' component. psi(D) = a G~ + b H~, so psi of the divisor is known in terms of x: log = a + b x mod l.

Decomposition (smoothness test). D = (u, v) reduced, deg u = 2: compute disc = u1^2 - 4 u0; split iff kronecker(disc, q) = 1 (disc = 0 is skipped and counted); roots x_1, x_2 by one modular square root; y_i = v(x_i); vertex of P_i = x_i, sign eps_i = +1 if y_i <= (q-1)/2 else -1 (canonical y of the vertex). deg u = 1: weight-1 relation; deg u = 0: skipped, counted. The relation row is: eps_1 L(x_1) + eps_2 L(x_2) - b * x = a (mod l), with L(x) the unknown log of psi([(x, y_c(x)) - W]) and L(-P) = -L(P). Vertices with y = 0 (rational affine 2-torsion points) have L = 0: their columns are deleted. Duplicate rows (same D seen twice) are dropped and counted.

Solve (shared by all IC arms). Sparse system over Z/l with one column per vertex plus the column of x. Structured elimination: repeat: delete rows containing a column of weight 1 (not the x column); for a column of weight 2 combine its two rows (one row operation); otherwise take the column of least weight (Markowitz), with the order restriction of the arm (IC-D: columns outside the factor base first). The solve attempt succeeds iff after eliminating all vertex columns some row has only the x column with a nonzero coefficient mod l. x = rhs / coefficient. An attempt that fails costs its row operations all the same (counted). Attempts happen at the arm's schedule (first attempt, then each time the accepted relation count has grown by the arm's factor). Remark: for weight-2 rows this is graph peeling, and x is determined once the relation graph has a balanced cycle (or two unbalanced cycles in one component) with nonzero x-coefficient; no individual-logarithm phase is needed.

## 4. Derivations of the numeric predictions

Assumptions (stated so they can fail): (i) walk elements are uniform in J, so a reduced divisor splits with p_split = 1/2 and its two points are independent uniform over V (this is what H7 and NC-A test); (ii) the relation graph on V behaves as a random multigraph with signs random +-1.

IC-D. Every split divisor is an edge on n_v = q/2 vertices. A balanced cycle (prob 1/2 per cycle by sign) or two unbalanced cycles in one component is needed; both appear at the giant-component threshold m* = n_v/2 edges (expected number of cycles of length k in G(n, m = c n) is (2c)^k/(2k), divergent at c = 1/2). Trials = m*/p_split = n_v = q/2, so T_D/q = 0.5. Attempts begin at 0.2 n_est and grow x1.1, so the first successful attempt overshoots by at most 10%; the finite-size critical window (n^(-1/3) of c) widens the band at small q. Prediction: T_D/q in [0.40, 0.95] at every size, median in [0.45, 0.80] for k >= 10; slope of log T_D on log q in [0.9, 1.1] over k = 8..16.
IC-G. m = n_est edges before the first attempt, trials = m / p_split = q. T_G/q in [0.95, 1.25].
IC-S. Edge types per trial: FF (1/2) theta^2, FL theta(1-theta), LL (1/2)(1-theta)^2; accepted: FF + FL. Effective graph on the n_F = theta n_v factor-base vertices: FF edges m_FF = T theta^2 / 2, plus pairs of FL edges sharing a large vertex, m_FL^2 / (2 n_L) with m_FL = T theta (1-theta), n_L = (1-theta) n_v. Threshold n_F/2 effective edges gives
  theta t + theta (1-theta) t^2 = 1, t = T/n_v, so t = (-theta + sqrt(theta^2 + 4 theta (1-theta))) / (2 theta (1-theta))
(theta = 1 gives t = 1, matching IC-D). T_S = t n_v. Values of T_S/q (theta_S = min(1, 2 q^(-2/5))): 0.930 (k=8), 1.079, 1.252, 1.452, 1.686, 1.956, 2.267, 2.627, 3.042 (k=16); asymptotically t ~ theta^(-1/2) so T_S ~ q^(6/5). Band: T_S measured / T_S formula in [0.6, 1.7]. Note: with structured elimination the linear algebra is cheap, so reducing the factor base does not help in this accounting and IC-S is predicted to be WORSE than IC-D in Jops; it is included because the lead specifies it and because "large-prime variations help" is a claim the measurement can test.
IC-F. Only FF edges: T theta^2/2 = n_F/2, T_F = n_v / theta_F = q^(3/2)/4. Values of T_F/q: 3.96 (k=8), 5.64, 7.99, 11.29, 15.99, 22.63, 31.997, 45.24, 63.99 (k=16). Band: measured / formula in [0.5, 2.0].
Setup per solve (counted, not a prediction): 16 entries x (2 scalar multiplications by alpha_j, beta_j, one by l, one X_j sum, 2 adds) + the start element, with ~1.5 (log2 l - 1) Jops per scalar multiplication; it is O(log^2 q) and is reported separately from the relation phase.
Rho. Expected steps of negation-map rho ~ R sqrt(l/2), with R measured 1.378..1.506 on 24..40 bits in G1; smaller l carries a larger DP and escape overhead. Band R in [1.2, 1.9], pooled and per size k >= 10. With l ~ q/h, h in {2,4,..}: R_E steps ~ 1.45 sqrt(q/(2h)) ~ q^(1/2).
Gap (H4). In Jops, cost(IC-D)/cost(R_E) at w = 0, kappa_J = 1 is about (T_D + setup) / (R sqrt(l/2) + table); with T_D = q/2 and R sqrt(l/2) = 1.45 sqrt(q/(2h)) the ratio is 0.69 sqrt(q) sqrt(h) before setup, i.e. growth q^(1/2). Prediction: log-log slope of cost(IC-D)/cost(R_E) on q in [0.35, 0.60] for k = 10..16 (the log^2 q setup terms and the l ~ q/h scatter move it by a few hundredths), and ratio > 1 at every size, all three w. The same for IC-G (about 2x larger) and IC-S (larger still for k >= 10). Unpriced native work (section 5) per step is larger for rho (about 5 native operations per step) than per IC trial (about 3-4), so at w = 1 the ratio is lower than at w = 0; the band prediction is that it stays above 1 at every size, but the numeric value at w = 1 is stated only as "above 1", not as a number.
H5 / NC-A derivation. IC arms use only the group law of J and the Weierstrass structure, and a random genus-2 Jacobian of the same q has the same p_split and the same graph model; hence equal cost to within sampling error. The E x E' decomposition does not enter any step except T.
H6 derivation. A general-case genus-2 addition in an explicit-formula implementation costs on the order of 22 M + 3 S + 1 I against 2 M + 1 S + 1 I for an elliptic addition; with S = 0.8 M, I = 10 M this is 34.4 / 12.8 ~ 2.7. These literature-style counts are NOT used as data: kappa_J is whatever the instrumented code counts, and the preregistered band is [2.0, 4.5].

## 5. Metrics, counters and the pricing band (AGENTS.md 5a)

Primary metric: OPERATION COUNTS per verified solve, per arm and size. Wall time (ms of the arm run, `getwalltime`) is secondary, labelled contention-affected (4 cores shared with other agents), never used in a ranking or a falsifier.

Counters emitted per (instance, arm), all by the code under test:
 - Core group work: Eop (R_E), Jop (R_J, IC arms, including setup, walk and the Jops of the transfer = 0).
 - Field operations M, S, I of the instrumented group law (E add: counted by the code; Jop: counted by the explicit-formula implementation; fallback Cantor cases counted at their own tally and flagged).
 - Decomposition attempts = trials (IC) and steps (rho); relations accepted by type (FF, FL, LL), rejected, duplicate, degenerate (deg u < 2, disc = 0); p_split numerator/denominator.
 - Solver/root-finding calls: modular square roots (one per split relation), kronecker tests (one per trial), solve attempts.
 - Linear algebra: row operations (combine), row deletions, nonzero-entry updates (sum of lengths of the rows touched), final matrix size (rows, columns, nonzeros) per attempt.
 - Transfer: field operations of T(G), T(H); curve_setup counters.
Unpriced native operations, listed per arm (an arm that names a counter must emit its count, else it is ranked as a lower bound):
 - R_E and R_J: SEL = DPT = CAN = steps, CYC = 2 * steps, LOOK = number of DPs, START = number of trails.
 - IC arms: SPLIT = trials (kronecker), ROOT = splits, HASH = trials (partition selector), CANON = 2 per accepted relation (sign eps), FBTEST = 2 per split (arms with theta < 1), STORE = accepted relations, LAROW = row operations + row deletions (nonzero-entry updates reported as sensitivity), XFER = 2 per instance.
Cost(arm, w, kappa_J) = (Eop or kappa_J * Jop) + w * (sum of native counters), with w in {0, 0.1, 1} GAE per native operation, the same way on every arm. Primary kappa_J = 1 (a Jop priced as one GAE: generous to the Jacobian arms, i.e. a lower bound on their cost). Secondary kappa_J = measured ratio of counted field costs (M + 0.8 S + rho I, Jop over Eop) for rho in {3, 10, 30}. The rho-reference R_E is priced with the same band.
Ranking rule: arm A is ranked below B at a size only if the 95% CI of the paired cost ratio A/B lies entirely on one side of 1 at every w in {0, 0.1, 1} (primary kappa_J); otherwise the pair is "unresolved at that boundary". Distance remaining is always reported: the geometric-mean ratio IC / R_E per size and w.

## 6. Arms, controls, sizes, samples, seeds

Sizes: q_k = the largest prime <= 2^k, k = 8, 9, ..., 16 (nine sizes). If the 24 instances of a size cannot finish within the per-size wall cap of 90 minutes of one core, the size is reported "incomplete" (completed instances kept, none imputed) and no larger size is attempted afterwards.
Main sample per size: 6 admissible curves (E, C), each with 4 instances = 24 instances (>= 20 verified solves required; all 24 are attempted and all failures reported). Every instance is run on arms R_E, R_J, IC-G, IC-S, IC-D (paired). IC-F runs only in PC-A.
Censoring: a run that hits its cap is "unsolved", counted in the solve rate, its cost set to the cap and flagged as a lower bound; ratios involving it are reported with and without the censored instances.

Positive controls (the instrument must see an effect where one exists by construction):
 PC-A (sensitivity to a q^(1/2) factor). IC-F against IC-D on the same instances at k = 8, 10, 12, 14, 20 instances each (4 curves x 5 instances; the IC-F relation counts at k = 14 are about 32 q = 5.2e5 trials, affordable). Predicted trials ratio IC-F/IC-D = (q^(3/2)/4)/(q/2) = q^(1/2)/2, i.e. 7.9, 16, 32, 64 at k = 8..14 for the formula values (using actual q). Pass: the measured ratio is in [0.5, 2] x formula at every size and the fitted slope of log(ratio) on log q is in [0.35, 0.65]. This proves the harness resolves a q^(1/2) gap in operation counts, which is the size of the effect under test.
 PC-B (planted-answer chain). 100% of verified solves for R_E and R_J (all sizes), the transfer checks of 1.2, the Jacobian cross-check of 1.1(e). Fail if any is below 100%, except rho solve rates: PC-B requires >= 95% (G1 saw 100%).
 PC-C (an arm that beats a baseline, detected as such). On random genus-2 curves y^2 = (monic squarefree quintic) with N = #J (hyperellcharpoly, k = 8, 10, 12 only) of the form N = h' * l'' with l'' prime and h' <= 4 (search up to 400 curves per size; size dropped if none), the instance is a random point of order l'' ~ q^2. Compare IC-D to PLAIN rho (no negation map; the A0 variant of G1: same walk, no canonicalisation and no cycle test, counted natives SEL = DPT = steps, LOOK, START) in the same Jacobian, 20 instances per size. Prediction: plain-rho steps = R_plain sqrt(l''/2) with R_plain = R / 0.75 (G1 steps ratio N1/A0 = 0.748; R = 1.45), about 1.93 * 1.45 / sqrt(2) * q = 1.98 q Jops for l'' ~ q^2, against IC-D about 0.5 q + setup. Pass: IC-D is below plain rho in the ranking sense (section 5, upper CI limit of IC-D / plain rho < 1 at all three w) at k = 10 and 12, and the measured IC-D / plain-rho ratio at w = 0 is in [0.2, 0.8]. This shows the pipeline can rank a genus-2 index-calculus arm as the winner when it should be. If PC-C fails, a negative main result carries no evidential weight beyond the raw counts and the exponent measurements, and is reported so.
Negative controls:
 NC-A (null population for "the arm profits from the gluing"). IC-D on random genus-2 Jacobians (the same curves as PC-C plus, for k = 13 and 8..12, other random draws with N having a prime factor l' >= q^2/32 and l'^2 not dividing N), 20 instances per size at k in {8, 10, 12, 13}. H5 requires the glued/random ratio of attempts T_D in [0.9, 1.1] (95% CI, pooled; per size reported). Fail if the CI excludes [0.85, 1.15] at any size (then the arm does depend on the structure and the claim of obliviousness is rejected and reported).
 NC-B (wrong-answer detector). For 20 instances at k in {8, 10, 12}, run IC-D and R_E with a target H' that is a point of E(F_q) NOT in <G> (possible because h >= 2): the verification of 1.3 must reject 100% of returned values (an unverified return is not a solve). Also, for every verified solve, x' + 1 must fail verification (100%).
 NC-C (sabotaged relation phase). IC-D with the Mumford sign of v flipped on T(H) only (an incorrect transfer): verified-solve rate must be 0/20 at k in {8, 10}; any verified solve means the verifier or the transfer check is blind.
Seeds (all fixed): master seed 20261107. Curve and instance generation at size k: setrand(20261107 * 1000 + k). Per-arm walk seed for (k, curve c, instance i, arm id a): 20261107 * 10^6 + k * 10^4 + c * 100 + i * 10 + a with arm ids R_E 1, R_J 2, IC-G 3, IC-S 4, IC-D 5, IC-F 6, plain rho 7. FB multiplier s from setrand(20261107 * 7 + k). Bootstrap: setrand(31337). Controls use the same rules with the control tag added to the master seed (PC-A +1, PC-C +2, NC-A +3, NC-B +4, NC-C +5, applied as master seed + tag * 10^9).

## 7. Falsification criteria (all fixed before data)

F1 (the central falsifier, from the lead). Any size k and any IC arm (IC-G, IC-S, IC-D) at which the 95% upper CI limit of the paired cost ratio IC / R_E is below 1 at EVERY w in {0, 0.1, 1} (primary kappa_J = 1). Then the claim "the Jacobian arm loses at that size" is falsified. A single-size hit triggers a replication on a fresh seed set (master seed + 10^10) at that size before it is reported as a win; a hit that does not replicate is reported as unreplicated. A Jacobian arm below R_J but above R_E does not trigger F1.
F2 (exponent). IC-D attempts: fitted slope of log T_D on log q (k = 8..16) outside [0.85, 1.15] falsifies the prediction q^1 (measured exponent reported either way). R_E: fitted slope of log steps on log q outside [0.4, 0.6] means the baseline is broken.
F3 (constants). Any of the band predictions of section 4 missed (T_D/q, T_G/q, T_S, T_F, R, kappa_J in [2.0, 4.5], p_split in [0.46, 0.54], H4 slope in [0.35, 0.60]) is reported as a miss with the measured value; a miss of H2 or H4 falsifies the stated theory model (not the "loses" conclusion if F1 does not fire).
F4 (controls). A failed PC-A, PC-B or NC-B/NC-C invalidates the instrument: every Jacobian-arm cost is then reported only as raw counts without a ranking claim. A failed PC-C: negative results are scoped as stated in section 6. A failed NC-A rejects H5 (the arm exploits structure or the model is wrong) and is reported.
F5 (transfer). Any failed transfer check (1.2) or Jacobian cross-check (1.1(e)) falsifies H1 for that instance; the instance is excluded from solve statistics, counted as a transfer failure, and if the failure rate exceeds 1% at a size the size is marked invalid.
F6 (solve rate). IC-D solve rate (verified, within cap) below 95% at a size with >= 20 instances is reported as an arm failure at that size and the cost columns are lower bounds.

## 8. Analysis plan

Per size and arm: verified-solve count / attempts (Wilson 95%); median, geometric mean and quartiles of each counter; instance-level paired ratios of cost (IC / R_E, IC / R_J, R_J / R_E) at each (w, kappa_J) with 95% bootstrap CIs resampling curves, then instances within curve (B = 2000, setrand(31337)). Fitted exponents: OLS of log(geometric mean) on log q over k = 8..16 and over k = 10..16, with bootstrap CIs (resampling instances within size). Measured/predicted ratios for T_D, T_G, T_S, T_F and R. Ranking by the 5a rule (section 5). Distance remaining table: IC-D/R_E and R_J/R_E by size and w. An instance table of the three H-checks (p_split, kappa_J, relation yield). Controls reported with their pass criteria verbatim. No number enters a report unless it comes from a file produced by the code (`out/*.txt`, `out/summary.txt`, `results.json`). The plot (`plot_summary.py`, venv python, `-I`) reads `results.json` only: cost vs q log-log for R_E, R_J, IC-G, IC-S, IC-D with the 5a band as shaded region, and the ratio panel with the slope. A report, a graph and a PDF follow the research-visuals skill when the run completes; no benchmark/leaderboard entry is created by this study.
Sample-size reasoning: ln(steps) of rho has sd ~ 0.5, so 24 instances give a 95% half-width of about 0.2 in ln cost for rho (about 22%); the predicted gap IC/R_E is a factor 5 to 60, far outside that; the IC attempt counts have much smaller relative spread; the slope CI over nine sizes is about +-0.05.

## 9. Known limits (stated now)

 - Prime fields only, E with rational 2-torsion (pattern [1,2] or [1,1,1]) and even cofactor h in {2,...,16}: the subgroup of order l is prime, but E itself is not of prime order. Pattern [3] (including odd #E) has no rational Weierstrass point and is not covered; the arm requires an imaginary model.
 - The gluing family is the one-parameter model Y^2 = f(a x^2 - mu) of P/E6; other (2,2)-gluings are not covered.
 - The matched rho on E is the G1 N1 design rewritten in GP, not the repository's native ecbench rho; its counted natives follow G1's accounting. No claim is made about rho on production curves.
 - Group-law operation counts are in GP-instrumented explicit formulas, not in a native harness; kappa_J is the instrumented ratio, and the Jacobian arms are priced at kappa_J = 1 as the generous bound in the primary comparison.
 - IC theory used here (random-graph threshold, p_split = 1/2) is my own derivation, not a literature constant; it is a prediction that can fail. The graph formulation makes linear algebra negligible for g = 2; this is why IC-S is predicted to lose to IC-D.
 - The exponent over k = 8..16 is a fit over 8 bits of q; it cannot distinguish q^1 from slowly varying log factors.
 - The sample is 24 instances over 6 curves per size; between-curve variation is captured only by the bootstrap over those 6.
 - Preprocessing per curve (construction, vertex indexing is not done at all) is not in the per-solve cost; the sensitivity column "incl. amortised setup" is reported.
 - Rho is not run on the full 2^20+ range; the study makes no statement beyond 2^16.

## 10. Execution, budget, disk, files

Code freeze: scripts `jac.gp` (explicit-formula Jacobian arithmetic with counters, reference Cantor), `glue.gp` (construction, transfer map, admissibility), `ic.gp` (walk, decomposition, sparse solve, arms), `rho.gp` (R_E, R_J, plain rho), `gen.gp` (curve and instance generation), `analyze.gp`, `plot_summary.py` and `run.sh` are written after this file is frozen and hashed into `FROZEN_SHA256.txt` before the first confirmatory run; `run.sh` aborts if a frozen file changed. Commands: `gp -q` per (size, arm group) with `timeout`, at most 2 concurrent processes (4 shared cores).
Disk: only per-instance counter lines (one line per instance and arm; no vertex tables, no relation matrices, no trails) and summaries, expected < 5 MB total, hard cap 150 MB for everything this study writes. `target/` of any Rust crate is not created (none is used); any stray one is deleted. Raw files over 1 MiB, if any appear, are replaced by their SHA-256 in `EXCLUDED_RAW_SHA256.txt`.
Reporting: `RESULTS.md`, `results.json`, `DEVIATIONS.md` (any change after the freeze, with reason and the effect on predictions), and `VERIFICATION.json` (independent replication by a Haiku runner on the same seeds, adversarial math and statistics review by Sonnet verifiers, as in P) are the deliverables; every claim cites a file.
