# Cell G5 protocol: Proposition T / Sigma census and the A2-prime overlap census at m = 2..8 (frozen design, written before any data)

Status: prepared, not run. This file fixes the question, claim classes, arms, controls, metrics, sizes and decision rule
before the first data point. Any later change is a dated deviation appended to section 13, never an edit.

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
| T3 | on an ordinary curve (b random outside F_2) and with the wrong shift direction the identity fails | control (detector sensitivity) | detection rate must be >= 0.99, otherwise CONTROL_FAIL |
| T4 | on a sigma-stable V the conjugate system is the same system after a linear change of the Boolean coordinates (n conjugates collapse to one attempt) | proved identity (instrument check) | stable arm at n = 17 |
| A2 | Lemma A2-prime for every m: S in Sol(R) and S in Sol(sigma^j R) iff some nonempty proper sub-multiset I of the legs, with signs, sums to (1 - s lambda^j)R/2 | proved lemma | halving-form check on every recorded coincidence; chain-oracle check on samples |
| B1 | the number U of same-instance coincidences matches the independence prediction U_pred (probability about (2^m - 2)/r per solution per lag) | measured, model-compared | decision table, section 8 |
| B2 | Frobenius coincidences equal those of random multipliers (Koblitz structure adds nothing): D = U_frob / U_null = 1 | measured, comparative, paired | decision table; A/A floor |
| B3 | log2 of the per-solution, per-lag rate grows by about +1 per unit m (until 2^m approaches r) | measured, trend | slope with the model's slope beside it |
| B4 | tuples whose legs lie in a single sigma-orbit have D < 1 (Z[tau] valuation obstruction, section 3.4) | derived here, unverified; measured | stratum table |
| B5 | indicator autocorrelation of "R is decomposable" along the orbit exceeds the null only through shared legs | measured | Frobenius minus null, bootstrap |
| S1 | Proposition Sigma, setwise form: Sol(R,V) is never closed under sigma^j, j != 0 | proved (prime n) | closure count, expected exactly 0 |
| S2 | on a sigma-closed base |Sol(R)| = |Sol(lambda^j R)| for all R, j (stable concordance) | proved identity (instrument check) | stable and orbit-union arms |
| S3 | the shared-leg overlap term is a lower bound on the joint solvability of conjugates | proved inequality | checked on every non-closed base where cheap |
| C1 | the n-1 conjugate rows of a relation set add zero rank in the orbit basis, and are exact lambda^j multiples of the original rows | proved identity (instrument check) | rank cells, expected equality in 100 percent of bases |
| C2 | in the plain basis the conjugate rows do add rank (n times the columns) | measured descriptive | ratio rank_plain(nN) / rank_plain(N) |
| E | any solver, SAT, wall-time, per-relation-cost or rho comparison; any ECC2K-130 statement beyond the arithmetic of section 9 | out of scope | not claimed |

T1, T2, T4, A2, S1, S2, S3, C1 are not experimental findings: they are instrument checks and regression tests. A failure voids
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
`census::halving_check` verifies this identity on every recorded coincidence (an instrument check).

### 3.4 The independence model and its blind spot (derived here, not in the ledger, unverified)

U = number of triples (S, class a in K(S), lag t) with class(mu_t a) in K(S), where mu_t = lambda^t for t = 1..(n-1)/2 (the other
lags are the same unordered pairs {R, sigma^j R}). Independence model: given a in K(S), class(mu a) is uniform on the other
(r-3)/2 classes, so E[U | the K(S)] = J sum_S c(c-1)/((r-3)/2), c = |K(S)|, J = (n-1)/2. For a *random* multiplier mu (any unit
other than +-1) this expectation is **exact**, which makes the null arm a calibration of the model (unit-tested).

