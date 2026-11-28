# Relation excess with lightest-row selection, genus 3 at p = 32771, 65537, 131101 — results (2026-11-21)

Status: `[pcc:unverified]` (shared host, no independent replay, no `VALIDATION.json`: not a leaderboard or
`benchmarks/README.md` row). Protocol [`PROTOCOL.md`](PROTOCOL.md), frozen before data (`frozen.sha256`);
binary `binary.sha256`; frozen command `run_cmds.sh`; data `grid.jsonl` (preregistered grid),
`explore.jsonl` (post-hoc, below); `joined.jsonl`/`joined_explore.jsonl` (runs plus the reused rho rows);
analysis `analyze.py` -> `analysis.json`, `analysis_explore.json`; `render.py` -> `tables.md`,
`scorecard.json`, `fig_light_rows.svg`; `data.sha256`.

## Headline
The preregistered grid is a **negative result**: collecting 1.2x to 2x the relations and keeping the lightest
85 / 70 / 50 % of the peeled core's rows made the linear algebra *more* expensive, not cheaper, at every prime
(FPM_mid 1.8x to 5.2x the plain SGE(8,3) arm). The reason is in the plain arm's core: the stopping rule already
leaves a core of `cols + margin` rows (1997 rows / 1933 cols at 32771), so every extra relation becomes an extra
row in a matrix that Wiedemann pays for per row, and a `keep` fraction of the enlarged core stays far above
`cols + margin`. A **post-hoc** variant (keep the lightest `cols + margin` rows, i.e. `keep` at its floor) does
cut the linear algebra: `la_mul` 0.48 to 0.65 of the plain arm and FPM_mid 0.84 to 0.93 of it, with the gain
growing with p (below). That variant was chosen after seeing the grid, on the same 3 curves per prime, so it is
a lead to preregister on fresh instances, not a result.

## What changed in the code
* `genus_ladder::ExcessParams { extra, keep }` and `run_ic_ext_excess(inst, prm, ext, excess)`; `run_ic_ext`
  is `run_ic_ext_excess(.., None)`, so `run_ic`, every module and test built on it run the new code with `None`.
  (Deviation: not a field of `IcParams`, see Deviations.)
* Collection: at the first stop of the rule (cycles >= columns + margin, `N0` relations) collection continues
  to `ceil((1 + extra) N0)` relations from the same stream. The relations collected at the attempt are recorded
  (`excess_rows_collected`).
* Selection `select_light_rows` after the peel: sort by (weight, index), drop from the heaviest while alive rows
  exceed `max(ceil(keep rows), cols + margin)`, never a row holding a column of live weight one (the target
  column included); re-peel the survivors with `peel`; undo (fallback, `ex_fallback`) if the re-peeled core is
  shorter than `cols' + margin` or no longer holds the target column. Counted in `la_sel` (included in `la`):
  one addition per entry for the weights, one per dropped entry, the re-peel as `peel` counts it, nothing when
  nothing can be dropped; comparisons of the sort free.
* Driver `--excess E,K` (repeatable), arm suffix `+ex(E,K)`, fields `excess_rows_collected`, `rows_kept`,
  `mean_weight_before`, `mean_weight_after`, `core_nnz_before_sge`, `core_nnz_after_sge`, `core_rows_pre`,
  `core_cols_pre`, `la_sel_mul`, `la_sel_add`, `ex_fallback`; without the flag the layout is unchanged.
* Tests (`cargo test --release --lib genus_ladder::`: 23 passed, 0 failed, 2 ignored): `sge_none_reproduces_frozen_counts`
  (every frozen count, through the new entry point since `run_ic` calls it); `excess_none_and_noop_reproduce_baseline_counts`
  (the frozen (67,3) row through `run_ic_ext_excess(.., None)`, and `extra = 0, keep = 1` equals the baseline in
  trials, relations, `rel`, `cantor_mul`, `la`, `la_wied`, `la_sge`, core shape, recovered, at genus 2, 3, 4, with and
  without SGE); `excess_certifies_on_twenty_instances` (20 instances: p in {1031, 2053} x genus {3, 4}, five each,
  excess (0.5, 0.7) + SGE(8,3): every run certifies with the planted t, relations >= baseline, `sge.rows_in = core_rows`,
  `la` = sum of its parts incl. `la_sel`, and the selection dropped rows in at least one run);
  `light_rows_never_uncover_a_column_and_keep_t` (random systems: after the drop step every column held by some row
  still has a holder; the selected core consists of original rows and holds the target column).

