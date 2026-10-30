# m = 4 Macaulay matrix as an object, and a fitted w (review D1 + F1)

2026-10-30. Class: **accounting on recorded measurements plus new read-only instrumentation**. It adds no
solver and no speed claim and moves no ECC2K-130 cost. Rho stays `2^60.81`. Counted metrics only
(host load 20 to 70). Not a performance result: no `benchmarks/README.md` row.

**Status: the new profile runs are NOT done.** The owner's S3 rule (`scripts/checkpoint.sh gate` before any
run) could not be completed from this session: the upload to S3 was refused by the session's network
policy (auto-mode classifier, "Command Network Lists"). `RESUME.md` is written and passes the local
check; nothing was uploaded and no run was started. Everything below is derived from the 1561 *recorded*
closure rows already committed, plus the instrumentation, which compiles and is ready to run
(`examples/m4_matrix_profile.rs`). Rows marked "pending" need those runs.

## 1. Fitted w (recorded data, no new runs)

Data: every `scan` row of every committed `research/dreg_closure_*/runs/*.jsonl` (1561 rows; `data/recorded_closure_rows.json`,
harvest in the commit). `closure_word_ops` is the dense full-MutantXL closure's counted word operations
(Macaulay reduction plus closure, Four Russians kernel). Model `log2 ops = a + w log2 columns`, OLS **with
intercept**, per degree block; 95% intervals from a bootstrap that resamples whole cells `(m, l, N)` (not
draws; `fit_w.py`). Saturating, non-refuting records only: a refuting closure collapses to the single row
`1` and its ops stop at the refutation, so refuted records are fitted separately and never pooled.

| block | rows / cells | log2 columns | w | 95% CI | intercept a | local w at largest size |
|---|--:|---|--:|---|--:|---|
| degree 3, m=3+4 | 495 / 37 | 7.8 to 13.5 | 2.00 | 1.83 to 2.07 | -1.2 | 1.8 (0.97 to 2.65) |
| degree 4, m=3+4 | 367 / 18 | 9.1 to 18.4 | **2.54** | 2.42 to 2.60 | -4.5 | 2.7 (2.4 to 3.1) |
| degree 4, m=4 only | 16 / 7 | 9.6 to 18.4 | 2.47 | 2.38 to 2.58 | -4.1 | 2.9 (2.1 to 3.4) |
| degree 5, m=3+4 | 345 / 12 | 10.0 to 16.1 | **2.83** | 2.59 to 2.92 | -5.9 | 3.2 (2.3 to 4.0) |
| degree 5, m=4 only | 10 / 3 | 10.6 to 13.6 | 2.25 | 2.07 to 2.96 | 1.3 | not fitted (3 cells) |
| pooled 3/4/5 with degree intercepts | 1207 / 37 | | 2.34 | 2.16 to 2.40 | | |

Reading.
- **w is not a constant and rises with size and degree.** The degree-4 and degree-5 curves are convex in
  log-log (positive curvature, local slope 2.7 to 3.2 at the top of the range), as a dense elimination
  approaches its cubic regime. The ratio `log2 ops / log2 cols` the earlier notes used (1.83 to 2.33)
  understates the slope by 0.3 to 0.6 because the intercept is negative.
- **Degree 3 and degree 4 separate cleanly** (2.00 vs 2.54; the CIs do not touch). The pooled slope
  (2.34) is a mixture and describes neither block.
- **Degree 5 is measured, at small N** (up to 68,406 columns, `m=3`; 12,616 columns, `m=4`). The earlier
  notes said degree 5 is unmeasured past N = 25; it is measured only as a saturating closure on
  *satisfiable* draws up to there. The m=4 degree-5 block has only 3 cells, so its CI (2.07 to 2.96) is
  wide and contains the m=3 value; the m=3+4 fit is the one used for pricing.
- The extrapolation from 2^16 columns to the `C(398, <=5)` about 2^36 columns of the real row is 20 bits
  of log-columns. It is the dominant uncertainty and is not removed by the CI.
- Caveat on what w is of: this is the exponent of *this dense Four-Russians kernel*, not a lower bound on
  every possible elimination. Section 3 treats the alternatives.

## 2. Repriced D = 5 row (m = 4), rho-step units

Model and positive controls are those of `research/m4_reprice_20261024` (`reprice.py` re-asserts the
`+10.32` and `+45.71` rows). Oracle `= a + w log2 C(N, <=5) + conv`, conv = `-8.44` bits (Ir per word-op
over Ir per n=83 rho step, `k_conversion_20261018/raw.jsonl`); K is **not** stacked on the elimination
term. `l = 34` for every row but the w=1 floor.

| oracle price | A: column-touch units | **B: rho-step units** | B 95% |
|---|--:|--:|--:|
| w = 1, a = 0 (the "+10.32" floor) | +10.32 | +6.95 | |
| w = 1.80, a = 0 (**crosses +30**) | +38.45 | +30.02 | |
| w = 2.00, a = 0 | +45.71 | +37.27 | |
| w = 2.25, a = 0 | +54.77 | +46.33 | |
| degree-4 fit, proxy at D = 5 (w 2.54, a -4.5) | +60.65 | +52.21 | +49.5 to +53.7 |
| **degree-5 fit (w 2.83, a -5.9)** | +69.89 | **+61.45** | +55.8 to +64.0 |
| dense-cubic stress (w = 3, matched at 2^16) | +73.36 | +64.92 | |

The interval is the bootstrap of `(a, w)` only; it does not include the extrapolation error or the
conversion range (-10.5 to -7.7 bits, a further +/-1.5).

