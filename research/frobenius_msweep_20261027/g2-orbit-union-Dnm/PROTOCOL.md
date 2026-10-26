# G2 protocol: per-slot-shift orbit unions priced against the exact class count D(n,m)

Status: FROZEN DESIGN, prepared, not run. Nothing in this directory is a result.
Lead: LEAD-ICB-frobenius-search-symmetry (closed), reopening "orbit-union descended systems with per-slot shifts".
Cell G2 of the 2026-10-27 m-sweep campaign. Audit source: `/home/user/crypto-work/frobenius_audit_20261026.json`.

## 1. Question

For a random, non-Frobenius-stable subspace V of dimension l in F_{2^n} and U = union_k sigma^k V, take a target R and
all decompositions R = Q_1 + ... + Q_m with x(Q_i) in U. Group them by shift pattern (multiset of the shifts k_i with
x(Q_i) in sigma^{k_i} V) modulo the diagonal shift, i.e. into the D(n,m) = C(n+m-1,m)/n classes.

* Q1. Is the union's yield spread over the classes as the size-aware independence null says (uniform up to the
  trivial shape effect of repeated shifts), or is it concentrated in few classes, replicably across independent draws of V?
  Concentration would be an m-dependent lever (a solver need only visit those classes). Uniformity means the union is
  cost-neutral on the yield side.
* Q2. Priced exactly: what are D(n,m), the per-class and per-representative-pattern yields, and by what factor does the
  n^(m-1) baseline of KN-FIND-47da4e / H-BINSTD-c72284 (m = 2) overcount distinct systems? (arithmetic; section 3)
* Q3 (descriptive). Total union yield versus plain V and versus a size-matched plain subspace.

## 2. Claim classes (controlled-experiment skill)

| id | claim | class | how it is settled |
|---|---|---|---|
| A1 | Z/n acts freely on size-m shift multisets for prime n > m; classes = D(n,m); every class has n patterns | exact theorem | unit tests (direct enumeration, Burnside by direct count, n = 5..11, m = 2..5), formula values n = 7..131 |
| A2 | per-slot Proposition T: count(sigma R, pattern + 1) = count(R, pattern), exactly, any m | exact identity | unit test on all patterns and targets; re-checked on 3 targets per union arm at run time (`audit` column) |
| A3 | sum over patterns of prod_s C(X_s, mu_s) = C(sum X_s, m) (Vandermonde); one representative per class holds exactly 1/n of it when blocks are equal; so D*B^m/|E| is a factor n below the whole-union total (the red-team flag) | exact identity | unit test |
| A4 | pure-shift class count of R = sum over j of plain-V counts of sigma^j R | exact identity | unit test |
| B1 | union yield is uniform over classes (size-aware null) / concentrated | measured, comparative, population = random V at the cell's (a,n,m,l) | experiment, section 7 |
| B2 | union yield per (base size)^m equals the plain size-matched base | measured, descriptive | experiment, ratio reported, no decision |
| C  | solver cost (conflicts, propagations, time per relation) of any arm | NOT claimed | belongs to the SAT cells of the campaign; this cell counts decompositions only |

## 3. Counting facts the cell is priced against (arithmetic, pre-data)

D(n,m) = C(n+m-1,m)/n; N_m = C(n+m-1,m) patterns; n^(m-1)/D -> m!.
Cells: D(7,2..5) = 4, 12, 30, 66; D(11,2..5) = 6, 26, 91, 273; D(13,2..5) = 7, 35, 140, 476; D(17,2..5) = 9, 57, 285, 1197.
n = 17, m = 2: 9 classes against the baseline 17 (ratio 1.89); n = 131, m = 5: D = 2,646,567 against 131^4 = 294,499,921 (ratio 111.3).
Factor-n flag, made exact (test `vandermonde_and_factor_n_relation`): for fixed R a class holds n patterns each with yield
about B^m/|E| (distinct shifts), the whole union holds C(nB,m)*2^m/|E| ~ |U|^m/(m!|E|), and D reps hold only 1/n of that.
A solver that visits one representative per class sees 1/n of the union's decompositions of R; the rest are the same
classes reached through the n conjugate targets sigma^j R (A2). Any cost statement must say which of the two it prices.
The run records both (`rep_per_target` vs `mean_total`; expected ratio 1/n up to the repeated-shift shape effect).

