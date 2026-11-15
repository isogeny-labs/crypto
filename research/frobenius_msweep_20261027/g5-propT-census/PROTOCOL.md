# Cell G5 protocol: Proposition T / Sigma census and the A2-prime overlap census at m = 2..8 (design, revision 2, written before any data)

Status: prepared, not run. This file fixes the question, claim classes, arms, controls, metrics, sizes and decision rule
before the first data point. Revision 2 (2026-10-27) is a pre-data, pre-freeze revision after an independent review; its
changes are listed in section 13. After `tools/experiment_protocol.py freeze` any change is a dated deviation appended to
section 13, never an edit.

Notation (collides with AGENTS.md 8a/8b, which call the field degree m): here **n is the field degree** (n in {11, 13, 17, 19}),
**m is the number of legs** of a decomposition (2..8), r is the order of the prime subgroup G, J = (n-1)/2 the number of lags.
Toy sizes only: nothing in this cell discharges the **m = 83 gate** of AGENTS.md 8a, and any transfer to the 131-bit challenge
field is extrapolation (section 9). Field structure of the toy sizes: ord_n(2) = 10, 12, 8, 18 for n = 11, 13, 17, 19, so only
n = 17 splits x^n + 1 enough to have sigma-stable subspaces (the `stable` arm); n = 11, 13, 19 have an irreducible nontrivial
block as 83 (ord 82) and 131 (ord 130) do, but n = 11 and 13 are far too small for the asymptotics.

## 1. Question and why it is still open

Target family: binary Koblitz curves E_a: y^2 + xy = x^3 + a x^2 + 1 over F_{2^n} (ECC2K-130 is a = 0, n = 131, cofactor 4);
sigma is the absolute 2-Frobenius, G the prime-order subgroup on which sigma acts as a scalar lambda of order n.

Two ledger records were written for this seam and never executed:

* EXP-CERTBIN-a14350 (H-CERTBIN-adccf3, "ZERO RUNS"): Proposition T (the n Frobenius conjugates of one target are n attempts over
  n conjugate bases, because the S3 coefficients lie in F_2), Proposition Sigma (the stabiliser of one fixed-target instance is
  S_m) and an exhaustive m = 2 census of conjugate-attempt correlation. `experiments/EXP-CERTBIN-a14350` holds only a
  specification (runs/ empty).
* H-BINSTD-6428a3 (Lemma A2-prime, "ZERO RUNS"): same-instance overlaps of Sol(R,V) and Sol(sigma^j R,V) are sporadic, of
  "halving form", with a Poisson count at m = 2, n in {17, 23, 31}.

The 2026-10-26 audit (/home/user/crypto-work/frobenius_audit_20261026.json, claim C4) found Proposition T proved for every m (a
two-line identity, the argument does not depend on m or on the chained encoding), the orbit-basis rank statement proved by hand
for every m, and **both stated and priced at m = 2 only**: nothing was ever measured at m >= 3, and the overlap was predicted to
scale with the number of signed sub-sums (about 2^m) at probability about 1/r each without any m >= 3 count.

This cell does three things on the same toy curves, for m = 2..8:

1. **T** (Proposition T instance by instance): for sampled instances (V, R, j, m) verify
   `system(sigma^j R, V)(c, sigma^j w) = sigma^j system(R, sigma^-j V)(c, w)` for the chained-S3 system (free auxiliaries), the
   Boolean recombination matrix of sigma^j, and the induced correspondence of solutions, with controls that must break it.
2. **Overlap** (Lemma A2-prime and the sigma census): exact counts of same-tuple coincidences between conjugate instances as a
   function of m, against an independence model and against a null that uses random multipliers instead of lambda^j; plus
   indicator correlation, stable-base equivariance and Proposition Sigma closure.
3. **Rank** (the second half of C4): the rank contributed by the n-1 conjugates of a set of relations in the plain basis and in the
   Frobenius-orbit basis.

This cell does not run a solver. It cannot show an improvement over the index-calculus baseline (CLAUDE.md success criterion) and
reports no distance to rho. At most it settles which of the "m = 2 only" statements survive at m up to 8 on toy groups.

## 2. Claim classes (controlled-experiment skill)

| id | claim | class | how it is settled |
|---|---|---|---|
| T1 | Proposition T: the field-level identity above holds for every m, every V, R, j when b in F_2; the n x n F_2 matrix of sigma^j is invertible | proved identity | proof in section 3; checked on every sampled instance. A violation is `PROP_T_FALSIFIED_OR_INSTRUMENT_FAIL`; the instrument is suspected first |
| T2 | the conjugated planted chain solution is a chain solution of sigma^j R with its legs in V (solution correspondence) | proved identity | same; planted instances at every m |
| T3 | on an ordinary curve (b random outside F_2) the identity fails in EVERY assignment (prime n: b^(2^j) + b != 0 for 1 <= j < n); with the wrong shift direction it fails in at least 99 percent of trials | control (detector sensitivity) | otherwise CONTROL_FAIL |
| T4 | on a sigma-stable V the conjugate system is the same system after a linear change of the Boolean coordinates (n conjugates collapse to one attempt) | proved identity (instrument check) | stable arm at n = 17 |
| A2 | Lemma A2-prime for every m: S in Sol(R) and S in Sol(sigma^j R) iff some nonempty proper sub-multiset I of the legs, with signs, sums to (1 - s lambda^j)R/2 | proved lemma | consistency check of the sampled coincidences (at most 16 per base, thinned) against the lemma with the multiplier that produced them (it follows algebraically from the sums, so it checks bookkeeping, not the lemma); chain-oracle check on samples |
| B1 | the number U of same-instance coincidences matches the independence prediction U_pred (probability about (2^m - 2)/r per solution per lag) | measured, model-compared | decision table, section 8 |
| B2 | Frobenius coincidences equal those of random multipliers (Koblitz structure adds nothing): D = U_frob / U_null = 1 | measured, comparative, paired | decision table; A/A floor |
| B3 | log2 of the per-solution, per-lag rate grows by about +1 per unit m (until 2^m approaches r) | measured, trend | slope with the model's slope beside it |
| B4 | on (tuple, lag) pairs whose legs lie in ONE sigma-orbit and whose lag/spread satisfy m^2 2^s (2^(j/2)+1)^2 < r, Frobenius coincidences are impossible (Z[tau] valuation theorem, section 3.4) | derived and proved here, not in the ledger; checked exhaustively by unit test; measured as an exact zero | stratum 1L (section 8.5); informative only if the null is expected to hit >= 5 times there |
| B5 | indicator autocorrelation of "R is decomposable" along the orbit exceeds the null only through shared legs | measured | Frobenius minus null, bootstrap |
| S1 | Proposition Sigma, setwise form: Sol(R,V) is never closed under sigma^j, j != 0 | proved for 2^(m-1) < n (a closed pair puts the whole lambda-orbit of n classes inside K(S), and |K(S)| <= 2^(m-1)); for larger m (n = 11, 13 with m >= 5) expected 0 by a probability argument only | closure count: a nonzero count with 2^(m-1) < n is an instrument error, otherwise a finding to replicate |
| S2 | on a sigma-closed base |Sol(R)| = |Sol(lambda^j R)| for all R, j (stable concordance) | proved identity (instrument check) | stable and orbit-union arms |
| S3 | the shared-leg overlap term is a lower bound on the joint solvability of conjugates | proved inequality | checked on every non-closed base where cheap |
| C1 | the n-1 conjugate rows of a relation set add zero rank in the orbit basis, and are exact lambda^j multiples of the original rows | proved identity (instrument check) | rank cells, expected equality in 100 percent of bases |
| C2 | in the plain basis the conjugate rows do add rank (n times the columns) | measured descriptive | ratio rank_plain(nN) / rank_plain(N) |
| E | any solver, SAT, wall-time, per-relation-cost or rho comparison; any ECC2K-130 statement beyond the arithmetic of section 9 | out of scope | not claimed |