Blind spot. lambda satisfies lambda^2 - mu_a lambda + 2 = 0 (mu_a = (-1)^(1-a)), so signed sums of conjugate legs of one point lie
in the rank-2 lattice Z[tau]. If all legs of S lie in one sigma-orbit, with exponent set E and circular spread s, then
a = alpha d_0, b = lambda^j a = tau^j alpha d_0 with alpha a signed sum of tau^k, k in E. Elements of K(S) are alpha' d_0 with
the same minimal tau-valuation (tau has norm 2), while tau^j alpha has valuation shifted by j. If |tau^j alpha -+ alpha'|^2 < r
the difference cannot lie in the prime of norm r unless it is zero, and zero is excluded by the valuation. So when
2^((j+s)/2) m < sqrt(r) (roughly) a single-orbit tuple has **no** coincidence at lag j, whereas the model charges it
(c-1)/((r-3)/2) per class. Tuples with legs in several orbits involve independent logarithms d_O and follow the model.
Prediction (B4): D < 1 in the single-orbit stratum when heights stay below sqrt(r); the effect on the total is the single-orbit
share of the tuples, large at m = 2 on sigma-closed bases (n = 17: stable V has about 4 orbits), small on random bases. The
stratum table measures it; I give the sign of the effect, not its size.

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
| ordinary | random in F_q \ F_2 | sigma^-j B | **negative control for the claim, positive control for the detector**: the identity must fail in every assignment |
| wrongdir | 1 | sigma^+j B | detector sensitivity to the basis direction: must fail in >= 99 percent of trials |
| stable (n = 17 only) | 1 | sigma^-j B, B a sigma-stable basis (dim 8) | T4: change of coordinates exists, identity holds |

Per trial: random V (dimension l with 2^l - 1 >= 2m, at most min(n-1, 8)), random nonzero target x_R, random j in 1..n-1,
K = 4 random assignments (c, w); checks: field-level identity, Boolean recombination by the explicit matrix of sigma^j,
invertibility of that matrix, an adaptively planted chain solution with legs in sigma^-j V (accepted by `member`), its image under
sigma^j (must be a chain solution of sigma^j R with legs in V), and agreement of `member` on a random (target, tuple) pair and its
conjugate. Two seed-disjoint halves ("main", "aa") of 20000 trials each per arm and cell, in blocks of 500.

### 4.2 Experiment OVERLAP (`g5census overlap`, 57 cells (a, n, m, l), see `g5census cells`)