## 4. Arms (identical code path: `Enumerator` + `PatternTable`; arms differ only in the block family)

All arms use blocks W_k of NONZERO x-values (x = 0 is excluded: it lies in every subspace, K(0) = all shifts).

| arm | blocks | role |
|---|---|---|
| frob | W_k = sigma^k V, V random l-dim subspace | arm under test (confirmatory) |
| geom | same, V = span(1,g,...,g^{l-1}), g random | exploratory (structured V; overlaps of sigma^aV, sigma^bV grow for small |a-b|); excluded from the verdict |
| rsub | W_k independent random l-dim subspaces | negative control: linear, no Frobenius tie |
| rset | W_k independent random subsets of size 2^l - 1 | negative control: no structure at all, same sizes and overlap statistics |
| skew | independent random subspaces, dimension l+1 for k < n/3, else l | positive control: planted replicable size skew; the NAIVE null (equal X_k) must flag it, the size-aware null must not |
| plain | one block, random l-dim subspace | compact baseline |
| plainbig | one block, random subspace of dimension l + ceil(log2 n) (>= |U|) | size-matched compact baseline |
| A/A | frob, draws 0..NV/2-1 versus NV/2..NV-1 (same spec, disjoint seeds) | noise floor; the split-half statistic is itself an A/A comparison |

Exact controls (must hold or the cell is INSTRUMENT_FAIL): `audit` = 1 on frob and geom (gauge identity A2), `audit` = 0 on rsub/rset
(the instrument sees a violation when the Frobenius tie is absent; checked on 3 targets at vd = 0).

## 5. Instrument and metric

