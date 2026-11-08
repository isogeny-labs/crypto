# Pricing the table oracles' per-step inserts in the §5a band (2026-11-08)

**Class.** Accounting note and tool change. Nothing was measured: `ecbench band` was re-run on
the committed sessions of two studies with the binary built from this branch. No pcc claim has
been filed, so every number below is an unverified assertion until it is reviewed
[pcc:unverified].

## The asymmetry

The band of AGENTS.md section 5a charges, at weight w, every counter a record names in
`cost.unpriced`, looked up in `counters` and then in each phase's `native` map
(`src/cryptanalysis/ecbench/band.rs`, `unpriced_counts`). Prime-field rho-neg names its per-step
negation canonicalisation there (`canonicalisations_uncharged`, about one per step), so its
per-step native work is in U and is charged at w > 0 [pcc:unverified].

The index-calculus table oracles (`wwalk` and the pair tables: the 11 oracles in
`src/cryptanalysis/ic_framework/plugins.rs` that call `table_counted()`) do one hash-table insert
per walk step, and record it only as `work_native.table_insert_attempts` behind the marker
`work_native["counted:table_insert_attempts"] = 1`. `work_native` feeds the bitop unit
(`src/cryptanalysis/ecbench/work.rs`) and nothing else; the counter is never listed in
`cost.unpriced`, so the band never charged it [pcc:unverified]. Each arm does one native operation
per step beside its group addition, and the band priced rho's and not the walk's. That is why the
walk-table arms read as "unresolved inside the band" against rho-neg, or below it at w = 1, in
`research/curve25519_f1_targetwalk_20261108` (README section 5, `scripts/sensitivity_inserts.py`)
and `research/curve25519_f1_walk_20261107` (README, "Limits") [pcc:unverified].

## The rule, as implemented

`band::Pricing { table_work: bool }`, default off. With `table_work` on, the phase sum of each
counter in `band::TABLE_WORK_COUNTERS` (today `["table_insert_attempts"]`) is added to U, when and
only when some phase of the record carries the matching `counted:<name>` marker; a count without
its marker is not known to be complete and is not charged. Everything else in the price is as
before: `priced_gae(w) = G - C + w (U + n_C)`. The `_with(pricing)` variants of `priced_counts`,
`unpriced_total`, `priced_gae`, `priced_s` and `band` take the option; the existing functions keep
their signatures and behaviour. On the command line: `ecbench band --price-table-work` and
`ecbench compare --price-table-work`.

What does not change:

- The record format. The price is a reading of the committed `work_native` maps. `cost.unpriced`,
  the phase `native` maps and `work_native` are written as before, so the replay audit
  (`audit.rs`, `compare_replay`) compares what it compared, and committed sessions keep replaying
  "identical".
- The default band. `ecbench band` without the flag reproduces the committed band files: the test
  `default_band_reproduces_the_committed_targetwalk_band_file` checks k20-s0 of the targetwalk
  study against `data/band-k20-s0.json` to 1e-9 per arm and weight, and the default JSON keeps
  its keys (one addition: a band row always lists `table_work_mean_counts`; the report's
  `priced_table_work` and a comparison's `priced_table_work` appear only when the flag is on).
- The schema name, `ecbench.band/v1`.

Tests (`src/cryptanalysis/ecbench/band.rs`): the default-band reproduction above; with
`table_work` on, rho-neg's row is unchanged and twcj-x15's S(w) rises by exactly
w · mean(table_insert_attempts) / sqrt(r), with the rho-neg vs twcj-x15 verdict going from
"unresolved inside the band" to "rho-neg < twcj-x15"; and a synthetic record with the counter and
no marker is not charged.

## Outputs

`scripts/run.sh` runs `ecbench band --ranks` with and without `--price-table-work` on the pooled
sessions `k20-s*` and `k24-s*` of each study (512 verified runs per arm and size) and writes
`data/band-<study>-k<k>[-table-work].{txt,json}`; `data/binary.sha256` and
`data/source-commit.txt` name the binary and the source commit the sessions were read with
[pcc:unverified]. Mean S over the verified runs; the ratios are of the two arms' mean S, not the
paired estimator of the studies' `pooled_compare.py`, so they differ from the studies' pooled
ratios in the third digit [pcc:unverified].

| study | k | arm | runs | inserts per run (per sqrt r) | S(w=1) default | S(w=1) priced | arm / rho-neg at w=1, default | arm / rho-neg at w=1, priced | arm / rho-neg at w=0 |
|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|
| targetwalk 20261108 | 20 | f1-x07 | 512 | 366.5 (1.012) | 2.976 | 3.988 | 1.133 | 1.518 | 1.536 |
| targetwalk 20261108 | 20 | tw-x15 | 512 | 511.6 (1.412) | 2.787 | 4.199 | 1.061 | 1.599 | 1.791 |
| targetwalk 20261108 | 20 | twcj-x15 | 512 | 503.3 (1.390) | 2.183 | 3.573 | 0.831 | 1.360 | 1.396 |
| targetwalk 20261108 | 24 | f1-x07 | 512 | 1459.2 (1.008) | 2.421 | 3.429 | 1.110 | 1.572 | 1.589 |
| targetwalk 20261108 | 24 | tw-x15 | 512 | 1995.3 (1.378) | 1.802 | 3.180 | 0.826 | 1.458 | 1.557 |
| targetwalk 20261108 | 24 | twcj-x15 | 512 | 2018.8 (1.394) | 1.634 | 3.028 | 0.749 | 1.388 | 1.411 |
| walk 20261107 | 20 | f1-x05 | 512 | 332.2 (0.917) | 2.979 | 3.896 | 1.115 | 1.458 | 1.479 |
| walk 20261107 | 20 | f1-x07 | 512 | 375.7 (1.037) | 2.894 | 3.931 | 1.083 | 1.471 | 1.496 |
| walk 20261107 | 20 | f1-x09 | 512 | 387.6 (1.070) | 2.861 | 3.931 | 1.071 | 1.471 | 1.502 |
| walk 20261107 | 20 | f2-inc | 512 | 495.7 (1.368) | 3.330 | 4.698 | 1.246 | 1.759 | 1.849 |
| walk 20261107 | 24 | f1-x05 | 512 | 1313.4 (0.907) | 2.495 | 3.402 | 1.128 | 1.538 | 1.556 |
| walk 20261107 | 24 | f1-x07 | 512 | 1452.5 (1.003) | 2.429 | 3.432 | 1.098 | 1.551 | 1.570 |
| walk 20261107 | 24 | f1-x09 | 512 | 1567.9 (1.083) | 2.402 | 3.484 | 1.086 | 1.575 | 1.596 |
| walk 20261107 | 24 | f2-inc | 512 | 2040.8 (1.409) | 2.540 | 3.949 | 1.148 | 1.785 | 1.836 |

Rho-neg's rows (S(w=1) 2.626 and 2.181 in the targetwalk sessions, 2.672 and 2.212 in the walk
sessions) are identical with and without the flag, because it counts no table work
[pcc:unverified]. The w = 0 and w = 0.1 columns of rho-neg are unchanged too; at w = 0 every arm
is unchanged [pcc:unverified].

