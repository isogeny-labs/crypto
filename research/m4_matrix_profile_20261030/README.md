# m = 4 Macaulay matrix as an object, and a fitted w (review D1 + F1)

2026-10-30. Class: **accounting on recorded measurements plus new read-only instrumentation**. It adds no
solver and no speed claim and moves no ECC2K-130 cost. Rho stays `2^60.81`. Counted metrics only
(host load 20 to 70). Not a performance result: no `benchmarks/README.md` row.

**Status: the profile runs are done** (34 rows: m = 4, five cells, degrees 3 and 4, satisfiable and
unsatisfiable draws; `data/profile_*.jsonl`, `profile_table.py`, `data/profile_table.{json,md}`). The owner's S3
gate (`scripts/checkpoint.sh gate`) was run by the user from their terminal, because this session's network
policy refused the upload; it passed (`20261030T173312Z-50092387d985`). **Instrumentation check:** the profiler's
counted word ops equal the recorded ones on every row that has a recorded counterpart (29 of 29 exact, 0
mismatches, including degree 4 at 124k to 150k columns; 5 rows are draws with no recorded row). Peak RSS was not
captured (`time -l` is blocked by the sandbox); the model estimate `0.59 * 3 c^2 / 8` is 5.0 GB at the largest
cell (150k columns), and no run was killed. Not run: (6,54) at degree 4 (342k columns, 25.8 GB, over the 16 GiB limit).

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

## 3. The matrix as an object (measured, m = 4, 34 runs)

Columns are monomials in the `N = 2n + 4l` coordinate variables. "Subspace columns" are monomials supported only
on the first `m l` subspace coordinates (the factor-of-linear-forms reading, stated explicitly; every column is a
monomial in coordinate variables). `rank` is the saturated closure basis; `XL rank` is the plain echelon basis;
`rho` is the mean weight of the unreduced Macaulay input rows; `E = rows_in / C(N, <=D-2)`.

| cell (n, l, N) | D | draw | cols | closure rank / cols | XL rank / cols | input rows (E) | rho | subspace cols d2 / d3 / d4 | closure pivots per block d1,d2,d3,d4 | closure nnz per entry | rounds | closure ops | Macaulay (base) ops |
|---|--:|:-:|--:|--:|--:|--:|--:|---|---|--:|--:|--:|--:|
| (11,5,42) | 3 | S | 5922 | 0.269 | 0.083 | 495 | 44 | 22% / 7.4% / | 1, 58, 1532 | 0.80% | 2 | 8.1e6 | 3.9e5 |
| (11,5,42) | 4 | S | 124314 | **0.765** | 0.087 | 10890 (12.0) | 52 | 22% / 9.9% / 4.3% | 3, 233, 6099, 88770 | 1.14% | 13 | 3.2e11 | 2.4e8 |
| (11,5,42) | 4 | U | 123814 | 0 (refuted) | 0.087 | 10890 | 55 | | | | 6 | 1.2e11 | 2.4e8 |
| (12,5,44) | 4 | S | 149905 | **0.684** | 0.086 | 12972 (13.1) | 60 | 20% / 8.6% / 3.5% | 3, 228, 6181, 96126 | 0.98% | 14 | 2.9e11 | 3.6e8 |
| (12,5,44) | 4 | U | 149986 | 0 (refuted) | 0.086 | 12972 | 61 | | | | 9 | 6.2e11 | 3.5e8 |
| (13,4,42) | 4 | S | 124314 | **1.000** | 0.102 | 12870 (14.2) | 54 | 14% / 4.9% / 1.6% | 21, 837, 11478, 111930 | 0.009% | 8 | 6.1e11 | 2.9e8 |
| (13,4,42) | 4 | U | 124314 | 0 (refuted) | 0.102 | 12870 | 56 | | | | 5 | 7.2e11 | 2.9e8 |
| (12,3,36) | 4 | S | 66712 | **1.000** | 0.131 | 8892 (13.3) | 40 | 10.5% / 3.1% / 0.8% | 26, 618, 7139, 58905 | 0.005% | 4 | 5.1e10 | 1.0e8 |
| (12,3,36) | 4 | U | 66712 | 0 (refuted) | 0.131 | 8892 | 39 | | | | 3 | 3.4e10 | 1.0e8 |
| (15,6,54) | 3 | S | 11662 | 0.236 | 0.073 | 855 | 70 | 19% / 10.4% / | 1, 77, 2670 | 0.71% | 2 | 2.7e7 | 2.3e6 |

