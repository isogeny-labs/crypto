# Pre-registration: how fast does the MutantXL closure degree grow with `m·ℓ`?

Registered 2026-10-18, before any cell below was measured. It is the
successor named in `research/dreg_closure_scale_20261016/RESULTS.md`
("the question that moves the derived rows is how fast the closure degree
grows with ℓ").

## Why

`research/dreg_closure_scale_20261016/` measured the fall-aware closure
degree of the chained Weil-descended `S₃` system at 4 on every draw with
`m·ℓ ≤ 18` and above 4 on every draw with `m·ℓ = 21`. Whether the algebraic
decomposition oracle can ever fit under matched rho depends on the *rate*
at which that degree grows, not on its value at one cell:

- The per-decomposition budget against matched rho is about
  `2^{n(1/2 − 1/m)}` (relations `2^ℓ/2n`, `m!·2^{n−mℓ}` attempts per
  relation, `ℓ ≈ n/m`), while a degree-`D` Macaulay-type solver on `N`
  Boolean unknowns costs at least `C(N, ≤D)` column touches. So the largest
  degree that fits grows like `n / (c·log₂ n)`, and a closure degree that
  grows linearly in `m·ℓ` with slope `s` fits at some `n` only if `s` is
  below about `0.03` degrees per summand unknown (derived; the exact table
  is computed natively by `dreg_score slope`, below).
- A slope of `0.10` or more, sustained, puts the route above rho at every
  `n`, for every subspace factor base, under this cost model.

So this study measures the closure degree at `m·ℓ = 20, 21, 24, 27, 30` and
fits the slope against the committed degree-4 cells at `m·ℓ = 12–18`.

## Instrument

- **Engine.** `examples/dreg_closure --sparse D`, which runs MutantXL's
  fixed-degree closure at one degree on a sparse echelon basis
  (`koblitz_bench::sparse_closure_refutes`). The dense engine of the
  previous study cannot build degree 5 at `N ≥ 40`. The sparse closure's
  saturated span equals the dense closure's, so the two agree on
  refutation (`sparse_closure_matches_the_dense_closure`); this study
  replays committed dense degree-4 refutations on the sparse engine as its
  instrument check (P4).
- **Draws.** Fresh cells come from `examples/dreg_ladder` with
  `--unsat 4 --ffd-max 3`, `d_max = 3`, seed `20261018`, and are committed
  under `draws/` before any closure runs. Every kept draw has exactly zero
  solutions by exact count.
- **Instrument change, made and tested before any draw was generated.**
  `chained_solution_count` and `chained_s3_solution_count` found the roots
  of each link `(a+c)²d² + ac·d + (ac)² + b = 0` by scanning all `2^n`
  values of `d`, which costs `2^{2ℓ+n}` per draw and is out of reach at
  `n ≥ 24`. They now solve the quadratic directly (Artin–Schreier
  `t² + t = c₀` by linear algebra over `F₂`, with the degenerate cases
  `a = c`, `ac = 0` handled separately). A test holds the new count equal to
  the scanning count on random instances at `n = 5..13`, both parities of
  `n`, and on the degenerate cases. The counted quantity is unchanged.
- **Run order.** One process per (draw, degree), degrees `4, 5, 6` in turn
  for fresh cells and `5, 6` for the two committed `m·ℓ = 21` cells, each
  draw stopping at its first refutation, cheapest cell first, at most two
  processes at a time:
  `prlimit --as=7000000000 timeout 10800 dreg_closure --sparse D --max-entries 350000000 --draw K FILE`.

## Cells (frozen)

`N = m·ℓ + (m−2)·n` is the unknown count of the chained system. `S = m·ℓ − n`.

| m | (n, ℓ) | m·ℓ | N | S | source | degrees run |
|--:|---|--:|--:|--:|---|---|
| 4 | (12, 3) | 12 | 36 | 0 | committed, closure 4 (`dreg_closure_scale_20261016`) | anchor |
| 4 | (14, 3) | 12 | 40 | −2 | committed, closure 4 | anchor |
| 3 | (11, 5) (13, 5) (15, 5) | 15 | 26–30 | −4..0 | committed, closure 4 | anchor |
| 4 | (13, 4) | 16 | 42 | +3 | committed, closure 4 | anchor; sparse replay at 4 on draws 0, 1 (P4) |
| 3 | (16, 6) (18, 6) | 18 | 34, 36 | −2, 0 | committed, closure 4 | anchor; sparse replay at 4 on (18, 6) draws 0, 1 (P4) |
| 4 | (20, 5) | 20 | 60 | 0 | fresh | 4, 5, 6 |
| 4 | (18, 5) | 20 | 56 | +2 | fresh | 4, 5, 6 |
| 3 | (19, 7) | 21 | 40 | +2 | committed, closure > 4 (`dreg_closure_scale_20261016/draws/m3-19-7.jsonl`) | 5, 6 |
| 3 | (21, 7) | 21 | 42 | 0 | committed, closure > 4 (`…/m3-21-7.jsonl`) | 5, 6 |
| 3 | (24, 8) | 24 | 48 | 0 | fresh | 4, 5, 6 |
| 3 | (22, 8) | 24 | 46 | +2 | fresh | 4, 5, 6 |
| 4 | (24, 6) | 24 | 72 | 0 | fresh | 4, 5, 6 |
| 3 | (27, 9) | 27 | 54 | 0 | fresh | 4, 5, 6 |
| 3 | (25, 9) | 27 | 52 | +2 | fresh | 4, 5, 6 |
| 3 | (30, 10) | 30 | 60 | 0 | fresh | 4, 5, 6 |
| 3 | (28, 10) | 30 | 58 | +2 | fresh | 4, 5, 6 |

- Four unsatisfiable draws per fresh cell; the four committed draws of
  each `m·ℓ = 21` cell.
- **Soundness control.** The first satisfiable draw the generator records
  in each fresh cell is replayed with `--satisfiable --sparse 4` and
  `--sparse 5`. Neither may refute. A refutation of a satisfiable draw is
  an instrument failure and voids the study.

## Verdict rules

Per draw:

- **degree D:** the sparse closure at degree `D` refutes and no lower
  scanned degree did.
- **≥ D+1:** degree `D` saturated without refuting and degree `D+1` is
  unmeasured.
- **unmeasured:** every scanned degree was capped (`"capped":true`) or
  killed. A cap or a kill is a resource limit, never evidence.

Per cell:

- **resolved at D:** at least three draws have an exact degree, and `D` is
  their median.
- **bounded below by D:** not resolved, but at least three draws read
  `≥ D` or an exact degree `≥ D`.
- **unmeasured:** otherwise.

Study:

- **Slope `s`:** the ordinary least-squares slope of cell degree against
  `m·ℓ` over the resolved cells, one point per cell, the committed anchors
  included at degree 4. Cells only bounded below are reported but not
  fitted. A second, descriptive statistic: the smallest `m·ℓ` at which a
  resolved cell reads 6.
- **"grows slowly":** `s < 0.03`. The report then computes, with
  `dreg_score slope`, the first `n` at which the extrapolated degree fits
  the budget for `m = 3..6`, and names it as the crossover of the cost
  model, derived.
- **"closed at every size":** `s ≥ 0.10`. The report then states the gap
  per rung as the distance remaining.
- **"undecided":** otherwise; the report names the next cell.

The slope is fitted over `m·ℓ ≤ 30` and read as the growth over that range.
It is not a law beyond it, and the report says so.

## Predictions

- **P1.** Every resolved draw at `m·ℓ = 20` and `21` reads exactly 5.
  Confidence about 65%. Reason: the previous study saw a sharp step from
  4 to above 4 between `m·ℓ = 18` and 21, and each earlier step was one
  degree.
- **P2.** Some resolved cell with `m·ℓ ≤ 30` reads 6. Confidence about
  55%. Reason: the XL fits rose one degree per 1–2 in `ℓ` and the closure
  stepped once in about 6 units of `m·ℓ`.
- **P3.** `s ≥ 0.10`. Confidence about 70%. Reason: 4 at 18 and ≥ 5 at
  21 already give a local slope above 0.3; a plateau long enough to pull
  the fit below 0.10 would need degree 5 to hold to `m·ℓ ≈ 30`.
- **P4 (instrument).** Every sparse degree-4 replay of a committed dense
  degree-4 refutation refutes. Confidence above 95%.
- **P5 (soundness).** No satisfiable draw is refuted at any degree.
  Confidence above 95%.

## Cost accounting, descriptive

Recorded per (draw, degree): rank, falls, products, maximum row weight,
wall seconds, and whether the entry cap was hit. No wall-clock claim. The
container is the 4-core, 15 GB, non-isolated one (`AGENTS.md` §10).

## Stop conditions

- Each (draw, degree) process has a 3-hour limit and a 7 GB address-space
  limit, and stops itself at 350 million stored entries. Any of these marks
  that degree unmeasured for that draw.
- A sparse replay of a committed degree-4 refutation that does not refute
  (P4 fails) stops the study: it is an instrument failure and is reported
  as such.
- Draw generation at a cell that has not produced four unsatisfiable draws
  after 4096 draws leaves the cell with the draws it has, and the report
  says so.

## Scope and class

- **Class: stage diagnostic.** No `S`, no rho ratio, no speed claim. The
  "distance remaining" table is derived from the cost model stated above
  and labelled so.
- **Systems.** The same chained `S₃` systems as the two previous closure
  studies: `b = 1`, random `F₂`-subspaces, `n ≤ 30`. Field subfields are
  as the generator draws them (`AGENTS.md` §8b).
- **Against ECC2K-130 and the success criterion.** This study measures the
  closure past the previous range and fits its growth. It does not improve
  the index-calculus baseline; it bounds one derived row. A null or
  negative outcome is recorded as such.
- **Conductor.** No Conductor task was taken: the CLI is not installed in
  this container, and its hooks fail open.