| G-NN | the whole decision pipeline returns D = 1 when the truth is 1 | instrument calibration | null-vs-null arm: a second independent random-multiplier set on the same enumeration; D_nn = U_null / U_null2 |
| G-PLANT | the detector sees a known excess | instrument positive control | planted multipliers with a guaranteed hit per lag; recovered excess (U_plant - U_null2)/(J bases) should be 1 on arms that are not sigma-closed, and about n on `orbunion` and `stable` (all n rotations of the planted tuple lie in the base and each hits); only the one-sided gate rec < 0.5 on informative cell-arms aborts |

T1, T2, T4, A2, S1 (for 2^(m-1) < n), S2, S3, C1 are not experimental findings: they are instrument checks and regression tests. A failure voids
the cell; a pass says only that the code implements the definitions. The measured quantities are B1..B5 and C2.

## 3. Definitions, the chained system, and what is proved

### 3.1 The chained system

S3(x1,x2,x3) = (x1x2 + x1x3 + x2x3)^2 + x1x2x3 + b, which is algebraically the repository's `binary_semaev_s3`
((x1+x2)^2 x3^2 + x1x2 x3 + (x1x2)^2 + b) and does not depend on a (the substitution y -> y + c x with c^2 + c = a fixes
x-coordinates). The chain for m legs has m - 1 equations and m - 2 free auxiliaries in F_q, wired as in
`ChainedS3Encoding::build`: S3(x1,x2,u2) = 0, S3(u2,x3,u3) = 0, ..., S3(u_{m-1}, x_m, x_R) = 0. `chain::member` decides the
existence of auxiliaries exactly (a frontier of at most 2^(m-2) values, two roots per quadratic by half-trace).

The repository notes that an intermediate partial sum can be the identity, which has no x-coordinate. The chain with F_q
auxiliaries then fails for the order in which the legs are listed while another order (or the all-sums formulation) succeeds. This
cell therefore defines Sol by signed sums in G (below) and checks the chain only as: chain solution implies signed-sum solution
(exact, all orders), and signed-sum solution implies chain solution for some order (exact at m = 2, 3 where no prefix can vanish,
tested up to order at m = 4, sampled at m up to 8).

### 3.2 Sol and K(S) on G

G = subgroup of prime order r = largest prime factor of #E(F_q) (requires r^2 not dividing #E, n | r - 1, r >= 900; the gate is
`curve_summary`). A **class** is an x-coordinate of a point of G\{O}; its unsigned logarithm d in 1..(r-1)/2 (x(kg) = x(-kg)).
The factor base of a base set is the set of classes with x in V (or in a given class set). For m classes S = {x_1..x_m}:
K(S) = the set of classes of the 2^m signed sums sum s_i d_i (mod r), without class 0. **Sol(a) = {S : a in K(S)}**. Legs outside
G (cosets of the 2-torsion) and twist legs are excluded in every arm; this is a restriction of the problem stated in section 11.

### 3.3 Proposition T and Lemma A2-prime (proved here for completeness)

*Proposition T.* sigma^j is a field automorphism fixing F_2, so for b in F_2 each S3 equation satisfies
sigma^j S3(u,v,w) = S3(sigma^j u, sigma^j v, sigma^j w), hence for the whole chain (auxiliaries transform like summands). With
x_i = sum_k c_ik b_k (c the Boolean coordinates, B = (b_k) a basis of V): the residual vector of system(R, sigma^{-j}B) at
(c, w), mapped by sigma^j, equals the residual vector of system(sigma^j R, B) at (c, sigma^j w). sigma^j acts on the n residual
bits of each block by an invertible F_2 matrix (recombination). Equation by equation, b in F_2 is exactly what is used; for b
outside F_2 the two sides differ by the nonzero constant b^(2^j) + b in every equation.

*Lemma A2-prime (every m).* Let S have legs P_i in G, S in Sol(R) with signs eps, and S in Sol(sigma^j R) with signs eps', that is
sum eps'_i P_i = s lambda^j R, s = +-1 (R and -R have the same x). Let I = {i : eps_i != eps'_i}. Subtracting, 2 sum_{i in I} eps_i
P_i = (1 - s lambda^j) R. I is not empty (lambda^j != s) and not all of [m] (else (1 + s lambda^j)R = 0). G has odd order, so
halving is unique: sum_I eps_i P_i = h R with h = (1 - s lambda^j)/2, and the complement sums to (1 + s lambda^j)R/2.
Conversely such a split puts S in both sets. So every coincidence is a nonempty proper signed sub-sum hitting a *forced
constant*; there are 2^m - 2 sign-subset choices per solution, each hitting its constant with probability about 1/r.
`census::halving_check` verifies, for each sampled coincidence and with the multiplier that produced it, that sb = s mu sa for a
sign s, that I is nonempty and proper, and that the I-part equals h sa with h = (1 - s mu)/2 computed through the inverse of 2.
This follows algebraically from the sums, so it is a bookkeeping consistency check (tables, signs, lag index), not an
independent test of the lemma; the unit test shows it rejects a wrong unit.