## Identity proof (X1)
On every prime the plain `+sge(8,3)` and the unmerged arm (12 runs per prime, 36 in all) equal the earlier studies'
rows field for field except `wall_ns` (0 mismatches; `analysis.json` `x1`), the 9 re-found instance lines equal
those of `sge_g3.jsonl` (32771) and `ext_g3.jsonl` (65537, 131101) in `f`, `order`, `h`, `r`, `curve_seed`, and the
ratios to the reused rho rows reproduce the earlier ones: unmerged 0.537 / 0.745 / 0.850, SGE(8,3) 0.283 /
0.416 / 0.507. All 54 + 36 = 90 excess-arm runs of the two data files certify with the planted t, none needed a
second attempt, none triggered the fallback. The same check holds for the plain arm in `explore.jsonl`.

## Per-p tables (means over 6 runs: 3 curves x 2 seeds; FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul)
Full tables with all columns: [`tables.md`](tables.md).

| p | arm | relations | core rows before / kept | cols | mean weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid / rho [95%] |
|--:|---|--:|--:|--:|--:|--:|--:|--:|---|
| 32771 | plain SGE(8,3) | 1.33e4 | 2.0e3 / 2.0e3 | 1.93e3 | 9.55 -> 9.55 | 1.91e4 -> 2.47e4 | 1.80e7 | 1.24e8 | 0.283 [0.221, 0.359] |
| 32771 | ex(0.2,0.85) | 1.60e4 | 3.74e3 / 3.16e3 | 2.01e3 | 9.29 -> 8.63 | 2.73e4 -> 3.13e4 | 3.99e7 | 1.49e8 | 0.504 [0.389, 0.638] |
| 32771 | ex(0.5,0.7) | 2.00e4 | 6.84e3 / 4.78e3 | 2.04e3 | 8.83 -> 7.91 | 3.78e4 -> 4.03e4 | 7.72e7 | 1.85e8 | 0.872 [0.675, 1.106] |
| 32771 | ex(1.0,0.5) | 2.67e4 | 1.28e4 / 6.4e3 | 2.05e3 | 8.25 -> 7.08 | 4.53e4 -> 4.65e4 | 1.14e8 | 2.46e8 | 1.261 [0.986, 1.600] |
| 65537 | plain SGE(8,3) | 2.60e4 | 3.2e3 / 3.2e3 | 3.12e3 | 10.47 -> 10.47 | 3.35e4 -> 4.40e4 | 5.19e7 | 3.08e8 | 0.416 [0.332, 0.659] |
| 65537 | ex(0.2,0.85) | 3.11e4 | 6.33e3 / 5.36e3 | 3.22e3 | 10.12 -> 9.41 | 5.04e4 -> 5.75e4 | 1.23e8 | 3.70e8 | 0.794 [0.633, 1.242] |
| 65537 | ex(0.5,0.7) | 3.89e4 | 1.21e4 / 8.48e3 | 3.25e3 | 9.52 -> 8.52 | 7.22e4 -> 7.57e4 | 2.39e8 | 4.62e8 | 1.402 [1.115, 2.198] |
| 65537 | ex(1.0,0.5) | 5.19e4 | 2.35e4 / 1.17e4 | 3.25e3 | 8.78 -> 7.57 | 8.88e4 -> 9.02e4 | 3.53e8 | 6.15e8 | 2.037 [1.622, 3.201] |
| 131101 | plain SGE(8,3) | 5.06e4 | 5.12e3 / 5.12e3 | 5.02e3 | 11.58 -> 11.58 | 5.93e4 -> 7.86e4 | 1.50e8 | 7.64e8 | 0.507 [0.352, 0.809] |
| 131101 | ex(0.2,0.85) | 6.07e4 | 1.08e4 / 9.12e3 | 5.14e3 | 11.00 -> 10.25 | 9.35e4 -> 1.04e5 | 3.74e8 | 9.16e8 | 1.030 [0.714, 1.644] |
| 131101 | ex(0.5,0.7) | 7.58e4 | 2.16e4 / 1.51e4 | 5.16e3 | 10.18 -> 9.15 | 1.38e5 -> 1.42e5 | 7.18e8 | 1.15e9 | 1.829 [1.269, 2.899] |
| 131101 | ex(1.0,0.5) | 1.01e5 | 4.33e4 / 2.16e4 | 5.16e3 | 9.30 -> 8.02 | 1.74e5 -> 1.75e5 | 1.06e9 | 1.53e9 | 2.653 [1.841, 4.215] |

