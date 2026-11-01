# G2 protocol: per-slot-shift orbit unions priced against the exact class count D(n,m)

Status: FROZEN DESIGN (revision 3, after two independent reviews), prepared, not run. Nothing in this directory is a result.
Revision 3 (2026-10-27, before any data) is documented in section 13; where a section below changed it says "(rev 3)".
Lead: LEAD-ICB-frobenius-search-symmetry (closed), reopening "orbit-union descended systems with per-slot shifts".
Cell G2 of the 2026-10-27 m-sweep campaign. Audit source: `/home/user/crypto-work/frobenius_audit_20261026.json`.
Frozen sha: the last commit that touches this directory (`git log -1 --format=%H -- <this dir>`); `run_cells.sh` refuses to run
on a dirty tree under this directory and writes that sha, the HEAD, `rustc -V` and the seed into `run_meta.txt`.

Notation. In this document n is the FIELD DEGREE (F_{2^n}) and m is the DECOMPOSITION SIZE (number of points summed). This is
the reverse of the repository's convention where m is the field degree (m = 83, m = 131). Where AGENTS.md 8a/8b is quoted the
field degree is called "field degree" explicitly. Section 12 gives the disclosures those rules require.

## 1. Question

For a random, non-Frobenius-stable subspace V of dimension l in F_{2^n} and U = union_k sigma^k V, take a target R and
all decompositions R = Q_1 + ... + Q_m with x(Q_i) in U. Group them by shift pattern (multiset of the shifts k_i with
x(Q_i) in sigma^{k_i} V) modulo the diagonal shift, i.e. into the D(n,m) = C(n+m-1,m)/n classes.

* Q1. Is the union's yield spread over the classes as the independence null says (uniform up to the trivial shape effect of
  repeated shifts and the trace-parity constraint of section 5), or is it concentrated in few classes, replicably across
  independent draws of V, and is that concentration specific to the Frobenius labelling (absent in the same union with the
  labels permuted)? Frobenius-specific concentration would be a candidate m-dependent lever (a solver need only visit those
  classes). Uniformity means the union is cost-neutral on the yield side.
* Q2. Priced exactly: what are D(n,m), the per-class and per-representative-pattern yields, and by what factor does the
  n^(m-1) baseline of KN-FIND-47da4e / H-BINSTD-c72284 (m = 2) overcount distinct systems? (arithmetic; section 3)
* Q3 (descriptive). Total union yield versus plain V, and versus a LARGER compact subspace, normalised per m-subset of
  liftable x-values (section 4: the plain-subspace arm is not size-matched and is not called that).

## 2. Claim classes (controlled-experiment skill)

| id | claim | class | how it is settled |
|---|---|---|---|
| A1 | Z/n acts freely on size-m shift multisets for prime n > m; classes = D(n,m); every class has n patterns | exact theorem | unit tests (direct enumeration, Burnside by direct count, n = 5..11, m = 2..5), formula values n = 7..131 |
| A2 | per-slot Proposition T: count(sigma R, pattern + 1) = count(R, pattern), exactly, any m | exact identity | unit test on all patterns and targets; re-checked on 12 targets per union arm at run time (`audit` column, reported) |
| A3 | sum over patterns of prod_s C(X_s, mu_s) = C(sum X_s, m) (Vandermonde); one representative per class holds exactly 1/n of it when blocks are equal; D*B^m/m! is a lower bound of the representative total, short by the repeated-shift weights B^m/prod(mu!) | exact identity | unit tests (`vandermonde_and_factor_n_relation`) |
| A4 | pure-shift class count of R = sum over j of plain-V counts of sigma^j R | exact identity | unit test |
| A5 | (rev 3: refined by A5b) halving criterion: a point P with x != 0 is in 2E iff Tr(x(P)) = Tr(a); all targets lie in hE inside 2E, so a decomposition has an even number of points with Tr(x) != Tr(a); trace and liftability are sigma-invariant, so the n Frobenius blocks share ONE trace profile | exact identity | unit tests: halving criterion on n = 7, 11 both curves; a ker-Tr family has exactly 0 decompositions (odd m) and the parity null gives weight 0 |
| A5b | (rev 3) a = 0: #E = 4 * odd with a CYCLIC 2-part Z/4, every target lies in 4E. A liftable x (point +-Q) has class 0, 2 or odd (+-1) in E/4E. Over distinct-x tuples the number of the 2^m sign choices whose sum is in 4E is 0 if #odd is odd, 2^(m-1) if #odd > 0 is even, and for #odd = 0 it is 2^m if #type-2 is even and 0 if odd | exact identity | unit tests `mod4_sign_count_structure_for_a0` (n = 7, 11, 13, 3000 random tuples each, zero violations; types computed by the independent test "(#E/4) Q is O / order 2 / order 4") and `type2_only_family_has_exactly_zero_decompositions_for_odd_m_when_a_is_zero` (exact zero from the enumerator; the mod-2 null predicts a positive count there) |
| A6 | group law of `Curve::add` | exact | unit test against an independent table-free GF(2^n) implementation (n = 7, 11, 13, both curves, doublings and negatives included); point counts equal the Koblitz order |
| B1 | the union yield is uniform over classes (parity-aware null) / concentrated, and a concentration is Frobenius-labelling-specific | measured, comparative, population = random V at the cell's (a,n,m,l) | experiment, section 7 |
| B2 | union yield per m-subset of liftable x equals that of a compact subspace | measured, descriptive | experiment, ratio reported, no decision |
| C  | solver cost (conflicts, propagations, time per relation) of any arm | NOT claimed | belongs to the SAT cells of the campaign; this cell counts decompositions only |

