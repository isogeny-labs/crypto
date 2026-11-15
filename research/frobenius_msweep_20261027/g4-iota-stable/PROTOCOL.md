# Cell G4 protocol: iota-stable non-subspace factor bases (frozen design, written before any data)

Status: prepared, not run. This file fixes the question, claim classes, arms, controls, metrics, sizes and decision
rule before the first data point. Any later change is a dated deviation appended to section 12, never an edit.
Revision 2 (2026-10-27, still before any data and before `protocol.json` is frozen): revised after an independent
review; the revisions are listed in section 12a. Nothing in this file has been run.

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
| B1 | O's own Fano factor over targets in G (Poisson = 1). That Fano(D) = 2^(m-1) Fano(O) is a corollary of A1 (D = 2^(m-1) O), not a measured distribution; it is printed as an instrument check only | measured distribution (Fano of O) | exact over all targets, replicated over random bases |
| B2 | per-target hit rate P(O >= 1) = P(D >= 1) on G4 bases versus Poisson at mean O and versus subspace bases | measured distribution | exact over targets, intervals over bases |
| C1 | relation-collection economics, per independent relation: model search cost per relation (fractional targets) to 90 percent rank, G4 versus subspace, same instrument, exactly matched item counts | measured, comparative, model cost | decision rule section 7 |
| C2 | total model search cost to 90 percent rank of each arm's own active columns (companion to C1; this is the estimand that sees the halved column count) | measured, comparative, model cost | decision rule section 7 |
| C3 | attribution: whether a C1/C2 difference is due to the stabiliser (wrong-involution control W against G) or to the construction / size | measured, comparative | Q ratios, section 7 |
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

Every arm is built in each replicate from its own derived seed (`derive_seed(seed, cell, arm, replicate, g-attempt, try)`),
so arms do not share an RNG stream; the arm order inside a replicate is a seeded permutation.

**Exact size matching (revision 2).** Within a replicate the decision arms vrand, g4rand, rset, riota and wrong all have
exactly the same number N of items (points with x in the set, after removing x = 0 and 1 and the order-dividing-h
points). The g4rand base is drawn first; every other arm is rejection-sampled (fresh derived seeds, up to 20000 tries) until
its item count equals N; rset and riota are drawn with the g4rand x-count. A g4rand draw is redrawn, with the whole
replicate, if N < 2m or C(N, m) > ENUM_CAP = 3e7 or some arm cannot be matched; the rule is the same for every arm, so it
truncates the common N distribution and cannot favour an arm. Consequences: (1) the enumeration cap can never abort a
matched replicate (the previous design sized l from the nominal count and aborted on the realised random tail); (2) the
mean-item-ratio gate is replaced by an exact per-replicate check (UNMATCHED if any replicate differs); (3) base-to-base
variation of N, which drives cost like N^(c-m), cancels inside each replicate pair and the ratios are bootstrapped over
replicate pairs. Rows record `n_target`, `g_attempt`, `tries`. vpoly and g4poly are single unmatched reference instances,
skipped if over the cap, and take no part in any decision. What matching does not do: V has twice as many classes as G (G is
stable, 4 items per class, V has 2), which is the structural column-count difference and is not removed (section 7, C2).

Controls, per the controlled-experiment skill:
* **Positive control** (detector can see the effect): g4rand/g4poly/riota must show collapse = sum D / sum O = 2^(m-1)
  exactly and `ident_ok = 1`, `closure = 1`, `quot_ok = 1` on every base. A cell where they do not is INSTRUMENT_FAIL.
  Unit tests additionally plant decompositions and check that their 2^(m-1) translates are present and distinct.
* **Negative controls** (null population): vrand and rset must show collapse <= 1.02 on average, `wrong` <= 1.10, and
  `closure` < 0.5 on every vrand, rset and wrong base (implemented in `judge_cell`, not only printed). A cell where they do
  not is CONTROL_FAIL (the detector fires on a null).