Read across the table: the walk-table arms do about one insert per sqrt(r) (f1) to 1.4 per
sqrt(r) (tw, twcj, f2), which is the same order as rho-neg's canonicalisations (1.03 to 1.10 per
sqrt(r), README section 5 of the targetwalk study), so pricing one side and not the other moves
the w = 1 ratio to rho by 0.4 to 0.6 [pcc:unverified]. With the inserts priced, every walk-table
arm is above rho-neg at every weight, and the distance at w = 1 is within 0.03 to 0.2 of the
w = 0 distance instead of 0.4 to 0.7 below it [pcc:unverified]. In the targetwalk study the
twcj-x15 / rho-neg ratio at w = 1 goes from 0.831 to 1.360 at k = 20 and from 0.749 to 1.388 at
k = 24, which are the post hoc figures of that study's README section 5 (1.360 [1.298, 1.426] and
1.388 [1.307, 1.474]) [pcc:unverified].

Verdicts that change (`data/*-table-work.txt` against `data/*.txt`); every other pair keeps its
verdict [pcc:unverified]:

| study | k | pair | default | priced |
|:--|--:|:--|:--|:--|
| targetwalk 20261108 | 20 | rho-neg vs twcj-x15 | unresolved inside the band | rho-neg < twcj-x15 |
| targetwalk 20261108 | 20 | f1-x07 vs tw-x15 | unresolved inside the band | f1-x07 < tw-x15 |
| targetwalk 20261108 | 24 | rho-neg vs tw-x15 | unresolved inside the band | rho-neg < tw-x15 |
| targetwalk 20261108 | 24 | rho-neg vs twcj-x15 | unresolved inside the band | rho-neg < twcj-x15 |
| walk 20261107 | 20 | f1-x05 vs f1-x07 | unresolved inside the band | f1-x05 < f1-x07 |
| walk 20261107 | 20 | f1-x05 vs f1-x09 | unresolved inside the band | f1-x05 < f1-x09 |
| walk 20261107 | 24 | f1-x05 vs f1-x07 | unresolved inside the band | f1-x05 < f1-x07 |
| walk 20261107 | 24 | f1-x05 vs f1-x09 | unresolved inside the band | f1-x05 < f1-x09 |
| walk 20261107 | 24 | f1-x07 vs f1-x09 | unresolved inside the band | f1-x07 < f1-x09 |

The within-walk orderings (f1-x05 < f1-x07 < f1-x09 at w = 1 once inserts are priced) follow from
the larger table of the larger x, and are mean-S orderings of the band, not the paired intervals
of the walk study, which found those pairs unresolved [pcc:unverified]. The two studies' headline
verdicts against their own incumbents (twcj-x15 < f1-x07, f1 < f2-inc) hold under both pricings
[pcc:unverified].

## Limits

- Pricing the inserts is still a simplification in the walk's favour: each walk step also does a
  sign canonicalisation inside `PrimeCurve::key`, an RNG draw and mod-r additions, none counted
  or listed; rho-neg's partition hash is unlisted too (targetwalk README section 5). The priced
  ratios are a lower bound on the walk arms' distance to rho at w > 0 [pcc:unverified].
- `TABLE_WORK_COUNTERS` names one counter. Other `work_native` counters behind `counted:` markers
  (`index_inserts`, `gauss_entries_scanned`, `row_build_*`, `lookups` in `oracle_setup`) are not
  priced by this option; they are the bitop unit's.
- The option is opt-in. Which reading is the §5a band is the owner's ruling (proposed wording in
  the session report); until then a note that quotes a walk-table arm's distance to rho at w > 0
  should give both.

## Files

- `scripts/run.sh`: the commands (sets `BENCHREC_SKIP`; this is analysis of committed sessions).
- `data/band-{targetwalk,walk}-k{20,24}.{txt,json}`: default band, pooled sessions.
- `data/band-{targetwalk,walk}-k{20,24}-table-work.{txt,json}`: with `--price-table-work`.
- `data/binary.sha256`, `data/source-commit.txt`.
- Code: `src/cryptanalysis/ecbench/band.rs`, `src/cryptanalysis/ecbench/compare.rs`,
  `src/bin/ecbench.rs`; docs: `docs/ecbench/README.md`.