(Complete table, every draw: `data/profile_table.md`. The rank is the closure rank; at degree 3 every cell gives
0.24 to 0.34 for `l >= 4` and 0.69 to 0.74 at `l = 3`.)

Reading.
- **The closure basis is high rank.** A satisfiable draw's saturated degree-4 closure has rank 0.68 to 0.77 of the
  columns at `l = 5` and **1.000 at `l = 3, 4`** (every column pivots, 111,930 of 111,930 in the top block). It
  is not low rank. (At rank = columns the reduced basis is nearly the identity, 0.005% to 0.009% nonzeros: the
  cost is in the elimination that gets there, not in the final matrix; at `l = 5` the basis has 1% density.) (An earlier version of this note quoted 0.63 to 0.75 from the recorded rows; the profile
  runs give 0.68 to 1.00, so that figure was a lower bound for the cells it covered, not a property of the matrix.)
  The unsatisfiable draws collapse to the single row `1`.
- **The Macaulay input is sparse and low rank, the closure is not.** Input rows weigh `rho` = 32 to 78 terms
  (grows with `n` and `l`; `rho / n` = 3.3 to 5.0), `E = 12` to 14 generators (about `n + 1`), plain XL rank is 0.07 to 0.13 of the columns and
  the rows are full rank (rank = rows_in). At `l = 5` the closure rows have 24 to 27 times the weight
  (about 1% of the columns, 1,400 terms against 55): fill-in.
- **The closure is over 99.7% of the work.** At degree 4 the plain Macaulay reduction costs 1.0e8 to 3.6e8 word
  operations and the closure 3.2e10 to 7.2e11, a factor of 340 to 2500. This is where an exponent has to be
  found, not in the elimination of the Macaulay matrix.
- **Subspace columns are 22% of the degree-2 columns, falling to 1.6% to 4.3% at degree 4.** Almost every column
  touches an intermediate-point coordinate, so the "products of linear forms in the subspace" structure is
  confined to the low blocks and does not describe the matrix where its mass is (d4 holds about 90% of the columns).

## 3a. Can a structured / block-Wiedemann elimination push w below 2? (measured inputs, extrapolated cost; nothing implemented)

A Krylov / block-Wiedemann solve of the plain XL matrix of `r` independent rows and `nnz = r rho` entries costs
about `2 (r / 64) nnz` word operations (block width 64, two sided; the iteration count is the dimension, so the rank
fraction does not help). Measured inputs, degree 4, unsatisfiable draws (plain XL does **not** refute here):

| cell | rows r | rho | nnz | Krylov `2(r/64)nnz` | dense Macaulay reduction | closure (what refutes) |
|---|--:|--:|--:|--:|--:|--:|
| (11,5,42) | 10776 | 55 | 5.97e5 | 2.0e8 | 2.4e8 | 1.2e11 |
| (12,5,44) | 12843 | 61 | 7.91e5 | 3.2e8 | 3.5e8 | 6.2e11 |
| (13,4,42) | 12728 | 56 | 7.16e5 | 2.8e8 | 2.9e8 | 7.2e11 |
| (12,3,36) | 8746 | 39 | 3.5e5 | 9.5e7 | 1.0e8 | 3.4e10 |

- **At the measured sizes a sparse solver gains nothing over the dense Four Russians reduction of the same
  matrix** (ratio 0.8 to 1.0): at `r` = 9 to 13 thousand the two costs are equal. The sparse gain only appears
  in the growth rate, `r^2 rho` against `cols^w`, and `r` is `cols / 2^3` to `cols / 2^3.5` here.
- **The route only pays if plain XL refutes**, which it does not at the degree where the closure does: 0 of 136
  committed unsatisfiable draws (m = 3 and 4) refuted under plain XL at degree 4, against 126 under the closure.
  Plain XL is therefore at least one degree behind the closure (degree 5 plain XL is unmeasured: the 10
  m = 4 rows at degree 5 are all satisfiable and cannot refute).

Extrapolation to the real row (`n = 131, l = 34, N = 398`, `E = n + 1 = 132`, `rho = 4 n` to `6.4 n` (measured `rho / n` is 3.3 to 5.0; the top end is a hedge); `rows = E C(N, <=D'-2)`, `cost = 2 (rows/64) rows rho`):