No decomposition is a discrete-log solve and none is independently verified by a second implementation; counts come from
`Curve::add`, validated by A6 and by the planted-decomposition test, nothing more. No certificate is emitted: no sample of
decompositions and no second-implementation verifier are written, so a validator of the eventual run cannot re-verify any counted
decomposition from the artifacts and should be expected to return insufficient_evidence for relation-level claims. This cell makes
none (counts and class shape only; section 9). AGENTS.md 5a: the cell makes no cross-method cost claim and prices no native work,
so no 0 / 0.1 / 1 GAE band applies; any later cost statement from these counts must apply it.

## 3. Counting facts the cell is priced against (arithmetic, pre-data)

D(n,m) = C(n+m-1,m)/n; N_m = C(n+m-1,m) patterns; n^(m-1)/D -> m!.
Cells: D(7,2..5) = 4, 12, 30, 66; D(11,2..5) = 6, 26, 91, 273; D(13,2..5) = 7, 35, 140, 476; D(17,2..5) = 9, 57, 285, 1197.
n = 17, m = 2: 9 classes against the baseline 17 (ratio 1.89); n = 131, m = 5: D = 2,646,567 against 131^4 = 294,499,921 (ratio 111.3).
The n = 131 values are closed-form arithmetic. NO yield, class concentration or solver measurement exists at n = 131 or
at any field degree above 17 in this cell.
Factor-n flag, made exact (test `vandermonde_and_factor_n_relation`): for fixed R a class holds n patterns each with yield
about B^m/(prod mu!)/|E|, the whole union holds C(nB,m)*2^m/|E|, and D reps hold only 1/n of that. A solver that visits one
representative per class sees 1/n of the union's decompositions of R; the rest are the same classes reached through the n
conjugate targets sigma^j R (A2). Any cost statement must say which of the two it prices. The run records both
(`rep_per_target` vs `mean_total`; expected ratio 1/n up to the repeated-shift shape effect).

Q2 addendum (conditional arithmetic, NOT recomputed from raw data). KN-FIND-47da4e prices the shift-encoded m = 2 system
against "the n gauge-fixed subspace systems it replaces" and reports conflict ratios 1.017, 1.017, 1.467, 1.150 at field
degree 13, 17, 19, 23. The exact count of distinct systems is D(n,2) = (n+1)/2. IF the denominators of those ratios sum n
systems, the ratio against D systems is multiplied by n/D = 2n/(n+1): about 1.89, 1.92, 2.79, 2.20. Whether that correction
applies depends on how the finding summed its per-system costs, which this cell cannot see (its raw per-system conflict
counts are not in this directory). That recomputation is an open item for the SAT cells, not a claim here.

## 4. Arms (identical code path: `Enumerator` + `PatternTable`; arms differ only in the block family)

All arms use blocks W_k of NONZERO x-values (x = 0 is excluded: it lies in every subspace, K(0) = all shifts).

| arm | blocks | role |
|---|---|---|
| frob | W_k = sigma^k V, V random l-dim subspace | arm under test (confirmatory) |
| frobperm | the SAME blocks as frob (same V, same draw, same seed label), block labels permuted by a random permutation of Z/n | NUISANCE-MATCHED control: identical union, liftable and trace profile, overlaps K(x) and shape structure; only the diagonal-gauge labelling is destroyed. Replicable class excess shared with frobperm is not Frobenius-specific. |
| rsub | W_k independent random l-dim subspaces | negative control for the null model: linear, no Frobenius tie, but its OWN independent trace profiles (so it does NOT reproduce the common-profile parity effect; that is why frobperm exists) |
| rset | W_k independent random subsets of size 2^l - 1 | negative control for the null model: no structure at all, same sizes |
| skew | independent random subspaces, dimension l+1 for k < n/3, else l | positive control for the split-half statistic only: a planted replicable SIZE skew that the naive (equal-size) null must flag. It is a size effect, not a Frobenius-labelling effect; it shows the statistic sees replicable class-shape deviations of that size, not that it sees a subtle gauge-specific lever. |
| geom | same as frob, V = span(1,g,...,g^{l-1}), g random | exploratory (structured V); excluded from the verdict |
| plain | one block, random l-dim subspace | compact baseline |
| plainbig | one block, random subspace of dimension l + ceil(log2 n) | LARGER compact baseline. Its x-set is 1.14x (n=7), 1.45x (n=11), 1.23x (n=13), 1.88x (n=17) the union's, so it is NOT size-matched and no text calls it that. Yields are compared per C(#distinct liftable x, m) (section 5). |
| A/A | frob, draws 0..NV/2-1 versus NV/2..NV-1 (same spec, disjoint seeds) | noise floor, printed per cell; the split-half statistic is itself an A/A comparison |

Exact controls: `audit` = 1 on frob and geom (gauge identity A2) or the cell is INSTRUMENT_FAIL. `audit` on rsub, rset and
frobperm is REPORTED (expected 0 when blocks are big enough to show a violation, but on tiny blocks it can pass by chance,
so it is not enforced). The detector's ability to see a violation is a unit test (`gauge_identity_exact_for_frob_and_broken_for_random_blocks`,
`frobperm_has_the_same_union_and_totals_as_frob_but_not_the_gauge`).

## 5. Instrument and metric