(`mean weight before` is the mean row weight of the peeled core; `nnz before SGE` is the non-zeros of the core handed to the
merge, so for the excess arms it is after the selection. The plain arm's weight does not change.) Paired FPM_mid ratio
to the plain arm (mean of per-pair ratios, hierarchical bootstrap): (0.2,0.85) 1.78 / 1.91 / 2.03, (0.5,0.7) 3.08 / 3.37 /
3.61, (1.0,0.5) 4.45 / 4.89 / 5.23 at 32771 / 65537 / 131101, every interval within +-0.07. Paired `la_mul` ratios at 131101: 2.50, 4.80, 7.07.
Rho rows are those of `scale_g3.jsonl` and `ext_g3.jsonl`, same instances, not re-run.

## Scorecard (preregistered X1–X4)
| id | prediction | outcome |
|---|---|---|
| X1 | plain arm equals the earlier studies field for field; every run certifies | **held**: 36/36 plain and unmerged rows identical but `wall_ns`, 9/9 instance lines identical, 90/90 excess runs certify |
| X2 | `mean_weight_after` falls monotonically with the amount dropped and is below the plain weight; `la_mul` of (0.5,0.7) <= 0.7 of the plain arm at 131101 | weight part **held** at all three p (131101: plain 11.58 > 10.25 > 9.15 > 8.02); `la_mul` part **falsified**: 4.80x [4.73, 4.89] the plain arm |
| X3 | best setting's FPM_mid <= 0.85 of the plain arm at 131101, the gain growing with p | **falsified**, both parts: best setting (0.2,0.85) is 2.03x the plain arm at 131101, and the ratio rises with p (1.78, 1.91, 2.03), the opposite direction of the stated expectation |
| X4 | relation phase of (1.0,0.5) <= 2.0x the plain one; net FPM_mid of (1.0,0.5) worse than (0.5,0.7) at 32771 and better at 131101 | first part **held** (1.984, 1.996, 1.998: the extra trials are exactly `extra`); second part **falsified**: (1.0,0.5) is worse than (0.5,0.7) at all three p (4.45 vs 3.08, 4.89 vs 3.37, 5.23 vs 3.61) |

My own pre-data expectation (in `PROTOCOL.md`) was that X3's 0.85 and X4's cross-over would fail and that the gain
direction would be right: the first two were right, the direction was wrong, and the size of the loss (2x to 5x
against my 0.96) was far beyond it, because I took the plain core to have slack that selection could remove.

## What the selection did
* It does what it says on weight: mean core row weight falls monotonically with the amount dropped (131101:
  11.58 plain, 10.25, 9.15, 8.02 for keeping 85 / 70 / 50 %), and the sort/selection is cheap (`la_sel` 7e4 to
  8e5 additions, 0 multiplications; a fraction of a percent of `la`). No run needed the fallback.
* It cannot offset the extra rows. The plain arm's peeled core is tight by construction: rows 1997 / cols 1933
  (32771), 3196 / 3118, 5116 / 5018, i.e. `cols + margin` (margin 8 + n/100). Extra relations add little to the
  core's columns (+2 % at (0.2), +3 % at (1.0) at 131101: 5018 -> 5138 -> 5163) but a lot to its rows (5116 ->
  1.08e4 / 2.16e4 / 4.33e4 pre-selection). Keeping a fraction `keep` of that leaves 9.1e3 / 1.5e4 / 2.2e4 rows (131101),
  1.8x to 4.2x the plain 5.1e3.
* The solver pays per row. After SGE the merged system has about as many generic (non-`±1`) entries as rows
  (`sge_generic_out` 3754 vs 3532 rows, 8352 vs 8249, 14858 vs 14786, 21613 vs 21526 at 131101), and the Wiedemann multiplications
  satisfy `la_wied_mul ~ c * cols_out * (generic_out + rows_out)` with c = 5.98 (plain), 5.29, 5.00, 4.86 at 131101
  (5.98 at all three p for plain): the model of `genus_ladder_sge_depth_20261121/model_check.md` with the row term
  dominant. Dropping the heaviest rows lowers the weight but the row count, not the weight, sets `la_mul` here; the merge
  also finds fewer merges (1581 plain, 873 / 313 / 118) because the lighter rows leave columns of higher weight.
* The linear-algebra share of FPM_mid (9 `la_mul` / FPM_mid) rises with p in every arm (plain 0.566 / 0.602 / 0.638 at
  32771 / 65537 / 131101) and with the excess (131101: 0.786 / 0.849 / 0.862), so the loss is a linear-algebra loss, not a
  relation-phase one: the relation phase costs exactly `1 + extra` (1.20, 1.50, 2.00 of the plain arm).

## Post-hoc exploration (not preregistered, not scored): `keep` at its floor
After the grid, the same driver run with `--excess 0.2,0.01 --excess 0.5,0.01` (`explore.jsonl`; `keep = 0.01` makes
the target `cols + margin` rows, so the lightest `cols + margin` rows of the enlarged core are kept). Same
instances; its plain arm again equals the earlier studies (X1 block of `analysis_explore.json`); 36/36 runs certify, 0 fallbacks.

