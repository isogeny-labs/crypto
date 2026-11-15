# Genus ladder, follow-up 2: structured Gaussian elimination before Wiedemann (frozen protocol, 2026-11-15)

Follow-up to `research/genus_ladder_20261114/` (next step 1 of its RESULTS.md: "the
elimination before the Wiedemann step") and `research/genus_ladder_bound_r_20261114/`.
Every figure `[pcc:unverified]`. Synthetic random curves only; no deployed-curve
statement. Nothing here becomes a leaderboard or `benchmarks/README.md` row (no
isolated host, no validator replay, no matched-rho evidence directory).

## 1. Question

In the primary cross-genus reading (`FPM_mid = cantor_mul + 10 cantor_inv + rel_mul +
g^2 la_mul`) the linear algebra is about half to two thirds of the best arm's cost at
genus 3 to 5. Does classic structured Gaussian elimination (merge of low-weight
columns, as in NFS filtering) between the peel and Wiedemann lower the counted
linear-algebra multiplications, merge work included, and by how much the whole
logarithm, on **identical relations**?

## 2. Instrument

Binary: `genus_ladder` built from the working tree of branch
`claude/ecstatic-edison-cgo57l` with this stage, SHA-256 in `binary.sha256`.
`src/cryptanalysis/genus_ladder.rs`, `IcParams.sge: Option<SgeParams { w_max,
fill_max }>`, function `sge_reduce`; driver flag `--sge W,F` (repeatable) of
`src/bin/genus_ladder.rs`, which runs each index-calculus arm once without and once per
setting on the same instance and seed, the merged arm named `<arm>+sge(W,F)`.

The merge, in brief (full text in the `sge_reduce` doc comment): repeatedly the live
column of least weight `w <= w_max` other than the `t` column; weight 0 dropped; weight
1 removed with its row; weight `w >= 2` eliminated by the shortest row holding it
(pivot; ties: a `±1` coefficient first), `row_i -= (a_i / a_p) pivot` for the other
`w - 1` rows, pivot row and column leave. A merge is refused when its worst-case fill
`(w - 1)(len_p - 2) - len_p` would take the non-zeros past `fill_max` times those of
the peeled core. Then empty and duplicate rows are dropped, the columns renumbered,
and Wiedemann (unchanged, `ic_control::wiedemann`) solves the reduced system; only
`t` is needed, so no back substitution.

Counting (`la_sge`, included in `la`): the convention of `ic_control`: a modular
product with a `±1` factor is free, any other is one multiplication; inverting a
non-unit pivot coefficient costs `pivot_inversion_steps(r)` multiplications;
`combine` one addition per entry of both operands; one addition (and one
multiplication unless the multiplier is `±1`) per right-hand side; index bookkeeping
one addition per entry touched, as `peel` counts it. `la_mul` of a merged run is
`graph + peel + merge + Wiedemann` multiplications.

Identity guarantees, tested in `cargo test --release --lib genus_ladder`
(`sge_none_reproduces_frozen_counts`, `sge_keeps_relation_phase_and_t`,
`skip_draws_match_wiedemann`, `sge_preserves_t_on_random_systems`):

- `sge: None` reproduces the counts recorded from the source before the stage existed
  (16 golden runs) and the driver without `--sge` reproduces the first study's rows
  (section 6);
- with the merge the relation phase is untouched: the merge runs after the admission
  check, and the merged system is solved with a copy of the random stream while the
  stream itself is advanced exactly as the unmerged solve would advance it, so even a
  failed attempt resumes the relation phase identically.

## 3. Design

Instances: exactly the first study's admitted curves (`--max-h 8 --curves 4`, curve
search from seed 1 is deterministic; the driver re-derives the same `curve_seed`s,
which are checked against `research/genus_ladder_20261114/analysis.json` `instances`):

| p | g | curve_seeds |
|--:|--:|---|
| 1031 | 3 | 2, 8, 9, 10 |
| 1031 | 4 | 35, 59, 61, 80 |
| 1031 | 5 | 3, 39, 42, 48 |
| 2053 | 3 | 3, 21, 25, 36 |
| 2053 | 4 | 2, 24, 33, 40 |

Arm: `c` (`C_double_lp+pre`, double large prime with the parity prefilter, `cc = 2.0`),
solver seeds 1, 2, 3, without merge and with three settings `w_max in {2, 4, 8}`,
`fill_max = 3.0`. 20 instances x 3 seeds x 4 = 240 runs. Two invocations (genus 5
has only `p = 1031`):