* **Null / calibration of the cost-ratio estimand itself (revision 2).** The controls above test the collapse detector. The
  ratio pipeline (matching, rank stream, MITM model, paired bootstrap, classify) is checked on null pairs: (i) the A/A pairs
  vrand main/aa and g4rand main/aa (same arm, disjoint seeds) with a design-level false-alarm gate; (ii) the construction
  null S/W = cpr_blind(rset) / cpr_blind(wrong) (same N, neither stable: it can differ from 1 only through additive
  structure of W union c/W*, not through any stabiliser) and the attribution contrast Q = cpr_blind(wrong) / cpr_best(G)
  (identical construction, differing in the stabiliser only). The pairs V/W and V/S and Q_blind are printed as calibration, not gated
  (amendment A1; revision 2 gave a wrong reason for not printing them: V, W and S are all non-stable with about N/2 classes, so V/W and V/S are
  expected near 1 up to additive structure; a clear departure is reported as a caveat on the cell, it does not void it).
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
  ceil(0.9 active) rank (NA = never; stall rule: when active > 120, stop after 3*active + 300 consecutive redundant rows, as implemented in `enumr.rs::rank_stream`); `rank_final`.
* Cost model (probe counts of a meet-in-the-middle search; **a model of group-law work, not a solver cost**). For N items
  and m terms a MITM has two orientations: table of all floor(m/2)-subsets and one probe of all ceil(m/2)-subsets per
  target, or the reverse. For t targets the cost is table + t * probe and **each arm uses the cheaper orientation for its
  own t** (revision 2; at odd m the reverse orientation is cheaper once t exceeds a few, which the earlier fixed choice
  missed). `cpr_blind` uses N = all items (symmetry-blind search, available on every arm). `cpr_best` uses N = number of
  signed classes (= items/2) for stable arms (quotient search in E/<T>, validated against the orbit counts) and equals
  `cpr_blind` for non-stable arms. The counted-MITM unit test runs an honest meet-in-the-middle in both orientations on
  tiny bases, checks that it returns exactly D[s] decompositions for sampled targets, and that its table and probe counts
  equal binom(N, a) and binom(N, m-a).
  - `cpr` = (table + tf * probe) / ceil(0.9 active): operations per independent relation at 90 percent rank, with the
    **fractional** target count tf = (whole targets before the one that completes 90 percent) + (share of that target's
    rows, in a seeded random order, that was needed). This removes the integer-target quantisation that overcharged
    saturated cells (one probe yielding more relations than 90 percent of the columns needs).
  - `tot` = table + t90 * probe with the whole-target count t90: total operations to 90 percent rank (a probe is paid in
    full). `cpr` is per relation; `tot` is per solved system and therefore also sees the halved column count of G.
  - Right censoring: a base whose stream never reached 90 percent rank has cpr = NA and `cpr_lb_*` = cost with t = targets
    processed (a lower bound on its true cost). The analysis gates on 5 percent censoring and demotes a class that changes
    when NA is replaced by the lower bound.
  - `sat` = (mean orbits per target in G) / ceil(0.9 active): above 1, one target alone supplies the rank that is needed
    (the saturated regime, where the sparse-regime closed forms in section 8 do not apply). Printed per arm and cell.
  - The rank stream stops at 90 percent rank when more than 120 columns are active (dense elimination is quadratic; `t100`
    is then NA by design and is descriptive only); at <= 120 active columns it continues to full rank. Stall rule: 3 active
    + 300 consecutive redundant rows. The echelon runs in u64 arithmetic modulo a prime below 2^32.
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
| 1, 11 (1982) | (2,6): 0.95; (2,7): 4.0; (3,5): 2.0; (4,5): 14; (5,5): 72; (6,5): 300; the cells (4,4), (5,4), (6,4) of revision 2 are dropped, amendment A1 |
| 1, 13 (8374) and 1, 17 (131174) | as the a = 0 rows at the same n, scaled by #E |

