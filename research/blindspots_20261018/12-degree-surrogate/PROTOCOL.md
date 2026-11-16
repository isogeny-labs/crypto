# Item 12 protocol: a surrogate for the solving-degree surface

Frozen before any measured run. Source design: section "12" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18.

Class: **stage diagnostic / planning instrument.** It measures nothing about a
curve, computes no `S`, no rho ratio and no GAE, so the section 5a three-point
pricing band does not apply to this item's rows. Its output is a model's error
on held-out cells of the committed degree record, and a ranked list of cells to
measure next. Nothing here is a speedup, and no number here is an `n = 131`
claim.

## Question

Can a model fitted on all committed toy-cell degree measurements predict the
solving degree `D` (and the Macaulay size at that degree) of cells it has not
seen, well enough to rank experiments, and does it do so better than the two
incumbent predictions: the semi-regular (Froberg) degree and the "levelled at a
constant" reading?

## What already exists, and what is reused

There is **no new algebraic measurement** in this item. Every label is read
from a committed record; the instrument is a native Rust harvester, three
baseline predictors, a gradient-boosted regressor, split-conformal intervals
and the validation harness. Nothing is re-measured, and each source below is
reused and cited by path and sha256 in `sources.tsv` (194 files, each pinned;
the instrument refuses to run on a file whose sha256 differs).

