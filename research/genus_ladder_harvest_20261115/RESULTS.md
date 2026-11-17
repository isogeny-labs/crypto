# Genus ladder, follow-up 3: harvesting Cantor's intermediate divisor: results

Protocol: `PROTOCOL.md` (frozen 2026-11-15T05:00:45Z, `frozen.sha256`, before any
harvest run). Run binary: `binary.sha256`. One session on a shared 4-core container,
2026-11-15 05:01 to 05:32 UTC (`run.log`). Every figure is `[pcc:unverified]`: one
producer, no independent replay, no isolated host. Synthetic random curves only. Not a
leaderboard or `benchmarks/README.md` row.

## Summary

* The idea works as arithmetic. The first-reduction intermediate exists in 99.7 to 99.9%
  of genus-3 trials, splits at the predicted rate 1/24 (measured 0.0405 to 0.0429, i.e.
  0.97 to 1.03 x 1/24), and every harvested relation has the class of the walk element
  (the stored `(u', -v mod u')` is `+cur`, not `-cur`: the briefing's sign expectation was
  wrong, verify mode confirms the derivation).
* It lowers group operations at genus 3: trials ratio 0.80 (full base) and 0.86 / 0.88 /
  0.90 (double large prime at p = 1031 / 2053 / 4099), group operations 0.81 to 0.84 and
  0.88 to 0.90. These match the pre-registered acceptance model E3 to within 0.01.
* It does NOT lower the total cost. The degree-4 smoothness test roughly doubles the
  relation-phase native multiplications (`rel_mul` x1.79 to x2.02) and the four-place rows
  make the linear algebra 4 to 10% heavier; `FPM_mid` rises by 8 to 15%, and `GAE` at
  w = 0.1 and 1 rises by 14 to 60%. Across the 5a band the order of the two arms flips
  (harvest cheaper at w = 0, dearer at w = 0.1 and 1), so no ranking holds across the band.
* Controls behave: genus 2 is identical pair for pair (no intermediate exists); genus 4
  splits at 1/720 (0.91 to 1.01 x 1/720) and changes trials by at most 3%.

Verdict: not an improvement of the index-calculus baseline under the lab's cost readings
(GAE band or FPM_mid). It is a group-operation saving bought with more native work than it
saves.

## Instrument identity (H1)

* `cargo test --release --lib genus_ladder_harvest`: 4 tests pass
  (`harvest_off_is_run_ic`: every `IcResult` count equal to `genus_ladder::run_ic` on 32
  runs at p = 131 g = 2, 3, 4 and p = 257 g = 3, both arms, prefilter on/off, 2 seeds;
  `intermediate_is_equivalent_to_the_sum`; `field_tally_matches`;
  `harvest_verify_mode_no_bad_relations`).
* Row-level reproduction (`repro_check.json`): the 48 `C_double_lp+pre` runs of
  `research/genus_ladder_sge_20261115/sge_g34.jsonl` (p = 1031, 2053; g = 3, 4; 4 curves x
  3 seeds), 1536 recorded fields compared, 0 mismatches.
* Cell-level reproduction of the first study (`part_a.jsonl` itself is not stored): all 80
  part-A hyperelliptic cells of `research/genus_ladder_20261114/analysis.json` (p = 67 to
  1031, g = 2 to 5, arms `A_full`, `A_full+pre`, `C_double_lp`, `C_double_lp+pre`, 960
  runs), 1040 cell means compared at relative tolerance 1e-9, 0 mismatches.

## Paired ratios harvest / off (geometric mean, 95% hierarchical bootstrap)

9 pairs per cell (3 curves x 3 seeds). Full table: `tables.md` (P1, P2); figure
`fig_ratio.svg`.