## 3. Can a structured / block-Wiedemann elimination push w below 2? (estimate; nothing implemented)

What the recorded data already say about the matrix:

- **The closure basis is not sparse in any useful sense and not low rank.** At the cells that matter
  (m=4, l = 5 and 6, degree 4) the saturated closure rank is **62% to 75% of the columns**
  (`closure_rank / columns`: 0.63 at N=54, 0.72 to 0.75 at N=42 to 44), and the closure inserts 3.2 to
  4.0 products per column. A Krylov method applied to the closure's matrix has nothing to exploit; the
  closure *is* the dense elimination.
- **The plain XL matrix is low rank**: `xl_rank / columns` falls from 0.49 (794 columns) to 0.087
  (N=44) and 0.069 (N=54, 342k columns). It is also the only sparse object (rows are one generator times
  one monomial).
- **But plain XL needs more degree.** At the cells where the closure refutes at degree 4, plain XL
  refuted at 4 in **0 of 16** committed draws (it is at least 5). So a Wiedemann route pays at least one
  extra degree: at `N = 394`, `C(N, <=6) / C(N, <=5)` is about 2^6.0 more columns.

Price of that route, as a bound that uses no assumption about the kernel: a Krylov method needs at least
about `r/64` block iterations, each at least `nnz >= rows_in >= r` word operations, so
`cost >= r^2 / 64`. At `D' = 6` there are `c = C(394, <=6) = 2^42` columns, so with rank `r = c 2^-s`
the bound is `2^(78 - 2s)` word ops. The measured fraction at N=54 is `2^-3.9` (s = 3.9); even a
hypothetical `s = 10` (a further `2^-6` below that, optimistic) gives an oracle of `2^58`, a total of
`34 + 58 = 2^92`, about **+31** over rho in word-op units (about +23 after the -8.4 conversion), and that
is before the row weight is counted. The row weight (`rho`, terms per Macaulay row) multiplies `nnz`
and is the **pending** input (`input.lead_weight_sum`). At the measured-trend `s` of 4 to 7 the bound
is `2^(78-8..14) = 2^64..70`, i.e. +37 to +43 in word-op units before `rho`.

The bound `r^2/64` is generous: the sparse input has `nnz = rows_in * rho >= r * rho` with
`rows_in = E * C(N, <=D'-2)` for `E` generators, so the real figure is `(r/64) * rows_in * rho`. With the
`s = 4` to `7` trend that is about `2^(35..38 - 6) * 2^38 * E rho` words, which is above the +30
crossing in rho-step units for any `E rho >= 2^4`, but `E` and `rho` are not yet measured here, so
this is **an estimate that the profile run must confirm, not a result**. What the recorded data do
settle: the closure matrix is dense with rank 0.6 to 0.75 of the columns, so no Krylov method helps
*there*; any gain below `w = 2` would have to come from the plain XL matrix at one more degree, from the
rank fraction (a polynomial factor `2^-s`, not an exponent) and from the row weight. A structured
elimination is therefore a constant-factor lever of order `2s` bits at the cost of one extra degree (+12
bits at w=2), not a route to w below 2; this statement is conditional on the pending profile.

## 4. Matrix profile per degree block (pending)

`examples/m4_matrix_profile.rs` (instrumentation in `src/cryptanalysis/inherited_f4/profile.rs`, read-only;
the only change to shared code is a thread-local trace call in `ReducedBasis::close`) prints, per draw and
degree: input Macaulay profile (`--input`), plain XL echelon profile, saturated-closure profile, base vs
closure word ops and per-round products. Per degree block `d` of columns: columns, subspace-only columns
(monomials in the first `m*l` coordinate variables only, the "products of linear forms" reading, stated
explicitly: every column is a monomial in coordinate variables, so the informative split is subspace
coordinates vs the `(m-2)n` intermediate-point coordinates), pivots per block (rank profile), nonzeros
(density) and row weight by leading degree. Planned cells (all under 16 GiB; closure memory
`0.59 * 3 c^2 / 8`): m=4 (l, N) = (3, 36), (4, 42), (5, 42), (5, 44) at degree 3 and 4, (6, 54) at degree 3
only (degree 4 is 25.8 GB on the large host and is not rerun); first check that the instrumented word ops
equal the recorded ones on the same draws.

## 5. Decision (F1 rule)

The rule: if the D = 5 row in rho-step units is above about +30 bits, downgrade the (4,6) cell to a
degree curiosity and do not rent memory.

- Fitted w at degree 5 is **2.83 (CI 2.59 to 2.92)**; the degree-4 proxy gives 2.54. Both are above the
  +30 crossing at w about 1.8 by 0.7 to 1.0 in w, which at 36 bits of log-columns is 25 to 36 bits.
- Repriced D = 5 row: **+61.5 bits (bootstrap 55.8 to 64.0), degree-4 proxy +52.2 (49.5 to 53.7)**, against
  the review's +37 to +49 bracket, which assumed w in 2 to 2.33 with no intercept.
- The decision holds under every pricing in section 2 except w <= 1.8, which no measurement supports and
  Section 3 gives no route to.

**Decision: downgrade the (4,6) cell to "degree curiosity"; do not rent memory for it.** This is a
statement about the price of the algorithm at a fitted w; it does not say the degree stays 5, and it is
not a measured row at `n = 131`.

## Files

`fit_w.py`, `reprice.py`, `plot.py`, `data/` (harvest, `fit_w.json`, `reprice.json`),
`w_fit_and_reprice.{png,svg}`, `report.pdf`.