| p | setting | relations | core rows before / kept | mean weight before -> after | la_mul / plain | relation phase / plain | FPM_mid / plain [95%] | FPM_mid / rho [95%] (plain) |
|--:|---|--:|--:|--:|---|---|---|---|
| 32771 | ex(0.2,min) | 1.60e4 | 3.74e3 / 1.88e3 | 9.29 -> 7.53 | 0.630 | 1.200 | 0.877 [0.871, 0.881] | 0.248 [0.194, 0.315] (0.283) |
| 32771 | ex(0.5,min) | 2.00e4 | 6.84e3 / 1.86e3 | 8.83 -> 6.60 | 0.497 | 1.492 | 0.928 [0.914, 0.940] | 0.263 [0.203, 0.334] |
| 65537 | ex(0.2,min) | 3.11e4 | 6.33e3 / 3.04e3 | 10.12 -> 8.09 | 0.645 | 1.200 | 0.866 [0.860, 0.875] | 0.361 [0.287, 0.568] (0.416) |
| 65537 | ex(0.5,min) | 3.89e4 | 1.21e4 / 2.98e3 | 9.52 -> 6.95 | 0.476 | 1.499 | 0.883 [0.876, 0.892] | 0.368 [0.293, 0.578] |
| 131101 | ex(0.2,min) | 6.07e4 | 1.08e4 / 4.91e3 | 11.00 -> 8.69 | 0.641 | 1.199 | **0.843 [0.832, 0.852]** | 0.428 [0.297, 0.678] (0.507) |
| 131101 | ex(0.5,min) | 7.58e4 | 2.16e4 / 4.79e3 | 10.18 -> 7.35 | 0.477 | 1.499 | 0.847 [0.840, 0.855] | 0.430 [0.298, 0.682] |

Read with the preregistered predictions as the yardstick: X2's weight and `la_mul` parts and X3's 0.85 would have held for
this variant (0.843 at 131101; `la_mul` 0.48 to 0.65), and the gain grows with p (FPM_mid / plain 0.877, 0.866, 0.843 for
(0.2,min)); at (0.5,min) the larger relation cost (1.5x) erases the gain at the smaller primes (0.928, 0.883, 0.847).
Distance to the matched rho at 131101 falls from 0.507 to 0.428 (FPM_mid, 95% interval [0.297, 0.678]; the interval of the
ratio to rho is wide because rho has 6 runs, the paired interval against the plain arm is tight because both arms share the
instances and the stream). Not claimed: that this is an improvement over the baseline. It was found on the grid's own data,
at one SGE setting, 3 curves per prime, with the relation excess chosen from {0.2, 0.5} only; the interval widths are the
bootstrap's, not a test against selection after the fact. The next step is a preregistered run on fresh instances with
`keep` at the floor and `extra` in {0.1, 0.2, 0.3}, at 262147 as well (where the LA share is highest).

## Deviations from the task text
1. `ExcessParams` is not a field of `IcParams`; it is an argument of the new `run_ic_ext_excess` (declared in the
   protocol before data). `IcParams` is built by struct literal in `genus_ladder_sieve.rs`, `genus_ladder_harvest.rs` and
   `bin/smith_transfer_cost.rs`, which this task does not own; a field there is a one-line `excess: None` in each,
   if the owner prefers it.
2. The excess starts at the rule's first stop (cycles >= columns + margin), not at the first stop whose peeled core is
   large enough; the baseline's retry after a short core is therefore counted from the extended row count. A short core
   did not occur in any run.
3. The selection is undone when the re-peel leaves the target column uncovered (an extra guard found by a test: a
   re-peel can drop the last row of the target column through another column of weight one); no run hit it.
4. Added post-hoc exploration (`keep` at its floor), reported separately and outside the scorecard.
5. FPM_mid/rho ratios and intervals are computed by `analyze.py` (ratio of means, hierarchical bootstrap) and not by
   `genus_ladder_analyze`; they reproduce the earlier published values of the plain and unmerged arms to the printed digits.

## Limits
Shared 4-vCPU host (no walls used); 3 curves x 2 seeds per prime, so the ratio-to-rho intervals are wide; no independent replay,
no validation gate; genus 3 only; one SGE setting (8,3); rho rows reused, not re-measured; the `keep` fractions
0.85 / 0.7 / 0.5 and extras 0.2 / 0.5 / 1.0 were fixed in advance and none of them is the setting that helps; the explored
`keep` floor is post-hoc; `rustfmt` clean for `genus_ladder.rs` and `src/bin/genus_ladder.rs`; clippy: see the report.

Files over 60 KB are stored gzipped (`*.jsonl.gz`); `data.sha256` lists the hashes of the decompressed files. `analyze.py` and `render.py` read `grid.jsonl`/`explore.jsonl`: `gunzip -k` them first.
