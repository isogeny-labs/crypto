# C1 toy ladder: standard base vs twisted all-lift line, n = 4

Registered 2026-10-27, before the confirmatory run. Seed: `0xC1_7715_7ED_2610_27`
(`src/experiment.rs::SEED`); every random stream is derived from it and a
named tag (`Rng::new(SEED, tag)`); bootstrap stream tag `bootstrap`.

Parent study: `research/lift_dense_factor_base_20261027/` (the twisted-line
construction, Amendments 1-3). Upstream cost model:
`research/lift_dense_factor_base_20261027/c2pnb/c1_model_upstream.py`, whose
README asks for this toy ladder to pin `c_trial`, the solving degree and the
memory. All computation is native Rust (std only). Python is used only for the
layout of the figure and for compiling the Typst report.

## 0. What is and is not measured

Measured (exact, counted, host independent):
- relation yield: exact numbers of ordered decompositions
  `R = P1+P2+P3+P4`, `Pi` in the factor base, over fixed target sets;
- the descended Gaudry system: monomial support, matrix dimensions, ranks,
  Hilbert function, closure, and counted `F_Q` multiplications of one fixed
  elimination algorithm.

Not measured: wall-clock time (no timing claim is made anywhere), any
end-to-end index-calculus cost, any comparison with Pollard rho, any
leaderboard or `benchmarks/` row. The cost result is for a **simplified,
well-defined solver** (weighted Macaulay matrix at one fixed degree, forward
elimination with sparsest pivots; section 4), not a full F4. No `F_Q`-rational
solution extraction (multiplication matrices, eigen step) is run; correctness
of the system is checked by plugging explicit relations into it (section 5).

## 1. Objects

`Q = 2^k`, k odd, `F = F_{Q^4} = F_Q[g]/(g^4+g+1)`, `N = 4k`. `T = Tr_{F/F_Q}`.
`E: y^2 + xy = x^3 + a x^2 + b`. `c = sqrt(b)`. `u` is *valid* if
`T(u) = T(c/u) = 0`. The twisted line is `{u y : y in F_Q^*}`; the standard
base is `{y : y in F_Q^*}`. A factor base (FB) is the set of both points over
every abscissa of the line that lifts to `E(F)`. `m = |FB|`.

Facts used (each re-checked numerically by the code, section 5): for
`Tr(a) = 0` and valid `u` every nonzero abscissa of the twisted line lifts, so
`m_tw = 2(Q-1)`; for `Tr(a) = 1` none does.

For `a, b in F_16` and k odd, `T(v) = Tr_{16/2}(v)` for `v in F_16`, so
`tau = Tr_{16/2}(b)` decides the standard line: for `tau = 1` it has
`(Q-2)/2`-ish lifting abscissae (density 1/2), for `tau = 0` and `Tr(a) = 0`
the standard line is itself all-lift (`m_std = m_tw`, predicted ratio 1).
This is a property of the mandated `b in F_16` family, found in the pilot, and
is registered here as a control, not as a failure.

## 2. Curves and arms

Per k (odd, k in the grids below):