| g | p | arm | trials | trials / relation | group ops | GAE w=0.1 | FPM mid | rel_mul | la_mul |
|--:|--:|---|---|---|---|---|---|---|---|
| 3 | 1031 | C_double_lp+pre | 0.856 [0.839, 0.870] | 0.855 [0.839, 0.873] | 0.876 [0.863, 0.888] | 1.570 [1.547, 1.599] | 1.145 [1.108, 1.200] | 1.917 | 1.099 |
| 3 | 2053 | C_double_lp+pre | 0.880 [0.853, 0.902] | 0.877 [0.857, 0.893] | 0.883 [0.860, 0.902] | 1.578 [1.553, 1.608] | 1.133 [1.115, 1.152] | 1.985 | 1.066 |
| 3 | 4099 | C_double_lp+pre | 0.898 [0.880, 0.916] | 0.895 [0.880, 0.909] | 0.903 [0.886, 0.920] | 1.579 [1.553, 1.603] | 1.124 [1.102, 1.149] | 2.015 | 1.040 |
| 3 | 1031 | A_full+pre | 0.807 [0.786, 0.823] | 0.807 [0.786, 0.822] | 0.839 [0.821, 0.853] | 1.231 [1.184, 1.277] | 1.080 [1.025, 1.135] | 1.801 | 1.072 |
| 3 | 2053 | A_full+pre | 0.799 [0.783, 0.813] | 0.799 [0.783, 0.813] | 0.820 [0.807, 0.833] | 1.166 [1.139, 1.194] | 1.076 [1.042, 1.105] | 1.789 | 1.071 |
| 3 | 4099 | A_full+pre | 0.803 [0.783, 0.825] | 0.803 [0.784, 0.825] | 0.814 [0.795, 0.835] | 1.141 [1.101, 1.184] | 1.092 [1.051, 1.134] | 1.790 | 1.090 |
| 2 | 1031, 4099 | both | 1.000 (all 36 pairs identical) | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 4 | 1031 | C_double_lp+pre | 0.997 [0.965, 1.034] | 0.999 [0.968, 1.033] | 0.996 [0.965, 1.030] | 2.231 | 1.170 [1.148, 1.192] | 2.940 | 1.010 |
| 4 | 4099 | C_double_lp+pre | 0.974 [0.953, 1.001] | 0.976 [0.954, 1.002] | 0.974 [0.950, 1.002] | 2.093 | 1.131 [1.117, 1.143] | 2.914 | 1.001 |
| 4 | 1031 | A_full+pre | 1.013 [0.981, 1.042] | 1.019 [0.990, 1.047] | 1.015 [0.981, 1.042] | 1.924 | 1.083 [1.062, 1.102] | 2.984 | 0.981 |
| 4 | 4099 | A_full+pre | 0.996 [0.980, 1.009] | 0.996 [0.980, 1.008] | 0.996 [0.979, 1.008] | 1.450 | 1.039 [1.022, 1.058] | 2.974 | 1.005 |

Genus 4, `A_full+pre`: every split intermediate has 6 base places, more than the 5 slots
of the relation record, and is dropped (`harvest_overflow` 145 and 626, 0 harvested
relations). These two cells are therefore an A/A comparison whose only difference is the
random stream consumed by root finding; their spread (0.98 to 1.04) is the noise floor of
a 9-pair trials ratio.

## Scorecard

| id | prediction (as briefed) | outcome |
|---|---|---|
| H1 | off path identical to `genus_ladder::run_ic` | **holds**: tests pass; 48 rows / 1536 fields and 80 cells / 1040 means, 0 mismatches |
| H2 | all certify; 0 bad relations in verify mode | **holds**: 315 / 315 runs certified with the right logarithm; verify mode (p = 131, g = 3, both arms, 3 seeds; g = 4) 0 bad relations |
| H3 | g = 3, C arm: trials/relation ratio in [0.76, 0.86] | **refuted overall**: holds at 1031 (0.855), inconclusive at 2053 (0.877 [0.857, 0.893]), refuted at 4099 (0.895 [0.880, 0.909]). The A arm, where every place is in the base, gives 0.80 at all three primes |
| H4 | g = 3, C arm: group ops ratio <= 0.85 and FPM_mid ratio <= 0.90 | **refuted** at all three primes: group ops 0.876 / 0.883 / 0.903 (intervals above 0.85); FPM_mid 1.12 to 1.15 |
| H5 | g = 2 ratio 1.00 exactly; g = 4 trials ratio in [0.97, 1.0] | **holds** for g = 2 (36 / 36 pairs identical) and for g = 4 C arm (0.997, 0.974) and A arm at 4099 (0.996); **inconclusive** for the g = 4 A arm at 1031 (1.013 [0.981, 1.042], an A/A cell, see above) |

