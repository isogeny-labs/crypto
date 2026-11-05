# rho-neg's sub-½ 16–26 exponent: set-up accounting, not the walk

Study frozen in [PROTOCOL.md](PROTOCOL.md) (commit 2c50f0614) before any
number here was computed; run by [run.sh](run.sh) with a release
`ecbench` built from this branch (source identical to commit 1433dd7ac).  Inputs: the committed hold-out sessions
`research/bitop_ladder_20261019/sessions/holdout-P16 … P40`, unmodified.
Analysis only, no new measurement.

## Verdict (PROTOCOL.md rule, applied mechanically)

**Yes: an accounting artifact.**  Positive control passes (6/6); over
16–26 rho-neg's total-basis exponent excludes ½ and its search-basis
exponent contains ½.

| rho-neg, 16–26 bits | α (additions) | 95% CI |
|---|---|---|
| total (`bitop/v1`) | 0.413 | [0.388, 0.447] |
| per-target (`bitop/v1+per_target`) | 0.413 | [0.388, 0.447] (= total, as stated in advance) |
| search (`bitop/v1+search`) | 0.477 | [0.455, 0.506] |

Mechanism: non-search work (set-up, verification, natives) is 36.6% of
rho-neg's mean total at 16 bits, falling to 9.3% at 22, stepping back to
14.0% at 23 (jump table 8 → 16) and 6.5% at 26.  A share that shrinks with
r flattens the total's slope; the search phase alone scales as ½.
Figures: `figures/rho_neg_cost_by_basis.svg`, `figures/alpha_forest.svg`,
`figures/ic_rho_ratios.svg`, drawn natively from the committed `analysis/`
files by `cargo run --release --example ic_acct2_figures --
research/ic_acct2_setup_20261021` (`examples/ic_acct2_figures.rs`; it reads
the sealed bounds and the `amortize` reports, recomputes nothing, and renders
each SVG to its 150 dpi PNG with `rsvg-convert`); report
[REPORT.pdf](REPORT.pdf) (`REPORT.typ`).

The total reproduces the committed bound
(`research/bitop_ladder_20261019/analysis/bounds/rho-neg-16-26.json`).

## Positive control: search basis, fixed-jump ranges

| arm | 16–22 | 23–32 |
|---|---|---|
| rho-neg | 0.497 [0.422, 0.583] | 0.499 [0.461, 0.529] |
| bsgs-neg | 0.498 [0.420, 0.579] | 0.487 [0.426, 0.554] |
| kangaroo | 0.519 [0.387, 0.661] | 0.462 [0.380, 0.510] |

All six intervals contain 0.5.

## All fits (α in additions, 95% CI)

| arm | basis | 16–22 | 23–32 | 16–26 | 33–40 |
|---|---|---|---|---|---|
| rho-neg | total | 0.390 [0.316, 0.476] | 0.473 [0.433, 0.507] | 0.413 [0.388, 0.447] | 0.504 [0.449, 0.575] |
| rho-neg | per-target | 0.390 [0.316, 0.476] | 0.473 [0.433, 0.507] | 0.413 [0.388, 0.447] | 0.504 [0.449, 0.575] |
| rho-neg | search | 0.497 [0.422, 0.583] | 0.499 [0.461, 0.529] | 0.477 [0.455, 0.506] | 0.506 [0.451, 0.576] |
| bsgs-neg | total | 0.494 [0.469, 0.518] | 0.496 [0.479, 0.517] | 0.492 [0.479, 0.507] | 0.497 [0.475, 0.520] |
| bsgs-neg | per-target | 0.506 [0.434, 0.579] | 0.488 [0.434, 0.549] | 0.489 [0.448, 0.536] | 0.488 [0.431, 0.544] |
| bsgs-neg | search | 0.498 [0.420, 0.579] | 0.487 [0.426, 0.554] | 0.487 [0.445, 0.535] | 0.491 [0.429, 0.552] |
| kangaroo | total | 0.496 [0.376, 0.621] | 0.460 [0.378, 0.507] | 0.526 [0.462, 0.578] | 0.491 [0.401, 0.576] |
| kangaroo | per-target | = total | = total | = total | = total |
| kangaroo | search | 0.519 [0.387, 0.661] | 0.462 [0.380, 0.510] | 0.541 [0.477, 0.593] | 0.492 [0.402, 0.577] |
| ic-pair-rule | total | 0.520 [0.487, 0.564] | 0.637 [0.623, 0.647] | 0.558 [0.530, 0.588] | 0.646 [0.629, 0.659] |
| ic-pair-rule | per-target | 0.460 [0.366, 0.551] | 0.689 [0.641, 0.737] | 0.494 [0.448, 0.533] | 0.693 [0.637, 0.772] |
| ic-pair-rule | search | 0.462 [0.367, 0.554] | 0.689 [0.641, 0.737] | 0.496 [0.450, 0.534] | 0.693 [0.637, 0.772] |

Sealed bounds and fit logs: `analysis/bounds/<arm>-<basis>-<lo>-<hi>.json`.
No fit was refused.  `run.sh` pins `--fit-version 1`, the procedure these
bounds were fitted with (main's default became version 2, which also
requires an audit receipt per session).  The 24 per-target and search
bounds whose stage table changed were refitted after a fix to
`work::basis_view`: a partial basis now keeps only the phases it counts, so
`stages[].share_of_gae` sums to 1 (before, e.g. `bsgs-neg-search-16-22`
listed set-up at 101.56% and search at 100% of the search total).  Their
fits, sizes and constants are unchanged; only `stages` and `bound_id`
moved.  The total-basis bounds, and rho-neg's and kangaroo's per-target
bounds (no reusable phase), re-derive byte for byte.

## Jump-count check

`analysis/rho-neg-jumps.tsv`: rho-neg ran with 8 jumps on every record of
16–22 bits and 16 on every record of 23–40 bits (80 records per size), as
`rho_jumps_for` predicts.  The stored set-up cost roughly doubles at 23
bits (5.15M → 13.1M gates), which is the step the 16–26 total fit absorbs.

## Amortization: stored vs target-aware split

`analysis/amortize/` (`ecbench amortize`, ic-pair-rule vs rho-neg, 16–40):
`summary.tsv` per curve.  Under the stored (pre-branch) split rho-neg was
credited 2.6M–42.0M reusable gates per curve; under the target-aware rule
it has 0.  IC's reusable work is unchanged.  Consequences:

- `cold_ratio` (total IC / total rho) is unchanged: 6.23 at 16 bits to
  78.3 at 40 bits.
- `per_target_ratio` (IC per-target / rho per-target) **falls**, because
  rho's per-target cost rises to its full total: 6.36 → 4.41 at 16 bits,
  8.65 → 8.11 at 25, 58.94 → 58.92 at 40.  The correction matters most at
  small r, where set-up is a larger share.
- `break_even_window` is null on every curve under both splits: IC never
  amortizes below rho within the model's target range.

This is an accounting correction, not an IC improvement (CLAUDE.md:
"Relabelling and accounting changes are not improvements"); the gap to
matched rho is unchanged in cold, total terms.
