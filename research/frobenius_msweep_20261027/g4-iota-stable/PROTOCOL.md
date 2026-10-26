# Cell G4 protocol: iota-stable non-subspace factor bases (frozen design, written before any data)

Status: prepared, not run. This file fixes the question, claim classes, arms, controls, metrics, sizes and decision
rule before the first data point. Any later change is a dated deviation appended to section 12, never an edit.

## 1. Question and why it is still open

Target family: binary Koblitz curves E_a: y^2 + xy = x^3 + a x^2 + 1 over F_{2^n} (ECC2K-130 is a = 0, n = 131, cofactor 4).
T = (0,1) is the rational 2-torsion point. Translation by T acts on x-coordinates as iota: x -> 1/x (checked exactly in
`tests/identities.rs`, all n and a used here). Given a fixed target R and an iota-stable x-set F (x in F implies 1/x in F),
replacing an *even* number of summands Q_i of a decomposition R = Q_1 + ... + Q_m by Q_i + T gives another decomposition of
the same R with x-coordinates in F. This is a same-instance stabiliser of order 2^(m-1) that grows with m (the 2026-10-26
audit's derivation; not a ledger theorem). It is the only per-instance symmetry in the audit that grows with m.

Why prior work does not answer it: LEAD-ICB-cofactor-torsion-quotients is closed_obstructed for *subspace* bases only
(the iota-stable subspaces are cosets of subfields, empty at prime n); the production 2-torsion census added 0 relations
on subspace bases. LEAD-ICB-nonlinear-frob-stable-bases concerns Frobenius, not iota. The FGHR 2-torsion symmetrisation is
implemented to m = 3 only. This cell asks what an iota-stable *non-subspace* base W union iota(W) does to relation
statistics and relation economics at m = 2..6, and what its membership system costs.

This cell does not run a solver. It cannot show an improvement over the index-calculus baseline (success criterion in
CLAUDE.md); at most it says whether the relation-collection economics are worth a solver-level cell (section 11).

## 2. Claim classes (controlled-experiment skill)

| id | claim | class | how it is settled |
|---|---|---|---|
| A1 | On an iota-stable F with [h]Q != O, translation by T acts freely on decompositions with pairwise distinct classes, so every orbit has exactly 2^(m-1) elements: D(R) = 2^(m-1) O(R) for every R | proved identity | proof in section 3; checked exactly per base and in unit tests (an instrument check, not a finding) |
| A2 | mean_R D(R) over uniform R equals (number of valid m-subsets)/#E; the same for every base of the same size | counting identity | reported as `lam_pred`, never as a result |
| B1 | Fano factor of D over targets in G is 2^(m-1) times the Fano factor of the orbit count O; O's own Fano factor (Poisson = 1) | measured distribution | exact over all targets, replicated over random bases |
| B2 | per-target hit rate P(O >= 1) = P(D >= 1) on G4 bases versus Poisson at mean O and versus subspace bases | measured distribution | exact over targets, intervals over bases |
| C1 | relation-collection economics: model search cost per independent relation to 90 percent rank, G4 versus subspace, same instrument | measured, comparative, model cost | decision rule section 7 |
| D1 | membership price: exact ANF degree of the indicator of F, and algebraic degree of the free-bits-to-x parametrisation | exact per base | reported; no decision on it alone |
| E | any SAT, Groebner, Weil-descent or wall-time solver claim, any rho comparison, any ECC2K-130 extrapolation | out of scope | not claimed |

A1 and A2 are not experimental findings. They are in the run as instrument checks: a failure voids the cell
(`INSTRUMENT_FAIL`), a pass says only that the code implements the definitions.

## 3. Definitions and the proof behind A1

* E = E_a(F_q), q = 2^n, cyclic of order ne (the plan uses only cyclic groups; `build_group` returns an error otherwise,
  which excludes (n, a) = (11, 0): #E = 2116 = 4 * 23^2 is not cyclic). Generator g; log l(Q) in Z/ne. T has log ne/2.
  h = 2-part of ne (4 for a = 0, 2 for a = 1). G = {l : h | l}, the odd-order subgroup, is the target set.
* Points of F: P_F = {Q : x(Q) in F, [h]Q != O}. This drops x = 0 (T) and x = 1 (the iota-fixed x, order-4 points,
  trivial logarithm in G). Both are removed from *every* arm.
* Class of Q: the 4-set {+-Q, +-Q + T}. Sign of Q in its class: the sign of l(Q) mod ne/2 relative to the class
  representative min(lam, ne/2 - lam), lam = l(Q) mod ne/2.
* Decomposition of R over F: an m-subset of P_F in m pairwise distinct classes with sum R. (Distinct classes remove
  Q + (-Q) and Q + (Q+T) degeneracies identically in all arms.)
* Orbit: decompositions of the same R with the same multiset of (class, sign). Two decompositions with equal key and equal
  sum differ by T-shifts of fixed parity, so orbits are exactly the T-translation orbits.
* Proof of A1. If F is iota-stable, every Q in P_F has Q + T in P_F (x(Q+T) = 1/x(Q), order unchanged). For a
  decomposition with distinct classes the 2^m shift patterns t in F_2^m give 2^m distinct m-subsets of P_F (each class
  holds one element), with sums R + (sum t) T. Exactly the even-weight patterns keep the sum R, so the orbit of every
  decomposition has 2^(m-1) elements, all valid. Hence D(R) = 2^(m-1) O(R). The same count shows the quotient search in
  E/<T> (signed class selections with sum lambda_R mod ne/2) returns O(R) for both R and R + T. QED.
* Relation row: for an orbit with key {(c_i, sign_i)} the row sum_i sign_i u_{c_i} = l(R) mod ne/2 over unknowns u_c =
  log of the class representative. All members of an orbit give the *same* row (this is "independent relations modulo the
  2-torsion identities"); `tests/identities.rs` checks the row/log congruence for every orbit of several bases.
* Independent relations: rank of the rows over F_p, p = largest prime factor of #E (Mersenne 2^31-1 if p < 101; the
  `rank_field` column says which). Columns are class ids, so the 2-torsion identification u_Q = u_{Q+T} is built in.

## 4. Arms (all through the same code path: `run::make_arm` then `run::measure`)

Let l be the cell's size parameter, V a subspace of dimension l, W a subspace of dimension l - 1.

| arm | set | role |
|---|---|---|
| vpoly | polynomial window span(1, t, ..., t^(l-1)), single instance | literature-standard subspace baseline |
| vrand | random l-dimensional subspace, replicates | **comparator** (subspace V); negative control for the stabiliser |
| g4poly | W_poly union iota(W_poly*), W_poly = span(1..t^(l-2)), single instance | G4 on the standard window |
| g4rand | W union iota(W*), W random (l-1)-dim subspace, replicates | **G4 arm** |
| rset | uniformly random set of x, same x-count as the paired g4rand base | size-matched random control (no structure) |
| riota | union of random iota-orbits {x, 1/x}, same x-count | stability without any additive structure: isolates "stable" from "built from a subspace" |
| wrong | W union (c/W*), c a random element != 0, 1 | same construction, same sizes, *wrong* involution: no stabiliser. Sharpest negative control for A1 |

Every arm is built in each replicate from its own derived seed (`derive_seed(seed, cell, arm, replicate)`), so arms do not
share an RNG stream; the arm order inside a replicate is a seeded permutation. rset and riota sizes are taken from the
g4rand base of the same replicate (paired). V cannot be matched exactly: |V \ {0,1}| = 2^l - 2 while the G4 set has
about 2^l - 4 minus the W/iota(W) overlap. The decision table prints the realised item ratio G/V and refuses to classify
(UNMATCHED) outside [0.75, 1.33]; yield is normalised by reporting `lam_pred`, orbits per target and cost per independent
relation rather than raw counts.

Controls, per the controlled-experiment skill:
* **Positive control** (detector can see the effect): g4rand/g4poly/riota must show collapse = sum D / sum O = 2^(m-1)
  exactly and `ident_ok = 1`, `closure = 1`, `quot_ok = 1` on every base. A cell where they do not is INSTRUMENT_FAIL.
  Unit tests additionally plant decompositions and check that their 2^(m-1) translates are present and distinct.
* **Negative controls** (null population): vrand and rset must show collapse <= 1.02 and `closure` < 0.5; `wrong` must show
  collapse <= 1.10. A cell where they do not is CONTROL_FAIL (the detector fires on a null).
* **A/A**: every replicated arm runs a second, seed-disjoint half ("aa"). The A/A ratio of the *same estimand* between the
  halves (vrand main/aa, g4rand main/aa) sets the noise floor delta = max(0.05, A/A half-width) used in section 7.
* **Same instrument on every arm**: one enumerator, one tally, one rank routine, one cost model; no arm has bespoke code.

## 5. Metrics (exact definitions; all per base, then summarised over bases)

Computed over the target set G (the odd-order subgroup, ne/h targets), exhaustively; nothing is sampled over targets.

* `mean_d`, `fano_d` (variance/mean over G), `hit_d` = P(D >= 1); `mean_o`, `fano_o`, `hit_pois_o` = 1 - exp(-mean_o);
  `collapse` = sum D / sum O over G; `lam_pred` = valid m-subsets / ne; `closure` = fraction of items whose T-translate is
  an item (1 on a stable set).
* `active` = columns occurring in some orbit row of some target in G; rank stream: targets of G taken in a seeded random
  order, all orbit rows of each target added to an incremental F_p echelon; `t90` = targets needed to reach
  ceil(0.9 active) rank (NA = never; stall rule: stop after 20 active + 1000 consecutive redundant rows); `rank_final`.
* Cost model (probe counts of a meet-in-the-middle search; **a model of group-law work, not a solver cost**):
  table = C(N, floor(m/2)), probe = C(N, ceil(m/2)) per target, N = number of items. `cpr_blind` uses N = all items
  (symmetry-blind search, available on every arm). `cpr_best` uses N = number of signed classes (= items/2) for stable
  arms (quotient search in E/<T>, validated against the orbit counts) and equals `cpr_blind` for non-stable arms.
  cpr = (table + t90 * probe) / ceil(0.9 active): operations per independent relation at 90 percent rank.
* Price: `deg` = algebraic degree of the indicator of the raw set (including 0 and 1) as a Boolean function of the n
  coordinate bits of x (exact Moebius transform; basis independent), `anf_terms` (basis dependent, descriptive only);
  `param_deg` = algebraic degree of the table from l free bits (selector bit on top for the iota-union arms) to x, which
  is what a solver encoding substitutes for each x_i. Subspace: 1. Analytic, unmeasured: a naive selector encoding of
  x in W union iota(W) is cubic (s * x * w) with l free bits and n equations per slot.
* `enum_ms`: wall time of the instrument per base. It is an instrument cost for the runtime estimate, not a result.

## 6. Cells, sizes and the power / resolution check

Cells (a, n, m, l) are listed by `g4iota cells` (`src/run.rs::cell_list`): a = 0 at n = 13, 17, 19 (primary; ECC2K-130's
curve), a = 1 at n = 11, 13, 17 (robustness, secondary), m = 2..6. Nominal yield lambda = C(2^l - 2, m) / #E:

| a, n (#E) | (m, l): lambda |
|---|---|
| 0, 13 (8012) | (2,7): 0.98; (2,8): 4.0; (3,6): 4.7; (4,5): 3.4; (5,5): 18; (6,5): 74 |
| 0, 17 (130972) | (2,9): 0.99; (3,7): 2.5; (4,6): 4.3; (5,5): 1.1; (6,5): 4.5 |
| 0, 19 (523492) | (2,10): 1.0; (3,7): 0.62; (4,6): 1.07; (5,5): 0.27; (6,5): 1.13 |
| 1, 11 (1982) | (2,6): 0.95; (2,7): 4.0; (3,5): 2.0; (4,4): 0.5; (4,5): 14; (5,4): 1.0; (5,5): 72; (6,4): 1.5; (6,5): 300 |
| 1, 13 (8374) and 1, 17 (131174) | as the a = 0 rows at the same n, scaled by #E |

The l values are the ones whose enumeration (at most C(2^l, m) <= 2.5e6 subsets, enforced by `ENUM_CAP` and a unit test)
fits and whose lambda is in 0.3 to 300; l is an integer, so lambda cannot be tuned and is not matched across m. The
orbit rate per target on the G4 arm is lambda / 2^(m-1) (as low as 0.035 at (0,19,6,5)): m >= 5 cells are low-yield by
construction and that is a finding about the arm, not a defect.

Replicates per half (bases per arm; main and A/A halves are independent): 100 at n = 11, 40 at n = 13, 24 at n = 17,
16 at n = 19; vpoly and g4poly are single instances (no interval, listed only as the standard-window reference).

Resolution check (reasoned before data; revisit with the realised spread):
* Targets are exhaustive, so the only sampling unit is the base. The total number of m-subsets (hence the mean count
  over all targets) is a fixed function of the set, so base-to-base noise sits in the Fano factors, the hit rate, the
  coupon-collector time t90 and the coset share of G. For the orbit count the relative SD of a per-base mean over G is
  about sqrt(Fano_O / (lambda' |G|)), lambda' = lambda / 2^(m-1): 1.5% at (0,13,6,5), 1.5% at (0,17,6,5), 2.1% at
  (0,19,5,5), but about 14% at (1,11,6,4) (|G| = 991, lambda' = 0.05): the n = 11 cells with m >= 5 are exploratory.
* cpr depends on t90, whose coefficient of variation I expect to be 0.1 to 0.3 (coupon collector over about 8 to 250
  columns); a 95% bootstrap half-width on a ratio of two such means is then about 2 * CV * sqrt(2 / R): 11 to 33 percent at
  R = 24 and 8 to 24 percent at R = 40. The predictions in section 8 differ from 1 by factors of 2 or more at m >= 4, which
  that resolution separates; a true ratio of 1.0 (m = 2, 3) is only classed PARITY if the interval fits inside
  [1/(1+delta), 1+delta], which with these spreads can fail: it will then be reported UNRESOLVED, not rounded to parity.
* A ratio inside 0.95 to 1.05 (the C3-style equivalence margin) cannot be resolved by this design at any n. The bootstrap
  with 16 to 24 bases is percentile-based and approximate; it is a spread statement, not a coverage guarantee.

## 7. Decision rule (fixed in advance; implemented in `analyze`)

Per cell, with V = vrand and G = g4rand (main halves):

1. Void conditions. UNMATCHED if items(G)/items(V) is outside [0.75, 1.33]. INSTRUMENT_FAIL if any stable-arm base has
   `ident_ok`, `quot_ok` or `closure = 1` false. CONTROL_FAIL if collapse(g4rand) != 2^(m-1) or collapse(vrand) or
   collapse(rset) > 1.02 or collapse(wrong) > 1.10. CENSORED if more than 20 percent of V or G bases never reach 90 percent
   rank. A voided cell yields no class; it is reported as such and not rerun with altered parameters.
2. Primary estimands: R_best = mean cpr_blind(V) / mean cpr_best(G) (each arm with its best known search: the
   subspace has no symmetry to use, the stable base is searched in the quotient) and R_blind = mean cpr_blind(V) /
   mean cpr_blind(G) (identical blind search on both: the cleanest same-instrument comparison). R > 1: G cheaper per
   independent relation.
3. delta = max(0.05, half-width of the A/A ratios of cpr_blind(V main/aa) and cpr_best(G main/aa)).
4. Class by the 95 percent bootstrap interval [lo, hi] of R: GAIN if lo > 1 + delta; LOSS if hi < 1/(1 + delta); PARITY if
   [lo, hi] lies inside [1/(1+delta), 1+delta]; otherwise UNRESOLVED.
5. Cell tier: primary = a = 0 and n >= 13; everything else secondary (robustness). Conclusions are stated per m over the
   primary cells; a trend across m is stated only if the class sequence is monotone in m in at least two of the three
   primary fields.
6. Verdict wording allowed: "relation-collection economics before membership price: GAIN / PARITY / LOSS / UNRESOLVED at
   (n, m)". GAIN is a license to build a solver-level cell (section 11), nothing more. It is never reported as an
   improvement over the index-calculus baseline.
7. Secondary, reported but not decisive: active-column ratio G/V (linear algebra dimension), Fano of D versus
   2^(m-1) * Fano of O, hit-rate ratios, membership degrees. The linear algebra saving from the smaller column count is
   *not* in cpr; it is shown beside it so that nobody counts it twice or forgets it.

## 8. Priors (honest, pre-data; to be scored against the run)

* A1, positive and negative controls: certain (identity); the run only confirms the code.
* Fano(D) on G4 = 2^(m-1) * Fano(O) exactly. Fano(O) near 1 (within 0.3) with probability 0.7: subset-sum counts inherit
  some repeated-structure variance from the additive structure of W. Hit rate of G4 is about 1 - exp(-lambda / 2^(m-1)),
  i.e. 2^(m-1) times lower than the subspace at small lambda. This is the whole mechanism: the stabiliser clusters
  relations into orbits of 2^(m-1) identical rows.
* Cost model: cpr ~ probe / (distinct rows per target) times a coupon factor that cancels between arms (column counts
  cancel in a per-relation cost). So R_blind ~ 2^(1-m); R_best ~ 2^(ceil(m/2) - m + 1) because the quotient halves the
  item count and meet-in-the-middle saves only 2^ceil(m/2) while orbits lose 2^(m-1). Point predictions R_best =
  1, 1, 0.5, 0.5, 0.25 for m = 2..6; R_blind = 0.5, 0.25, 0.125, 0.0625, 0.03. My probability of GAIN in any primary
  cell: 0.10; PARITY or UNRESOLVED at m = 2, 3: 0.65; LOSS at m >= 4: 0.75. The audit's hope that the group growing with
  m gives a growing advantage is, under this model, backwards: it grows the clustering faster than the search saving.
* Price: `deg` of G4 equals n or n - 1 (as for a random set; inversion has algebraic degree n - 1), against n - l for the
  subspace (probability 0.9); `param_deg` of G4 is l - 1 or l against 1 for the subspace (probability 0.9). If so the
  membership system of the stable base is as expensive as an unstructured set and no cheap Weil-descent-compatible test
  exists in the sense of LEAD-ICB-nonlinear-frob-stable-bases.
* Where this prior is weakest: the cost model is one search algorithm (MITM). A solver that exploits the stabiliser in its
  proof search (symmetry-aware CDCL, canonical-orbit constraints) can save more than MITM does, and the clustering of
  relations into orbits may even help it. Nothing in this cell measures that.

## 9. Variable ledger (what could confound, and how it is held)

| variable | held how |
|---|---|
| instrument | one binary, one commit (`commit.txt`), one SHA-256 (`binary.sha256`); identity checks run first; arms share code |
| host / layout | single-threaded cells on one machine, host recorded (`host.txt`); wall time is not a result variable |
| arm order | seeded permutation of arms inside each replicate (`run_cell`) |
| seeds | `derive_seed(base, [a, n, m, l, arm, replicate])`, no shared stream; A/A half = different replicate indices |
| size mismatch | realised item ratio printed and gated (section 4); paired sizes for rset and riota |
| field polynomial / generator | fixed in `field::poly_for`; the group generator is the first x giving a full-order point; relation statistics are generator independent |
| spot preemption | none: runs are short and restartable per cell (each cell writes its own file; a rerun overwrites that file) |
| analyst | the analysis is a fixed program with hard-coded roles and thresholds (section 7); no manual arm selection. Label masking adds nothing for a fixed program, so it is not performed; an analyst reading the report cannot reassign roles |
| censoring | t90 = NA is counted and gated, not dropped silently |

## 10. Runtime estimate (from code reasoning, not measured)

Per base: enumerating and sorting C(N, m) <= 6e5 (tuple, key) records (about 20 to 60 ms), tally passes over ne entries
(up to 5e5 at n = 19), ANF Moebius pass n * 2^n (10M simple ops at n = 19, about 15 ms), rank stream (bounded by
roughly 600 rows * columns^2 / 2 on the heaviest m = 2 non-stable bases, about 50 to 150 ms). Roughly 0.01 s (n = 11) to
0.15 s (n = 19) per base. Bases per cell: 10 R + 2 (1002, 402, 242, 162 for n = 11, 13, 17, 19). Total over the 36 cells
about 12 minutes single core, plausible range 4 to 40. With 3 parallel jobs: 5 to 15 minutes wall. Memory below 200 MB per
job. Disk: a few MB of TSV.

## 11. What does not count, and what this cell cannot resolve

* A1, A2 and the exact orbit-size statements are theorems or counting identities; they confirm the instrument and are not
  results.
* The cost columns are probe counts of a meet-in-the-middle model. They are not conflicts, propagations, Groebner
  operations or wall time of any solver, are not priced in GAE (AGENTS.md section 5a would require 0, 0.1 and 1 GAE per
  operation for every arm of a *cross-method* comparison; this cell compares two bases under one model search and makes no
  cross-method claim), and have no matched Pollard rho reference. No row from this cell may enter the IC leaderboard or
  `benchmarks/README.md` (validation gate).
* Toy fields (n <= 19) with m up to 6 on very small bases (a few dozen points): class collisions, tiny |G| and low lambda
  distort asymptotics. The cells show the mechanism and its m-dependence, not an n = 131 cost.
* The subspace comparator uses x in V; the G4 base uses W (dimension l - 1) so sizes match. A different pairing (equal
  dimension, double size) would change absolute levels; the model ratios in section 8 would shift by 2^m-type factors, which
  is why the realised item ratio is a gate and why `lam_pred` is printed.
* One curve family (b = 1), one prime-degree field set, cyclic groups only. The non-cyclic case (n = 11, a = 0:
  E is Z/23 x Z/92) is excluded because the single-generator logarithm table needs a cyclic group. (In characteristic 2
  an ordinary curve has E[2] of order 2, so there is no full 2-torsion to worry about; the non-cyclicity is in the odd
  part.) ECC2K-130's group is 4r with r prime, cyclic.
* Nothing in this cell establishes or refutes a solver-level benefit of the stabiliser. If R_best is GAIN or UNRESOLVED
  at m >= 3 in a primary field, the next cell is a chained-S3 SAT or WDSat comparison with the iota-union encoding versus
  the subspace at matched conflicts per relation, with symmetric-orbit-breaking on and off, a matched rho reference and the
  section 5a band. That cell must price the membership system (section 5, `param_deg`) inside the solver, which this one
  only measures as a degree.
* The machine-readable `experiment.protocol/v1` record (`protocol.json`, checker `tools/experiment_protocol.py freeze`) is
  not written in this phase; the director must produce and freeze it from this file, with the same cell list and decision
  rule, before the run. Until then this file is the preregistration of record.

## 12. Deviations (append only)

None yet.