Analyst expectations frozen before data: E3 (C arm 0.86 / 0.88 / 0.90, A arm 0.80)
matched to within 0.01 at every cell. E4 (relation-phase native work rises, FPM_mid near or
above 1.0, H4's FPM bound fails, only `gae_w0` can favour harvesting) matched; `rel_mul`
rose more than the rough per-trial estimate (x1.8 to x2.0).

## Distance to rho (`analysis_main.json`, from `genus_ladder_analyze`)

Ratio to the cell's negation-map rho, cell means (below 1: cheaper than rho):

| g | p | arm | GAE w=0 | GAE w=0.1 | GAE w=1 | FPM mid |
|--:|--:|---|--:|--:|--:|--:|
| 3 | 4099 | C_double_lp+pre | 0.144 | 4.27 | 22.9 | 0.890 |
| 3 | 4099 | C_double_lp+pre+harvest | 0.130 | 6.75 | 36.6 | 1.001 |
| 3 | 1031 | C_double_lp+pre | 0.145 | 3.20 | 17.1 | 0.661 |
| 3 | 1031 | C_double_lp+pre+harvest | 0.127 | 5.03 | 27.2 | 0.757 |
| 4 | 4099 | C_double_lp+pre | 0.014 | 0.52 | 2.81 | 0.102 |
| 4 | 4099 | C_double_lp+pre+harvest | 0.014 | 1.09 | 5.95 | 0.115 |

The analyzer's band verdicts are "unresolved" at genus 3 for both arms (below rho at
w = 0, above at w = 0.1 and 1); harvesting widens the gap at w >= 0.1 and moves the
genus-3 FPM_mid reading at p = 4099 from 0.89 to 1.00 of rho.

## Deviations from the briefing

1. Sign: the intermediate's class is `+cur` (Cantor stores `v' = -v mod u'`), so harvested
   rows carry the main row's right-hand side (`rhs = h a`, `tcoef = -h b`), not the
   negated one. Derived in `PROTOCOL.md` before data and checked in verify mode.
2. `genus_ladder::splits_completely` / `roots_of` hold degree <= 5; the module carries
   copies sized for degree 8 (same arithmetic and counts, covered by the identity test).
3. `part_a.jsonl` is not stored; reproduction used the cell means of the first study's
   `analysis.json` (all 80 part-A cells) and the per-run rows of `sge_g34.jsonl`.
4. Arms run: `A_full+pre` and `C_double_lp+pre`, each with and without harvest, plus
   `R_rho_neg`; the unprefiltered arms were not run (the prefilter changes no trial count).
5. Only the trial's own addition is harvested (jump step or restart sum), not the doublings
   inside scalar multiplications.
6. Genus 4, full base: harvested relations with 6 base places do not fit the 5-slot
   relation record and are dropped (A/A cells, see above).

## Not verified

No independent replay; no isolated host (wall times in the JSONL are on a shared, loaded
container and are not readings); 3 curves x 3 seeds per cell; field-operation prices
(`I = 10`, `kappa = g^2`) are the first study's conventions, not measurements.

## Possible next step (not tested)

The loss is in the degree-4 `x^p mod u'` test (about 200 extra native multiplications per
trial). Harvesting pays only if that test becomes cheaper than about one group operation's
worth of native work divided by the extra relation share, e.g. a cheaper early-abort
(distinct-degree step on the degree-2 part only) or batching the `x^p` powers across walks.