Sources used (the design's list, checked against what exists):

| design source | what the repository holds | used as |
|---|---|---|
| `research/dreg_*` | `dreg_ell_grid_20260925`, `dreg_fixed_surplus_20260923`, `dreg_surplus_control_20260925`, `dreg_degree7_ell5_20260930`, `dreg_m4_chain_20261006` (XL refutation degree, exact or `at_least`); `dreg_closure_mutantxl_20261016`, `dreg_closure_scale_20261016`, `dreg_closure_driver_20261016` (MutantXL closure degree, scans with columns, ranks, word ops) | engine `xl` (older five), engine `cl` (three closure dirs) |
| `research/ic_next10_20261014/06-*` | `runs/c1_xl_*.jsonl` (XL, m = 4); `runs/{c2_calibration,e_sat,m_main,m_main_b,m_main_c,n_nocount}.jsonl` (msolve F4 step degree `d_f4`, `max_matrix`, exit status); `runs/d_random.jsonl` (random cubic, all cut at 300 s) | engine `xl`; engine `f4`; censored-only family `f4r` |
| `research/fall_degree_bounds*` | `fall_degree_bounds_20261008/runs/*.jsonl` (`m = 2`, `xl_solving`, satisfiable and unsatisfiable draws) | engine `xl`, `m = 2` |

Sources of the design list that contribute **no rows**, with the reason:

* `research/ffd_*`: no such directory exists in the repository (checked with a
  glob). The first fall degree appears as an `ffd` field inside the `dreg_*`
  and `fall_degree_bounds` records; it is **not** used as a feature, because it
  is itself measured on the cell being predicted.
* `research/f6_*`: audited. The `f6_*` directories hold index-calculus end-to-end
  and phase measurements at `n` about 83 (wall, work, relation matrices); the
  only `matrix_rows` fields (`f6_ic_ecbench_ladder_20261005/sessions/*/records.jsonl`)
  are 5 to 18-row relation matrices, not Macaulay matrices, and no `f6_*`
  record carries a solving degree. No row.
* Items 04, 05 and 07 of this campaign: **not finished** when this protocol was
  frozen (their lanes were still writing `raw/`). Their output is not used and
  not parsed. When they finish, their cells are a prospective test set for the
  frozen model, scored by a later addendum that cannot change this item's
  verdict.
* `research/dreg_closure_specialise_20261017`: its draws have variables
  **fixed** (field `fixed`), so they are not cell-shaped systems. Excluded.
* `research/ic_next10_20261014/06-*/runs/g2_full_pass_*.jsonl`: equivariant
  elimination timings, no solving degree. Excluded.
* `research/pdp_split_dreg_20261011`, `research/boolean_degree_replay_20260922`:
  outside the design's named list. Excluded (not loosening: fewer rows).

Layout of the records was inspected (field names, outcome kinds, counts of
satisfiable against unsatisfiable lines) before this freeze so the parsers could
be specified. **No arm was fitted and no error was computed on any real label
before the freeze.**

## Unit of the data

A **record** is one draw. Its covariates are the cell shape only:

* `m` (summands), `n` (field degree), `l` (factor-base dimension);
* derived: `N = m*l + (m-2)*n` unknowns, `E = (m-1)*n` equations,
  `S = E - N = n - m*l` (surplus), `m*l`, `engine` (xl = 0, cl = 1, f4 = 2),
  `sat` (1 if the draw has a solution, else 0).

The chained `S_3` system has `E` equations: `(m-2)*n` cubic and `n` quadratic
(`m = 2`: `n` quadratic), the convention of `dreg_score semireg`. A harvested
record is dropped and counted if its recorded unknown count disagrees with `N`.

Label `D` per engine:

* `xl`: the committed XL refutation degree (`outcome.kind = resolved`, exact),
  or `at_least k` (right-censored, `D >= k`). Satisfiable lines with no degree
  carry no label and are dropped. For `m = 2` the label is `xl_solving`
  (defined for satisfiable and unsatisfiable draws; `null` is censored at
  `max computed degree + 1`).
* `cl`: the least scanned degree with `closure_refuted`, exact; none refuted up
  to `measured_to` is censored at `measured_to + 1`. Satisfiable draws dropped.
  XL fields inside closure records are **not** used (the older studies hold the
  same draws' XL degrees exactly; reading both would double-count).
* `f4`: msolve's `d_f4` (maximum step degree) when the run exited 0, exact; an
  unfinished run (timeout, signal) gives `D >= d_f4`, censored. Satisfiable
  draws are kept (flag `sat`).
* `f4r` (random cubic, label `r62` = `(m,n,l) = (4,6,2)`, `r93` = `(4,9,3)`):
  all censored; used only in the censored-consistency check.

Second label, **Macaulay size**: `size = log2 sum_{i<=D} C(N, i)`, the column
count of the degree-`D` matrix. It is a function of `(N, D)`, so a model of `D`
induces a model of size; error in size is reported beside error in `D`. A
third, *measured* label `cols = log2(max_matrix columns)` exists for finished
`f4` runs and is modelled separately (descriptive, `T3` below).

Duplicates (the same system measured twice in one engine) are removed by key:
`xl` and `f4` by `(m, n, l, v_basis, x_r)`, `cl` by `(source draws file, cell,
draw)`, `fdb` by `(n, seed)`. The harvest audit prints every drop count.

**Family** = engine and `m`: `xl-m2`, `xl-m3`, `xl-m4`, `cl-m3`, `cl-m4`,
`f4-m3`, `f4-m4`, `f4r-m4`. A **cell** is `(family, n, l, sat)`. Only exact
labels train models and enter the error metrics; censored records enter only
the consistency check.

## Arms

* **(a) Semi-regular (Froberg) prediction.** `D_reg` of the Hilbert series
  `(1+z)^N / prod_i (1+z^{d_i})` (degrees `[2]*n + [3]*((m-2)*n)`), the index
  of its first non-positive coefficient, exactly the formula of
  `research/dreg_degree7_ell5_20260930/semireg.py` and `dreg_score semireg`
  (known answers below). No fitting. Predicts the same `D` for every engine.
* **(b) "Levelled" constant.** The median of the training `D` of the record's
  own engine; the global median if the engine is absent from training.
* **(c1) Gradient-boosted regressor, primary.** Native Rust, squared loss,
  depth 3, 200 rounds, learning rate 0.1, minimum 5 records per leaf, no
  subsampling, features `[m, n, l, N, E, S, m*l, engine, sat]`. Constants are
  frozen here; nothing is tuned.
* **(c2) Same, plus the `D_reg` of arm (a) as a tenth feature.** Ablation
  (does the data correct Froberg?). **Not** the primary arm.
* **(e) Linear baseline** (ridge, `lambda = 1e-3`, features `1, m, l, n, m*l`,
  engine dummies, `sat`). Added so a win over (a) and (b) can be seen not to
  need trees. Not part of the rule.
* **(d) Leave-one-family-out validation** is the evaluation scheme, not an
  arm: below.

The design's arm (c) is taken to be (c1); (c2) and (e) are additions, so the
rule is tighter, not looser.

## Evaluation (arm (d) and the metric)

**Folds.** Every family with at least 3 cells holding an exact label is held
out in turn; the models see every other family's exact records **except
those whose `(m, n, l)` shape occurs anywhere in the held-out family** (the
shape-disjoint rule). Without it, holding out `cl-m4` would leave the same
`(m, n, l)` draws measured by `xl-m4` and `f4-m4` in training, and the
held-out cells would not be unmeasured. The same folds without the rule are
reported as a sensitivity (`leaky_*`), outside the verdict.

**Split conformal, per fold, repeated.** For `r = 0..19` (seed `20261018 + r`),
the training cells are shuffled and split 70 / 30 by **cell**. All arms are
fitted on the 70 % (arm (a) needs no fit), absolute residuals on the 30 % give
`q` = the `ceil(0.9 * (n_cal + 1))`-th smallest, and the interval for a
held-out record is `y_hat +/- q`. Every metric below is computed per split and
averaged over the 20 splits. The held-out family is a covariate shift from the
calibration cells, so the nominal 90 % is a hypothesis, measured here.

**Metrics (exact labels of the held-out family).**

1. `MAE_D`: mean over the fold's cells of the mean over a cell's draws of
   `|D - y_hat|`. Primary aggregate: the **macro mean over folds**.
2. `MAE_size`: the same, for `|size(N, D) - size(N, round(y_hat))|`
   (`y_hat` clamped to `[1, N]`).
3. `cover`: fraction of draws with `|D - y_hat| <= q`, cell mean then fold mean
   then macro over folds.
4. `rho`: Spearman correlation between the per-cell mean prediction and the
   per-cell mean `D`, macro over folds with at least 3 cells and a non-constant
   `D` (a constant predictor scores 0).

**Uncertainty.** Cluster bootstrap: 2000 resamples of cells with replacement
within each fold, macro aggregate recomputed, 95 % percentile interval, applied
paired (the same resample for every arm) to `MAE_a - MAE_c1` and
`MAE_b - MAE_c1`.

**Censored consistency (descriptive).** For censored records (held-out
families, plus `f4r` under a model fitted on all exact data): the share with
`y_hat + q >= bound`, per arm.

**T3 (descriptive, outside the rule).** The measured `cols` of finished `f4`
runs, leave-one-family-out over `f4-m3` and `f4-m4`; arms: (a) `log2 C(N, <= D_reg)`,
(b) the training median, (c1), (e); MAE and rho only.

## Controls

All run by `selftest` (cheap, first card) and the `nullperm`/`planted` runs (card 2).

* **Known answers.** `D_reg(m, n, l)` must equal the committed reference values
  (`dreg_degree7_ell5_20260930/RESULTS.md`, `dreg_m4_chain_20261006/RESULTS.md`):
  m = 3 `(10,5)` 7, `(11,5)` 7, `(13,5)` 7, `(15,5)` 8; m = 4 `(4,1)` 5, `(5,1)` 5,
  `(6,1)` 5, `(5,2)` 6, `(6,2)` 7, `(7,2)` 7, `(8,2)` 7, `(9,3)` 8, `(10,3)` 9.
  The column count `sum_{i<=d} C(N, i)` must give 124,314 at `N = 42, d = 4`
  (the committed `columns` field of `dreg_closure_driver_20261016`). The JSON
  parser, SHA-256, quantile and Spearman helpers are checked on fixed vectors.
* **Positive control (planted label).** On the real covariates, replace every
  exact label by the planted `D_plant = 3 + l + [S < 0]` and run the whole
  leave-one-family-out pipeline. It passes if `MAE_D(c1) <= 0.5 * MAE_D(b)`.
  A failure means the pipeline cannot see an effect that exists.
* **Conformal control.** On synthetic i.i.d. data (200 repetitions of a noisy
  nonlinear function, split conformal at 90 %), mean coverage must lie in
  [0.87, 0.93].
* **Negative control (label-permutation null).** 100 permutations of the
  exact labels **within engine**, across all families at once (marginal of `D`
  per engine kept, covariate link broken; censored labels untouched), the whole
  pipeline run with arm (c1) and 3 splits each; gives the null band of macro
  `MAE_D`. The real macro `MAE_D(c1)`, taken over the same 3 splits, must
  fall below the band's 2.5 % quantile for the rule to count it as a signal.
* **Validity.** If the known answers, the positive, or the conformal control
  fail, the verdict is INCONCLUSIVE regardless of the rule.

## Decision rule (frozen)

The design's rule is: "Useful if (c) beats (a) and (b) on held-out error with
coverage in [0.85, 0.95]". Frozen reading, stricter than the design:

* **ADVANCES (primitive)** if all hold: controls valid; macro `MAE_D(c1)` is
  below both macro `MAE_D(a)` and macro `MAE_D(b)` with each paired bootstrap
  difference interval excluding 0; the null band excluded; and macro `cover(c1)`
  in [0.85, 0.95].
* **TRADES (primitive)** if the error conditions hold but coverage is outside
  [0.85, 0.95] (point predictions better, intervals miscalibrated).
* **REGRESSES** if macro `MAE_D(c1)` is above (a) or (b) with the paired
  interval excluding 0 in that direction.
* **MATCHES** otherwise (no separation from (a) or (b)).
* **INCONCLUSIVE** if a control fails or fewer than 5 folds are evaluable.

The verdict names its level (`primitive`: a planning instrument, not a cost
exponent or constant). The gap to matched rho is not applicable (no `S`).

**Output required by the design: the next five cells to buy.** Fit (c1) on all
exact records (20 conformal splits, mean prediction). Candidates: engine in
`{cl, f4}`, `sat = 0`, `m` in `{3, 4}`, `l = 2..7`, `n = 3..24`, `N <= 60`, not
already present (any record, exact or censored) in that engine, and predicted
size `size(N, round(y_hat)) <= 22`. Score = `|y_hat_c1 - D_reg|` (degrees by
which the fitted model and the semi-regular reading disagree: the cell that
best separates "levelled" from "growing"); ties by smaller predicted size;
at most one cell per `(m, l)` (ties between the two engines go to `cl`);
the top five are the list. This list is
a model output, not a measurement, and is ranked without peeking at any
future label.

## Budget

One thread per job. Every job runs under `prlimit --as=4294967296`. The whole
item is a few CPU-minutes (`harvest` is milliseconds; the heaviest card, the
permutation null, is bounded by 100 x folds x 3 gradient-boosted fits). Cap: at
most 2 CPU-hours (far above need); every card is under 9 minutes wall.

## Implementation clarifications (fixed before the first measured run)

* `rho` is the Spearman correlation of the split-averaged prediction (mean of
  `y_hat` over the 20 splits) with the cell mean `D`; `MAE`, `cover` and the
  size error are means over the 20 splits.
* Gradient-boosted trees are fitted on records aggregated by identical feature
  vector (count, mean label), which is the same fit as on the records.
* A cell's coverage is the mean over its draws; a fold's, the mean over its
  cells; the macro value, the mean over evaluable folds (at least 3 exact
  cells).
* `T3` uses the same splits and rules (shape-disjoint folds `f4-m3`, `f4-m4`;
  `cols` is the log2 of the largest matrix's column count of a finished run).
* `selftest` and `planted` are the smoke test. A hidden switch `--plant`
  replaces every label by the planted law so every subcommand can be exercised
  without reading a real result; no card uses it.
* Hyperparameters are constants in the source (rounds 200, depth 3, rate 0.1,
  minimum leaf weight 5, ridge 1e-3, 20 splits, 2000 bootstrap resamples,
  seeds from 20261018).

## Deviations from the design (each a tightening or a clarification)

1. **Arm (c) is pinned to (c1)** (no `D_reg` feature); (c2) is an ablation.
   Reason: with `D_reg` as a feature (c) would inherit arm (a)'s information.
2. **Rule tightened** with a paired bootstrap, the permutation null, and the
   controls; the design asked only for "beats" and coverage in [0.85, 0.95].
3. **Macro averaging by fold and cell**, because closure families hold hundreds
   of draws at `D = 4` and would otherwise decide a pooled mean.
4. **Censored labels are excluded from training and error** (their true value is
   unknown), and only checked for consistency, instead of being treated as exact.
5. **Items 04, 05, 07 not used** (unfinished); `f6_*` and `ffd_*` contribute no
   rows (audited above). The design says "if finished first".
6. **Build with `rustc`, not `cargo build --example`.** The instrument is
   std-only; a cargo build would recompile the library whenever another lane
   edits `src/`, so a card could exceed its time limit for reasons unrelated to
   this item. Each card builds its binary under `target/blindspots-12/` and
   removes it (the pattern items 07 and 11 use). No per-item `CARGO_TARGET_DIR`.
7. **Language.** Rust, std only. The data are read by the instrument itself;
   there is no Python in the path.
8. **Shape-disjoint training** inside each leave-one-family-out fold, so the
   held-out cells are unseen in every engine, not only in their own (see
   Evaluation). A tightening: it removes training rows.

## Card plan

Three cards, all `kind: frozen_command`, each building its own binary under
`target/blindspots-12/card-NN/` and removing it. The exact argv are in
`card-01.yaml` to `card-03.yaml`.

1. `card-01`: build, `selftest` (writes `raw/selftest.kv`), `harvest` (verifies
   the 194 sha256s, writes `raw/data.tsv`, prints the per-source audit and the
   per-family counts).
2. `card-02`: build, `planted` (`raw/planted.kv`), `evaluate`
   (`raw/eval.kv`), `nullperm` (`raw/null.kv`).
3. `card-03`: build, `t3` (`raw/t3.kv`), `rank` (`raw/rank.txt`), `verdict`
   (applies the frozen rule to the `*.kv` files in `raw/` and prints one line).