### 3.4 The independence model and the valuation theorem (derived here, not in the ledger)

U = number of triples (S, class a in K(S), lag t) with class(mu_t a) in K(S), where mu_t = lambda^t for t = 1..(n-1)/2 (the other
lags are the same unordered pairs {R, sigma^j R}). Independence model: given a in K(S), class(mu a) is uniform on the other
(r-3)/2 classes, so E[U | the K(S)] = J sum_S c(c-1)/((r-3)/2), c = |K(S)|, J = (n-1)/2. For a *random* multiplier mu (any unit
other than +-1) this expectation is **exact**, which makes the null arm a calibration of the model (unit-tested).

*Theorem (valuation obstruction).* lambda is a root of x^2 - mu_a x + 2 mod r (mu_a = (-1)^(1-a)), so G is a module over
Z[tau] (tau^2 - mu_a tau + 2 = 0, discriminant -7, norm of tau equal to 2) through the prime ideal p = (r, tau - lambda) of norm r.
Let all m legs of S lie in one sigma-orbit, leg i being +-lambda^(k_i) P_0 with distinct exponents k_i; rotate so that the exponents
lie in [0, s], s the circular spread, with exponent 0 used. Every signed sum of the legs is alpha d_0 with
alpha = sum eps_i tau^(k_i) in Z[tau]; exactly one term has exponent 0, so alpha = +-1 mod tau and tau does not divide alpha, while
|alpha| <= m 2^(s/2). A Frobenius coincidence at lag j means tau^j alpha - (+-alpha') lies in p for two signed sums alpha, alpha';
it is not zero (tau divides tau^j alpha but not alpha'), so its norm is a nonzero multiple of r, hence at least r. But the norm is
at most (m 2^(s/2) (2^(j/2) + 1))^2. Therefore **if m^2 2^s (2^(j/2) + 1)^2 < r there is no coincidence at lag j for this tuple.**
Tuples with legs in several orbits involve independent logarithms and follow the model. The statement is exact, so the measured
quantity in stratum 1L is the count of Frobenius hits, which must be 0; the model charges these pairs c(c-1)/((r-3)/2) each.
Whether the cell can see a contradiction depends on the null expectation on the same pairs (section 6, 8.5): at the toy sizes
the 1L pairs are few (the condition is satisfied only for small spreads), so most cell-arms will be NOT_INFORMATIVE and the
theorem is mainly checked by the exhaustive unit test (`low_height_single_orbit_pairs_never_coincide`, n = 11, 13, 19).
The earlier, vaguer prediction "D < 1 on all single-orbit tuples" is dropped: outside the low-height pairs the theorem is silent
and a diluted D_1 is only descriptive (B5).

### 3.5 The orbit basis and the rank lemma

Plain basis: one unknown u_x = d_x per class. Orbit basis: one unknown v_O = log of the orbit representative point P_rep; a class
x = sigma^k(rep) has the point +- lambda^k P_rep (the sign sgn_x is read from the tables). A relation sum eps_i d_i = a becomes
the row sum_i eps_i sgn_{x_i} lambda^{k_i} v_{O_i} = a. The conjugate relation (legs x_i^(2^j), signs eps_i tau_i with
tau_i = +-1 from lambda^j d_i = tau_i d_{x'_i}, right-hand side lambda^j a) has orbit row lambda^j times the original: the leg
x'_i has k' = k + j and sgn' = sgn tau. The code builds the conjugate row from the conjugate leg's tables (never by scaling the
original row) and then compares; any difference is counted in `nonproportional`. This holds for every m and every base.

## 4. Experiments, arms and controls

All arms of a cell go through the same function in one invocation, interleaved per replicate in a seeded random order, each
with its own derived seed (`derive_seed(seed, cell, arm, half, replicate)`); arms never share an RNG stream.

### 4.1 Experiment T (`g5census propt`, cells (n, m), n in {11, 13, 17, 19}, m = 2..8)

| arm | b | basis used on the right-hand side | role |
|---|---|---|---|
| koblitz | 1 | sigma^-j B | **the claim**; also positive control for the instrument (planted solutions are accepted) |
| ordinary | random in F_q \ F_2 | sigma^-j B | **negative control for the claim, positive control for the detector**: the identity must fail in every assignment (prime n) |
| wrongdir | 1 | sigma^+j B | detector sensitivity to the basis direction: the identity must fail in >= 99 percent of trials (it holds only if every x_i is fixed by sigma^(2j), i.e. in F_2) |
| stable (n = 17 only) | 1 | sigma^-j B, B a sigma-stable basis (dim 8) | T4: change of coordinates exists, identity holds |

Per trial: random V (dimension l with 2^l - 1 >= 2m, at most min(n-1, 8)), random nonzero target x_R, random j in 1..n-1,
K = 4 random assignments (c, w); checks: field-level identity, Boolean recombination by the explicit matrix of sigma^j,
invertibility of that matrix, an adaptively planted chain solution with legs in sigma^-j V (accepted by `member`), its image under
sigma^j (must be a chain solution of sigma^j R with legs in V), and agreement of `member` on a random (target, tuple) pair and its
conjugate. Two seed-disjoint halves ("main", "aa") of 20000 trials each per arm and cell, in blocks of 500.

### 4.2 Experiment OVERLAP (`g5census overlap`, 54 cells (a, n, m, l), see `g5census cells`)

Curves: (a, n) = (1,11) r = 991, (0,13) r = 2003, (1,17) r = 65587 (the ledger's K17), (0,19) r = 130873 (K19), (1,19) r = 262543
if the gate passes. Other (a, n) with n <= 19 fail the gate (r too small or r^2 | #E); `g5census cells` prints a `CELL_NOT_RUN` line on stderr
(saved as `plan_notes.txt`) for any listed curve that gets no cell. For each (curve, m) the planner takes the smallest l whose
expected yield nu >= 1 inside the per-base cost cap **and whose censoring probability P(nx < m) is at most 2 percent** (nx is
Binomial(2^l - 1, (r-1)/(2q)); a base with fewer than m classes is skipped as TOO_FEW_CLASSES), plus l + 1 if it also fits, where
nu = C(nx, m) 2^m / (r - 1) is the expected number of solutions per target class and nx = 2^l (r - 1) / (2 q) the expected number
of classes in a subspace. Skipped bases (TOO_FEW_CLASSES or OVER_BUDGET) count towards CENSORED in section 8; conditioning on
nx >= m slightly biases the surviving bases and the 2 percent bound limits that. The (1,11) cells at m = 4..7 were moved from
l = 5 (P(nx < m) between 5 and 30 percent) to l = 6 by this rule; (1,17) additionally gets l = 8 at m = 2..4 (sigma-stable
dimension). Subspace sizes are integers, so nu jumps by large factors between l and l + 1 and cannot be matched across arms by
subspaces alone; the `tuned` arm exists for that.

| arm | base | role |
|---|---|---|
| rand | uniformly random l-dimensional F_2-subspace, replicates (main + aa) | **the subspace comparator** (the ledger's non-stable V) |
| poly | window span(1, t, .., t^(l-1)), one instance | literature-standard reference, no interval |
| stable | the two sigma-stable subspaces of dimension 8 at n = 17 | positive control for S2 and the sigma-closed subspace case |
| rset | uniformly random set of classes with |X| equal to the paired rand base | size-matched null for "subspace structure" |
| orbunion | union of round(nx/n) whole random sigma-orbits (sigma-closed, not a subspace) | the orbit-union base of the campaign; sigma-closed non-subspace |
| tuned | random class set of the smallest size with nu >= 4 at this m | **matched relation yield across m** (AGENTS.md matched-yield rule) |

Every base is measured with four multiplier sets in one enumeration: the Frobenius set {lambda^t}, a null set of J random
distinct units, a second independent null set (null2) and a planted set (drawn per base from derived seeds). **D = U_frob / U_null
is therefore a paired comparison on identical tuples**; the independence model U_pred is a second, unpaired reference.

Controls (same code path):
* **Negative control (null)**: random multipliers on the same bases. D = 1 means Koblitz structure adds nothing beyond sign-pattern
  coincidences.
* **Null-vs-null (G-NN)**: null2 is an independent random-multiplier set on the same enumeration, so the truth of D_nn = U_null /
  U_null2 is exactly 1. It runs through the same bootstrap as D and calibrates the pipeline (the fraction of 95 percent intervals
  missing 1 should be about 5 percent).
* **Planted positive control (G-PLANT)**: for each lag a random m-subset S0 of the base and two classes a != b of K(S0) are drawn
  and mu = b/a. Every lag therefore has at least one guaranteed hit (an exact floor, required on every base: a violation is a
  counter failure), and the recovered excess (U_plant - U_null2)/(J bases) should be about 1. It is read only where the planted
  excess is at least 3 sd above the count noise (`informative`); in high-count cells the planted hits are invisible by design.
* **Positive control for the counter**: unit tests plant overlaps (a multiplier mapping one solution class to another) and require
  recall 100 percent and equality with a naive enumerator; in-run instrument columns require halving_bad = chain_pos_bad =
  chain_neg_bad = lb_bad = 0 on every base.
* **Positive control for the biology of the effect**: the stable arm and orbit-union arm must show |Sol(a)| = |Sol(lambda^j a)|
  exactly (S2); rand must not (descriptive).
* **A/A**: rand/rset/orbunion/tuned run a second, seed-disjoint half ("aa"); the 95 percent interval of the ratio D_main / D_aa must
  contain 1 (an instrument gate, section 8); if it does, the two halves are pooled for the estimate.

### 4.3 Experiment RANK (`g5census rank`, cells = the overlap cells with m in {2,3,5,8}, largest planned l)

Arms rand and tuned, two halves, 6 bases per half. For 16 random target classes (and their sigma-orbits excluded from the
independent sample) all relations over the base are found, shuffled (enumeration order is lexicographic and would concentrate the
kept rows on the first orbits) and capped at **N <= min(40, floor(touched orbits / 3))** rows (at least 2). The cap exists because
the orbit-basis column count is at most the number of orbits touched; with more original rows than a third of that,
rank_orbit(N) saturates by pigeonhole, rank_orbit(nN) = rank_orbit(N) holds trivially and the controls cannot grow (the first
design had this flaw at every m = 5 and m = 8 cell). The n-1 conjugates of the N rows are added, and ranks mod r are computed: plain
basis for N rows and for n N rows; orbit basis for N rows and for n N rows; and two controls that must be able to move, **both
row-matched to the (n-1) N conjugate rows**: (i) orbit rows of the conjugates built with a wrong multiplier (a unit that is not a
power of lambda), (ii) (n-1) N rows of fresh independent targets (the same per-target cap as the originals; `n_rel_indep` against
`indep_need` is recorded and the matched fraction reported). Also recorded: rows failing their identity with the true logarithms
(0 expected), inconsistencies (0), non-proportional conjugate rows (0 expected), the effective cap.

## 5. Metrics (exact definitions)

* T: counts of trials with a failing identity, assignments with a failing identity, failing Boolean recombination, singular M_j,
  bad plant, failing conjugate solution, failing domain correspondence, disagreeing random pair; detection rates for the controls;
  upper bound 3/N (rule of three).
* Overlap, per base: nx, nsub (= C(nx, m)), nsol = sum_S |K(S)|, nu = nsol / ((r-1)/2), U_frob, U_null, U_null2, U_plant (sums
  over the J lags), U_pred, per-lag vectors, indicator autocorrelation corr (mean over lags) for Frobenius and null, concordance
  Pr[chi(a) = chi(mu a)], count equality Pr[|Sol(a)| = |Sol(mu a)|], strata by number of distinct orbits among the legs (1, 2, >= 3),
  the low-height single-orbit stratum 1L (pairs, sum c(c-1), U_frob, U_null), sigma-closure counts (when nsol <= 20000), instrument
  columns.
* Pooled per cell and arm over the main and aa bases (after the A/A gate): U_frob/U_pred, U_null/U_pred, D = U_frob/U_null, D_nn =
  U_null/U_null2, each with a 95 percent (D also 90 percent) percentile bootstrap over bases (2000 resamples); the A/A ratio
  D_main/D_aa with a 95 percent interval (1000 resamples).
* Trend: rate = U / (nsol J) per solution per lag; log2 rate regressed on m on the `tuned` arm (matched yield), per curve, with a
  bootstrap interval of the slope (bases resampled within each m); the model's log2 slope beside it.
* Rank: ranks as above, the equality rank_orbit(nN) = rank_orbit(N), the growth fractions of the two controls, the matched
  fraction of the independent control.
* Reconciliation: for every job of `jobs.txt` the exit code, the file, the row count against the expected count (overlap
  1 + stable + 8 reps; rank 4 reps; propt ceil(trials/500) x 2 halves x arms) and ragged lines.

## 6. Sizes and the power / resolution check

Overlap (formulas from the code; c = 2^(m-1) ignores collisions inside K(S), which only lowers it):
U_pred per base = C(nx, m) c (c - 1) J / ((r - 3)/2). The paired count noise of D from H pooled hits is SE = sqrt(2/H); the
equivalence rule needs the 90 percent interval inside +-10 percent, i.e. roughly H >= 2200 pooled null hits to call a true D = 1
equivalent with 90 percent probability. The table is the **operating characteristic of the frozen rule on synthetic Poisson counts**
(unit test `decision_rule_operating_characteristic`, 60 simulations per entry, so each entry has about +-6 percentage points of
simulation error; counts are independent Poisson per base, i.e. **no between-base overdispersion, so the real intervals are wider
and every power below is optimistic**). Columns: expected null hits per base, reps per half (bases per half), then for truth D = 1
the fractions E-OVERLAP / UNRESOLVED / AA_FAIL, for truth D = 1.15 the fraction E-EXTRA, for truth D = 1/1.15 the fraction E-DEFICIT.

| cell (a,n,m,l) | U per base | bases per half | D=1: E-OVERLAP | D=1: UNRESOLVED | D=1: AA_FAIL | D=1.15: E-EXTRA | D=0.87: E-DEFICIT |
|---|---|---|---|---|---|---|---|
| (1,11,2,7) | 9.4 | 200 | 0.88 | 0.02 | 0.10 | 0.48 | 0.40 |
| (1,11,4,6) | 774 | 200 | 0.95 | 0.00 | 0.05 | 0.93 | 0.98 |
| (1,11,8,6) | 1.1e6 | 22 | 0.95 | 0.00 | 0.05 | 0.93 | 0.95 |
| (0,13,2,9) | 23 | 200 | 0.97 | 0.00 | 0.03 | 0.88 | 0.73 |
| (0,13,4,7) | 612 | 200 | 0.98 | 0.00 | 0.02 | 0.93 | 0.98 |
| (0,13,8,7) | 1.3e6 | 9 | 0.95 | 0.00 | 0.05 | 0.95 | 0.97 |
| (1,17,2,8) | 1.0 | 200 | 0.00 | 0.97 | 0.03 | 0.07 | 0.05 |
| (1,17,4,7) | 491 | 39 | 0.95 | 0.00 | 0.05 | 0.92 | 0.95 |
| (1,17,8,6) | 5.1e4 | 8 | 0.90 | 0.00 | 0.10 | 0.90 | 0.95 |
| (0,19,2,12) | 36 | 36 | 0.82 | 0.13 | 0.05 | 0.45 | 0.35 |
| (0,19,4,8) | 277 | 35 | 0.97 | 0.00 | 0.03 | 0.92 | 0.95 |
| (0,19,8,7) | 2.9e4 | 8 | 0.90 | 0.00 | 0.10 | 0.90 | 0.95 |

Reading. (i) **Margin and floor.** The rule can never claim equivalence tighter than +-10 percent, and a D between 1 and 1.10 is
by construction not distinguishable from 1 ("E-OVERLAP" means inside the margin, not equal). (ii) **Low-count cells are
underpowered**: at m = 2 with fewer than about 40 hits per base a true 15 percent excess is called only 45 to 50 percent of the
time, and (1,17,2,8) (the stable-dimension cell, about 1 hit per base) can only ever be UNRESOLVED for rand/rset/tuned; its rand-type
arms carry no weight (the per-m statements use the cell with the largest model weight, section 8.6). A non-detection in such a
cell does not exclude effects below about 30 percent. (iii) The high-m cells (8 or 9 bases per half) reach their power only in the
Poisson idealisation; their real between-base variation (nx and K(S) overlaps differ by base) is not in this table and the A/A
gate misses about 10 percent of the time there. (iv) **Multiplicity**: about 54 cells x 4 arms = 216 intervals, each with a
nominal 5 percent miss rate for each of the A/A, NN and 95 percent "D != 1 detectable" flags, so about 11 of each flag are
expected by chance; none of them is a claim. Claims come only from the per-m majority statements of section 8.6 (at least 3 of 4
curves agreeing), and the alarm for the A/A and NN miss rates is 12 percent (mean 5 percent, sd about 1.5 percent at 216).

Resolution of the m-trend: 7 values of m per curve, each with a ratio-to-model known to a few percent in well-counted cells: the
slope of log2 rate is determined to about 0.05 to 0.1 per unit m when no saturation occurs, and the slope now carries a bootstrap
interval. Saturation does occur: (c-1)/(N-1) reaches 0.13 at (0,13,m=8) and the rate stops doubling; the model includes saturation
through c, so the ratio to the model (not the raw slope) is the test.

T: 40000 trials per arm and cell give a one-sided 95 percent bound of 7.5e-5 on the violation rate if none is seen (rule of three),
and 4 assignments per trial. The planted-solution checks use fewer instances (every trial plants; some draws fail, reported).

B4 power: informative only where the null is expected to hit at least 5 times on the 1L pairs (probability of seeing zero null hits
under the model at most e^-5, 0.7 percent). The 1L condition m^2 2^s (2^(j/2)+1)^2 < r holds only for small spreads and lags
(for example n = 19, r = 130873, m = 2: spread s <= 5 for every lag j <= 9; m = 4: s <= 3), so the expected null hits there are
of the order of 1 to a few per cell-arm at the planned sizes, and **most cell-arms will be reported NOT_INFORMATIVE; the stable
arm (2 bases) is never informative.** The theorem is therefore checked by the exhaustive unit test (a correctness check on
tiny fixtures), not by this run, which can only fail to contradict it where informative.

Rank: exact identities, no statistics. The cap N <= touched/3 keeps rank_orbit(1) below the column count, so the controls can grow;
a cell where either control (the within-orbit control or the independent control) fails to grow in every base is reported NOT_INFORMATIVE rather than as support (code: both must grow, `wg > 0 && ig > 0`).

## 7. Variable ledger

| variable | how held |
|---|---|
| instrument | one binary (sha256 recorded), one enumerator, one tally, one rank routine; no arm has bespoke code; all multiplier sets (Frobenius, null, null2, planted) share one enumeration |
| group, curve, lambda | built once per cell and verified (lambda order n, class equivariance); the same for all arms |
| bases | arms differ only in the base set; rset/orbunion are size-paired to rand; tuned fixes nu |
| seeds | `derive_seed(seed, cell, arm, half, rep)`; null, null2 and planted multipliers from further derived seeds; arm order shuffled per replicate |
| host / time / layout | outcomes are exact integer counts and deterministic functions of the seed and the code, so they do not depend on the host; wall time is never an outcome and is not used in any decision (it is read only by the pre-flight projection, which can abort the launch but cannot change a result) |
| route | frozen in RUN_CARD section 1 (isolab-eks by default, AGENTS.md 9a); the executor runs `run_cells.sh` unchanged |
| spot preemption | none expected to matter; every cell is a deterministic function of the seed, so a killed cell is rerun whole from the same command (the reconciliation table lists unfinished jobs) |
| provenance | `commit.txt` (HEAD, branch, porcelain line counts, cell tree id; the driver refuses a dirty cell directory), `toolchain.txt`, `inputs.sha256` (Cargo.toml/lock, PROTOCOL, RUN_CARD, driver, every source and test file), `binary.sha256`, `host.txt` |
| analyst | the decision rule is implemented in `analyze` and applied without a human step (classes, gates, per-m statements, B4, abort conditions, reconciliation). Arm labels are NOT masked: the arms are structurally distinguishable and the rule is mechanical, so masking would protect nothing |

## 8. Decision rule (fixed in advance; implemented in `analyze`)

1. **Voids and gates** (a voided cell-arm yields no class and is not rerun with altered parameters).
   * T: any violation in koblitz or stable voids the cell (`PROP_T_FALSIFIED_OR_INSTRUMENT_FAIL`; the instrument is suspected
     first). ordinary must fail the identity in every assignment; wrongdir must fail in at least 99 percent of trials; otherwise
     `CONTROL_FAIL`. No other threshold is applied (conjugate-solution and domain failure rates of the controls are reported,
     not thresholded). `run_cells.sh` runs the T cells first and `g5census gate` aborts the launch before the overlap and rank
     cells if any T cell is void.
   * Overlap: any base with halving_bad, chain_pos_bad, chain_neg_bad or lb_bad > 0, or any base on which some lag of the planted
     set has no hit (the guaranteed floor), voids the cell-arm (`INSTRUMENT_FAIL`); **stable or orbit-union** equivariance !=
     1.000 is `CONTROL_FAIL`; a closure count != 0 is reported (S1 wording above); more than 5 percent of a cell-arm's bases
     skipped (TOO_FEW_CLASSES or OVER_BUDGET) makes it `CENSORED`.
   * A/A gate: the 95 percent bootstrap interval of D_main / D_aa must contain 1; otherwise the cell-arm is `AA_FAIL` (unreadable)
     and counts towards the A/A alarm. Arms without halves (poly, stable) have no interval and are reported `SINGLE_INSTANCE`.
   * Rank: equality failing, nonproportional > 0 or rows_bad > 0 is `INSTRUMENT_FAIL`.
   * Reconciliation: a job without its exit code, file or expected row count is `INCOMPLETE` (CELL_NOT_RUN, exit 2, is recorded as
     the instrument's refusal and is not an error).
2. **Model calibration.** The null's U_null / U_pred 95 percent interval must contain 1; otherwise the model column is declared
   `NULL_MISCALIBRATED` and only the paired D is used.
3. **Margin.** delta = 0.10, declared here and not derived from the data. The A/A interval is a gate (rule 1), not a tolerance.
4. **Classes** by the pooled D (main + aa, after the A/A gate): **E-OVERLAP** if the 90 percent bootstrap interval lies inside
   [1/(1+delta), 1+delta] (two one-sided 5 percent tests); **E-EXTRA** if the 95 percent interval lies above 1 + delta;
   **E-DEFICIT** if the 95 percent interval lies below 1/(1+delta); otherwise **UNRESOLVED**. A separate flag "D != 1 detectable"
   marks a 95 percent interval excluding 1; it is descriptive (expected by chance in about 5 percent of cell-arms) and never a claim.
5. **Per-m statements** over the four primary curves ((1,11), (0,13), (1,17), (0,19); (1,19) is secondary). The primary cell of a
   curve and m is the planned l with the largest total model weight sum(U_pred) for the arm (a function of the K(S) sizes only,
   chosen without looking at Frobenius or null hits). A statement for (arm, m) is made only if at least 3 of the 4 curves have the
   same class. A trend in m is stated only if the per-curve class sequences over m = 2..8 are identical in at least 3 of 4 curves.
   The ECC2K-130 family E_0 (a = 0, cofactor 4) has two toy curves (n = 13 and 19); a = 1 (n = 11 and 17, cofactor 2) is a different
   family (the ledger's K17 is an a = 1 model). The two families are tabulated separately ("both X" or the two classes) and never
   pooled into one claim; a statement specific to E_0 rests on two toy curves.
6. **Wording allowed:** "same-tuple overlap of conjugate instances at (n, m): E-OVERLAP / E-EXTRA / E-DEFICIT / UNRESOLVED relative
   to random multipliers". Nothing is worded as a cost, a speed-up, an improvement or a distance to rho.
7. **B4** (valuation theorem, stratum 1L): the observed Frobenius count on provably free pairs must be 0 (a positive count sets the cell-arm class to `VOID`, before the E-class is computed, so it counts in the 25 percent VOID abort, and is a counterexample to the derivation or an instrument error); a cell-arm is informative only if the expected null
   hits on the same pairs E = sum c(c-1)/((r-3)/2) are at least 5, otherwise `NOT_INFORMATIVE`. The stratum table by number of orbits
   (B5) is descriptive and has no decision rule.
8. **Abort conditions** (`analyze` prints them; RUN_CARD section 5): any T void; more than a quarter of the evaluated cell-arms VOID
   or more than a quarter CENSORED; A/A misses or NN misses above 12 percent; the planted control missing on more than 12 percent of
   its informative cell-arms; INCOMPLETE jobs (reported as an incomplete run).

## 9. Scale arithmetic (not measured; for the report only)

Per solution and per lag the overlap probability is about (2^m - 2)/r. For ECC2K-130, r is about 2^129: m = 2 gives 2^-128, m = 8
gives 2^-121, m = 20 gives 2^-109; times the 130 conjugate lags 2^-114 and 2^-102. The sporadic overlap is irrelevant at the
real size on subspace or random bases (the "independence" regime). The valuation theorem of section 3.4 does apply on sigma-closed
bases at every size (the low-height condition becomes m^2 2^s (2^(j/2)+1)^2 < 2^129; for m = 2 it holds up to spread about 60 even at the largest
lag j = 65, and the largest possible spread at n = 131 is 65, so nearly every 2-leg single-orbit tuple would be covered), which is the only part that may carry over; the toy census checks the exact zero at m <= 8 and n <= 19, not any statement
at n = 131. The stable-base positive control cannot be transferred (no sigma-stable index-calculus subspace exists for n = 131 prime
with ord_131(2) = 130; likewise ord_83(2) = 82). **No result of this cell discharges the m = 83 gate of AGENTS.md 8a**, and
transfer to m = 131 is extrapolation until separately checked there.

## 10. Priors (honest, pre-data; to be scored against the run)

* T1, T2, T3, T4, A2, S1 (2^(m-1) < n), S2, S3, C1: certain as identities. The run only confirms the code (probability of no
  violation 0.99; the residual is an implementation slip in the chain solver at some m, which the tests pass at m = 2..8 on n = 11).
* B1/B2: E-OVERLAP (D inside delta = 10 percent of 1) on rand, rset, tuned in at least 80 percent of the cell-arms whose operating
  characteristic in section 6 is at least 0.8 (the well-counted ones; about 40 of the 54 cells): probability 0.7 (lowered from the
  first design because the rule is stricter than it looked; cells with power below 0.5 will often be UNRESOLVED even if D = 1). A
  deficit or excess beyond 10 percent on subspace bases would be a surprise (probability 0.1 each way).
* Model: U_null / U_pred inside [0.97, 1.03] in the well-resolved cells: 0.9. U_frob / U_pred follows U_null.
* B3: slope of log2 rate about +1 per m at small m, flattening toward the saturation limit; ratio to the model inside 5 percent.
* B4: Frobenius count on 1L pairs equal to 0 in every informative cell-arm: 0.97 (the theorem is proved, the residual is an
  error in the proof or the spread bookkeeping, both unit-tested); fraction of cell-arms that are informative at all: 0.3.
* B5: indicator autocorrelation: Frobenius minus null positive on rand only through shared legs (small, positive), 1 on stable.
* G-NN: about 5 percent of D_nn intervals miss 1 (0.8 that the rate is below 12 percent). G-PLANT: recovery inside [0.7, 1.3] on the
  informative cell-arms of rand, rset, tuned (0.9); no prior is registered for orbunion/stable, where the expected recovery is about n.
* C1: equality in 100 percent of bases (0.99). C2: plain-basis rank multiplies by close to n until columns or rows saturate. Rank
  cells informative (both controls grow): 0.8 of the 20 cells; the m = 2 cells may be NOT_INFORMATIVE when the rows touch exactly as
  many orbits as the rank.

The main point of the cell is a negative one that I expect: nothing here is a cost saving. The orbit basis already makes the
conjugate rows free in the sense of costing nothing, but they carry no information (rank gain 0); the plain basis gains rank only by
multiplying the number of unknowns by n. Whether orbit-union systems pay for themselves per relation is the question of cell G2, not
of this one.

## 11. What does not count / limits of the design

* No solver, no SAT, no Groebner, no wall-time, no rho comparison, no cost per relation. Nothing here beats or approaches any
  baseline; the CLAUDE.md success criterion is not engaged and no leaderboard or benchmarks row is warranted.
* Toy sizes only: n in {11, 13, 17, 19}, r between 991 and 262543. At these sizes 2^m approaches r for m near 8 (saturation); the
  ratios to the model at high m are saturation statements, not asymptotics. m > 8 is not covered; the ledger's "m up to 20" is
  extrapolation. Nothing discharges the m = 83 gate (AGENTS.md 8a).
* Family: three of the five curves are a = 1 (cofactor 2); only (0,13) and (0,19) belong to the ECC2K-130 family E_0 (a = 0,
  cofactor 4). The two families are reported separately (section 8.5).
* The domain is G-legs only. Legs in the 2-torsion cosets and twist legs are excluded. Twist legs can only sum into the base curve
  through the 2-torsion, but the full F_q chain over all x in V (Sol as the SAT solvers see it) can contain cosets; that difference is
  not measured. The 2-torsion translation iota is not exercised here at all.
* The chained system is re-implemented here, not imported from `src/cryptanalysis/binary_semaev_chain_sat.rs`; the S3 polynomial
  and the chain wiring were compared by reading, not by running the two against each other. Cross-checking the SAT encoder's
  satisfying models against the dlog census is an open thread.
* n = 23 and n = 31 of H-BINSTD-6428a3, and n = 17, 19 only partly for H-CERTBIN (its cells K17 and K19 are included), are not all
  reproduced: field tables stop at n = 19 here. The m = 2 cells reproduce the ledger's setting on (1,17) and (0,19) except for the
  ledger's specific bases (polynomial window l = 9, 10), which `poly` covers only when the planner chooses that l.
* The independence model is a model. D (paired, same tuples) is the claim-bearing quantity; U/U_pred only describes. Subspace sizes
  jump by factors of 10 or more in nu between l and l + 1, so subspace arms are not yield-matched across m; the `tuned` arm is the
  yield-matched comparison and it is not a subspace.
* Bases with fewer than m classes are skipped, and the surviving bases are slightly biased towards larger nx; the planner bounds
  the skipped fraction at 2 percent per cell.
* The rank cell shows that conjugate rows are redundant in the orbit basis. That is true by construction of the tables and the test
  compares two code paths; it is not an independent confirmation of a physical phenomenon.
* Proposition Sigma closure is expected to be trivially 0; its informative content is small.
* The A/A and null-vs-null gates and the power table use the bootstrap over bases; with 8 bases per half (the high-m cells) the
  bootstrap undercovers somewhat and the nominal 5 percent gate miss rate is about 10 percent there.

## 12. Runtime estimate and pre-flight (from code reasoning; nothing has been run)

* Overlap: the planner's operation count (`census::cell_ops`, built on `census::est_ops` in a unit that counts two multiplier sets:
  subsets x (2 c + 4 c J); arms rand, rset, orbunion, tuned x 2 halves x reps, poly and stable once) summed over the 54 cells is
  1.7e11 inner-loop operations in that unit. The run uses four multiplier sets (Frobenius, null, null2, planted), which roughly
  doubles the hit-counting term that dominates, so about 3e11 real operations (the extra multiplier tables also cost memory: 4 sets
  x J tables of (r+1)/2 words, at most about 20 MB for r = 262543). Random-access table lookups at 3 to 6 ns give 15 to 30 CPU
  minutes; bookkeeping, closure and lower-bound checks add about 2 to 3 minutes.
* Rank: dominated by Gaussian elimination mod r on at most (n-1) N rows (N <= touched/3, so a few hundred rows) and at most N m
  columns per base: well below the first design's 700 x 1200; 20 cells x 24 bases: 2 to 10 CPU minutes.
* T: 28 cells x 120000 to 160000 trials at 20 to 80 microseconds: 3 to 10 CPU minutes.
* Total 25 to 60 CPU minutes, so about 7 to 20 minutes on 3 to 4 free cores. Range I would not be surprised by: 8 to 120 minutes.
  Memory per job below 300 MB.
* **Pre-flight (skill: controlled-experiment, item 3).** After the gate passes and before the first cell, `run_cells.sh` times one
  `--reps 1` invocation of the most expensive planned cell of each curve (scratch output in `reports/run-*/preflight/`, never read
  by `analyze`), computes the ns per planning-unit operation, projects the total CPU time by scaling (using the maximum ns/op over
  curves, so an upper bound) and aborts if it exceeds `G5_MAX_CPU_MIN` (default 400). The timing influences only whether the run
  starts, never a result.

## 13. Revisions and deviations (append only, dated)

* 2026-10-27, revision 2, **before any data and before the freeze** (so these are edits, not deviations), after an independent
  review of revision 1. Changes: (a) orbit-union equivariance is now checked as `CONTROL_FAIL` like the stable arm (the code
  checked only stable); (b) the decision rule is implemented in full in `analyze` (per-m statements, trend rule, abort conditions,
  reconciliation against jobs.txt), and B4 is replaced by the valuation theorem with an exact-zero test on low-height pairs; (c)
  the ordinary/wrongdir thresholds are now the ones stated in 4.1 (every assignment / 99 percent of trials), without additional
  conjugate-solution and domain thresholds; (d) delta is fixed at 0.10 with a TOST criterion on the pooled estimate, A/A became a
  gate, and a null-vs-null arm and a planted positive control were added (the first design's delta = max(0.05, half-width of the
  A/A ratio) returned E-OVERLAP in only about half of the low-count cells even at D = 1, and had no in-run positive control);
  (e) the rank cap is tied to the touched orbits and the independent control is row-matched (the first design saturated by
  pigeonhole in about 12 of 20 cells); (f) `halving_check` now uses the multiplier and is described as a consistency check;
  (g) the planner skips lag-starved subspace sizes (P(nx < m) <= 2 percent), giving 54 cells; (h) provenance, phase order
  (T first, fail-fast), exit codes, pre-flight timing, route; (i) family reporting, glossary, m = 83 disclosure, ord table.
* 2026-10-27, revision 3, still before any data and before the freeze (edits, not deviations), after a second review:
  (a) the trend rule (8.5) in `analyze` was inverted (the exclusion of the missing-cell placeholder matched every class containing
  '-', including E-OVERLAP); it is now `trend_agreement`, which ignores only sequences with a "-" class or no cells, with a unit
  test; (b) B4 (8.7) now sets the class to VOID before the E-class is computed, as written; (c) the cell `.gitignore` re-includes
  `Cargo.lock` (the root ignores `**/Cargo.lock`) so `--locked` builds and the lockfile hash work from a clean checkout; the lockfile
  must be committed with the cell; (d) the rank informativeness sentence in section 6 now states the code rule (both controls
  must grow); (e) G-PLANT truth is per arm (about n on sigma-closed arms); (f) caveat for the decision table: hit counts on
  `orbunion`/`stable` bases come in rotation clusters of n (K(sigma^k S) = lambda^k K(S)), so effective counts are U/n and the
  section 6 Poisson table is optimistic by about sqrt(n) there; the claim B2 'D = 1' is scoped to non-sigma-closed bases, and
  for orbunion/stable D < 1 is expected from the theorem; (g) `run_cells.sh` aborts if git is unavailable or the repo has no HEAD.
* 2026-10-27, revision 4, still before any data and before the freeze (edits, not deviations), after a third review (defect N4,
  route/driver only, no science change): (a) `run_cells.sh` no longer calls `scripts/checkpoint.sh gate`: the gate is a session-side
  launcher gate (it needs the gitignored `.resume/RESUME.md` and an upload credential that must not be in the job). The session runs
  it before submitting and passes the non-secret line of `.resume/.last-push` as `G5_CHECKPOINT_STAMP`; the driver aborts (exit 3)
  if it is empty and records it in `commit.txt`. (b) A `finish` EXIT trap writes `driver_exit.txt`, packs `reports/run-<UTC>/`
  into a deterministic `.tar.xz` with a `.sha256`, and uploads archive and run directory to `$ISOLAB_RESULTS_URI` on every exit
  (exit 90 if the upload fails), because `/work` is temporary. (c) The RUN_CARD argv is now an array (`/bin/bash`, script path, `4`)
  with env fields, not a shell string; the card states that a `.git` checkout and rustc/cargo >= 1.78 are required. A pipeline
  test checks the driver statically (no gate call, stamp required, upload trap) and that it aborts before building when the stamp
  is missing.
* No deviations after the freeze yet.
