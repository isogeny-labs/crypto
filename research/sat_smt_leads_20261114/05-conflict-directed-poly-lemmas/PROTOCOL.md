# PROTOCOL — idea 5: conflict-directed polynomial lemmas (pilot)

Written 2026-11-14 before any measured run. Pilot only: toy sizes, one core of a
shared 4-core container, every number an unverified assertion `[pcc:unverified]`.

## Claim under test

Adding degree-2 ANF lemmas (XORs over the encoding's existing monomial
variables), derived by local Macaulay elimination on the variables that occur in
the solver's recent learnt clauses, reduces the total conflicts a CDCL solver
needs to decide the m=3 binary point-decomposition system, by more than adding
the same number of lemmas derived from random variable supports, and lowers the
fitted conflicts slope (bits per l).

## Prediction (prior)

Not expected to hold. Static degree-2 Macaulay rows already failed in this
repository; the prior is that the lemma space over existing monomials is small
or empty and that directed and random supports are indistinguishable.

## Kill criterion (fixed)

Kill if EITHER
(a) geometric-mean ratio conflicts(random-support arm) / conflicts(directed arm)
    over all instances of the largest l cell is < 2.0, OR
(b) fitted slope (log2 median conflicts vs l, least squares over the l cells) of
    the directed arm is within 0.25 bits/l of the untreated arm's slope.
"Alive" needs both (a) >= 2.0 and slope lower by >= 0.25. If the directed
generator yields no lemma at all on the measured instances the verdict is
kill-criterion-met (treatment is a no-op), stated as such.

## Instance generator

- Field F_{2^n}, polynomial basis, fixed irreducible polynomial per n (lowest
  weight found by search, recorded in results.json).
- Curve y^2 + xy = x^3 + a x^2 + b, a = 1, b drawn from the instance RNG
  (nonzero). No subgroup structure is used.
- V = span of l random linearly independent field elements. x_i = sum of
  bits * basis vectors.
- m = 3. System (bits): unknown x1,x2,x3 in V, auxiliary t,p in F_{2^n}:
    p = x1*x2,  s = x1+x2,
    S3(x1,x2,t) = p^2 + s^2 t^2 + p t + b = 0,
    S3(x3,X_R,t) = x3^2 X_R^2 + x3^2 t^2 + X_R^2 t^2 + X_R x3 t + b = 0.
  Variables a_i,b_i,c_i,s_i (l each), p_k,t_k (n each). Monomial variables
  a_i b_j, s_i t_j, p_k t_j, c_i t_j (AND gates). 3n quadratic equations + l
  linear ones, each one XOR over variables and monomial variables.
- Targets per cell: 8 = 4 planted (R = P1+P2+P3, P_i curve points with x in V,
  so SAT) + 4 random curve points (status whatever it is). Reported separately
  by actual SAT/UNSAT status.
- Seeds: instance seed = 100000*n + 1000*l + idx, idx 0..7 measured,
  idx >= 900 calibration only (never reported as results).

## Validation before measurement

- planted targets must be SAT and the model must lift: x_i from the model are x
  coordinates of curve points with some sign choice summing to R (checked with
  real curve arithmetic); when the model does not lift over F_{2^n} this is
  recorded (S4 = 0 does not imply rational y) but the algebraic check
  S3-chain = 0 must hold exactly.
- smallest cell: SAT/UNSAT of every measured instance compared with brute-force
  enumeration of all (x1,x2,x3) in V^3 of the same algebraic system.
- every lemma is checked to vanish on the planted solution (soundness).

## Solver and metric

- Primary solver: Glucose 4 via python-sat (exact conflict / decision /
  propagation counters, learnt clauses readable through the DRUP proof log, which
  is the "conflict" signal). XORs are CNF-encoded by cutting into arity-<=4
  pieces (identical for all arms). This is a deviation from "pycryptosat
  outer loop": the installed pycryptosat 5.18 exposes neither counters nor
  learnt clauses. If time allows, a secondary CryptoMiniSat (native XOR) replay
  of the final lemma sets is run with chunk-granular conflict counting.
