# incremental-gauss-echelon: a cheaper incremental LA solver

Host: local L0 sandbox (4 shared cores, `--cpus none --allow-busy`, no isolation; wall is indicative only).
Base: main 0e5689117. Counted ops (`row_ops`, trials, relations, GAE, bitop/v1) are deterministic, so the counts are valid at L0.
Not an L2 benchmark: no replay certificate, no validator report, no leaderboard or benchmarks/README.md row.

## What changed
`IncrementalEchelon` (`src/cryptanalysis/ic_framework/linalg.rs`), registered as `incremental-gauss-echelon` in `matrix_by_name` and `MATRIX_NAMES`; `ecbench` `linalg` help updated.
`FixedMul::{new,mul,sub_from}` in `koblitz_sparse_la.rs` became `pub(crate)` (no behaviour change). `incremental-gauss`, `-full-rank`, `-full-rank-checked`, `structured-gauss`, `calibrate_row_ops` are untouched.

- Forward elimination only: a new row is scanned left to right; at a column with a pivot it is reduced by that pivot row (fill lands right of the column, so the scan reaches it); the first non-zero at a pivot-free column becomes the new pivot (normalised to 1). Nothing is eliminated from stored rows.
- Pivot rows store only the non-zeros right of the pivot (sparse list), so a reduction is O(nnz).
- Pin: `pinned(col)` = pivot row at `col` has an empty tail, O(1) via pivot-column to row map. The target column d is last, so a pivot there has an empty tail by construction.
- Inner loop `x - f*y mod m` uses Shoup's `FixedMul` (f fixed per applied pivot row; one division of set-up per application; falls back to `u128` for m >= 2^63).
- `row_ops`: one per non-zero of each pivot row applied (its pivot entry included, though this kernel cancels it structurally: a conservative over-count of 1 per application) plus 1 for the rhs; same for normalising a new pivot. Same unit as `IncrementalGauss`.

### Why the pin and the log are the same (argued, then tested)
The set of leading columns of the row space is invariant of the echelon form kept. `e_d` has leading column d, so `e_d` is in the span iff d is a pivot column, in both solvers, at the same row. The new pivot column equals `IncrementalGauss`'s: the fully reduced row and the forward-reduced row differ by a span element whose leading column is a pivot column, so neither leading column can be it; hence equal. Dependent/Inconsistent likewise (lhs in span, rhs compared). At column d no entry lies to the right, so the pivot row is `c*e_d` and the log is rhs/c (normalised: rhs).
`pinned(col)` for col != d is sufficient, not necessary (only d is queried by the pipeline). The new solver does not support the full-rank completion rules (not wired to them).

## Verification
- `cargo test --release --lib echelon` and `ic_framework::linalg`: pass (7 linalg tests incl. registry). New test `echelon_matches_incremental_gauss_row_for_row`: 1200 random sparse systems (2-3 non-zeros among 3..40 factor-base columns plus a target-column entry, moduli 101, 1000003, 2^32-5, 2^61-1, every 7th system with a corrupted rhs to force inconsistencies). Per row: identical `RowStatus`, identical `pinned(d)` (so same row index and value), identical rank; identical dependent count; the pinned value equals the planted log on consistent systems. All 1200 pass (>400 of them pin).
- `cargo fmt` (rustfmt --check) clean on touched files. Clippy: see the end of this file.

## End to end
Specs: `specs/R-p18.json` (from the exhaustive-planner spec: rho, `plan`, `m141`, each IC arm with an `_ech` twin differing only in `linalg`, 16 targets) and `specs/B-hb19.json` (rho, `plan` and twin, 16 targets instead of 8). 4 rounds + warm-up, isolation relaxed to L0.
```
target/release/ecbench run --spec research/ic_gauss_echelon_20261027/specs/R-p18.json  --out $OUT/run-R-p18  --cpus none --allow-busy --quiet
target/release/ecbench run --spec research/ic_gauss_echelon_20261027/specs/B-hb19.json --out $OUT/run-B-hb19 --cpus none --allow-busy --quiet
target/release/ecbench work --counted $OUT/run-<spec>     # bitop/v1
```
Sessions: R-p18 ECBS1hb89d17ed56f2 (400 records, all verified), B-hb19 ECBS1h7bdf21fa1d36 (240 records, all verified).

Per run, trials, relations, rank, recovered log and S are identical between each arm and its twin (checked on every record, warm-up included: True for all three pairs). Totals over the 64 measured runs per arm (16 targets x 4 rounds; counts repeat across rounds):

| spec / arm | LA row_ops gauss | LA row_ops echelon | ratio | LA wall gauss (ms) | echelon (ms) | wall ratio |
|:--|--:|--:|--:|--:|--:|--:|
| R-p18 plan (fb 30) | 13465 | 8197 | 0.609 | 1.55 | 1.25 | 0.81 |
| R-p18 m141 (fb 42) | 18426 | 10911 | 0.592 | 1.74 | 1.40 | 0.80 |
| B-hb19 plan | 13690 | 8354 | 0.610 | 1.45 | 1.29 | 0.89 |

LA share of the total: S (ecbench.gae) prices LA at 0 (row_ops is unpriced native work), so S is identical between twins by construction and LA's share of S is 0. In bitop/v1 (`ecbench work`, LA phase = row_ops-priced work incl. scans and pivot inversions; complete for all runs), share of total bitops per run:

| arm | LA bitops/run | total bitops/run | LA share |
|:--|--:|--:|--:|
| R-p18 plan, gauss | 1.489M | 113.0M | 1.32% |
| R-p18 plan, echelon | 1.040M | 112.6M | 0.92% |
| R-p18 m141, gauss | 2.137M | 111.8M | 1.91% |
| R-p18 m141, echelon | 1.458M | 111.1M | 1.31% |
| B-hb19 plan, gauss | 1.757M | 49.9M | 3.52% |
| B-hb19 plan, echelon | 1.237M | 49.4M | 2.51% |

Reading: at these sizes (|F| 30-42, 23-31 rows) LA is a small part of the total, so the whole-run saving is 0.4-1.0 points of bitops; this is a measured reduction of LA work, not a change in distance to rho. Rho references were run in the same sessions but are unchanged by this work; no gap-to-rho or 0/0.1/1 GAE band analysis was made for the new kernel.

## Caveats
- The pricing of this kernel's wall is not calibrated: `calibrate_row_ops` measures a dense multiply-subtract loop; a sparse Shoup kernel would need its own calibration. Pricing was not changed.
- The bitop/v1 LA entries use this solver's own `gauss_entries_scanned` (new-row scan only) and `pivot_inversion_steps`; the stored-row column scans of the dense matrix do not exist here, which is part of the saving.
- Wall is L0, unisolated, microsecond-scale; indicative only.
- Clippy: `cargo clippy --release --lib` gives no warnings in the touched files (one pre-existing unknown-lint warning from the command-line flags).