Decompositions: unordered sets {Q_1..Q_m} of affine points of E_a(F_{2^n}) with PAIRWISE DISTINCT x (this removes Q+(-Q)
and doublings; degenerate m-2 relations are not counted), x(Q_i) in the union, sum = R. Meet-in-the-middle over strictly
x-increasing tuples, exact; a brute-force nested-loop reference agrees in tests for m = 2, 3, 4 and 5 (the m = 5 path with
a = 3, b = 2 included).
Pattern assignment: each point carries the set K(x) = {k : x in W_k}; a decomposition is compatible with every multiset
of shifts obtained by choosing one k in K(x_i) per point. `cover` counts a decomposition once for each compatible pattern
(what a pattern-k solver would see); `frac` gives each compatible pattern 1/(#compatible), so frac is a partition of the
total (the headline yield Y_c). Ambiguity (|K| > 1) is reported (`unamb`) and never hidden.
Curves: E_a: y^2 + xy = x^3 + a x^2 + 1, a in {0,1}; field degree n in {7,11,13,17}; m = 2..5 (n > m so the action is free).
Targets: R = h*P (h = cofactor 4 / 2), DISTINCT, at most one per orbit {+-R, +-sigma R, ...} of the group generated by Frobenius and
negation (rev 3). sigma-conjugate targets have identical class vectors for the Frobenius arms by A2, and -R has the same x as R, hence
identical class vectors for EVERY arm; both add no information. Revision 2 removed only the Frobenius duplicates and left the +-R pairs
(measured: at n = 7 every target had its negative's orbit also chosen, at n = 11 a = 0 all 48 orbits were in +- pairs), which overstated
the independent targets by up to 2x. The same targets are used for every arm and draw (paired). NT = 48 requested; fewer exist at n = 7
(2 for a = 0, 5 for a = 1) and n = 11 (24 for a = 0, 45 for a = 1; test `targets_are_distinct_one_per_frobenius_orbit`), and each row
records the effective nt; plan.rs uses (|hE| - 1)/(2n).
All of the cell's controls (frobperm, rsub, rset) see the same targets, so target-specific structure shared by the
split halves cancels against them; the split-half rho of frob alone also contains target-specific structure and is never read alone.
Draws: NV = 16 independent base draws per arm per cell; arm order shuffled per draw (interleaving); every seed is derived
from (a,n,m,l,arm,draw) and stored in every row; frobperm uses frob's label so it draws the same V.

Null (frozen, "parity-aware"; rev 3: a = 1 uses the mod-2 form below, a = 0 the mod-4 form stated after it). A point is in 2E iff Tr(x) = Tr(a) (A5). Every target lies in hE inside 2E, so a decomposition
needs an even number of odd points (Tr(x) != Tr(a)). With A_s even and B_s odd liftable x in block s (recorded per row as
`xs`, `xodd`) and multiplicity mu_s of block s in the pattern,
weight(pattern) = prod_s P_s(1) + prod_s P_s(-1), P_s(t) = sum_j C(B_s,j) C(A_s,mu_s-j) t^j, and the expected pooled class
total is nt * 2^m/|E| * sum_{patterns in class} weight (global constants are absorbed below). Why this is needed: the n Frobenius
blocks are isometric copies with ONE common trace profile, while independently drawn blocks (rsub, rset, skew) have independent
profiles. Without the parity term the expected count of a pattern is wrong by a factor 1 + (-1)^m r^m (r = (A-B)/X for distinct
shifts, a replicable, shape-dependent excess for even m) that only the Frobenius arms carry; a first draft of this protocol
used that size-only null and its negative controls could not have seen the artefact. a = 0 (rev 3). The 2-part of E is Z/4 and the targets lie in 4E, so the mod-2 constraint is only part of the story: with per block s
the counts X_s (liftable), O_s (odd, Tr(x) = 1), T_s (type 2: Tr(x) = 0 and (#E/4) Q != O) and Z_s = X_s - O_s - T_s, a tuple of distinct
x contributes (number of valid sign choices)/2^m = 0 (#odd odd), 1/2 (#odd > 0 even) or [#type-2 even] (#odd = 0) to the expected count (A5b).
`combi::pattern_weight_mod4` returns twice the sum of this over x-tuples in closed form, (E_even - N0) + 2 N0_Teven, with E_even and N0_Teven
given by the +-1 generating-function products in its doc comment; a unit test checks it against explicit tuple enumeration and against
twice the size weight when O = T = 0. The type counts `xtwo` are recorded per block in every row (24th column set; `xodd` as before).
Without this term the mod-2 null averages the #odd = 0 case; the Frobenius blocks share ONE (Z, T, O) profile (sigma acts as +-1 on Z/4)
and rsub/rset do not, so on a = 0 curves the mod-2 null left a shared nuisance with synthetic median z(frob) about 4 at (13,4,l=4..5) (reviewer's
power arithmetic). The Z/4 structure is the complete 2-primary structure, so the remaining nuisances are overlaps K(x) > 1 and small-block effects.
The size-only null is still computed
and printed (`z_frob_sizeonly_null` in `verdicts.tsv`) as a diagnostic of how large that artefact was.
Each row's null is rescaled to that row's observed total (absorbing the global bias and the factor 2) and leaves only shape.
Naive null: all X_k replaced by their mean, no parity (used only for the skew positive control).
Statistics (pooled classes; analysis in `src/stats.rs`, frozen before data):
* phi = sum_c (Y_c - mu_c)^2 / mu_c / (D - 1), over classes with mu_c > 0 (descriptive; not Poisson-calibrated, read against controls).
* rho = mean over 40 random balanced splits of the V draws of Pearson(r_A, r_B), r_c = (Y_c - mu_c)/sqrt(mu_c) pooled
  within each half. Replicable concentration shows as rho > 0 because random V-to-V noise is independent across halves.
  (rev 3) z = (rho - m0)/SE, where m0 and SE are the mean and sd of the same split-averaged rho over 400 SIMULATED null datasets: class counts
  drawn Poisson around each row's own null (rescaled to the row's observed total), pushed through the same 40 splits and the same code
  (`stats::split_stats`). Revision 2 took SE from a class-label permutation null; because the 40 splits reuse the same 16 draws, the actual
  sd of rho is larger (z had sd 1.37 to 1.60 on pure Poisson nulls at D = 30..476, reviewer's scratch measurement), so the nominal p-values
  and the resolution of revision 2 were wrong in the anti-conservative direction. Calibration is now a test:
  `z_is_calibrated_under_the_null_at_two_values_of_d` (400 null datasets each at D = 26 and D = 140: sd(z) = 0.97 and 1.02, mean z 0.01 and -0.18,
  0 of 400 at z >= 3 in both). The normal tail at z = 4 is NOT verified empirically (it would need about 1e5 datasets); the SE itself has a
  Monte-Carlo error of about 3.5 percent, which fattens the tail by roughly a factor 1.3 at z = 4.
* contrast z (rev 3) = ((rho - m0)_frob - (rho - m0)_frobperm) / sqrt(SE_frob^2 + SE_frobperm^2): the DIRECT frob-versus-frobperm test.
  The arms share V, so under a shared nuisance their rhos covary positively and this z is conservative (under independent nulls it is exact);
  (amended 2026-10-27, see the amendment section: the earlier claim that its power is lower than z(frob) by only about sqrt(2) is WITHDRAWN. rho is bounded by 1; when a shared class-profile
  nuisance already makes rho(frobperm) large, rho(frob) saturates and this contrast has no power. Synthetic demonstration by the reviewer at 3.3e5 per class per half: shared nuisance 1 percent with
  a frob-only effect of 3 percent gives z(frob) 15.8, z(frobperm) 15.4, contrast 0.3.)
* transfer = share of half-B yield in the top 10 percent classes selected on half A, divided by their null share (1 = none).
* phi ratio (only in the low-D branch) = phi(frob)/max(phi(rsub), phi(rset), phi(frobperm)), 400 bootstrap resamples of the V draws within each arm, 95 percent interval.
  The max over three noisy controls biases the ratio DOWN, which makes CONCENTRATED_VS_CONTROLS harder, not easier.
* rel. excess = sqrt(max(rb(frob) - rb(frobperm), 0)), rb = mean_c [((Y_c - mu_c)^2 - Y_c)/mu_c^2] (Poisson-de-noised, scale-free);
  descriptive, used only for the l ladder (section 7).
* yield per subset = decompositions per target / C(#distinct liftable x in the base, m), for the Q3 comparison; the raw
  frob/plainbig ratio is printed with its expected size (|U|/|plainbig|)^m, never as "size-matched".

## 6. Grid, sizes, power and resolution

Cells (a, n, m, l), 52 in all (`g2dnm cells`, `g2dnm plan`):
* base grid, 32 cells (n = 7 and 11 now have 2..5 and 24..45 effective targets, rev 3): a in {0,1}, n in {7,11,13,17}, m in 2..5, l = max(2, round(n/m)) raised while the pooled expected class
  count per half is < 5 and the left table stays under the cap (`choose_l`); this confounds m with l (m = 2 runs at l = 4..9,
  m = 5 at l = 2..3);
* fixed-l column, added in revision 2 so that an m-trend is not an l-trend: l = 3 for every m = 2..5 at n = 13 and 17
  (16 cells, 10 of them new: the default l is already 3 for (13,4), (13,5), (17,5));
* l ladder at m = 4: l = 2, 3, 4, 5 at n = 13 and 17 (separates a toy-size artefact from a persistent effect, section 7); 10 further new cells.
  Together the additions bring the grid from 32 to 52 cells. Even at fixed l the m-trend is read only where D allows: at l = 3 only (13,4), (13,5),
  (17,4), (17,5) are rho-resolved, so the fixed-l column mostly gives phi-based, descriptive m-dependence.

Planning table (`plan.tsv` in each run directory is exact; rev 3: effective targets count orbits of <Frobenius, negation>, so the n = 7
and n = 11 rows changed; a = 0 / a = 1 are given where they differ by more than 10 percent):

| (a,n,m,l) | D | E[dec/target] | mu/class/half | left tuples | est s/cell |
|---|---|---|---|---|---|
| n=7: (m,l) = (2,4), (3,2), (4,2), (5,2) | 4 / 12 / 30 / 66 | 23 / 8.3 / 29 / 70 (a = 0) | 92 / 11 / 15 / 17 (a = 0, nt_eff 2); 188 / 23 / 32 / 34 (a = 1, nt_eff 5) | 74..960 | < 0.1 |
| 11: m=2..5 at l=6,4,3,2 | 6, 26, 91, 273 | 84, 311, 558, 66 | a = 0 (nt 24): 2677, 2294, 1178, 46; a = 1 (nt 45): 5359, 4592, 2357, 93 | 6e2..1.3e4 | 0.3..6.2 (a = 0), 0.5..11.7 (a = 1) |
| 13: m=2..5 at l=7,4,3,3 (default) | 7, 35, 140, 476 | 141, 143, 297, 4880 | 7754, 1565, 816, 3937 | 1.5e3..1.1e5 | 1.3, 16.4, 3.7, 117 |
| 17: m=2..5 at l=9,6,4,3 (default) | 9, 57, 285, 1197 | 271, 1533, 1263, 1223 | 11551, 10326, 1701, 392 | 8e3..5.7e5 | 7.7, 633, 37, 494 |
| 13 / 17, m = 2..5 at l = 3 (fixed-l column) | 7..1197 | 0.05..4880 | 2.2..3937 | 1e2..2.6e5 | 0.1..494 |
| (13|17), m = 4, l = 2,3,4,5 (ladder) | 140 / 285 | 1.5..1.2e5 | 2..3.3e5 | 6.8e2..1.4e5 | 0.7..156 |

Cells with mu/class/half under 5 (17,2,l=3: 2.2; 17,4,l=2: 2.1) are Poisson-limited and flagged by the plan; they are kept
because they are the low end of the ladder and fixed-l column.

Resolution, stated honestly.
* The split-half rho test has null sd 0.73/sqrt(D - 1) (measured on the cell's own analysis: se * sqrt(D - 1) = 0.77, 0.78, 0.71, 0.72, 0.71, 0.75, 0.70 at
  D = 12, 26, 57, 91, 140, 273, 285; the revision-2 figure 1/sqrt(D - 1) was neither this statistic's sd nor its permutation SE) and a minimum detectable
  correlation (z = 4, about 90 percent power, sd under the alternative taken equal to the null sd) rho_min = (4 + 1.28) * 0.73/sqrt(D - 1) = 3.85/sqrt(D - 1):
  0.41 at D = 91, 0.33 at D = 140, 0.23 at D = 285, 0.52 at D = 57 (just outside), 0.48 at D = 66. rho_min <= 0.5 needs D >= 61 (`MIN_D_FOR_RHO`).
  Revision 2's D >= 113 cut described a different statistic and was too strict. Rho-resolved cells: 26 of the 52 (13 per curve; base grid 14 of 32), of
  which 8 are tiny-block cells (mean liftable x per block < 2.5: (7,5,2), (11,5,2), (13,4,2), (17,4,2), both curves) leaving 18 resolved non-tiny cells.
  The other 26 cells (all m = 2, all D <= 57) are NOT rho-resolved; `summary.md` prints "rho resolved: NO" next to each.
* "Rho-resolved" is a statement about D only. Tiny blocks undercut it: at l = 2 the mean liftable count per block is about 1.5,
  a draw with X = 1 leaves most classes with zero expectation, and about 1/8 of frob draws have X = 0 (and for a = 1 with odd m about
  1/4 of draws have V inside ker Tr, which gives exactly zero decompositions, A5). Those rows are EMPTY, are counted and printed
  per arm ("empty rows"), and are never dropped; rsub rows are never empty. Cells with mean X per block < 2.5 are flagged "tiny
  blocks" in the table and excluded from the campaign tallies of section 7.
* The phi-ratio branch cannot certify uniformity at low D. For a chi-square on D - 1 degrees of freedom the ratio of two
  independent phi has relative sd about 2/sqrt(D - 1); a 95 percent upper bound below 1.25 needs D - 1 of about 250 or more, i.e.
  D >= 250. For D <= 57 (26 cells) the interval is wider than any informative threshold (no test reproduces a number for it). Hence NO "uniform" verdict exists in the low-D branch; only a positive detection
  (lower bound > 1.5) or UNRESOLVED. A low-D null result is UNRESOLVED and says nothing.
* n = 7 has 2 (a = 0) or 5 (a = 1) independent targets; the n = 7 cells are plumbing and identity checks, not evidence (m = 5 at D = 66 is formally rho-resolved
  and tiny-block, hence excluded from tallies).
* UNIFORM is not an effect-size statement by itself: rho + 1.96 SE < 0.25 means a replicable relative class deviation below roughly 0.57/sqrt(mu) per
  class (mu = pooled expected count per class): about 6 percent at (11,5,2) (mu 93), 2.9 percent at (17,5,3), 0.1 percent at (13,4,5). `summary.md` prints rho
  and SE per cell; read UNIFORM at that resolution only.
* The positive control (skew) is a size skew; in synthetic power checks it is detected at z about 11 to 35 in every rho-resolved cell except (17,4,2) (min z 2.8,
  Poisson-limited, mu/class/half 2.1), which therefore has about a one-in-three chance of INSTRUMENT_FAIL. The frob-minus-frobperm contrast is validated only on
  synthetic rows (tests `contrast_decides_frobenius_specificity_not_the_power_dependent_zperm`), never on a planted gauge-specific effect of real curve data.
* Regime: l = 2..9 with m*l ~ n, blocks of 1 to 255 liftable x. The production regime (l about 10, n = 131) is not reached.

## 7. Decision rule (frozen, rev 3; applied per cell to arm frob by `g2dnm analyze`, mechanically, written to `verdicts.tsv`)

Order of checks, first hit wins:
1. NO_DATA if frob, frobperm, rsub or rset rows are missing. INCOMPLETE (cell reported NOT RUN) if any of those four arms has fewer than NV = 16 rows
   (a killed process); a truncated last line is skipped and counted, never aborts the analysis.
2. INSTRUMENT_FAIL if the gauge audit is 0 on frob or geom, or z(rsub) >= 4 or z(rset) >= 4 (a negative control flags: the null
   model is wrong for this cell), or (D >= 61) the positive control skew is not flagged under the naive null (z_naive < 4). Tests pin audit = 0 and a missed skew.
3. D < 61: CONCENTRATED_VS_CONTROLS if the phi-ratio lower bound (against max of rsub, rset, frobperm) is > 1.5; else UNRESOLVED.
   No positive control or null calibration exists in this branch and no UNIFORM verdict is possible.
4. D >= 61, with zc = the direct contrast z of section 5:
   * z(frob) >= 4 and zc >= 4: the excess is replicable and larger than frobperm's: CONCENTRATED if transfer >= 2, else CONCENTRATED_WEAK;
   * z(frob) >= 4, zc < 4 and z(frobperm) >= 4: NUISANCE_REPLICABLE (a replicable excess that the label permutation does not significantly reduce:
     no Frobenius-specific claim);
   * z(frob) >= 4, zc < 4 and z(frobperm) < 4: REPLICABLE_NOT_SEPARATED (frob is replicable but the instrument cannot say frobperm is lower: specificity
     NOT resolved; this is NOT counted as a lever and NOT as uniform);
   * else UNIFORM if rho + 1.96 SE < 0.25 (frob), else UNRESOLVED.
   Revision 2 decided Frobenius-specificity from the power-dependent non-event z(frobperm) < 4 and never tested frob against frobperm; a shared nuisance with
   z(frobperm) just under 4 was then CONCENTRATED(_WEAK) and a real frob-specific excess with both z >= 4 was masked as NUISANCE. The tests pin one fixture of each
   (planted frob-over-frobperm excess 0.10 vs 0.04: z 20.9 / 10.8 / contrast 7.0 -> CONCENTRATED_WEAK; 0.04 vs 0.01: 9.1 / 3.75 / 3.4 -> REPLICABLE_NOT_SEPARATED).
z = 4 is the flag level; nominal one-sided p at z = 4 is 3.2e-5 under a normal tail (not verified, see section 5; allow a factor of about 1.3 and treat as order of 4e-5).
Tests read: 104 negative-control tests (rsub, rset in 52 cells) -> chance of a spurious INSTRUMENT_FAIL somewhere about 4e-3 under a correct null; confirmatory z(frob)
in the 18 resolved non-tiny cells -> family about 7e-4, and a CONCENTRATED needs the contrast too; every z the rule reads in all cells (frob, frobperm, contrast, skew in 26 resolved
cells, rsub and rset in 52) is about 210 tests, about 7e-3. Revision 2 stated the same tails for a statistic whose empirical false-flag rate was 15 to 60 times nominal.
Nothing is reported as positive on a single cell at that level; one CONCENTRATED cell would be a candidate to replicate with fresh seeds.
Campaign-level reading, fixed now (tiny-block cells excluded from all tallies):
* LEVER CANDIDATE: CONCENTRATED in at least one cell on each curve at the same (n,m), with transfer >= 2, replicated under a second
  seed set, AND ladder PERSISTENT (coded in `report.rs`, printed in the "l ladder" section): along m = 4 at n = 13 and 17, the l = 3 cell is
  CONCENTRATED or CONCENTRATED_WEAK with rel. excess > 0, the l = 5 cell is also CONCENTRATED or CONCENTRATED_WEAK, and rel. excess(l = 5) >= 0.5 * rel. excess(l = 3).
  (rev 3: the l = 2 cell is a tiny-block cell, printed but not used; the clip at 0 cannot make the test vacuous because l = 5 must itself be flagged.) A
  Frobenius-twisted decomposition (Q and sigma^d Q both landing in neighbouring blocks automatically) gives a class-specific excess of relative size about
  2^-l (12 percent at l = 3, 3 percent at l = 5, ratio 1/4); an excess that falls like that is read as the toy-size artefact, not a lever (status FALLING_OR_ABSENT).
  (amended 2026-10-27) FALLING_OR_ABSENT is assigned only when the l = 5 verdict is CONCENTRATED, CONCENTRATED_WEAK, UNIFORM or UNRESOLVED. If the l = 5 verdict is INSTRUMENT_FAIL,
  NUISANCE_REPLICABLE, REPLICABLE_NOT_SEPARATED, INCOMPLETE, NO_DATA or CONCENTRATED_VS_CONTROLS the ladder is NOT_EVALUABLE: that is a failure of the instrument or of specificity
  resolution, not decay of the effect, and is never read as "toy-size artefact".
  rel. excess has no confidence interval; at mu of 2 to 21 per class (l = 2) it is noisy, which is why only l = 3 and l = 5 are used.
  Only then does the "visit only a few classes" idea survive; the next step would be the SAT cell restricted to those classes.
* NO LEVER FROM YIELD: no CONCENTRATED or CONCENTRATED_WEAK cell and UNIFORM in every rho-resolved non-tiny cell. Cost-neutrality
  is then supported on the yield side only, at these toy sizes; it is not a cost result.
* NUISANCE_REPLICABLE or REPLICABLE_NOT_SEPARATED anywhere: report it; the null is incomplete there or the contrast lacks power. (rev 3: for a = 0 the Z/4 structure is
  modelled, so the likely sources are overlaps K(x) > 1, small-block hypergeometric effects and plain lack of contrast power; the revision-2 text blaming K(x) > 1 for a = 0 was wrong
  for the mod-4 cause.)
* Pre-registered expectation (rev 3): if a Frobenius-twisted excess of relative size 2^-l exists it is replicable and frob-specific at counts of 1e3 to 3e5 per class, so the
  likely labels on l <= 3 cells are CONCENTRATED(_WEAK); the campaign reading then rests on the ladder status, not on a single cell label.
* (amended 2026-10-27) Saturation rule: when the frob yield is at least 10000 per class per half (integer floor of total frob yield / (2 D) >= 10000; `report::SATURATION_COUNT`),
  NUISANCE_REPLICABLE and REPLICABLE_NOT_SEPARATED mean "Frobenius-specificity NOT RESOLVABLE", never a negative: the cell note says so and `verdicts.tsv` column
  `specificity_resolvable` is false. The l = 5 cells at (13,4,5) carry about 3.3e5 per class per half and fall in this regime. The contrast resolves specificity only while the shared
  nuisance is below about 1.3 times the Poisson noise sd of a pooled-half class (about 0.2 percent at l = 5). No paired-difference statistic was implemented (the reviewer's
  alternative); this is the pre-registered option B.
* Mixed or UNRESOLVED-dominated: report as such. The 26 low-D cells cannot establish uniformity.
geom, skew and all other arms are descriptive; no verdict is drawn from geom. Unit tests pin each verdict on planted fixtures
(`analysis_pipeline_pins_verdicts_on_planted_fixtures`, `contrast_decides_...`, `instrument_fail_on_gauge_audit_zero_missed_skew_and_incomplete_cells`,
`ladder_persistence_is_coded`).

## 8. Runtime estimate (code reasoning, not measurement)

Cost of one target pass = (#left tuples) * (one point addition with table inversion + one hash probe), assumed 150 ns
(uncertain by a factor of about 3). #left = 2^a*C(n_x, a) with a = ceil(m/2), n_x = expected DISTINCT liftable x of the union
(overlap-corrected) or of plainbig (2^(l+ceil(log2 n)) - 1 nonzero x, half liftable). Per arm per cell = #left * NV * nt passes.
Plain arms are NOT cheaper: plainbig has the larger table (2.1e6 tuples at (17,3,l=6), 2.7e6 at (17,5,l=3) against
5.7e5 and 2.6e5 for the union). Per cell = 6 union arms (frob, geom, frobperm, rsub, rset, skew) + plain + plainbig.
`g2dnm plan` sums to about 3183 s single core over the 52 cells (53 minutes), plus the analysis (400 simulated null datasets per split-half statistic: a few minutes single core, estimated from the D = 476 fixtures, not measured at D = 1197), longest cell (17,3,l=6) about 633 s, then
(17,5,l=3) about 494 s. With 3 parallel jobs: lower bound max(3183/3, 633) = about 1061 s, so about 18 minutes of wall time if
the 150 ns figure is right; with the factor-3 uncertainty on 150 ns the wall time is 6 to 54 minutes, and contention on the
shared CPUs can add more. Allow one hour; do not read a longer run as a hang before 90 minutes.
Memory: Ent is 28 bytes; the left-table cap is 1e7 tuples (about 280 MB per process, 3 jobs under 1 GB). The old cap of 4e6 would have
tripped at (17,5,l=3) plainbig with about 1 percent probability per draw (about 30 percent over the 32 plainbig draws of the two curves); `plan`
prints the mean + 6 sd bound of the plainbig table and a test asserts it stays under the cap for every cell (largest 7.0e6).
If the cap still trips, the cell aborts, the driver continues with the other cells, `run_meta.txt` records failed_jobs > 0, and the
cell is reported NOT RUN (never re-run with altered parameters).
Disk: rows files hold three class vectors per row at 5 decimals plus the extra `xtwo` column; the largest, (17,5,l=3), has 1197 classes * 96 union rows,
about 3 MB per curve; (13,5,3) about 1.1 MB. Files over 1 MiB follow AGENTS.md section 13 (artifact store, `.artifacts.json`), not `git add`.
The whole run is under 25 MB.

## 9. What does not count

* Any statement about SAT conflicts, propagations or time per relation. This cell enumerates; it prices nothing the solver does.
* Reporting the representative-only yield as "the union yield" or the union total as "per system" (the factor n of section 3).
* Calling plainbig size-matched, or quoting frob/plainbig as a size-matched ratio.
* Relabelling: choosing a different class convention, rep convention, null, or control after seeing data; changing NV, NT, l,
  thresholds, the statistic list or the verdict rule after the first run (a change makes it a new protocol with a new run directory).
* Reading UNRESOLVED as uniform; reading a D < 61 cell as a rho-level negative; reading REPLICABLE_NOT_SEPARATED as either a lever or a nuisance; reading any m-trend from the base grid alone (m is confounded with l there; use the fixed-l column).
* Any cell that is INSTRUMENT_FAIL; any run whose `run_meta.txt` frozen_sha differs from the sha the PR names.
* any statement that treats +-R targets as independent (rev 3: they are merged), or a count of relations as validated (no certificate, section 2);
* geom results as evidence about random V; or any extrapolation to n = 131 (counts at n <= 17 are toy-scale).
* This is not an index-calculus speed-up and does not touch the leaderboard or `benchmarks/README.md` (no performance claim, no matched-rho row, no
  validation-gate row). The research-visuals report/graph/PDF for the run are still required (RUN_CARD, post-run).

## 10. Contrarian notes: what the design can and cannot resolve

* Prior: for a random V the blocks sigma^kV look like independent subspaces to each other once the common trace profile is modelled;
  Proposition T gives only the diagonal gauge, which is exactly the quotient already counted by D. The expected outcome is UNIFORM
  (parity-aware null holds), P(CONCENTRATED on a rho-resolved non-tiny cell) of order 10 percent by the first-draft reasoning, but see the pre-registered expectation in section 7: if the twisted excess exists the labels at l <= 3 are likely CONCENTRATED(_WEAK) and the ladder carries the reading, not the 5 percent of the first draft: the Frobenius-twisted
  excess of section 7 is a replicable, class-specific effect of relative size about 2^-l that the rule can see at l = 2..3 and that is a toy artefact. The informative
  surprise would be a Frobenius-specific excess that frobperm does not share AND that survives the ladder.
* The first draft's controls were confounded (rsub and rset carry independent trace profiles, frob a common one). Revision 2 adds the
  parity term and frobperm. Residual unmodelled nuisance shared with frobperm (overlaps K(x) > 1, small-block hypergeometric effects) now lands in NUISANCE_REPLICABLE, or REPLICABLE_NOT_SEPARATED when the contrast lacks power, not CONCENTRATED. Revision 3 added the mod-4 term for a = 0 and the direct contrast (section 13).
* The positive control is a size skew. It certifies the split-half statistic against replicable shape deviation of that size; it does not
  certify sensitivity to a small gauge-specific effect. The minimum detectable effect is set by the synthetic planted-effect tests (20 percent on 10 percent of classes at D = 476) and by section 6, not by skew.
* Uniform yield does not mean cost-neutral for a solver: the solver cost per class (sharing deficit between patterns, loss of the geometric linear oracle for mixed shifts) is outside this instrument. Concentration would be necessary, not sufficient, for a lever.
* The distinct-x restriction removes plus-or-minus pairs; the x-only S_m summation polynomial roots also admit repeated x, so counts differ from a SAT relation count by that degenerate part.
* The two curves are near-replicates (same field, trace parity differs), which is why the lever criterion asks for both.
* The cell cannot resolve anything at the low-D cells that the earlier m = 2 pricing needs; for m = 2, 3 its contribution is the exact arithmetic of section 3 and the dispersion detector's positive side.

## 11. Variable ledger

instrument (one `Enumerator`, one `PatternTable`, one analysis binary, sha256 recorded); host (`run_meta.txt`, `host.txt`: nproc, load, kernel, rustc, cargo, jobs);
layout/order (arm order shuffled per draw, seeded); seeds (derived, stored in every row); targets (shared, paired, distinct Frobenius orbits);
analyst (analysis code frozen, arm labels not masked in the tool but the verdict is computed mechanically from section 7); spot preemption
(not applicable: local; ROUTE: AGENTS.md 9a makes isolab-eks the default for authorized x86 CPU experiments. This card runs `run_cells.sh` on the shared local container. Decomposition counts and class vectors are exact and independent of isolation, host load and timing, so the route cannot change a result, only wall time; the experiment director must either keep the local route and record that reason in the PR or submit through isolab-eks before the freeze); curve (a), field polynomial (fixed per n in `field.rs`), cofactor subgroup targets; trace profile (modelled, controlled by frobperm).

## 12. Disclosures (AGENTS.md 8a, 8b)

(rev 3: `g2dnm analyze` now writes the field-degree / decomposition-size legend, the no-m=83 statement and the Frobenius-module note into `summary.md`.)

* No m = 83 evidence exists in this cell, and none is implied. Per 8a the result at field degrees 7..17 is exploratory only and does not
  discharge the 83 gate; per 8b ECC2K-130 is over F_{2^131} and any statement at 131 is an extrapolation.
* Frobenius-module structure of the toy field degrees (ord_n(2)): n = 7: 3, so x^7 - 1 = (x+1)(x^3+x+1)(x^3+x^2+1) and the nontrivial
  cyclotomic block SPLITS (as at 31); n = 11: 10, irreducible block (as at 53, 83, 131); n = 13: 12, irreducible; n = 17: 8, so the block splits into two degree-8 factors
  (like the 31 case: ord_31(2) = 5). Frobenius-invariant linear subspaces exist for n = 7 and 17 that do not exist at 83 or 131; V here is random, not invariant, but
  overlaps sigma^aV and sigma^bV and the ladder results at n = 7, 17 must be read with this difference in mind and are reported per n.
* No proper intermediate subfields over GF(2) beyond GF(2) itself (all n prime). a = 0 is the ECC2K-130 model y^2+xy=x^3+1 at a toy
  field degree; a = 1 is its quadratic twist over F_2 (a different instance family, never silently the same). Field polynomials are fixed in `field.rs`.

## 13. Revision log

Revision 1 (prepared 2026-10-27). Revision 2 after independent review: added the parity-aware null (A5) and the frobperm
nuisance-matched control; removed the unsupported low-D UNIFORM verdict; removed the sentences promising enforcement of audit = 0 on rsub/rset and of
"aware null must not flag skew" (both are now reported, not enforced); added the fixed-l column and the m = 4 l ladder; raised the left-table cap to 1e7 and bounded it by test; relabelled plainbig and added
size-normalised yields; targets now distinct Frobenius orbits; audit 12 targets; added m = 5 brute-force and independent group-law tests, replaced a tautological
assertion, pinned verdicts on planted fixtures; recomputed runtime and disk; added the disclosures and the reproducibility binding. Nothing was run.

Revision 3 (2026-10-27, second independent review; nothing has been run, so the freeze is still open). Changes, each with its reason:
1. a = 0 curves modelled mod 4 (A5b, `pattern_weight_mod4`, per-block `xtwo` recorded in a 24th column; reason: Z/4 2-part, mod-2 null misses a term that only Frobenius blocks and frobperm share).
2. z calibrated by simulating the null through the identical split analysis (SE and null mean from 400 Poisson null datasets); permutation SE dropped (reason: z sd was 1.4 to 1.6, so p, the
   INSTRUMENT_FAIL false-alarm rate and the D cut were anti-conservative). `rho_min` now 3.85/sqrt(D - 1) and the rho-resolved cut D >= 61 (was 113). The cell partition changed from 22 resolved / 30 not to 26 / 26.
3. Frobenius-specificity from the direct contrast z (frob minus frobperm) instead of the non-event z(frobperm) < 4; new verdict REPLICABLE_NOT_SEPARATED; new column z_contrast in summary.md / verdicts.tsv.
4. Targets merged under negation as well as Frobenius; nt_eff in plan.rs follows (n = 7: 2 / 5; n = 11: 24 / 45).
5. l-ladder persistence criterion coded and redefined on l = 3 versus l = 5 (the l = 2 cell is tiny and excluded); a zero or undefined value cannot satisfy it.
6. INCOMPLETE verdict for arms short of NV rows; truncated lines skipped and reported; summary.md carries the section 12 disclosures and the "no 5a band" statement.
7. Not changed (stated, not fixed): no decomposition certificate is emitted (section 2); the route question is left to the experiment director (section 11); rows files were not hashed in revision 2; `run_cells.sh` now writes `rows.sha256`).

## 14. Amendment 2026-10-27 (round-2 review; nothing has been run, the freeze is still open)

1. Ladder (blocking defect 1): `report::ladder_status` returns NOT_EVALUABLE unless the l = 5 verdict is CONCENTRATED, CONCENTRATED_WEAK, UNIFORM or UNRESOLVED. Before this, any other l = 5
   verdict (INSTRUMENT_FAIL, NUISANCE_REPLICABLE, REPLICABLE_NOT_SEPARATED, INCOMPLETE, NO_DATA) printed FALLING_OR_ABSENT, e.g. l = 3 CONCENTRATED at rel. excess 0.3938 and l = 5 INSTRUMENT_FAIL at 0.3945.
   Test: `ladder_l5_instrument_failures_are_not_evaluable_not_falling`.
2. Contrast resolution (blocking defect 2, option B): the sqrt(2) power claim of section 5 is withdrawn; the saturation limit and the pre-registered reading are in section 7
   (`SATURATION_COUNT` = 10000 per class per half, integer comparison; verdicts.tsv gains column `specificity_resolvable`; the cell note says "SPECIFICITY NOT RESOLVABLE").
   Verdict labels are unchanged. Test: `specificity_saturation_threshold_is_an_integer_count_comparison`.
3. Not changed (caveats, not in scope): the positive-control power sentence of section 6, the UNIFORM effect-size figure and the lever-candidate combination rule are as in revision 3.