| plain-XL degree D' | rows | columns | Krylov word ops | oracle after conv (-8.4) | row, bits over rho |
|---|--:|--:|--:|--:|--:|
| 6 (closure degree 5 plus the measured one-degree lag) | 2^37.0 | 2^42.3 | 2^78.0 to 2^78.7 | 2^69.6 to 2^70.3 | **+42.8 to +43.5** (+51.2 to +51.9 with no conversion) |
| 5 (only if plain XL refuted at the closure's degree, contradicted by the 0 / 136) | 2^30.4 | 2^36.3 | 2^64.8 to 2^65.5 | 2^56.4 to 2^57.1 | +29.6 to +30.3 (+38.0 to +38.7 with no conversion) |

Total uses the same pricing as section 2: `log2(2^(34 + oracle) + 2^70)` against rho `2^60.81`. The conversion
(-8.4, `k_conversion_20261018`) was measured for the Four Russians kernel; a sparse matrix-vector kernel is
cache-unfriendly and its Ir per word operation was not measured, so the "no conversion" column is the safer
pricing. The conversion range (-10.5 to -7.7) moves the D' = 6 row by about 2 bits.

What this says. A structured elimination lowers the D = 5 row from **+61.5 (dense, fitted w) to about +43**, an
18-bit gain, but not through a lower exponent: the Krylov cost is `rows^2 rho / 32` with `rows = cols / 2^5.3`, so
it is an exponent 2 in `rows` with a constant. It does not reach w below 2 in the sense of a better exponent, and
it stays above the +30 line unless plain XL refuted at the closure's degree, which the data contradict.

## 4. Matrix profile per degree block

Section 3 is the table. Method: `examples/m4_matrix_profile.rs` (instrumentation in
`src/cryptanalysis/inherited_f4/profile.rs`, read-only; the one change to shared code is a thread-local trace call
in `ReducedBasis::close`) replays the committed draws as `dreg_closure` does, with the same saturating closure
(`1_000_000` rounds, as `closure_refutation_scan`) and `F4_F2_MAX_ROWS = F4_F2_MAX_COLS = 5e7`; it prints per
draw and degree the input Macaulay profile, the plain XL echelon profile and the saturated closure profile per
degree block, the per-round products and the base vs closure word ops. A first batch with `rounds = degree`
stopped before saturating at degree 4; it was discarded and every cell rerun with the saturating rounds.

## 5. Decision (F1 rule)

The rule: if the D = 5 row in rho-step units is above about +30 bits, downgrade the (4,6) cell to a
degree curiosity and do not rent memory.

- Fitted w at degree 5 is **2.83 (CI 2.59 to 2.92)**; the degree-4 proxy gives 2.54. Both are above the
  +30 crossing at w about 1.8 by 0.7 to 1.0 in w, which at 36 bits of log-columns is 25 to 36 bits.
- Repriced D = 5 row: **+61.5 bits (bootstrap 55.8 to 64.0), degree-4 proxy +52.2 (49.5 to 53.7)**, against
  the review's +37 to +49 bracket, which assumed w in 2 to 2.33 with no intercept.
- The decision holds under every pricing in section 2 except w <= 1.8, which no measurement supports.
- **The structured route (section 3a) does not change it.** Block Wiedemann on the plain XL matrix prices the row
  at about **+43** (+51 with no conversion) if plain XL needs one degree more than the closure, as measured (0 of
  136 refutations at the closure's degree). It reaches the +30 line (+29.6 to +30.3) only under the assumption
  that plain XL refutes at the closure's own degree, which the data contradict, and then only with the Four
  Russians conversion applied to a sparse kernel it was not measured for. The measured Krylov cost at degree 4
  equals the dense Macaulay reduction (ratio 0.8 to 1.0), so there is no gain at the sizes that exist.

**Decision: downgrade the (4,6) cell to "degree curiosity"; do not rent memory for it.** This is a
statement about the price of the algorithm at a fitted w; it does not say the degree stays 5, and it is
not a measured row at `n = 131`.

## Files

`fit_w.py`, `reprice.py`, `plot.py`, `profile_table.py`, `plot_profile.py`, `data/` (harvest, `fit_w.json`,
`reprice.json`, `profile_*.jsonl` and `.log` per cell, `profile_table.{json,md}`), `w_fit_and_reprice.{png,svg}`,
`matrix_profile.{png,svg}`, `report.pdf`.