The l values are chosen so that the nominal enumeration C(2^l - 2, m) is at most about 5e7 and lambda is in 0.3 to 300; l is
an integer, so lambda cannot be tuned and is not matched across m. The enumeration cap is not a nominal property: the
realised item count is random (about half of the x carry points, and on iota-stable sets they come in pairs x, 1/x), so
the cap is enforced on the realised N by the matched draw (section 4), and a unit test checks that at least half of the
g4rand draws are accepted in every cell under the amendment A1 rule (ncls >= m). The orbit rate per target on the G4 arm is lambda / 2^(m-1) (as low as 0.017 at
(0,19,5,5) and 0.035 at (0,19,6,5)): m >= 5 cells are low-yield by construction. Distinct classes (amendment A1): the lambda column and the closed-form priors ignore the
constraint that the m summands lie in m distinct classes; at l = 5 the G arm has only about 7 to 8 classes at m = 5, 6, so the realised
yield (`lam_pred`) of G is below nominal there and the priors R_best = 1, 0.5, 0.25 are not expected to hold; the data decide. The lambda column is nominal (all 2^l - 2 x carrying points are not realised; realised lambda is `lam_pred`).

Replicates per half (matched replicate sets; main and A/A halves are independent): 100 at n = 11, 40 at n = 13, 24 at n = 17,
16 at n = 19; vpoly and g4poly are single instances (no interval, listed only as the standard-window reference).
Size bookkeeping (corrected): a subspace V of dimension l has 2^l - 1 non-zero x, of which about half carry points; the G4 set
W union iota(W*) has up to 2^l - 2 non-zero x (pairs x, 1/x, less the overlap), again about half carrying, and after
removing x = 1 and the order-dividing-h points the matched N is the same for all arms by construction.