- Primary metric: total conflicts to decide (sum over rounds). Secondary:
  decisions, propagations, lemma-generation bit operations (64-bit-word XORs in
  the elimination, counted), wall time.
- Outer loop identical in all arms: rounds of B conflicts
  (solve_limited + conf_budget), B fixed per cell from calibration seeds
  (about median untreated conflicts / 8, min 50). Between rounds an arm may add
  lemmas.

## Arms (same instances, same solver seed, interleaved per instance)

- A0 untreated: no lemmas.
- A1 static: before round 1, all lemmas of the full generator (multipliers =
  all variables, all equations). Control known to fail here.
- A2 directed: after each undecided round, S = top-k variables by occurrence in
  the round's learnt clauses (a monomial variable counts for both factors;
  XOR-cut auxiliaries ignored); local generator with multipliers S; add up to
  Lmax new lemmas (shortest first).
- A3 random-support: as A2 but S uniform random of size k; lemma count per
  round matched to A2's count for that instance and round (A2's mean per-round
  count for later rounds), resampling supports up to 5 times.
- Reference counts (not solver runs): enumeration 2^{3l}, MITM 2^{2l} (approx),
  linear 2+1 scan 2^{l} leaf solves — the baseline any SAT arm has to approach.

Generator: rows = {f} U {v*f : v in multipliers, f in equations} in multilinear
ANF (v^2 = v); columns ordered cubic > quadratic monomials that are not solver
variables > existing monomials, linear, constant; Gaussian elimination by
leading column; rows with leading column in the last block that are
independent of the original equations and of earlier lemmas are the lemmas.

Calibration-fixed parameters (chosen on calibration seeds only, recorded in the
README before the measured run): n and the three l values (largest set with
untreated median <= ~2e5 conflicts), B, k in {8,12,16} (smallest giving >= 1
lemma on calibration instances), Lmax = 64. Contingency: if the lemma space
over existing monomials is empty even for the static full generator, a second
variant "aug" is run in which the monomials S x S are added as new AND
variables and count as existing (for A1: not run in aug form if > 5000 new
variables); both variants are reported.

## Budget

About 40 minutes total, single runs <= 5 minutes, one core.

## Addendum (after calibration on idx 900-902 only, before the measured run)

Calibration log (unreported seeds, Glucose 4, n = 13): untreated conflicts
l=2: 2505 / 3798 / 7648; l=3: 18071 / 10954 / 12881; l=4: 15839 (planted) /
76508; l=5 not affordable for 8 targets x arms in the budget.
Static full generator: 142 new lemmas (l=2), 101 (l=3) over existing monomials,
so the "aug" contingency is NOT triggered and is not run.

Fixed for the measured run:
- n = 13, l in {2, 3, 4}, idx 0..7 (0-3 planted, 4-7 random), m = 3.
- B = 470 (l=2), 1600 (l=3), 9500 (l=4).
- k = 8 (protocol rule: smallest k with >= 1 lemma on calibration). Because the
  single l=4 calibration instance behaved very differently at k=8 (658409
  conflicts) and k=12 (68891), a second pair A2/A3 at k = 12 is run and
  reported as a secondary, declared-here arm pair. Kill criterion is evaluated
  on k = 8; k = 12 is reported next to it.
- Lmax = 64. Per-run time cap 150 s (TIMEOUT rows are censored and reported).
- Known instrument limit found in calibration: pysat Glucose honours the
  conflict budget only at restarts, so a round ends at the first restart after
  B conflicts. At l = 2 many instances finish before the first such restart, so
  A2/A3 can be identical to A0 there; this is reported per instance (rounds,
  lemma counts) and the l=2 cell cannot support the claim either way.
  MiniSat22 (exact budgets) has no proof log in pysat and was not usable.
- Arm order per instance: A0, A1, A2k8, A3k8, A2k12, A3k12 (A3 must follow its
  A2 for count matching), instances interleaved planted/random.
- Brute-force status check on all three cells (not only the smallest).