| id | family | a | b | role |
|:--|:--|:--|:--|:--|
| A1, A2 | A (task) | 0 | the two orbit representatives (under `b -> b^2`) of `F_16^*` with `Tr_{16/2}(b)=1` | main arms: std density 1/2; these are the only two non-isomorphic `a,b in F_16`, `Tr(a)=0`, `tau=1` curves |
| A0 | A | 0 | 1 (`tau = 0`) | control: std line already all-lift, predicted ratio 1 |
| NEG | A | `g^3` (Tr = 1) | A1's b | negative control: twisted line must give 0 lifts and 0 relations |
| B1, B2, B3 | B (extension) | 0 | uniform random in `F^*` | generic curves, `b` not in `F_16`, `u` outside `F_16` (found by search in `ker T`, `u = z^Q + z`, rejection on `T(c/u)=0`, rejecting lines meeting `F_16` or the other arm's line) |

Why family B: with `a, b in F_16` only two isomorphism classes with `tau = 1`
exist; the task asks for at least 3 curves per k, and the standard base on a
`b` in `F_16` curve is not generic. Family B is an extension and reported
separately. Family A is the task's mandated family.

Valid `u`: family A takes the first two valid `u in F_16^*` (in coordinate
order) as arms `tw_a`, `tw_b` (`u in F_16` makes the line `pi`-stable; every b
in `F_16^*` has valid `u` in `F_16`, so the exhaustive search in `ker T` that
the task allows for was needed only for family B, where it is the generic
route). Arms: `std` (u = 1), `tw_a`, `tw_b`. `tw_a` versus `tw_b` is the A/A
arm: two different valid `u` on the same curve.

## 3. Yield

Grid (ordered; same targets for all arms of a cell; targets uniform in
`E(F) \ {O, T}`, `T = (0, c)`, drawn as uniform lifting abscissa times a
uniform sign):

| k | `N=4k` | targets/cell | curves |
|--:|--:|--:|:--|
| 5 | 20 | 20000 | A0 A1 A2 NEG B1 B2 B3 |
| 7 | 28 | 8000 | A0 A1 A2 NEG B1 B2 B3 |
| 9 | 36 | 1500 | A0 A1 A2 NEG B1 B2 B3 |
| 11 | 44 | 150 | A1 A2 B1 |

Statistic per target: `N(R)` = number of ordered `(P1..P4)` in `FB^4` with sum
`R`, by a meet-in-the-middle pair table (exact; section 5 validates against
direct enumeration). Predictions follow Amendment 3 of the parent study:
`pred = (m^4 - C_O - C_T)/(#E - 2)`, `C_O = N(O)`, `C_T = N(T)` exact. `#E`:
families A and NEG by the Frobenius recurrence over `F_16`
(`#E(F_{16^k}) = 16^k + 1 - t_k`, checked against a direct count at k = 5, 7);
family B by direct count at k <= 7; unknown at k >= 9 (ratio tests only, no
absolute prediction).

Tests (empirical-variance, not Poisson: counts are clustered because one
unordered solution appears up to 24 times as an ordered tuple):

- **H-Y1 (instrument check).** Per (cell, arm) with a prediction and
  `T * pred >= 50`: `z = (mean - pred)/(s/sqrt(T))` with `s` the sample
  standard deviation of the per-target counts. Pass: `|z| <= 3` on at least
  90 % of such rows and no `|z| > 5`.
- **H-Y2 (yield ratio).** For each tau = 1 cell and each of `tw_a`, `tw_b`:
  `R_obs = mean(tw)/mean(std)` on the same targets, `R_pred = pred_tw/pred_std`
  (the exact-count prediction, close to 16 = `(2Q-2)^4/m_std^4`). Interval: 95 %
  percentile paired bootstrap over targets (2000 resamples). Pass: `R_pred`
  inside the interval for at least 80 % of the comparisons, with
  `R_obs/R_pred` pooled over cells (sum of means) in `[0.9, 1.1]`. Cells
  without an absolute prediction (family B, k >= 9) use `R_pred = (m_tw/m_std)^4`,
  and are marked as such.
- **H-Y3 (positive control, A0).** Same as H-Y2 with `R_pred ~ 1`.
- **H-Y4 (negative control, NEG).** Exact: twisted-line lifting count 0 and
  all twisted counts 0 at every k.
- **H-Y5 (A/A).** `tw_a/tw_b`: the 95 % paired-bootstrap interval of
  `mean(tw_a)/mean(tw_b)` contains 1 in at least 80 % of cells, and the pooled
  ratio is in `[0.9, 1.1]`.

## 4. Cost: the solver and what is counted

System. `S_5` is built symbolically (`src/semaev.rs`): `S_3 = (x1+x2)^2 x3^2 +
x1 x2 x3 + x1^2 x2^2 + b`, `S_4 = Res_X(S_3(x1,x2,X), S_3(x3,x4,X))`,
`S_5 = Res_X(S_3(x1,x2,X), S_4(X,x3,x4,x5))`; symmetrised in `x1..x4` over
`e1..e4` (100 terms; measured: even exponents in `e1,e2,e3`). With `x_i = u y_i`,
`y_i in F_Q`, `E_j = e_j(y)^2` (j <= 3), `E_4 = e_4(y)`, the four coordinates of
the `F`-valued equation on `1,g,g^2,g^3` are four equations over `F_Q` in
`E1..E4`, weights (2,2,2,1), weighted degree 8, the same 105-monomial shape for
both bases; only the coefficients change (monomial `i` is multiplied by
`u^{i1+2 i2+3 i3+4 i4}`; `u = 1` is the standard base). Weighted Bezout bound
512.

Solver (identical for every arm). Weighted Macaulay matrix at degree D (rows
`m * f_r`, `wdeg(m) + 8 <= D`; columns all monomials of weighted degree <= D,
descending weighted degree then reverse lexicographic), forward elimination by
leading column, sparsest pivot in each bucket, pivot rows normalised.

Counted: every `F_Q` multiplication of two nonzero entries (one per nonzero
entry of a pivot row beyond its lead, per row it eliminates, plus one per
nonzero entry normalised) and every `F_Q` inversion (one per pivot). Not
counted: additions, table lookups, row copying. Setup (forming the four
equations from `x_R`) is reported separately in `F_Q` products (one product in
`F` = 16 `F_Q` products). Memory: dense matrix bytes `rows * cols * 2`.

Per (cell, target, arm) D = 26 (`4(8-1) - (2+2+2+1) + 1`, the regularity
degree of a complete intersection in these weights) is run and recorded; for
one target per cell of A1 and B1, the whole ladder D = 8..27 is recorded
(matrix dimensions per degree step). Fields: rows, cols, rank, Hilbert
function `hf = cols - rank`, maximal degree of a standard (non-pivot) monomial
`std_max_deg`, `closed := std_max_deg < D`, mults, invs.

Grid: k in {5, 7, 9, 11, 13, 15}; curves A1 A2 B1 B2 B3 (A0 only for k <= 9);
6 targets per cell (the first six of the cell's yield target stream, so the
same targets as the yield run); arms std, tw_a, tw_b.

Tests:

- **H-C1 (structure).** For every pair (std, tw) on the same target:
  `rows`, `cols` equal (true by construction), and `rank`, `hf` equal. Pass if
  rank and hf agree on at least 90 % of pairs.
- **H-C2 (cost unchanged, the hypothesis).** `rho = mults(tw)/mults(std)` at
  D = 26 on the same target, over pairs in which both systems are closed.
  Estimator: geometric mean of `rho`. Interval: 95 % two-level percentile
  bootstrap (resample cells, then pairs within the cell; 2000 resamples).
  Tolerance `[0.8, 1.25]`. Verdict: *unchanged* iff the interval lies inside
  the tolerance; *changed* iff it lies outside; otherwise *inconclusive*.
  Reported overall, per family, per k; the same estimator on `tw_a/tw_b` is
  the A/A noise reference. If *changed*, the report gives the ratio and its
  interval ("by how much it moves").
- **H-C3 (closure).** The fraction of closed systems per arm. Pass if both
  are at least 0.9 and differ by at most 0.1 in absolute terms. Systems that
  are not closed at D = 26 are listed (HF_top diagnostic from the pilot:
  the top-weighted forms have a common zero at infinity, so no fixed D closes
  them).
- **H-C4 (k dependence).** Descriptive: log-log slope of `mults` against k.

Verdict labels use only these rules. Nothing else is promoted to a claim.

## 5. Controls and checks run before and with the data

- `selfcheck`: field axioms, Artin-Schreier solver, `#E` (formula = direct
  count, k = 5, 7), `S_5(x(P1),...,x(P4), x(sum)) = 0` on random relations,
  meet-in-the-middle count = triple enumeration (6 targets, O, T; k = 5).
- `validate`: at k = 5 on A1, A2, B1, B2 and 150 targets per cell, every
  ordered relation found by direct enumeration satisfies all four descended
  equations (`equation_violations = 0`), has `x_i/u in F_Q`, and the
  enumeration count equals the pair-table count.
- `linecheck`: 64 sampled `y` per arm and cell: the twisted lines lift on all
  of them (Tr(a)=0 curves) and none (NEG).

## 6. Pilot, disclosed

Before this protocol: `selfcheck`; `hf`/`hfb`/`topscan` at k = 5 on
a handful of targets (streams `hf`, `hfb`, `topscan`, `topscanB`, not used in
the confirmatory run). Pilot findings that shaped the registered rules:
(i) Hilbert function 512 at D >= 25 and closure at D = 26 for regular systems,
both bases; (ii) the top-weighted forms of the system occasionally have a
common zero at infinity (HF_top = 4 or 8), on both bases, depending on the
target, so the Macaulay matrix never closes (hence H-C3 and "closed pairs");
(iii) `tau = 0` makes the standard line all-lift (hence A0); (iv) counted
mults vary by about +-20 % between targets and curves, so a point comparison
of single targets means little (hence the tolerance, the bootstrap and the
A/A reference).

## 7. Reporting rules

Numbers in `README.md` are copied from `reports/run-<UTC>/summary.txt` verbatim.
Verdicts are the labels above. No wall-clock quantity appears. The cost model
`c_trial` conclusion applies to this solver and these curves only.