Decompositions: unordered sets {Q_1..Q_m} of affine points of E_a(F_{2^n}) with PAIRWISE DISTINCT x (this removes Q+(-Q)
and doublings; degenerate m-2 relations are not counted), x(Q_i) in the union, sum = R. Meet-in-the-middle over strictly
x-increasing tuples, exact; a brute-force nested-loop reference agrees in tests.
Pattern assignment: each point carries the set K(x) = {k : x in W_k}; a decomposition is compatible with every multiset
of shifts obtained by choosing one k in K(x_i) per point. `cover` counts a decomposition once for each compatible pattern
(what a pattern-k solver would see); `frac` gives each compatible pattern 1/(#compatible), so frac is a partition of the
total (the headline yield Y_c). Ambiguity (|K| > 1) is reported (`unamb`) and never hidden.
Curves: E_a: y^2 + xy = x^3 + a x^2 + 1, a in {0,1}; n in {7,11,13,17}; m = 2..5 (n > m so the action is free);
targets R = h*P (h = cofactor 4 / 2), P uniform, NT = 48 per draw, the same targets for every arm and draw (paired).
Draws: NV = 16 independent base draws per arm per cell; arm order shuffled per draw (interleaving); every seed is derived
from (a,n,m,l,arm,draw) so a replay reproduces any single row.

Null (aware): for one target and class c, mu_c = (2^m/|E|) sum_{p in c} prod_s C(X_{k_s}, mult_s), X_k = number of
liftable x in W_k. Each row's null is rescaled to that row's observed total (this absorbs the model's global bias and
leaves only shape). Naive null: all X_k replaced by their mean (diagnostic, used only for the positive control).
Statistics (pooled classes; analysis in `src/stats.rs`, frozen before data):
* phi = sum_c (Y_c - mu_c)^2 / mu_c / (D - 1), over classes with mu_c > 0 (descriptive; not Poisson-calibrated, always
  read against the controls, never alone).
* rho = mean over 40 random balanced splits of the V draws of Pearson(r_A, r_B), r_c = (Y_c - mu_c)/sqrt(mu_c) pooled
  within each half. Replicable concentration shows as rho > 0 because random V-to-V noise is independent across halves.
  SE of rho from 400 class-label permutations; z = rho/SE.
* transfer = share of half-B yield in the top 10 percent classes selected on half A, divided by their null share (1 = none).
* phi ratio = phi(frob)/max(phi(rsub), phi(rset)), 400 bootstrap resamples of the V draws within each arm, 95 percent interval.

## 6. Sizes, power and resolution

l = max(2, round(n/m)) (the m*l ~ n regime of the audit), raised while the pooled expected class count per half is < 5 and
the left table stays <= 4e6 tuples (`choose_l`). Frozen grid (from `g2dnm plan`; printed by the test `plan_grid_is_feasible_and_printed`):

| a | n | m | l | D | n^(m-1)/D | E[dec/target] (a=0) | mu/class/half | left tuples | est s/arm | rho resolved |
|---|---|---|---|---|---|---|---|---|---|---|
| 0,1 | 7 | 2,3,4,5 | 4,2,2,2 | 4,12,30,66 | 1.75,4.08,11.4,36.4 | 47.5,11.4,45.5,127 | 4562,364,583,742 | 1e2..1e3 | <0.2 | no |
| 0,1 | 11 | 2,3,4,5 | 6,4,3,2 | 6,26,91,273 | 1.83,4.65,14.6,53.6 | 114,347,622,93.6 | 7263,5130,2624,132 | 7e2..1.4e4 | 0.1..1.6 | m=5 only |
| 0,1 | 13 | 2,3,4,5 | 7,4,3,3 | 7,35,140,476 | 1.86,4.83,15.7,60.0 | 170,152,326,5475 | 9332,1666,894,4417 | 2e3..1.2e5 | 0.2..14 | m=4,5 |
| 0,1 | 17 | 2,3,4,5 | 9,6,4,3 | 9,57,285,1197 | 1.89,5.07,17.2,69.8 | 288,1559,1303,1334 | 12292,10502,1756,428 | 9e3..5.7e5 | 1..66 | m=4,5 |

(a = 1 values differ by under 10 percent through |E|; the exact table is `plan.tsv` written into each run directory.)

Resolution, stated honestly. The split-half rho test needs enough classes: its minimum detectable correlation at z = 4 and
about 90 percent power is rho_min = (4 + 1.28)/sqrt(D - 1). rho_min <= 0.5 needs D >= 113: only the cells (11,5), (13,4),
(13,5), (17,4), (17,5) on both curves (10 of 32 cells). The other 22 cells (all m = 2, 3 and the small-n cells) use the
phi-ratio branch only, which detects a ratio above 1.5 and certifies a ratio below 1.25 and is otherwise UNRESOLVED; with
NV = 16 draws the interval may well be wider than that, and an UNRESOLVED cell is a legitimate outcome, not a failure to
be re-run with more draws chosen after seeing it. Per-class counts are large (mu/class/half 130..12000), so Poisson noise
is not the limit; the limits are D (few classes) and the V-to-V variation. Smaller l than the frozen value (l = 2..4 for
m >= 4) means blocks of 1..8 liftable x-values; this is the regime where m*l ~ n, not the l ~ 10 production regime.

## 7. Decision rule (frozen; applied per cell to arm frob by `g2dnm analyze`)

Order of checks, first hit wins:
1. INSTRUMENT_FAIL if the gauge audit is 0 on frob/geom, or z(rsub) >= 4 or z(rset) >= 4 (negative control flags: the null model is wrong for this cell), or (D >= 113) the positive control skew is not flagged under the naive null (z_naive < 4).
2. D < 113: CONCENTRATED_VS_CONTROLS if the phi-ratio lower bound > 1.5; UNIFORM_VS_CONTROLS if its upper bound < 1.25; else UNRESOLVED.
3. D >= 113: CONCENTRATED if z(frob) >= 4 and transfer >= 2; CONCENTRATED_WEAK if z >= 4 and transfer < 2; UNIFORM if rho + 1.96 SE < 0.25; else UNRESOLVED.
z = 4 is the per-cell flag level for a 32-cell grid with three tests per cell (Bonferroni, about 5e-4 per family). Nothing is
reported as positive on a single cell below that level; one CONCENTRATED cell among 32 at z just above 4 would be read as
a candidate to replicate with fresh seeds, not as a finding.
Campaign-level reading, fixed now:
* LEVER: CONCENTRATED in at least one cell on each curve at the same (n,m), with transfer >= 2, replicated under a second seed set. Only then does the m-dependent "visit only a few classes" idea survive; the next step would be the SAT cell restricted to those classes.
* NO LEVER FROM YIELD: no CONCENTRATED cell and UNIFORM in every resolved cell. Cost-neutrality is then supported on the yield side only at these toy sizes; it is not a cost result.
* Mixed or UNRESOLVED-dominated: report as such. The 22 low-D cells cannot establish uniformity at the rho level.
geom, skew and all other arms are descriptive; no verdict is drawn from geom.

## 8. Runtime estimate (code reasoning, not measurement)

Cost of one target pass = (#left tuples) * (one point addition with table inversion + one hash probe), assumed 150 ns
(uncertain by a factor of about 3). #left = 2^a*C(n_x, a) with a = ceil(m/2), n_x = n*(2^l - 1)/2.
Per arm per cell = #left * NV * NT = #left * 768 passes. Largest: (17,3,l=6): 5.7e5 * 768 * 150 ns = 66 s per arm; (13,5,l=3): 14 s.
Whole grid, seven arms (plain arms are cheaper, so an upper bound): about 1700 s single core; with 3 parallel jobs about
10 to 15 minutes wall, allowing 30 for contention on the shared CPUs and for table construction (the add per tuple node)
and the 3-target gauge audit. Memory: left table 24 bytes/tuple, at most 14 MB. Disk: rows files hold 4 class vectors per
row; the largest (17,5): 1197 classes * 4 * 16 draws * 7 arms, under 3 MB.

## 9. What does not count

* Any statement about SAT conflicts, propagations or time per relation. This cell enumerates; it prices nothing the solver does.
* Reporting the representative-only yield as "the union yield" or the union total as "per system" (the factor n of section 3).
* Relabelling: choosing a different class convention, rep convention, or null after seeing data; changing NV, NT, l, thresholds, the statistic list or the verdict rule after the first run (a change makes it a new protocol with a new run directory).
* Reading UNRESOLVED as uniform; reading a D < 113 cell as a rho-level negative.
* Any cell that is INSTRUMENT_FAIL; any cell run with a commit different from the frozen one (the run records commit and binary hash).
* geom results as evidence about random V; or any extrapolation to n = 131 (counts at n <= 17 are toy-scale; the closed form D(131,m) is exact, the yield behaviour is not measured there).
* This is not an index-calculus speed-up and does not touch the leaderboard or `benchmarks/README.md` (no performance claim, no matched-rho row). The research-visuals report/graph/PDF for the run are still required (RUN_CARD, post-run).

## 10. Contrarian notes: what the design can and cannot resolve

* Prior: for a random V the blocks sigma^kV look like independent subspaces to each other; Proposition T gives only the diagonal gauge, which is exactly the quotient already counted by D. The expected outcome is UNIFORM (aware null holds), P(CONCENTRATED on a random-V cell) below 5 percent. The informative surprise would be a Frobenius-specific excess that rsub does not share.
* Where structure could leak in even for random V: tiny l (blocks of 1..3 liftable x-values) makes the null approximate (the 2^m/|E| sign factor is an average); overlaps K(x) > 1 at n = 7. The rescaling to observed totals and the paired controls absorb the first; `unamb` reports the second. The null model is approximate, so phi alone is meaningless in absolute terms; only arm contrasts count.
* Uniform yield does not mean cost-neutral for a solver: the solver cost per class (sharing deficit between patterns, loss of the geometric linear oracle for mixed shifts) is outside this instrument. Conversely concentration would be necessary, not sufficient, for a lever.
* The distinct-x restriction removes ± pairs; the x-only S_m summation polynomial roots also admit repeated x, so counts differ from a SAT relation count by that degenerate part (which is unrelated to shift classes).
* D <= 66 cells (22 of 32) cannot resolve modest concentration with a correlation statistic; they are included because the m = 2, 3 comparison with the earlier n^(m-1) pricing needs them.
* Cells are not independent (same targets, same curve family); the Bonferroni count is conservative for the family of 32 but the two curves are near-replicates, which is why the lever criterion asks for both.

## 11. Variable ledger

instrument (one `Enumerator`, one `PatternTable`, one analysis binary, hash recorded); host (record `nproc`, load, kernel in RUN dir);
layout/order (arm order shuffled per draw, seeded); seeds (derived, recorded); targets (shared, paired); analyst (analysis
code frozen, arm labels not masked in the tool but the verdict is computed mechanically from section 7); spot preemption
(not applicable: local, minutes); curve (a), field polynomial (fixed per n in `field.rs`), cofactor subgroup targets.