```sh
genus_ladder run --primes 1031,2053 --genera 3,4 --curves 4 --seeds 3 --arms c \
    --cc 2.0 --max-h 8 --threads 3 --sge 2,3 --sge 4,3 --sge 8,3 --out sge_g34.jsonl
genus_ladder run --primes 1031 --genera 5 --curves 4 --seeds 3 --arms c \
    --cc 2.0 --max-h 8 --threads 3 --sge 2,3 --sge 4,3 --sge 8,3 --out sge_g5.jsonl
```

Host: the shared 4-vCPU container (other agents active); counted units are the
measurement, wall times informational only.

## 4. Analysis (fixed before data)

`paired.py` (layout and ratios of recorded numbers only): every merged run is paired
with the unmerged run of the same `(p, g, curve_seed, seed)`. Per pair the ratios
`la_mul(+sge) / la_mul` and `FPM_mid(+sge) / FPM_mid`. Statistic: the mean of the
per-pair ratios per (setting, genus), pooled over `p`, and per (setting, genus, p);
95% interval by hierarchical bootstrap (curves, then seeds within a curve; 2000
resamples, seed 20261115). The **best setting** is the one with the lowest pooled mean
`la_mul` ratio over the genus-4 and genus-5 pairs. `genus_ladder_analyze` (unchanged)
gives the cell means of each arm. Smoke runs at `p <= 257` before the freeze are
labelled smoke in RESULTS.md and are not data.

## 5. Predictions

| id | prediction | falsified if |
|---|---|---|
| P1 | relation-phase fields (`trials, candidates, prefilter_rejects, splits, relations, rels_by_lp, rel_mul, rel_add, group_ops, setup_ops, walk_ops, verify_ops, cantor_mul, cantor_inv, attempts, core_rows, core_cols, fb_cols`) identical on every pair | any pair differs |
| P2 | every run certifies, with the planted `t` | any run not certified or `ok = false` |
| P3 | best setting: `la_mul` ratio below 0.5 at genus 4 and at genus 5, with the 95% upper bound below 1 | a point estimate `>= 0.5` or an upper bound `>= 1` |
| P4 | best setting: `FPM_mid` ratio below 0.8 at genus 4 and at genus 5 | a point estimate `>= 0.8` |
| P5 | the `FPM_mid` gain grows with the genus (best setting, point estimates): ratio g5 <= g4 <= g3 | either inequality reversed |

Where the numbers come from: one smoke run before the freeze (`p = 257`, genus 3 and 4,
2 curves x 2 seeds, all three settings; labelled smoke in RESULTS.md, not data) gave
24/24 identical relation phases, 24/24 certified, and for `w_max = 8` mean `la_mul`
ratios 0.16 (g = 3) and 0.24 (g = 4), `FPM_mid` ratios 0.59 and 0.57. P3 and P4 are
set looser than that, since the larger cores at `p >= 1031` are untested. **P5 is at
risk** and is kept as the hypothesis under test: in the smoke the genus-3 and genus-4
`FPM_mid` ratios are nearly equal, and the first study reports the linear algebra's
share of `FPM_mid` falling from about two thirds at genus 3 to about half at genus 5,
which by itself would make the whole-logarithm gain shrink with the genus.

Also reported (no prediction): per cell the rows, columns, non-zeros and non-`±1`
entries before and after, merges, weight-1 removals, empty and duplicate rows, refused
merges, the merge's own multiplications and additions, and the Wiedemann share.

## 6. Reproduction check of the unmerged instrument

Before the grid, the rebuilt driver without `--sge` reruns
`--primes 67,131 --genera 2,3 --curves 4 --seeds 3 --arms AaCcR --max-h 8 --threads 2`
and every `run` row is compared field by field (all but `wall_ns`) with the same rows
of `research/genus_ladder_20261114/part_a.jsonl` (SHA-256 `32061f65…`). The count of
rows compared and of mismatches is reported in RESULTS.md. The unmerged arm of the
grid is in addition compared with the first study's `C_double_lp+pre` rows of the same
cells (part_a for `p = 1031`, part_b for `p = 2053`).

## 7. Limits

Five cells, 4 curves x 3 seeds each: small. One factor-base constant (`cc = 2.0`, the
first study's), at which the double-large-prime core is the first study's. The merge's
fill measure counts non-zeros, while Wiedemann's multiplications follow the non-`±1`
entries; the merge may therefore trade additions for multiplications, which the
counts show. The `g^2` price of a multiplication modulo `r` is the first study's
assumption, not a measurement.
