# Preregistration: does plain XL at degree 5 refute the chained system at m·ℓ = 21?

Lead L2 of `research/dreg_closure_slope_20261018/`. Class: stage diagnostic.
No `S`, no rho ratio, no speed claim. Written before any registered cell ran.

## Question

Is the constant `1` in the row span of {equation × monomial : degree of the
product ≤ 5} (x_i² = x_i, plain XL: no MutantXL closure, no fall products) on
the committed `m·ℓ = 21` draws: m = 3, (n, ℓ) = (19, 7), N = 40, and
(21, 7), N = 42?

Logic. XL refuting at 5 implies the closure refutes at ≤ 5; with the
committed closure degree `> 4` (`dreg_closure_slope_20261018/RESULTS.md`) the
closure degree of that draw is exactly 5. XL not refuting at 5 says nothing
about the closure: recorded as "XL degree ≥ 6, closure undecided".

## Draws (frozen)

Unsatisfiable draws of `research/dreg_closure_scale_20261016/draws/`
(solutions = 0 by exact count): `m3-19-7.jsonl` draws 0, 3, 5, 7;
`m3-21-7.jsonl` draws 0, 2, 3, 6. Same bases, `x(R)`, `b = 1`, built by
`build_decomposition_system`.

## Instrument

`examples/xl_dense_degree.rs` (new). Rows = every equation × every monomial
of degree ≤ 5 − deg(equation), equal terms cancelling, as
`koblitz_groebner::visit_macaulay_rows` builds them. Columns = monomials that
occur, descending (degree, mask), constant last. Exact preprocessing:
rows holding a column no other row holds are peeled (they cannot be part of a
combination equal to `1`; each adds exactly 1 to the rank). Dense `u64` bit
rows, incremental echelon (lowest-set-bit pivot, pivots stored from their
leading word on), batches of 64 reduced in parallel on ≤ 2 threads then
inserted in order, AVX2 XOR when present. `1` ∈ span exactly when a row
reduces to the constant alone: stop then (`refuted`). Native scorer: the
`score` subcommand of the same file.

Unit tests (`cargo test --release --example xl_dense_degree`, passing before
this file was committed): the engine agrees with `closure_refutation_scan`'s
XL verdict and rank on 48 random systems of 6–10 unknowns at degrees 2–4, with
and without peeling and early stop, over both refuted and open cases; a
planted-solution system is never refuted; the committed chained draw
`dreg_ell_grid_20260925/runs/cell-10-2-7.jsonl` draw 0 (n = 10) gives open at
4 and refuted at 5, matching its committed XL degree 5.

## Size, measured before registering (count only, no elimination)

| cell | N | rows | columns occurring (C(N,≤5)) | dense bytes, rows × cols/8 |
|---|--:|--:|--:|--:|
| (19, 7) draw 0 | 40 | 218,918 | 619,790 (760,099) | 16.95 GB |
| (21, 7) draw 0 | 42 | 279,048 | 778,394 (974,982) | 27.14 GB |

The brief's 34,482 rows (= 42 × C(40, ≤2)) assumed every equation has degree
3; the systems carry lower-degree equations, so the real count is 218,918 and
the dense matrix is 17 GB, not 3.3 GB. Trimmed pivots store about 61% of the
dense size (smoke below), so N = 40 is expected near 10 GB of pivots and
N = 42 near 16 GB.
Memory rule (hard): `prlimit --as=8000000000`, pivot store stopped at 6.3 GB
(`--max-bytes`), one process, `RAYON_NUM_THREADS=2`, `timeout 10800`. A run
that reaches the cap without a verdict is **unmeasured**; limits are not
raised.

## Smoke tests disclosed (not registered cells; no registered draw was eliminated)

1. Tiny systems and the n = 10 draw, in the unit tests above.
2. `m3-16-6.jsonl` draw 0 (m = 3, (16, 6), N = 34, unsatisfiable), degree 5,
   full run: rows 114,816, columns 274,326, rank 108,150, **not refuted**,
   71.8 s, pivots 2.40 GB (`runs/smoke-m3-16-6.d0.D5.jsonl`). Used for speed
   and the 61% memory ratio.
3. Count-only size runs on registered draw 0 of each cell and on
   `m3-18-6.jsonl` draw 0 (`runs/sizes-count-only.jsonl`).

## Verdict rules

Per draw (degree 5):
- **refuted at 5**: status measured, `refuted: true` (closure degree is exactly 5).
- **not refuted at 5**: the run finished all rows, `refuted: false`
  ("XL degree ≥ 6, closure undecided").
- **unmeasured**: cap, timeout or kill; no reading.

Per cell: the count of each over its four draws. The cell reads "refuted at
5" only if all four measured draws are refuted and none unmeasured; "not
refuted" likewise; otherwise "mixed/partial" with the counts.

## Controls (run with the same binary; any failure voids the registered results)

- Positive, degree 5: `dreg_ell_grid_20260925/runs/cell-12-2-7.jsonl` and
  `cell-10-2-7.jsonl`, unsatisfiable draws 0–3 (m = 3, ℓ = 2, N = 18 and 16,
  committed XL degree exactly 5): open at degree 4, refuted at 5.
- Positive, degree 6: `cell-5-3-7.jsonl` draws 9, 14 (m = 3, ℓ = 3, N = 14,
  committed XL degree 6): open at 5, refuted at 6.
- Soundness: `cell-4-3-7.jsonl` satisfiable draws 0–3 at degrees 5 and 6, and
  the satisfiable draws of the registered cells (`m3-19-7` 1, 2, 4, 6; `m3-21-7`
  1, 4, 5) at degree 5: must never be refuted (those at N ≥ 40 may be
  unmeasured).

## Predictions (made before the registered runs)

- P1. N = 40, degree 5: at least 3 of 4 draws end unmeasured at the pivot cap
  (6.3 GB). Confidence 75%.
- P2. No draw of either cell is refuted at 5 (the N = 34 smoke is not).
  Confidence 85%.
- P3. All four N = 42 draws unmeasured. Confidence 90%.
- P4. All controls pass. Confidence 95%.
- P5. If a draw is measured it is "not refuted". Confidence 85%.

## Stop conditions

Per process: 10,800 s, 8 GB address space, 6.3 GB pivots. After the cells:
stop. If a refutation at 5 occurs, report it and the derived point/slope (see
task); no further degrees are run.

## Reporting

`RESULTS.md`, and `reports/run-<UTC start>/` with graph and PDF, from the
recorded lines only. Python only lays out recorded values.