Curves: (a, n) = (1,11) r = 991, (0,13) r = 2003, (1,17) r = 65587 (the ledger's K17), (0,19) r = 130873 (K19), (1,19) r = 262543
if the gate passes. Other (a, n) with n <= 19 fail the gate (r too small or r^2 | #E) and are listed as `CELL_NOT_RUN` by
construction, not run. For each (curve, m) the planner takes the smallest l whose expected yield nu >= 1 inside the per-base cost cap
(and l + 1 if it also fits), where nu = C(nx, m) 2^m / (r - 1) is the expected number of solutions per target class and
nx = 2^l (r - 1) / (2 q) the expected number of classes in a subspace; (1,17) additionally gets l = 8 at m = 2..4 (sigma-stable
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

Every base is measured with two multiplier sets in one enumeration: the Frobenius set {lambda^t} and a null set of J random
distinct units (drawn per base from a derived seed). **D = U_frob / U_null is therefore a paired comparison on identical
tuples**; the independence model U_pred is a second, unpaired reference.

Controls (same code path):
* **Negative control (null)**: random multipliers on the same bases. D = 1 means Koblitz structure adds nothing beyond sign-pattern
  coincidences.
* **Positive control for the counter**: unit tests plant overlaps (a multiplier mapping one solution class to another) and require
  recall 100 percent and equality with a naive enumerator; in-run instrument columns require halving_bad = chain_pos_bad =
  chain_neg_bad = lb_bad = 0 on every base.
* **Positive control for the biology of the effect**: the stable arm and orbit-union arm must show |Sol(a)| = |Sol(lambda^j a)|
  exactly (S2); rand must not (descriptive).
* **A/A**: rand/rset/orbunion/tuned run a second, seed-disjoint half ("aa"); the A/A ratio of D between the halves sets the noise
  floor delta (section 8).

### 4.3 Experiment RANK (`g5census rank`, cells = the overlap cells with m in {2,3,5,8}, largest planned l)

Arms rand and tuned, two halves, 6 bases per half. For 16 random target classes (and their sigma-orbits excluded from the
independent sample) all relations over the base are found (cap 40), their n-1 conjugates are added, and ranks mod r are computed:
plain basis for N rows and for n N rows; orbit basis for N rows and for n N rows; and two controls that must be able to move:
(i) orbit rows of the conjugates built with a wrong multiplier (a unit that is not a power of lambda), (ii) rows of n N relations
of fresh independent targets. Also recorded: rows failing their identity with the true logarithms (0 expected), inconsistencies (0),
non-proportional conjugate rows (0 expected).

## 5. Metrics (exact definitions)

* T: counts of trials with a failing identity, failing Boolean recombination, singular M_j, bad plant, failing conjugate solution,
  failing domain correspondence, disagreeing random pair; detection rates for the controls; upper bound 3/N (rule of three).
* Overlap, per base: nx, nsub (= C(nx, m)), nsol = sum_S |K(S)|, nu = nsol / ((r-1)/2), U_frob, U_null (sums over the J lags),
  U_pred, per-lag vectors, indicator autocorrelation corr (mean over lags) for Frobenius and null, concordance
  Pr[chi(a) = chi(mu a)], count equality Pr[|Sol(a)| = |Sol(mu a)|], strata by number of distinct orbits among the legs (1, 2, >= 3),
  sigma-closure counts (when nsol <= 20000), instrument columns.
* Pooled per cell and arm over the main-half bases: U_frob/U_pred, U_null/U_pred, D = U_frob/U_null, each with a 95 percent
  percentile bootstrap over bases (2000 resamples).
* Trend: rate = U / (nsol J) per solution per lag; log2 rate regressed on m on the `tuned` arm (matched yield), per curve; the
  model's log2 slope beside it.
* Rank: ranks as above and the equality rank_orbit(nN) = rank_orbit(N).

## 6. Sizes and the power / resolution check

Overlap (formulas from the code; c = 2^(m-1) ignores collisions inside K(S), which only lowers it):
U_pred per base = C(nx, m) c (c - 1) J / ((r - 3)/2). Examples at the planned sizes:

| cell (a,n,m,l) | nx | nu | U_pred per base | bases per half | paired Poisson SE of D (one half) |
|---|---|---|---|---|---|
| (1,11,2,7) | 31 | 1.9 | 9.4 | 200 | 3.3 % |
| (0,13,4,7) | 16 | 14.6 | 612 | 200 | 0.4 % |
| (1,17,3,8) | 64 | 5.1 | 122 | 66 | 1.6 % |
| (0,19,2,12) | 511 | 4.0 | 36 | 36 | 3.9 % |
| (0,19,5,7) | 16 | 1.07 | 144 | 148 | 1.0 % |
| (0,19,8,7) | 16 | 25 | 2.9e4 | 8 | 0.3 % |
| (1,19,4,7) | 32 | 2.2 | 138 | 35 | 2.0 % |

The Poisson SE is the *count* noise only (coincidence counts of the Frobenius and null sets on the same tuples). The decision
interval is the bootstrap over bases, which also contains the between-base variation of nx and of the K(S) overlaps; with 8 bases
per half (the high-m cells, capped by cost) that interval will be wider than the table (I expect 3 to 10 percent), and the A/A floor
`delta = max(0.05, half-width of the A/A ratio)` guarantees the classification never claims resolution below 5 percent.
**This design cannot separate D from 1 below about 5 percent in any cell, and a class E-OVERLAP means "inside delta", not "equal".**
It resolves a factor of 2 everywhere and the stratum effect of section 3.4 where the single-orbit share of tuples is above about 10
percent (sigma-closed bases at m = 2, 3).

Resolution of the m-trend: 7 values of m per curve, each with a ratio-to-model known to a few percent: the slope of log2 rate is
determined to about 0.05 per unit m when no saturation occurs. Saturation does occur: (c-1)/(N-1) reaches 0.13 at
(0,13,m=8) and the rate stops doubling; the model includes saturation through c, so the ratio to the model (not the raw slope) is the
test.

T: 40000 trials per arm and cell give a one-sided 95 percent bound of 7.5e-5 on the violation rate if none is seen (rule of three),
and 4 assignments per trial. The planted-solution checks use fewer instances (every trial plants; some draws fail, reported).

Rank: exact identities, no statistics. The controls (wrong multiplier, independent targets) are expected to grow the orbit rank in
most bases; their growth fraction is reported, and a control that never grows (for example because the orbit-column count is
already reached) makes that cell's equality uninformative: it is then reported as NOT_INFORMATIVE rather than as support.

## 7. Variable ledger

| variable | how held |
|---|---|
| instrument | one binary (sha256 recorded), one enumerator, one tally, one rank routine; no arm has bespoke code |
| group, curve, lambda | built once per cell and verified (lambda order n, class equivariance); the same for all arms |
| bases | arms differ only in the base set; rset/orbunion are size-paired to rand; tuned fixes nu |
| seeds | `derive_seed(seed, cell, arm, half, rep)`; null multipliers from a further derived seed; arm order shuffled per replicate |
| host / time / layout | shared 4-CPU machine; wall time is never an outcome and is not used in any decision |
| spot preemption | none expected; every cell is a deterministic function of the seed, so a killed cell is rerun whole |
| analyst | the decision rule is implemented in `analyze` and applied without a human step. Arm labels are NOT masked: the arms are structurally distinguishable and the rule is mechanical, so masking would protect nothing |

## 8. Decision rule (fixed in advance; implemented in `analyze`)

1. Void conditions. T: any violation in koblitz or stable, or a control detecting < 0.99, voids the corresponding cell
   (`PROP_T_FALSIFIED_OR_INSTRUMENT_FAIL` / `CONTROL_FAIL`); nothing from that cell is read as a result. Overlap: any base with
   halving_bad, chain_pos_bad, chain_neg_bad or lb_bad > 0 voids the cell-arm (`INSTRUMENT_FAIL`); stable or orbit-union
   equivariance != 1.000 is `CONTROL_FAIL`; a closure count != 0 is reported as an instrument error or a headline and replicated;
   more than 5 percent of a cell-arm's bases skipped (over the cost cap) makes it `CENSORED`. Rank: equality failing, nonproportional
   > 0 or rows_bad > 0 is `INSTRUMENT_FAIL`. A voided cell yields no class and is not rerun with altered parameters.
2. Model calibration. The null's U_null / U_pred interval must contain 1; otherwise the model column is declared
   `NULL_MISCALIBRATED` and only the paired D is used.
3. delta = max(0.05, half-width of the 95 percent bootstrap interval of the A/A ratio (D_main / D_aa)).
4. Class by the bootstrap interval [lo, hi] of D: **E-EXTRA** if lo > 1 + delta; **E-DEFICIT** if hi < 1/(1 + delta);
   **E-OVERLAP** if [lo, hi] lies inside [1/(1+delta), 1+delta]; otherwise **UNRESOLVED**. E-OVERLAP is the ledger's expected
   outcome ("coincidences are sign-pattern events and need no endomorphism").
5. Conclusions are stated per m over the four primary curves ((1,11), (0,13), (1,17), (0,19)); (1,19) is secondary. A trend in m is
   stated only if the class sequence is the same in at least three of the four curves.
6. Wording allowed: "same-tuple overlap of conjugate instances at (n, m): E-OVERLAP / E-EXTRA / E-DEFICIT / UNRESOLVED relative to
   random multipliers". Nothing is worded as a cost, a speed-up, an improvement or a distance to rho.
7. B4 (single-orbit stratum) is called supported only if D_1 has hi < 1/(1+delta) in at least half of the (cell, arm) pairs where the
   stratum holds >= 2000 tuples and heights stay below sqrt(r) (the table reports counts, not heights; the condition is checked by
   hand from nx, j and r and written in the report).

## 9. Scale arithmetic (not measured; for the report only)

Per solution and per lag the overlap probability is about (2^m - 2)/r. For ECC2K-130, r is about 2^129: m = 2 gives 2^-128, m = 8
gives 2^-121, m = 20 gives 2^-109; times the 130 conjugate lags 2^-114 and 2^-102. The sporadic overlap is irrelevant at the
real size on subspace or random bases (the "independence" regime). The caveat that may not carry over is section 3.4: on
sigma-closed bases (orbit unions) the tuples are orbit-heavy and the Z[tau] structure applies at every size; the toy census measures
its sign at m <= 8, not its size at n = 131. The stable-base positive control cannot be transferred to ECC2K-130 at all (no sigma-stable
index-calculus subspace exists for n = 131 prime with ord_131(2) = 130).

## 10. Priors (honest, pre-data; to be scored against the run)

* T1, T2, T3, T4, A2, S1, S2, S3, C1: certain as identities. The run only confirms the code (probability of no violation 0.99; the
  residual is an implementation slip in the chain solver at some m, which the tests pass at m = 2..8 on n = 11 only).
* B1/B2: E-OVERLAP (D inside delta of 1) on rand, rset, tuned in at least 80 percent of the cells with >= 20 bases: 0.8. A
  deficit or excess beyond 10 percent on subspace bases would be a surprise (probability 0.1 each way).
* Model: U_null / U_pred inside [0.97, 1.03] in the well-resolved cells: 0.9. U_frob / U_pred follows U_null.
* B3: slope of log2 rate about +1 per m at small m, flattening toward the saturation limit; ratio to the model inside 5 percent.
* B4: D < 1 in the single-orbit stratum on stable/orbunion at m = 2, 3 (probability 0.6; the size is genuinely unknown to me);
  strata 2 and 3 at D = 1 (0.8).
* B5: indicator autocorrelation: Frobenius minus null positive on rand only through shared legs (small, positive), 1 on stable.
* C1: equality in 100 percent of bases (0.99). C2: plain-basis rank multiplies by close to n until columns or rows saturate.

The main point of the cell is a negative one that I expect: nothing here is a cost saving. The orbit basis already makes the
conjugate rows free in the sense of costing nothing, but they carry no information (rank gain 0); the plain basis gains rank only by
multiplying the number of unknowns by n. Whether orbit-union systems pay for themselves per relation is the question of cell G2, not
of this one.

## 11. What does not count / limits of the design

* No solver, no SAT, no Groebner, no wall-time, no rho comparison, no cost per relation. Nothing here beats or approaches any
  baseline; the CLAUDE.md success criterion is not engaged and no leaderboard or benchmarks row is warranted.
* Toy sizes only: n in {11, 13, 17, 19}, r between 991 and 262543. At these sizes 2^m approaches r for m near 8 (saturation); the
  ratios to the model at high m are saturation statements, not asymptotics. m > 8 is not covered; the ledger's "m up to 20" is
  extrapolation.
* The domain is G-legs only. Legs in the 2-torsion cosets and twist legs are excluded. Twist legs can only sum into the base curve
  through the 2-torsion, but the full F_q chain over all x in V (Sol as the SAT solvers see it) can contain cosets; that difference is
  not measured.
* The chained system is re-implemented here, not imported from `src/cryptanalysis/binary_semaev_chain_sat.rs`; the S3 polynomial
  and the chain wiring were compared by reading, not by running the two against each other. Cross-checking the SAT encoder's
  satisfying models against the dlog census is an open thread.
* n = 23 and n = 31 of H-BINSTD-6428a3, and n = 17, 19 only partly for H-CERTBIN (its cells K17 and K19 are included), are not all
  reproduced: field tables stop at n = 19 here. The m = 2 cells reproduce the ledger's setting on (1,17) and (0,19) except for the
  ledger's specific bases (polynomial window l = 9, 10), which `poly` covers only when the planner chooses that l.
* The independence model is a model. D (paired, same tuples) is the claim-bearing quantity; U/U_pred only describes.
* The rank cell shows that conjugate rows are redundant in the orbit basis. That is true by construction of the tables and the test
  compares two code paths; it is not an independent confirmation of a physical phenomenon.
* Proposition Sigma closure is expected to be trivially 0; its informative content is small.

## 12. Runtime estimate (from code reasoning; nothing has been run)

* Overlap: the planner's operation count (`census::est_ops`: subsets x (2 c + 4 c J)) summed over the 57 cells and 4 measured arms
  (rand, rset, orbunion, tuned) x 2 halves x reps is about 1.2e11 to 1.7e11 inner-loop operations (1.18e11 before the orbunion arm
  was added). Random-access table lookups at 3 to 6 ns give 6 to 17 CPU minutes; the null tables and bookkeeping add about 2 minutes.
* Rank: dominated by Gaussian elimination mod r on at most about 700 rows x 1200 columns per base (1 to 2 s at n = 19), 20 cells x
  24 bases: 5 to 20 CPU minutes.
* T: 28 cells x 120000 to 160000 trials at 20 to 80 microseconds: 3 to 10 CPU minutes.
* Total 20 to 50 CPU minutes, so 8 to 20 minutes on 3 free cores. Range I would not be surprised by: 5 to 60 minutes. Memory per
  job below 300 MB (the sigma-closure hash sets are capped by nsol <= 20000).

## 13. Deviations from this protocol (append only, dated)

(none)
