# Negation-map re-measurement of v3, tower_large and cover_pushforward: results (2026-11-13)

Protocol: [`PROTOCOL.md`](PROTOCOL.md), hashed in [`frozen.sha256`](frozen.sha256) (`8771b2dca99318cd058a9c19ddc1a92d8ac7bef68aeeed630e06c2bda53cba86`)
before the first run. Code: `src/cryptanalysis/chained_transfer/negation_remeasure.rs`,
`src/bin/negation_remeasure.rs` (plus one `mod.rs` line). Data: [`remeasure.json`](remeasure.json)
(every row), [`remeasure.svg`](remeasure.svg), [`run.log`](run.log), the parts
`remeasure_v3.json`, `remeasure_cover.json`, `remeasure_tower*.json`; derived files
[`tables.md`](tables.md) (every arm-row), [`summary.json`](summary.json),
[`scorecard.json`](scorecard.json) and the scripts that make them (`render_tables.py`,
`scorecard.py`, `condensed.py`, `assemble_results.py`, `add_metrics.py`). Bench record (left
open): `benchmarks/records/20261113T0320Z-negation-remeasure.json`. Every number in this file
is a `[pcc:unverified]` assertion: no independent validator has replayed it, and no pcc claim
was filed.

## Answer

The three targets now have what the validation asked for: a negation-map rho measured in the
same process on the same curve and instance, with its canonicalisations, partition hashes, DP-table
lookups and inserts (and the walk's word operations) emitted per run, and every arm's emitted
native counters priced the same way at the 5a band and at a host-calibrated weight
[pcc:unverified]. All 162 arm-rows in 3 targets were classified. The classification
result:

- **No arm is below the matched rho at all three prices** (0 rows below). Every improved arm
  (A, B, C) and every recorded arm is `above` the negation rho in the band, except the cover
  arms that stop at the first collision and emit no counter at all, which are `above` or
  `unresolved (lower bound)` [pcc:unverified].
- **Distances grow when the reference is corrected.** The recorded plain `rho_reference` spent
  about 400 operations on set-up where the tuned negation walk spends 50 to 80, so the recorded
  distances were too small. At the largest prime of each target the recorded arm's distance
  (0 / 0.1 / 1 GAE) moves from 40.8 / 112 / 754 to
  376 / 864 / 2,353 (v3, p = 1619), from
  394 / 4,720 / 43,658 to
  2,473 / 20,667 / 51,693 (cover ic_jc, p = 12841) and from
  142 / 159,687 / 1,595,592 to
  616 / 446,318 / 995,132 (tower genus 3, p = 64007)
  [pcc:unverified]. This is an accounting change, not an improvement.
- **Recorded verdicts that change** (decision rule of the protocol): tower genus 2 at p = 251,
  `unresolved` to `above`; and in cover_pushforward the three arms that stop at the first collision
  (J(C), E1 fibre, birthday control) were recorded `below` the plain rho at p = 211 to 3203
  (15 rows, ratios 0.17 to 0.85); against the negation rho they are `above` (or
  `unresolved (lower bound)` for the birthday control at p = 211 to 1619 and for all three at
  p = 211): the recorded `below` was the start-up of the plain rho, not a property of the cover
  [pcc:unverified]. The other 48 recorded-arm rows keep their class.
- **The improved arms are a measured improvement of the index-calculus baseline** (N7): arm C
  uses 2,583 group operations at v3 p = 1619 against 34,521
  recorded, and its priced totals 2,583 / 23,440 / 211,155 are below the recorded arm's
  lower-bound totals 34,521 / 94,893 / 638,242 at all three prices; the same holds for tower
  genus 3 at p = 16001 and for cover J(C) at p = 12841 (`scorecard.json` N7). The gap to the
  matched rho remains: arm C is 28.1 / 213 / 778 times the negation rho at v3 p = 1619
  (0 / 0.1 / 1 GAE) [pcc:unverified].
- **Calibrated reading.** `w_cal` (best-of-N, unisolated VM, see Method) is 0.0068 to 0.0079 for a
  mod-p multiplication on the word genus-2 Jacobian and 0.0041 to 0.0044 on the genus-3 one; rho's
  canonicalisation and partition hash cost 0.011 to 0.016 and 0.010 to 0.015 of an elliptic
  addition. The calibrated order (0, `w_cal`, 10 `w_cal`) agrees with the band verdict for
  every improved arm and every recorded arm with an emitted counter; it differs only for the
  first-collision arms whose counters are not emitted (band `unresolved (lower bound)`,
  calibrated `above`) [pcc:unverified].

Readiness for the validation gate (the task's list), per target:

| target | negation rho in the same process | rho native counters emitted | every IC arm's counters listed (gaps flagged) | every log certified | `w_cal` with method and host | one-time transfer apart |
|--|--|--|--|--|--|--|
| v3 | yes (32 seeds per prime) | yes | yes; recorded collector is a lower bound | 104/104 IC, 256/256 rho (and the recorded rho and IC) | yes | yes |
| tower_large | yes (64 seeds x 3 instances per prime) | yes | yes; recorded `ic::solve` counters all emitted | 1020/1020 IC, 3456/3456 rho | yes | yes |
| cover_pushforward | yes (32 seeds per prime) | yes | yes; E1 arms and J(C) collector are lower bounds | 315/315 IC, 448/448 rho | yes | yes |

The binary exited 0 only when every log (IC and rho, recorded and new, and the in-process recorded
plain rho) passed `[k]G == T`, and the reproduction gates passed at every row (v3: the recorded
candidate, rho ops, IC ops, trials and LA counter of all four primes; tower: the instance
(r, #E, #J) of every instance and, where the seed counts agree, the recorded rho and IC means;
cover: the recorded group ops of four arms for every seed). Admission to the gate is the
validator's call, not this run's [pcc:unverified].

## Method (and where it departs from PROTOCOL.md)

- One process per section, 4 threads, host `Intel(R) Xeon(R) Processor @ 2.10GHz`, 4 vCPU, a
  shared VM (other agents compiled and ran tests during the runs; `run.log` records the load
  average at each launch: 1.9 to 5.9). It ran locally under the bench record, as the earlier
  instrument rerun did, not on isolab-eks, and no S3 resume checkpoint was possible (no
  upload token in this container).
- Matched rho: `rho_reference_walk`, tuned negation walk (primary) and plain walk, seeds as
  in the protocol, on the same `E`/`E1` as the IC arms; the recorded plain `rho_reference`/
  `rho_on_ec` is run in the same process as the "old" column. rho counts every group operation
  (set-up, starts, walk, verification). Native counters emitted per run: `canonicalisations`
  (the walk's counter), `partition_hashes` (derived exactly from a `key`-counting wrapper:
  all key evaluations minus the two per canonicalisation minus one per table lookup),
  `dp_lookups` (= distinguished points), `dp_inserts` (= lookups minus table hits), `word_ops`
  (`work_native` of the walk: the short-cycle window compares). `negation_remeasure::tests`
  checks that the wrapper leaves the walk unchanged (totals equal `instrument::rho_on_curve`)
  and that the counters add up.
- **Deviation 1: word operations are priced.** PROTOCOL.md listed four rho classes (canon, hash,
  lookup, insert) in the pricing paragraph; the code also prices the walk's `work_native` word
  operations (one class, `word`), because AGENTS.md 5a prices every emitted counter. This raises
  rho's priced total and so lowers every IC/rho ratio (it works against the finding that IC is
  above rho). N2 and N3 are scored on the four named classes and, for N3, also as measured.
- **Deviation 2: the 5a calibration uses 25 batches of 20,000 operations for every class and every
  group addition** (as frozen) but `inv_r` uses batches of 1,000 (a Fermat inversion is about 40
  multiplications); the class `draw` is a xorshift step; the table classes use the walk's own
  `MixHasher` table (512 entries). `w_cal` = class time over the time of one addition of the
  arm's own group (E for rho; the word Jacobian for arms A, B, C; the recorded `Genus2Curve`
  implementation for the recorded v3/cover collectors; the genus-2 or genus-3 word Jacobian
  per rung for the tower). It is a best-of-N figure on an unisolated host, not a `perf-ipc`
  measurement [pcc:unverified].
- **Deviation 3: tower p = 64007 used 4 algorithm seeds for the recorded arm and 2 for A, B, C**
  (frozen text: 8 and 4; all other primes used 8 and 4). The reproduction gate at 64007 therefore
  checks the instances and the 64 old rho seeds only. All three instances completed at
  p = 32003 and p = 64007.
- **Deviation 4: the cover recorded arms use the original 3 seeds** and the matched rho, the
  improved arms and the aut check use the seed-0 instance of each prime (rho 32 seeds, A, B, C 8
  seeds); recorded distances are the original per-seed median ratios.
- **Deviation 5: the PROTOCOL.md prose has 5 lint-prose findings** (the words "improved" and
  "improvement" in paragraphs without a tag); the file is frozen by hash and was not edited.
  The files written after the freeze lint at 0.
- **Deviation 6: three binary builds.** The measured parts (`remeasure_v3`, `_cover`, `_tower`,
  `_tower_p32003`, `_tower_p64007`) came from the first build of `negation_remeasure`; after them the
  binary was reformatted (`rustfmt`) and a per-instance `curves_checked` field and the merge of
  `_aut` parts were added, which touch no measured value. The `_aut` part (tower with all
  seed counts 0, no calibration: it builds every tower instance up to p = 64007 and lists its
  `j`-invariant) and the final `merge` used the rebuilt binary. The bench record binds the dirty
  working tree of branch `claude/ecdlp-chained-transfers-8cpiek` at origin/main `47dc95a934`;
  the code is not committed in this session (the validation's wish for a record bound to a clean
  commit containing the measuring code needs the PR commit and a re-run or an amendment)
  [pcc:unverified].
- Automorphism fold: 38 curves checked (v3 4, cover 7, tower instances at
  p = 251 to 64007); 0 has `j` in {0, 1728}. The fold is therefore vacuous on this data
  and was not implemented for the `chained_transfer::ec::Curve` type [pcc:unverified].

## Matched rho: group operations and native counters (mean over seeds)

### v3

| p | r | neg total | neg walk | canon | part. hash | DP lookup | DP insert | word ops | native / step (all) | native / step (4 classes) | plain total | plain native / step (3 classes) | old plain rho | walk share (neg) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 211 | 109 | 62.7 | 12.0 | 16.2 | 16.2 | 4.09 | 3.09 | 31.6 | 7.1 | 3.96 | 68.3 | 2.37 | 428 | 0.192 |
| 419 | 113 | 64.9 | 13.6 | 18.6 | 18.6 | 4.81 | 3.75 | 35.8 | 7.26 | 4.08 | 71.6 | 2.21 | 357 | 0.209 |
| 821 | 71.0 | 66.4 | 10.5 | 15.7 | 15.7 | 5 | 3.97 | 23.6 | 7.2 | 4.55 | 69.6 | 2.28 | 370 | 0.158 |
| 1619 | 401 | 91.9 | 26.2 | 31.2 | 31.2 | 4.91 | 3.84 | 108 | 8.18 | 3.25 | 97.4 | 1.79 | 847 | 0.285 |

### cover_pushforward

| p | r | neg total | neg walk | canon | part. hash | DP lookup | DP insert | word ops | native / step (all) | native / step (4 classes) | plain total | plain native / step (3 classes) | old plain rho | walk share (neg) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 211 | 109 | 64.2 | 13.5 | 18.0 | 18.0 | 4.47 | 3.47 | 38.4 | 7.11 | 3.8 | 70.1 | 2.35 | 442 | 0.211 |
| 419 | 113 | 75.8 | 24.8 | 30.1 | 30.1 | 5.06 | 4.06 | 72.9 | 7.52 | 3.67 | 68.5 | 2.32 | 395 | 0.327 |
| 821 | 71.0 | 66.0 | 11.2 | 16.2 | 16.2 | 4.84 | 3.81 | 26.0 | 7.19 | 4.4 | 68.4 | 2.36 | 376 | 0.17 |
| 1619 | 401 | 92.9 | 27.1 | 32.5 | 32.5 | 5.25 | 4.25 | 106 | 7.98 | 3.3 | 100 | 1.7 | 571 | 0.292 |
| 3203 | 313 | 84.2 | 22.3 | 27.5 | 27.5 | 5.09 | 4.09 | 83.0 | 7.67 | 3.35 | 89.6 | 1.76 | 592 | 0.265 |
| 6421 | 3,191 | 159 | 78.1 | 87.2 | 87.2 | 9 | 8 | 446 | 9.87 | 2.96 | 173 | 1.33 | 1,269 | 0.492 |
| 12841 | 3,169 | 151 | 75.0 | 84.1 | 84.1 | 9.03 | 8.03 | 460 | 10.1 | 2.89 | 173 | 1.35 | 871 | 0.498 |

### tower_large

| p | r | neg total | neg walk | canon | part. hash | DP lookup | DP insert | word ops | native / step (all) | native / step (4 classes) | plain total | plain native / step (3 classes) | old plain rho | walk share (neg) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 251 | 107 | 68.6 | 12.5 | 18.3 | 18.3 | 5.65 | 4.64 | 29.3 | 7.3 | 4.5 | 74.2 | 2.56 | 416 | 0.183 |
| 503 | 260 | 84.2 | 24.8 | 29.7 | 29.7 | 4.67 | 3.66 | 108 | 8.28 | 3.19 | 87.3 | 1.69 | 523 | 0.294 |
| 1009 | 409 | 85.7 | 24.5 | 30.1 | 30.1 | 5.48 | 4.47 | 93.6 | 7.89 | 3.38 | 92.7 | 1.79 | 618 | 0.286 |
| 2003 | 1,010 | 103 | 36.3 | 44.3 | 44.3 | 8.01 | 7.01 | 145 | 8.02 | 3.34 | 117 | 1.73 | 714 | 0.353 |
| 4001 | 1,018 | 104 | 38.3 | 46.1 | 46.1 | 7.72 | 6.72 | 159 | 8.13 | 3.26 | 116 | 1.74 | 720 | 0.37 |
| 8009 | 3,322 | 148 | 74.8 | 83.6 | 83.6 | 8.73 | 7.72 | 475 | 10.4 | 2.89 | 167 | 1.45 | 958 | 0.504 |
| 16001 | 6,672 | 197 | 118 | 129 | 129 | 11.0 | 9.95 | 813 | 11.0 | 2.8 | 222 | 1.35 | 1,067 | 0.598 |
| 32003 | 13,395 | 252 | 165 | 182 | 182 | 16.5 | 15.5 | 1,161 | 11.0 | 2.79 | 281 | 1.37 | 1,220 | 0.655 |
| 64007 | 26,650 | 313 | 215 | 237 | 237 | 21.6 | 20.5 | 1,458 | 10.8 | 2.82 | 367 | 1.37 | 1,359 | 0.687 |

The old plain rho column is the recorded `rho_reference` (v3: the single recorded seed; cover: the
three original seeds; tower: 64 seeds). The negation total is `4.3`
to `7.2` times
smaller on the tower, `5.2` to `8.0` on cover and
`5.5` to `9.2` on v3. The negation walk is not always cheaper than the plain tuned walk on a
small `r` (cover p = 419 and others, a few seeds' difference): the table shows both and the
`best of negation and plain` columns of `tables.md` use the cheaper at each price
[pcc:unverified].

## Calibration (best-of-N timing, ns per operation) and `w_cal`

| target | p | mul_p | mul_r | inv_r | canon | hash | lookup | insert | draw | word | t_add: E / J (ns) | w_cal mul_p on the arm's J | w_cal canon (E) | w_cal hash (E) |
|--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--|--:|--:|--:|
| cover_pushforward | 211 | 6.21 | 6.21 | 64.2 | 2.17 | 2.07 | 4.2 | 7.32 | 2.07 | 1.04 | E1 142, J_recorded_impl 3,159, J_word_impl 785 | 0.0079 | 0.0153 | 0.0146 |
| cover_pushforward | 419 | 6.21 | 6.21 | 66.6 | 2.28 | 2.07 | 4.18 | 7.26 | 2.07 | 1.04 | E1 148, J_recorded_impl 3,266, J_word_impl 783 | 0.0079 | 0.0154 | 0.0140 |
| cover_pushforward | 821 | 6.21 | 6.21 | 59.3 | 2.19 | 2.07 | 4.17 | 7.22 | 2.07 | 1.04 | E1 158, J_recorded_impl 3,404, J_word_impl 792 | 0.0078 | 0.0139 | 0.0131 |
| cover_pushforward | 1619 | 6.21 | 6.21 | 79.0 | 2.18 | 2.07 | 4.21 | 7.26 | 2.07 | 1.04 | E1 165, J_recorded_impl 3,549, J_word_impl 816 | 0.0076 | 0.0132 | 0.0125 |
| cover_pushforward | 3203 | 6.21 | 6.21 | 79.0 | 2.17 | 2.07 | 4.2 | 7.24 | 2.07 | 1.04 | E1 169, J_recorded_impl 3,544, J_word_impl 829 | 0.0075 | 0.0129 | 0.0123 |
| cover_pushforward | 6421 | 6.21 | 6.21 | 100 | 2.18 | 2.07 | 4.2 | 7.33 | 2.07 | 1.04 | E1 181, J_recorded_impl 3,721, J_word_impl 840 | 0.0074 | 0.0121 | 0.0114 |
| cover_pushforward | 12841 | 6 | 6.21 | 102 | 2.38 | 2.07 | 4.19 | 7.24 | 2.07 | 1.04 | E1 190, J_recorded_impl 3,814, J_word_impl 818 | 0.0073 | 0.0126 | 0.0109 |
| tower_large | 251 | 6.21 | 6.21 | 66.6 | 2.28 | 2.07 | 4.2 | 7.27 | 2.07 | 1.04 | E 147, J_g2 837, J_g3 1,438 | 0.0074 | 0.0155 | 0.0141 |
| tower_large | 503 | 6.21 | 6.21 | 74.2 | 2.27 | 2.07 | 4.21 | 7.22 | 2.07 | 1.04 | E 158, J_g2 845, J_g3 1,458 | 0.0073 | 0.0144 | 0.0131 |
| tower_large | 1009 | 6.21 | 6.21 | 77.6 | 2.19 | 2.07 | 4.19 | 7.37 | 2.07 | 1.04 | E 168, J_g2 855, J_g3 1,454 | 0.0073 | 0.0130 | 0.0123 |
| tower_large | 2003 | 6.21 | 6.21 | 92.4 | 2.19 | 2.07 | 4.19 | 7.2 | 2.07 | 1.04 | E 167, J_g2 860, J_g3 1,461 | 0.0072 | 0.0131 | 0.0124 |
| tower_large | 4001 | 6.21 | 6.21 | 90.0 | 2.18 | 2.07 | 4.17 | 7.2 | 2.07 | 1.04 | E 179, J_g2 882, J_g3 1,475 | 0.0070 | 0.0121 | 0.0116 |
| tower_large | 8009 | 6.21 | 6.21 | 101 | 2.17 | 2.07 | 4.19 | 7.27 | 2.07 | 1.04 | E 188, J_g2 885, J_g3 1,478 | 0.0070 | 0.0115 | 0.0110 |
| tower_large | 16001 | 6.21 | 6.21 | 100 | 2.18 | 2.07 | 4.19 | 7.35 | 2.07 | 1.04 | E 202, J_g2 895, J_g3 1,501 | 0.0069 | 0.0108 | 0.0102 |
| tower_large | 32003 | 6.21 | 6.21 | 113 | 2.19 | 2.07 | 4.21 | 7.2 | 2.07 | 1.04 | E 196, J_g2 917, J_g3 1,523 | 0.0068 | 0.0112 | 0.0105 |
| tower_large | 64007 | 6.21 | 6.21 | 126 | 2.29 | 2.07 | 4.19 | 7.26 | 2.07 | 1.04 | E 201, J_g2 861, J_g3 1,408 | 0.0072 | 0.0114 | 0.0103 |
| v3 | 211 | 6.21 | 6.21 | 64.1 | 2.26 | 2.07 | 4.19 | 7.36 | 2.07 | 1.04 | E1 142, J_recorded_impl 3,191, J_word_impl 870 | 0.0071 | 0.0159 | 0.0146 |
| v3 | 419 | 6.21 | 6.21 | 66.6 | 2.26 | 2.07 | 4.17 | 7.34 | 2.07 | 1.04 | E1 148, J_recorded_impl 3,258, J_word_impl 868 | 0.0071 | 0.0152 | 0.0140 |
| v3 | 821 | 6.21 | 6.21 | 59.3 | 2.18 | 2.07 | 4.2 | 7.31 | 2.07 | 1.04 | E1 158, J_recorded_impl 3,393, J_word_impl 879 | 0.0071 | 0.0138 | 0.0131 |
| v3 | 1619 | 6.21 | 6.21 | 79.0 | 2.18 | 2.07 | 4.2 | 7.3 | 2.07 | 1.04 | E1 165, J_recorded_impl 3,533, J_word_impl 919 | 0.0068 | 0.0132 | 0.0126 |

## Recorded arms and improved arms, old versus new distance

Old = the recorded ratio to the recorded plain `rho_reference` (group ops plus the emitted LA
counter at the price, as in the original files). New = the arm's total, every emitted native
counter priced at the same weight, over the negation-map rho total with its native counters priced at
that weight. `cal.` is the calibrated reading. Lower-bound arms have unemitted counters and are
never classed `below`. [pcc:unverified]


#### cover_pushforward: recorded arms, old (recorded ratio to the plain `rho_reference`) versus new (to the negation-map rho)

| p | arm | group ops | old ratio 0 / 0.1 / 1 | old class | new ratio 0 / 0.1 / 1 | new class (band) | cal. ratio 0 / w_cal / 10 w_cal | class (cal) | lower bound |
|--:|--|--:|--|--|--|--|--|--|--|
| 211 | ic_jc | 3,359 | 7.61 / 10.9 / 40.8 | above | 52.4 / 66.6 / 123 | above | 52.4 / 51.9 / 48.3 | above | yes |
| 211 | ic_e1_two_term | 762 | 1.73 / 2.63 / 10.4 | above | 11.9 / 16.1 / 32.8 | above | 11.9 / 14.4 / 33.4 | above | yes |
| 211 | ic_e1_fibre_full | 3,359 | 7.61 / 8.48 / 16.3 | above | 52.4 / 51.7 / 49.2 | above | 52.4 / 54.0 / 66.8 | above | yes |
| 211 | ic_jc_first_collision | 131 | 0.338 / 0.338 / 0.338 | below | 2.05 / 1.81 / 0.896 | unresolved (lower bound) | 2.05 / 2.01 / 1.74 | above | yes |
| 211 | ic_e1_fibre_first_collision | 131 | 0.338 / 0.338 / 0.338 | below | 2.05 / 1.81 / 0.896 | unresolved (lower bound) | 2.05 / 2.01 / 1.74 | above | yes |
| 211 | birthday_e1_first_collision | 131 | 0.338 / 0.338 / 0.338 | below | 2.05 / 1.81 / 0.896 | unresolved (lower bound) | 2.05 / 2.01 / 1.74 | above | yes |
| 419 | ic_jc | 8,932 | 23.7 / 37.7 / 157 | above | 118 / 157 / 279 | above | 118 / 117 / 107 | above | yes |
| 419 | ic_e1_two_term | 673 | 1.73 / 2.43 / 8.79 | above | 8.89 / 11.0 / 17.8 | above | 8.89 / 10.4 / 21.5 | above | yes |
| 419 | ic_e1_fibre_full | 8,932 | 23.7 / 25.4 / 39.2 | above | 118 / 106 / 67.5 | above | 118 / 118 / 122 | above | yes |
| 419 | ic_jc_first_collision | 296 | 0.77 / 0.77 / 0.77 | below | 3.91 / 3.29 / 1.36 | above | 3.91 / 3.82 / 3.18 | above | yes |
| 419 | ic_e1_fibre_first_collision | 296 | 0.77 / 0.77 / 0.77 | below | 3.91 / 3.29 / 1.36 | above | 3.91 / 3.82 / 3.18 | above | yes |
| 419 | birthday_e1_first_collision | 178 | 0.35 / 0.35 / 0.35 | below | 2.34 / 1.97 / 0.815 | unresolved (lower bound) | 2.34 / 2.29 / 1.91 | above | yes |
| 821 | ic_jc | 11,426 | 30.5 / 73.8 / 467 | above | 173 / 384 / 1,323 | above | 173 / 175 / 192 | above | yes |
| 821 | ic_e1_two_term | 467 | 1.24 / 1.68 / 5.55 | above | 7.08 / 8.79 / 16.4 | above | 7.08 / 7.98 / 15.1 | above | yes |
| 821 | ic_e1_fibre_full | 11,426 | 30.5 / 32.5 / 50.9 | above | 173 / 168 / 144 | above | 173 / 175 / 193 | above | yes |
| 821 | ic_jc_first_collision | 163 | 0.398 / 0.398 / 0.398 | below | 2.47 / 2.24 / 1.22 | above | 2.47 / 2.44 / 2.17 | above | yes |
| 821 | ic_e1_fibre_first_collision | 163 | 0.398 / 0.398 / 0.398 | below | 2.47 / 2.24 / 1.22 | above | 2.47 / 2.44 / 2.17 | above | yes |
| 821 | birthday_e1_first_collision | 118 | 0.308 / 0.308 / 0.308 | below | 1.78 / 1.62 / 0.883 | unresolved (lower bound) | 1.78 / 1.76 / 1.57 | above | yes |
| 1619 | ic_jc | 34,876 | 70.5 / 195 / 1,315 | above | 375 / 858 / 2,337 | above | 375 / 379 / 409 | above | yes |
| 1619 | ic_e1_two_term | 2,755 | 5.68 / 16.5 / 114 | above | 29.6 / 70.5 / 196 | above | 29.6 / 49.2 / 196 | above | yes |
| 1619 | ic_e1_fibre_full | 34,876 | 70.5 / 85.8 / 223 | above | 375 / 381 / 399 | above | 375 / 397 / 564 | above | yes |
| 1619 | ic_jc_first_collision | 567 | 0.167 / 0.167 / 0.167 | below | 6.1 / 5.11 / 2.08 | above | 6.1 / 5.99 / 5.1 | above | yes |
| 1619 | ic_e1_fibre_first_collision | 567 | 0.167 / 0.167 / 0.167 | below | 6.1 / 5.11 / 2.08 | above | 6.1 / 5.99 / 5.1 | above | yes |
| 1619 | birthday_e1_first_collision | 227 | 0.167 / 0.167 / 0.167 | below | 2.44 / 2.05 / 0.831 | unresolved (lower bound) | 2.44 / 2.4 / 2.04 | above | yes |
| 3203 | ic_jc | 60,063 | 97.1 / 483 / 3,955 | above | 713 / 3,000 / 10,492 | above | 713 / 749 / 1,023 | above | yes |
| 3203 | ic_e1_two_term | 2,203 | 3.62 / 8.48 / 56.1 | above | 26.1 / 56.6 / 156 | above | 26.1 / 40.3 / 148 | above | yes |
| 3203 | ic_e1_fibre_full | 60,063 | 97.1 / 117 / 301 | above | 713 / 733 / 799 | above | 713 / 754 / 1,068 | above | yes |
| 3203 | ic_jc_first_collision | 476 | 0.853 / 0.853 / 0.853 | below | 5.65 / 4.81 / 2.06 | above | 5.65 / 5.55 / 4.8 | above | yes |
| 3203 | ic_e1_fibre_first_collision | 476 | 0.853 / 0.853 / 0.853 | below | 5.65 / 4.81 / 2.06 | above | 5.65 / 5.55 / 4.8 | above | yes |
| 3203 | birthday_e1_first_collision | 324 | 0.473 / 0.473 / 0.473 | below | 3.84 / 3.27 / 1.4 | above | 3.84 / 3.77 / 3.26 | above | yes |
| 6421 | ic_jc | 188,083 | 198 / 1,226 / 10,476 | above | 1,185 / 5,295 / 12,660 | above | 1,185 / 1,249 / 1,681 | above | yes |
| 6421 | ic_e1_two_term | 28,508 | 29.5 / 447 / 4,201 | above | 180 / 1,873 / 4,907 | above | 180 / 987 / 6,474 | above | yes |
| 6421 | ic_e1_fibre_full | 188,083 | 198 / 452 / 2,736 | above | 1,185 / 1,949 / 3,317 | above | 1,185 / 1,662 / 4,904 | above | yes |
| 6421 | ic_jc_first_collision | 1,750 | 1.39 / 1.39 / 1.39 | above | 11.0 / 7.87 / 2.2 | above | 11.0 / 10.7 / 8.33 | above | yes |
| 6421 | ic_e1_fibre_first_collision | 1,750 | 1.39 / 1.39 / 1.39 | above | 11.0 / 7.87 / 2.2 | above | 11.0 / 10.7 / 8.33 | above | yes |
| 6421 | birthday_e1_first_collision | 1,463 | 1.05 / 1.05 / 1.05 | above | 9.22 / 6.58 / 1.84 | above | 9.22 / 8.93 / 6.96 | above | yes |
| 12841 | ic_jc | 372,747 | 394 / 4,720 / 43,658 | above | 2,473 / 20,667 / 51,693 | above | 2,473 / 2,819 / 5,163 | above | yes |
| 12841 | ic_e1_two_term | 28,904 | 30.4 / 486 / 4,585 | above | 192 / 2,251 / 5,763 | above | 192 / 1,143 / 7,574 | above | yes |
| 12841 | ic_e1_fibre_full | 372,747 | 394 / 940 / 5,858 | above | 2,473 / 4,125 / 6,941 | above | 2,473 / 3,475 / 10,257 | above | yes |
| 12841 | ic_jc_first_collision | 1,963 | 1.85 / 1.85 / 1.85 | above | 13.0 / 9.12 / 2.47 | above | 13.0 / 12.6 / 9.79 | above | yes |
| 12841 | ic_e1_fibre_first_collision | 1,963 | 1.85 / 1.85 / 1.85 | above | 13.0 / 9.12 / 2.47 | above | 13.0 / 12.6 / 9.79 | above | yes |
| 12841 | birthday_e1_first_collision | 1,963 | 1.85 / 1.85 / 1.85 | above | 13.0 / 9.12 / 2.47 | above | 13.0 / 12.6 / 9.79 | above | yes |

#### cover_pushforward: improved arms against the negation-map rho

| p | arm | group ops | native (by class) | ratio 0 / 0.1 / 1 | class | cal. ratio | class (cal) |
|--:|--|--:|--|--|--|--|--|
| 211 | A | 824 | mul_p 36,089, mul_r 34,750 | 12.8 / 109 / 489 | above | 12.8 / 21.2 / 85.1 | above |
| 211 | B | 762 | mul_p 30,841, mul_r 11,125 | 11.9 / 68.5 / 292 | above | 11.9 / 16.8 / 54.1 | above |
| 211 | C | 670 | mul_p 25,174, mul_r 1,501 | 10.5 / 46.1 / 187 | above | 10.5 / 13.5 / 36.8 | above |
| 419 | A | 1,752 | mul_p 86,433, mul_r 103,935 | 23.1 / 231 / 881 | above | 23.1 / 42.1 / 181 | above |
| 419 | B | 1,553 | mul_p 73,049, mul_r 3,925 | 20.5 / 103 / 360 | above | 20.5 / 27.9 / 82.2 | above |
| 419 | C | 1,993 | mul_p 94,729, mul_r 1,276 | 26.3 / 129 / 450 | above | 26.3 / 35.5 / 103 | above |
| 821 | A | 2,186 | mul_p 156,164, mul_r 134,447 | 33.1 / 430 / 2,198 | above | 33.1 / 66.7 / 332 | above |
| 821 | B | 1,407 | mul_p 92,024, mul_r 8,254 | 21.3 / 157 / 763 | above | 21.3 / 32.8 / 123 | above |
| 821 | C | 1,953 | mul_p 127,933, mul_r 1,028 | 29.6 / 204 / 983 | above | 29.6 / 44.3 / 161 | above |
| 1619 | A | 3,658 | mul_p 334,834, mul_r 1,197,399 | 39.4 / 1,414 / 5,621 | above | 39.4 / 162 / 1,081 | above |
| 1619 | B | 2,993 | mul_p 251,513, mul_r 27,761 | 32.2 / 279 / 1,033 | above | 32.2 / 54.0 / 218 | above |
| 1619 | C | 2,583 | mul_p 207,187, mul_r 1,385 | 27.8 / 211 / 773 | above | 27.8 / 44.0 / 166 | above |
| 3203 | A | 7,325 | mul_p 680,173, mul_r 3,131,700 | 86.9 / 3,925 / 16,500 | above | 86.9 / 418 / 2,948 | above |
| 3203 | B | 4,500 | mul_p 381,996, mul_r 38,780 | 53.4 / 471 / 1,837 | above | 53.4 / 89.2 / 363 | above |
| 3203 | C | 5,932 | mul_p 493,623, mul_r 1,899 | 70.4 / 561 / 2,166 | above | 70.4 / 112 / 433 | above |
| 6421 | A | 10,634 | mul_p 1,544,468, mul_r 23,298,106 | 67.0 / 11,215 / 31,198 | above | 67.0 / 1,185 / 8,788 | above |
| 6421 | B | 7,064 | mul_p 938,285, mul_r 87,711 | 44.5 / 493 / 1,297 | above | 44.5 / 89.4 / 394 | above |
| 6421 | C | 8,592 | mul_p 1,118,677, mul_r 2,948 | 54.2 / 543 / 1,419 | above | 54.2 / 103 / 435 | above |
| 12841 | A | 21,489 | mul_p 3,355,584, mul_r 59,144,251 | 143 / 29,137 / 78,581 | above | 143 / 3,176 / 23,691 | above |
| 12841 | B | 13,945 | mul_p 2,004,799, mul_r 115,520 | 92.5 / 1,050 / 2,683 | above | 92.5 / 190 / 846 | above |
| 12841 | C | 15,063 | mul_p 2,142,707, mul_r 3,627 | 99.9 / 1,067 / 2,717 | above | 99.9 / 198 / 859 | above |

#### tower_large: recorded arms, old (recorded ratio to the plain `rho_reference`) versus new (to the negation-map rho)

| p | arm | group ops | old ratio 0 / 0.1 / 1 | old class | new ratio 0 / 0.1 / 1 | new class (band) | cal. ratio 0 / w_cal / 10 w_cal | class (cal) | lower bound |
|--:|--|--:|--|--|--|--|--|--|--|
| 251 | g2_recorded | 279 | 0.67 / 8.77 / 81.7 | unresolved | 4.06 / 72.7 / 365 | above | 4.06 / 9.65 / 52.8 | above | no |
| 251 | g3_recorded | 767 | 1.84 / 22.8 / 211 | above | 11.2 / 436 / 2,246 | above | 11.2 / 31.1 / 185 | above | no |
| 503 | g2_recorded | 532 | 1.02 / 20.5 / 196 | above | 6.32 / 146 / 553 | above | 6.32 / 18.5 / 109 | above | no |
| 503 | g3_recorded | 1,648 | 3.15 / 35.3 / 324 | above | 19.6 / 747 / 2,868 | above | 19.6 / 56.0 / 326 | above | no |
| 1009 | g2_recorded | 1,033 | 1.67 / 40.9 / 394 | above | 12.1 / 323 / 1,286 | above | 12.1 / 38.6 / 239 | above | no |
| 1009 | g3_recorded | 3,137 | 5.08 / 55.4 / 508 | above | 36.6 / 1,465 / 5,883 | above | 36.6 / 107 / 641 | above | no |
| 2003 | g2_recorded | 2,024 | 2.84 / 75.7 / 731 | above | 19.7 / 569 / 2,015 | above | 19.7 / 68.0 / 417 | above | no |
| 2003 | g3_recorded | 6,025 | 8.44 / 114 / 1,065 | above | 58.6 / 2,555 / 9,130 | above | 58.6 / 186 / 1,109 | above | no |
| 4001 | g2_recorded | 4,048 | 5.62 / 147 / 1,419 | above | 39.1 / 1,099 / 3,776 | above | 39.1 / 131 / 802 | above | no |
| 4001 | g3_recorded | 11,815 | 16.4 / 346 / 3,317 | above | 114 / 5,801 / 20,160 | above | 114 / 406 / 2,534 | above | no |
| 8009 | g2_recorded | 8,054 | 8.4 / 212 / 2,046 | above | 54.3 / 1,323 / 3,422 | above | 54.3 / 180 / 1,026 | above | no |
| 8009 | g3_recorded | 24,401 | 25.5 / 993 / 9,698 | above | 164 / 9,705 / 25,493 | above | 164 / 722 / 4,477 | above | no |
| 16001 | g2_recorded | 16,048 | 15.0 / 375 / 3,613 | above | 81.6 / 1,857 / 4,295 | above | 81.6 / 268 / 1,487 | above | no |
| 16001 | g3_recorded | 47,918 | 44.9 / 4,616 / 45,755 | above | 244 / 23,688 / 55,887 | above | 244 / 1,694 / 11,152 | above | no |
| 32003 | g2_recorded | 32,004 | 26.2 / 656 / 6,328 | above | 127 / 2,840 / 6,243 | above | 127 / 415 / 2,224 | above | no |
| 32003 | g3_recorded | 95,572 | 78.4 / 26,097 / 260,265 | above | 379 / 89,891 / 202,177 | above | 379 / 6,027 / 41,428 | above | no |
| 64007 | g2_recorded | 63,987 | 47.1 / 1,175 / 11,322 | above | 205 / 4,640 / 10,101 | above | 205 / 712 / 3,879 | above | no |
| 64007 | g3_recorded | 192,688 | 142 / 159,687 / 1,595,592 | above | 616 / 446,318 / 995,132 | above | 616 / 31,305 / 222,705 | above | no |

#### tower_large: improved arms against the negation-map rho

| p | arm | group ops | native (by class) | ratio 0 / 0.1 / 1 | class | cal. ratio | class (cal) |
|--:|--|--:|--|--|--|--|--|
| 251 | g2_C | 752 | mul_p 34,605, mul_r 1,300 | 11.0 / 57.0 / 253 | above | 11.0 / 14.6 / 42.7 | above |
| 251 | g3_C | 845 | mul_p 81,954, mul_r 10,031 | 12.3 / 132 / 642 | above | 12.3 / 17.8 / 60.2 | above |
| 503 | g2_C | 1,474 | mul_p 98,223, mul_r 2,287 | 17.5 / 113 / 392 | above | 17.5 / 25.7 / 86.6 | above |
| 503 | g3_C | 1,270 | mul_p 176,649, mul_r 15,279 | 15.1 / 201 / 743 | above | 15.1 / 24.3 / 92.3 | above |
| 1009 | g2_C | 2,171 | mul_p 170,572, mul_r 1,900 | 25.3 / 190 / 700 | above | 25.3 / 39.2 / 144 | above |
| 1009 | g3_C | 1,929 | mul_p 335,682, mul_r 21,713 | 22.5 / 369 / 1,441 | above | 22.5 / 39.6 / 168 | above |
| 2003 | g2_C | 3,235 | mul_p 339,082, mul_r 1,658 | 31.4 / 292 / 979 | above | 31.4 / 54.0 / 217 | above |
| 2003 | g3_C | 3,114 | mul_p 711,593, mul_r 40,982 | 30.3 / 614 / 2,150 | above | 30.3 / 59.9 / 273 | above |
| 4001 | g2_C | 6,154 | mul_p 674,210, mul_r 2,271 | 59.4 / 567 / 1,848 | above | 59.4 / 103 / 419 | above |
| 4001 | g3_C | 4,978 | mul_p 1,311,507, mul_r 54,182 | 48.0 / 1,087 / 3,710 | above | 48.0 / 101 / 487 | above |
| 8009 | g2_C | 9,192 | mul_p 1,299,770, mul_r 2,890 | 61.9 / 651 / 1,625 | above | 61.9 / 119 / 507 | above |
| 8009 | g3_C | 9,315 | mul_p 3,093,659, mul_r 122,436 | 62.7 / 1,544 / 3,995 | above | 62.7 / 149 / 727 | above |
| 16001 | g2_C | 15,684 | mul_p 2,579,985, mul_r 5,358 | 79.8 / 897 / 2,019 | above | 79.8 / 165 / 719 | above |
| 16001 | g3_C | 14,457 | mul_p 5,528,664, mul_r 161,955 | 73.5 / 1,909 / 4,429 | above | 73.5 / 186 / 921 | above |
| 32003 | g2_C | 20,587 | mul_p 3,926,854, mul_r 6,078 | 81.7 / 1,015 / 2,186 | above | 81.7 / 179 / 792 | above |
| 32003 | g3_C | 28,619 | mul_p 12,438,794, mul_r 312,484 | 114 / 3,198 / 7,067 | above | 114 / 306 / 1,515 | above |
| 64007 | g2_C | 52,898 | mul_p 11,694,102, mul_r 9,178 | 169 / 2,398 / 5,143 | above | 169 / 420 / 1,987 | above |
| 64007 | g3_C | 44,494 | mul_p 23,093,987, mul_r 429,917 | 142 / 4,699 / 10,311 | above | 142 / 454 / 2,397 | above |

#### v3: recorded arms, old (recorded ratio to the plain `rho_reference`) versus new (to the negation-map rho)

| p | arm | group ops | old ratio 0 / 0.1 / 1 | old class | new ratio 0 / 0.1 / 1 | new class (band) | cal. ratio 0 / w_cal / 10 w_cal | class (cal) | lower bound |
|--:|--|--:|--|--|--|--|--|--|--|
| 211 | recorded | 3,419 | 7.99 / 11.4 / 42.2 | above | 54.6 / 70.0 / 135 | above | 54.6 / 54.2 / 50.9 | above | yes |
| 419 | recorded | 8,475 | 23.7 / 38.3 / 169 | above | 131 / 187 / 413 | above | 131 / 130 / 124 | above | yes |
| 821 | recorded | 11,353 | 30.7 / 75.2 / 476 | above | 171 / 382 / 1,352 | above | 171 / 173 / 191 | above | yes |
| 1619 | recorded | 34,521 | 40.8 / 112 / 754 | above | 376 / 864 / 2,353 | above | 376 / 380 / 411 | above | yes |

#### v3: improved arms against the negation-map rho

| p | arm | group ops | native (by class) | ratio 0 / 0.1 / 1 | class | cal. ratio | class (cal) |
|--:|--|--:|--|--|--|--|--|
| 211 | L | 3,419 | mul_r 14,993 | 54.6 / 70.5 / 137 | above | 54.6 / 54.2 / 51.0 | above |
| 211 | A | 824 | mul_p 36,089, mul_r 34,750 | 13.2 / 113 / 535 | above | 13.2 / 20.9 / 80.9 | above |
| 211 | B | 762 | mul_p 30,841, mul_r 11,125 | 12.2 / 71.1 / 319 | above | 12.2 / 16.7 / 51.7 | above |
| 211 | C | 670 | mul_p 25,174, mul_r 1,501 | 10.7 / 47.8 / 204 | above | 10.7 / 13.5 / 35.4 | above |
| 419 | L | 8,475 | mul_r 52,288 | 131 / 187 / 415 | above | 131 / 130 / 125 | above |
| 419 | A | 1,752 | mul_p 86,433, mul_r 103,935 | 27.0 / 284 / 1,312 | above | 27.0 / 47.1 / 202 | above |
| 419 | B | 1,553 | mul_p 73,049, mul_r 3,925 | 23.9 / 127 / 536 | above | 23.9 / 31.8 / 92.7 | above |
| 419 | C | 1,993 | mul_p 94,729, mul_r 1,276 | 30.7 / 159 / 669 | above | 30.7 / 40.6 / 116 | above |
| 821 | L | 11,353 | mul_r 165,008 | 171 / 382 / 1,353 | above | 171 / 173 / 191 | above |
| 821 | A | 2,186 | mul_p 156,164, mul_r 134,447 | 32.9 / 429 / 2,246 | above | 32.9 / 62.9 / 301 | above |
| 821 | B | 1,407 | mul_p 92,024, mul_r 8,254 | 21.2 / 157 / 780 | above | 21.2 / 31.4 / 113 | above |
| 821 | C | 1,953 | mul_p 127,933, mul_r 1,028 | 29.4 / 204 / 1,004 | above | 29.4 / 42.5 / 147 | above |
| 1619 | L | 34,521 | mul_r 605,131 | 376 / 865 / 2,358 | above | 376 / 380 / 412 | above |
| 1619 | A | 3,658 | mul_p 334,834, mul_r 1,197,399 | 39.8 / 1,429 / 5,662 | above | 39.8 / 150 / 977 | above |
| 1619 | B | 2,993 | mul_p 251,513, mul_r 27,761 | 32.6 / 282 / 1,041 | above | 32.6 / 52.1 / 199 | above |
| 1619 | C | 2,583 | mul_p 207,187, mul_r 1,385 | 28.1 / 213 / 778 | above | 28.1 / 42.6 / 152 | above |

CLASS CHANGES (recorded class vs new class):
  ('cover_pushforward', 211, 'ic_jc_first_collision', 'below', 'unresolved (lower bound)')
  ('cover_pushforward', 211, 'ic_e1_fibre_first_collision', 'below', 'unresolved (lower bound)')
  ('cover_pushforward', 211, 'birthday_e1_first_collision', 'below', 'unresolved (lower bound)')
  ('cover_pushforward', 419, 'ic_jc_first_collision', 'below', 'above')
  ('cover_pushforward', 419, 'ic_e1_fibre_first_collision', 'below', 'above')
  ('cover_pushforward', 419, 'birthday_e1_first_collision', 'below', 'unresolved (lower bound)')
  ('cover_pushforward', 821, 'ic_jc_first_collision', 'below', 'above')
  ('cover_pushforward', 821, 'ic_e1_fibre_first_collision', 'below', 'above')
  ('cover_pushforward', 821, 'birthday_e1_first_collision', 'below', 'unresolved (lower bound)')
  ('cover_pushforward', 1619, 'ic_jc_first_collision', 'below', 'above')
  ('cover_pushforward', 1619, 'ic_e1_fibre_first_collision', 'below', 'above')
  ('cover_pushforward', 1619, 'birthday_e1_first_collision', 'below', 'unresolved (lower bound)')
  ('cover_pushforward', 3203, 'ic_jc_first_collision', 'below', 'above')
  ('cover_pushforward', 3203, 'ic_e1_fibre_first_collision', 'below', 'above')
  ('cover_pushforward', 3203, 'birthday_e1_first_collision', 'below', 'above')
  ('tower_large', 251, 'g2_recorded', 'unresolved', 'above')


## Prediction scorecard (frozen in PROTOCOL.md)

| id | prediction | result |
|--|--|--|
| N1 | No arm below the negation rho at all three prices; recorded-arm rows `above` or `unresolved (lower bound)`. | **holds** |
| N2 | Native/step in [1.8, 3.5] (negation), [0.9, 1.6] (plain), four classes. | **refuted** |
| N3 | Negation rho w=1 over w=0 in [1.3, 4]; recorded rho 4 to 10 times the negation total at v3. | **refuted** |
| N4 | Recorded-arm distance up in at least 9 of 11 rows; cover ic_jc new/old in [3, 12]. | **holds** |
| N5 | Calibrated order agrees with the band for recorded arms and arm C; mod-p `w_cal` in [0.002, 0.03]. | **refuted** |
| N6 | No curve with j = 0 or 1728 among the 32 or more. | **holds** |
| N7 | Arm C (or best of A, B, C) below the recorded arm at all three prices: v3 p = 1619, tower g3 p = 16001, cover ic_jc p = 12841. | **holds** |
| N8 | p = 32003 completes; p = 64007 completes at least one instance. | **holds** |
| N9 | Arm C distance at w = 0 rises with p (rank correlation at least 0.7) in both genera. | **holds** |

Details (`scorecard.json`):

- N1: {"below_rows": [], "recorded_rows_outside": []}
- N2: negation native/step (four classes) ranges 2.79 to 4.55; plain 1.33 to 2.56; 7 of 20 negation rows and 14 plain rows outside the predicted bands (the per-step overhead of walk starts and DP handling is larger at small r than predicted) [pcc:unverified].
- N3: negation rho w = 1 over w = 0 ranges 1.96 to 7.31 as measured (word operations included) and 1.61 to 2.65 on the four named classes; the second half (recorded rho 4 to 10 times the negation total at v3: 6.83, 5.50, 5.57, 9.22) holds. The first half is scored as measured, so refuted; on the four classes it holds [pcc:unverified].
- N4: 11 of 11 rows up; cover ic_jc new/old at w = 0: 6.88, 4.96, 5.68, 5.32, 7.34, 5.98, 6.28.
- N5: 6 disagreement(s) between band and calibrated class, all on arms with unemitted counters (cover first-collision and birthday arms); the mod-p `w_cal` on the word Jacobian lies in [0.0068, 0.0079]. Scored as frozen (every recorded-arm row), so refuted [pcc:unverified].
- N6: 38 curves checked, 0 with an automorphism beyond negation.
- N7: v3 p = 1619: C totals 2,583 / 23,440 / 211,155 against recorded lower bound 34,521 / 94,893 / 638,242; tower g3 p = 16001: 14,457 / 583,519 / 5,705,076 against 47,918 / 7,242,539 / 71,994,126; cover p = 12841: 15,063 / 229,696 / 2,161,397 against 372,747 / 4,448,291 / 41,128,195 [pcc:unverified].
- N8: tower primes completed [251, 503, 1009, 2003, 4001, 8009, 16001, 32003, 64007].
- N9: rank correlation genus 2 1.00 (n = 9), genus 3 1.00 (n = 9).

## Limits

- The unit is one group operation of the arm's own group. A Cantor step costs several elliptic
  additions (the calibration measured it: `t_add` J / E above), so every IC/rho ratio across group types
  understates the gap in time [pcc:unverified].
- The recorded v3 and cover collectors do not emit their relation-phase field work; their
  totals are lower bounds, listed as an implementation gap. Arm L (counted elimination of the same
  relations) changes the LA count by under 3% (accounting only).
- Cover arms that stop at the first collision emit no native counter; the birthday control and
  fibre arm cannot be ranked at the calibrated price below the lower-bound line.
- The aut fold was not needed (no curve with j = 0 or 1728); no claim is made for other curves.
- Host timing is best-of-N on a shared VM under variable load; `w_cal` varies by a few tens of
  percent between runs of this kind and never enters the band verdict.
- No replay certificate or `VALIDATION.json` exists for these rows; they are pre-gate.