Resolution check (reasoned before data; the realised spread is reported by the run and the first cells decide nothing):
* Targets are exhaustive, so the only sampling unit is the matched replicate. For the orbit count the relative SD of a
  per-base mean over G is about sqrt(Fano_O / (lambda' |G|)), lambda' = lambda / 2^(m-1): 1.5% at (0,13,6,5), 1.5% at
  (0,17,6,5), 2.1% at (0,19,5,5), but large at the n = 11 cells with m >= 5 ((1,11,6,4) is dropped by amendment A1; (1,11,6,5) remains): those are
  exploratory.
* The earlier unmatched design had item-count SD of 12 to 18 percent and cost scaling like N^(c-m), c = ceil(m/2): a cpr CV of
  25 to 55 percent at m >= 4. Exact matching removes that component from every replicate pair and the paired bootstrap
  cancels it. What is left is the coupon-collector time to 90 percent rank and structure; I assume a CV of 0.1 to 0.3 per
  arm (unmeasured; the run reports the realised one).
* With that, the 95 percent half-width of a ratio of two means is about 1.96 * sqrt(2) * CV / sqrt(R) (unpaired upper
  bound; the paired interval is narrower by the share of variance explained by the common N): R = 100 (n = 11): 2.8 to 8
  percent; R = 40 (n = 13): 4.4 to 13 percent; R = 24 (n = 17): 5.7 to 17 percent; R = 16 (n = 19): 6.9 to 21 percent for
  CV 0.1 to 0.3. (The earlier text called 2 CV sqrt(2/R) a half-width and quoted 11 to 33 percent at R = 24; that figure is
  the full width.) A model-predicted factor of 2 or more (m >= 4) is separable at every n >= 13; a true ratio of 1.0 (m = 2, 3)
  is classed PARITY only if the whole interval lies inside the fixed margin 1/1.10 .. 1.10, which fails at n = 17, 19 when
  CV >= 0.25: it is then reported UNRESOLVED, not rounded to parity.
* A ratio inside 0.95 to 1.05 cannot be resolved by this design at any n. The percentile bootstrap with 16 to 24 pairs is
  approximate; it is a spread statement, not a coverage guarantee. No pilot was run to check the CV assumptions (none may
  be run before the checkpoint gate), so the resolution statements above are priors.

## 7. Decision rule (fixed in advance; implemented in `analyze::judge_cell`, tested on planted rows)

Per cell, with V = vrand, G = g4rand, W = wrong, S = rset (main halves; replicate pairs share N exactly):

1. Completeness. `analyze` reads `plan.tsv` (a n m l reps). A cell with no rows is NOT_RUN; a cell whose matched arms do
   not each have exactly `reps` rows with replicate indices 0..reps-1 in both halves is INCOMPLETE. Neither gets a class;
   both are listed and counted against the abort rule.
2. Void conditions. UNMATCHED if any replicate has different item counts across the five matched arms. INSTRUMENT_FAIL if any
   stable-arm base has `ident_ok`, `quot_ok` or `closure = 1` false. CONTROL_FAIL if collapse(g4rand) or collapse(riota)
   != 2^(m-1), or collapse(vrand) or collapse(rset) > 1.02, or collapse(wrong) > 1.10, or any vrand / rset / wrong base has
   closure >= 0.5. CENSORED if more than 5 percent of the V or G main-half bases never reach 90 percent rank. A voided cell
   yields no class and is not rerun with altered parameters.
3. Estimands (paired bootstrap over replicate pairs, ratios of means; R > 1 means G cheaper):
   * primary, per independent relation (C1): R_best = cpr_blind(V) / cpr_best(G) (V blind; G in the quotient) and
     R_blind = cpr_blind(V) / cpr_blind(G) (the same blind search on both);
   * companion, total to 90 percent rank (C2): R_tbest = tot_blind(V) / tot_best(G), R_tblind = tot_blind(V) / tot_blind(G).
     R_total is about 2 R when the stable base has half the active columns, which is the whole point of registering it;
   * attribution (C3): Q_best = cpr_blind(W) / cpr_best(G), Q_blind = cpr_blind(W) / cpr_blind(G);
   * construction null: S/W = cpr_blind(S) / cpr_blind(W); calibration only: V/W, V/S.
4. delta = max(0.05, half-width of the A/A ratios cpr_blind(V main/aa) and cpr_best(G main/aa), unpaired). Class by the 95
   percent interval [lo, hi]: GAIN if lo > 1 + delta; LOSS if hi < 1/(1 + delta); PARITY if [lo, hi] lies inside the
   **fixed** margin [1/1.10, 1.10] (it does not widen with delta: a noisy A/A must not turn a noisy cell into parity; a unit
   test plants exactly that); otherwise UNRESOLVED.
5. Censoring sensitivity. R_best and R_blind are recomputed with never-90-percent bases replaced by their lower-bound cost
   (`cpr_lb_*`); if the class changes, the cell is UNRESOLVED for that estimand.
6. Design-level gates. (a) A/A false-alarm gate: the number of A/A intervals (2 per complete cell) that exclude 1 must not
   exceed the 99th percentile of Bin(k, 0.10) (conservative: a percentile bootstrap with 16 to 100 pairs undercovers);
   otherwise the noise floor is miscalibrated and every class is VOID_AA (design CONTROL_FAIL). (b) If more than a quarter of
   cells are NOT_RUN, INCOMPLETE, VOID, CENSORED or UNMATCHED, the design is reported under-resolved.
7. Cell tier: primary = a = 0 and n >= 13; the rest are secondary (robustness). **No trend rule** (the earlier "monotone in
   two of three fields" was neither specified for disagreeing fields nor implemented and is withdrawn). The summary prints,
   per m over the primary cells, the class counts of R_best and R_tbest; a cell is LICENSED if R_best is GAIN and Q_best is
   GAIN and R_tbest is not LOSS.
8. Wording allowed. A per-m licence needs at least two licensed primary cells at that m. "Closed at the model level at m" needs
   at least two non-void primary cells at m, every one of them with R_best and R_tbest both LOSS or PARITY (no UNRESOLVED), and
   the design gates passed. Anything else is "mixed / unresolved". The sentence "the halved column count helps only linear
   algebra" from revision 1 was wrong for collection (the number of relations needed scales with the active columns) and is
   withdrawn: R_best alone (per relation) can say nothing about total cost; R_tbest does. GAIN is a licence to build a
   solver-level cell (section 11), never an improvement over the index-calculus baseline.
9. Multiple comparisons. There are 33 cells and four non-void estimands per cell; no family-wise correction is applied and
   none is claimed. Under a global null a nominal 95 percent interval clears 1 by chance in about 5 percent of estimands, so a
   single GAIN or LOSS cell is a descriptive observation only; statements are made per m through the counts in rule 7 and
   the licence needs two cells at the same m plus the Q and R_tbest conditions. The null S/W, the A/A flags and the Q
   contrast are the guard against reading construction effects as stabiliser effects. No p-values are reported.
10. Secondary, reported but not decisive: active-column ratio G/V, saturation, Fano of D versus 2^(m-1) Fano of O (instrument
    check), hit-rate ratios, membership degrees.

## 8. Priors (honest, pre-data; to be scored against the run)

* A1, positive and negative controls: certain (identity); the run only confirms the code.
* Fano(D) on G4 = 2^(m-1) * Fano(O) exactly. Fano(O) near 1 (within 0.3) with probability 0.7: subset-sum counts inherit
  some repeated-structure variance from the additive structure of W. Hit rate of G4 is about 1 - exp(-lambda / 2^(m-1)),
  i.e. 2^(m-1) times lower than the subspace at small lambda. This is the whole mechanism: the stabiliser clusters
  relations into orbits of 2^(m-1) identical rows.
* Cost model, sparse regime (sat << 1; closed forms derived by hand, not from data): the stable base has N/4 classes against
  N/2 for V (so about half the active columns) and yields 1/2^(m-1) as many distinct rows per target. Per relation the column
  count cancels: R_blind ~ 2^(1-m); with the cheaper MITM orientation (probe of floor(m/2) items in the sparse regime, where t is
  large) the quotient halves N, so R_best ~ 2^(floor(m/2) + 1 - m). Point predictions for m = 2..6:
  R_best = 1, 0.5, 0.5, 0.25, 0.25 (revision 1 had 1, 1, 0.5, 0.5, 0.25 from the fixed, worse orientation; the correction moves
  the priors against G at odd m); R_blind = 0.5, 0.25, 0.125, 0.0625, 0.031. Total to 90 percent rank sees the halved
  columns: R_tbest ~ 2 R_best = 2, 1, 1, 0.5, 0.5 and R_tblind ~ 2 R_blind = 1, 0.5, 0.25, 0.125, 0.0625. So the prior is:
  per relation, parity at m = 2 and a growing loss; in total, a gain at m = 2 (about 2x), parity at m = 3, 4 and a loss at
  m = 5, 6. The audit's hope that a group growing with m gives a growing advantage is, under this model, backwards at fixed
  matched N. My probability of a LICENSE (R_best GAIN) in any primary cell: 0.10; of R_tbest GAIN at m = 2 in at least one
  primary cell: 0.5; of PARITY or UNRESOLVED for R_best at m = 2, 3: 0.65; of LOSS for R_best at m >= 4: 0.75. Cells with a large nominal lambda
  (for V: (0,13,5,5) lambda 18 against about 27 rows needed, (0,13,6,5) lambda 74, (1,13,5/6,5), (1,11,4..6,5) lambda 14 to 300) approach
  or exceed saturation (sat >= 1) and do not follow the closed forms; they are read from the data with fractional targets,
  their trend with m is confounded with lambda (0.98 up to 74 at n = 13), and it is reported next to `sat`, not interpreted.
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
| size mismatch | exact per-replicate item-count matching of the five decision arms (section 4); UNMATCHED if any replicate differs |
| field polynomial / generator | fixed in `field::poly_for`; the group generator is the first x giving a full-order point; relation statistics are generator independent |
| spot preemption | none: runs are short and restartable per cell (each cell writes its own file; a rerun overwrites that file) |
| analyst | the analysis is a fixed program with hard-coded roles and thresholds (section 7); no manual arm selection. Label masking adds nothing for a fixed program, so it is not performed; an analyst reading the report cannot reassign roles |
| censoring | cpr = NA counts toward a 5 percent CENSORED gate; a lower-bound-cost sensitivity can demote a class to UNRESOLVED (section 7); no silent dropping beyond the gate |
| partial runs | `analyze` reconciles with plan.tsv: NOT_RUN / INCOMPLETE cells get no class and count against the abort rule |

## 10. Runtime estimate (from code reasoning, not measured)

Per matched replicate and base: the draw (cheap; rejection sampling costs 10 to 50 tries of building a base, up to 20 ms at
l = 10), the enumeration and sort of C(N, m) (typically N = 20 to 60 items, C(N, m) <= 1e6, 10 to 100 ms; a rare tail up to
3e7 records, 1 GB, a few seconds), tally passes over ne entries (up to 5e5 at n = 19), the ANF Moebius pass n * 2^n (10M
operations at n = 19, about 15 ms) and the rank stream. The echelon is the uncertain part: dense rows, u64 modulo arithmetic,
bounded at 90 percent rank for more than 120 active columns; at n = 19, m = 2, l = 10 (the heaviest, about 500 active columns
and about 600 rows) I estimate 0.1 to 1 s per base, and a stall (a base that never reaches 90 percent, up to 3 active + 300
redundant rows at about 1 ms) adds seconds. Per-cell budgets (single core, wall): n = 11 and n = 13 cells under 1 to 2 minutes
each; n = 17 cells 1 to 5 minutes; n = 19 cells 2 to 10 minutes, the m = 2 cell the longest; the group build (log table by
repeated addition, ne up to 5e5) about 1 to 3 s per cell. Total over 33 cells: 40 to 90 minutes single core (the previous 12
minutes was too optimistic for the echelon); with 3 parallel jobs 15 to 35 minutes, budget 60. Memory under 300 MB per job
typically, up to about 1.2 GB in the rare tail (three jobs: keep 4 GB free). Disk: a few MB of TSV. `timing.tsv` records the
seconds per cell; the first finished cells should be checked against this table and the run stopped, not altered, if a cell
exceeds 3 times its budget.

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
* Scope limit stated explicitly (reporting contract). This cell does not address the m = 83 gate (AGENTS.md 8a), the m = 31
  exploratory size, or any n >= 23. n = 31, 53 and 83 are infeasible for this instrument: it needs the full logarithm table
  over a group of about 2^n points (n = 31: 2^31 entries and a walk of 2^31 point additions, tens of GB; 53 and 83: impossible)
  and an exhaustive pass over all targets. Therefore **no statement about an improvement on ECC2K-130 or on any index-calculus
  baseline follows from any outcome of this cell**; the success criterion (verified improvement over the current baseline) is
  not touched. Frobenius disclosure: ord_n(2) is 10 (n = 11), 12 (n = 13), 8 (n = 17, so x^17 - 1 splits into two degree-8
  blocks over F_2 and the Frobenius orbit structure is not that of a prime-degree-with-primitive-root field), 18 (n = 19). This
  cell does not use Frobenius, but a sibling cell that does must read these.
* Second-order effects documented but not removed: for a = 0, odd-part collisions (the same log modulo the odd part r across
  different classes {+-Q} and {+-Q + P4}, P4 of order 4) are not merged in the columns; this is the same for every arm and
  affects only the absolute rank counts. The toy fields have nearly all x of a given l-bit set as distinct classes.
* One curve family (b = 1), one prime-degree field set, cyclic groups only. The non-cyclic case (n = 11, a = 0:
  E is Z/23 x Z/92) is excluded because the single-generator logarithm table needs a cyclic group. (In characteristic 2
  an ordinary curve has E[2] of order 2, so there is no full 2-torsion to worry about; the non-cyclicity is in the odd
  part.) ECC2K-130's group is 4r with r prime, cyclic.
* Nothing in this cell establishes or refutes a solver-level benefit of the stabiliser. If R_best is GAIN or UNRESOLVED
  at m >= 3 in a primary field, the next cell is a chained-S3 SAT or WDSat comparison with the iota-union encoding versus
  the subspace at matched conflicts per relation, with symmetric-orbit-breaking on and off, a matched rho reference and the
  section 5a band. That cell must price the membership system (section 5, `param_deg`) inside the solver, which this one
  only measures as a degree.
* The machine-readable `experiment.protocol/v1` record (`protocol.json`; checker `experiment_protocol.py freeze` in the sibling
  crypto-autoresearcher `tools/`) is not written in this phase; the director must produce and freeze it from this file, with
  the same cell list and decision rule, and record the SHA-256 of `src/analyze.rs` (the analysis program) in it before the
  run. After the run the rows are bound with `experiment_protocol.py from-rows ... --protocol protocol.json` and checked with
  `check-export` (RUN_CARD.md section 6). Until then this file is the preregistration of record.

## 11a. Amendment A1 (2026-10-27, before any data; from the round-1 fresh review)

Nothing has been run, so the frozen design is revised here and the changes are recorded.
1. **Feasibility floor.** A stable base has exactly 4 items per class, so the number of classes is ncls = N/4 and the enumerator needs m
   pairwise distinct classes. The revision-2 floor N >= 2m left many g4rand replicates with no decomposition at any target, which would have
   been counted as CENSORED (probability of tripping the 5 percent gate about 0.9 at m = 6, l = 5 and 1 at the l = 4 cells). The matched
   draw now redraws the g4rand base (hence N for all arms) unless ncls >= m, and a matched arm is accepted only if its item count equals N and
   ncls >= m (`run.rs::draw_matched`, `shape_of`). The acceptance check uses the same rule: accepted fractions of g4rand draws are
   0.98 / 0.94 / 0.85 for m = 4 / 5 / 6 at l = 5 (binomial model of the cell test) and 1.0 for the larger-l cells.
2. **Cells dropped.** (1,11,4,4), (1,11,5,4), (1,11,6,4) accept 0.50, 0.23 and 0.06 of draws, so a truncated N distribution would be the
   estimand; they are removed. The plan has 33 cells; n = 11 keeps m = 4, 5, 6 at l = 5. The earlier "36 cells" is superseded everywhere.
3. **Per-m wording** (rule 8) is implemented as written: `analyze.rs::per_m_closed` filters to the non-void primary cells for both the count
   and the all-LOSS/PARITY condition (the code previously let one void cell block "closed"); a synthetic-row test covers it.
4. **Calibration printed**: V/W, V/S and Q_blind appear in the decision table as an ungated column.
5. **Censoring gate** now also covers the wrong and rset main-half bases (cpr_blind), since censored control bases would silently drop pairs from
   Q and the null.
6. Stall-rule text in section 5 reconciled with the code. Convention: in this cell m is the number of summands and n the field degree; the
   AGENTS.md "m = 83 / m = 31" gates use m for the field degree, and section 11 refers to those by "field degree".
7. Not changed: riota stays a collapse positive control only (no G/riota ratio is registered); the `closure < 0.5` control is brittle at
   N about 8 to 14 (one chance coincidence gives closure 4/N); with the l = 4 cells gone the smallest N is 4m = 16, so one coincidence gives
   0.25 at most and the control is not tripped by a single coincidence. The execution route (this container versus isolab-eks) is a decision for
   the task card; this cell does not freeze it. The point-level certificate is the unit test only; an artifact-store step for the rows_*.tsv files
   and a post-run independent expansion of sample orbit rows are listed as pre-run steps for whoever launches the run.

## 12. Deviations (append only)

None yet (no data has been taken).

## 12a. Pre-freeze revision log (revision 2, 2026-10-27; made before any run, from an independent review)

1. Enumeration cap: replaced a hard per-base abort on the realised random item count by exact N-matching with a common redraw
   rule (sections 4, 6). 2. `analyze` reconciles with `plan.tsv`; NOT_RUN / INCOMPLETE are void conditions. 3. Registered the
   total-cost estimand (C2) and withdrew the claim that the halved column count helps only linear algebra; priors rewritten.
4. Fractional-target accounting for cpr; saturation printed. 5. Registered the construction null, the Q attribution
   contrast and the A/A false-alarm gate; synthetic-row tests of GAIN, LOSS, PARITY, UNRESOLVED and every void condition. While
   writing them a flaw of my own design appeared: with PARITY defined by the A/A-widened delta, a noisy cell is classed
   PARITY by default. PARITY now uses the fixed margin 0.10. 6. Exact matching and paired bootstrap; resolution section
   redone (the earlier "11 to 33 percent half-width" was a full width). 7. Censoring gate 5 percent plus a lower-bound
   sensitivity. 8. MITM orientation chosen per arm; counted-MITM test. 9. Closure test added to the controls; trend rule
   withdrawn; B1 reclassified as a corollary; minimum orbit rate corrected to 0.017 at (0,19,5,5). 10. Scope limits, ord_n(2),
   infeasibility, multiple comparisons stated (sections 7, 11). 11. Driver records dirty tree, toolchain, seed, sources hash and a
   manifest; `.gitignore`. 12. Test expanding orbit members to curve points; echelon moved to u64 and stopped at 90 percent
   rank; runtime table re-estimated.

## 12b. Amendment A2 (2026-10-27; pre-freeze, no data taken; from the round-2 independent review)

1. **Draw rule: a base must have a row in G.** For a = 0 every item of a class has the same parity, and a target in G needs an even
   sum, so a base with exactly m classes has one class selection and, with probability about 1/2, no decomposition at any target in G
   (active = 0, no finite cost, counted as censored by construction). Fraction of accepted g4rand bases with no row in G under the
   review's binomial model: 1.96 percent at m = 4, 4.71 percent at m = 5, 8.49 percent at m = 6 (l = 5, a = 0). `draw_matched` now
   redraws the whole replicate unless the g4rand base has at least one decomposition at a target of G (`has_g_row`: exact enumeration,
   cap-bounded, any sum divisible by h), and applies the same acceptance to every matched arm (vrand, rset, riota, wrong), so the rule
   truncates the common N distribution identically for all arms. The acceptance rule of A1 item 1 (ncls >= m, C(N,m) <= cap) is kept and
   this is added to it; the m = 6 acceptance is about 0.78 by the review's recomputation (still above 0.5; model arithmetic, not run).
   Superseded text: the body of section 4 ("redraw if N < 2m") and A1 item 1 describe only the ncls >= m part.
2. **CENSORED gate is an integer comparison.** Registered rule (section 7 rules 2 and 9): CENSORED if MORE than 5 percent of a
   gated arm's bases never reach 90 percent rank. The code used `1 - finite/n > 0.05` in f64, which classes exactly 5 percent (2 of 40,
   5 of 100) as CENSORED because 1 - 38/40 = 0.05000000000000004. It now tests `censored * 20 > n` (`censored_over_5pct`), per gated
   arm and estimand (vrand cpr_blind, g4rand cpr_best and cpr_blind, wrong and rset cpr_blind). The printed censored fractions are unchanged.
3. Tests added: `bases_with_no_row_in_g_are_rejected_by_the_draw_rule` (a = 0, n = 13, m = 4: m-class bases of both parities occur; the
   rejected ones have active = 0 and t90f = NA in `measure`, the accepted ones a finite cost; every arm of eight matched replicates
   has a row in G) and `censoring_gate_is_an_integer_comparison_at_exactly_five_percent` (2 of 40 allowed, 3 of 40 CENSORED, 5 and 6
   of 100, 1 of 16 and 1 of 20, n = 0). The unit tests now number 24.
4. Not changed (caveats, not part of this fix): ncls is still not printed next to m, the resolution priors for the l = 5, m >= 4 cells
   are still unmeasured, and the stale sentences listed by the review are not edited in the body.
