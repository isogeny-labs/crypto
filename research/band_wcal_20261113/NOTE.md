# Calibrated field-arithmetic weight in the ecbench band (2026-11-13)

Implements the AGENTS.md section 5a bullet of 2026-11-12 ("A calibrated
weight for field-arithmetic counters") in `src/cryptanalysis/ecbench/band.rs`.
The 0 / 0.1 / 1 band is unchanged and still governs the leaderboard tables.

## What changed

- `band::FIELD_ARITHMETIC_COUNTERS`: the counter class of base-field (and
  mod-`r`) multiplications and squarings, listed by name like
  `NON_OPERATION_COUNTERS`. Members: `factor_base_fp_muls_uncharged`,
  `root_search_field_muls_uncharged`, `oracle_fp_muls_uncharged`,
  `native_field_muls_uncharged`, `native_field_sqrs_uncharged`,
  `linalg_mulmods_uncharged`, `smoothness_field_muls_uncharged`,
  `endomorphism_field_muls_uncharged`, `transfer_field_muls_uncharged`,
  `scalar_mulmod_uncharged`, `lift_field_multiplies_uncharged`,
  `lift_field_squares_uncharged`, `min_key_field_squares_uncharged`,
  `base_point_sqrt_field_multiplications_uncharged`,
  `aps_f4_multiplications_uncharged`, `aps_root_multiplications_uncharged`,
  `aps_core_expression_multiplications_uncharged`, and the seven
  `pkm_*_fp_mults_uncharged` counters. Not members: inversions, square roots,
  additions, row operations, the lumped `field_arithmetic_uncharged` and
  `modular_combination_uncharged`, and the byte counters. The class is
  disjoint from `NON_OPERATION_COUNTERS`.
- `band::WCal { w_cal, method, host }`; `method` is `perf-ipc` or
  `best-of-N timing`, anything else is refused, as are `w_cal <= 0` and an
  empty host.
- `band::band_with_wcal(records, weights, pricing, Option<&WCal>)`.
  `band_with` calls it with `None`.
- `ecbench band --w-cal W --w-cal-method M --w-cal-host H` passes the input.
- `ecbench wcal --p P --a A --b B --host H` runs the helper
  `ecbench/wcal.rs`: best-of-N timing of a mulmod and an affine group addition
  (with its inversion) on `y^2 = x^3 + a x + b` over `F_p`, `p < 2^63`. It
  prints `ecbench.wcal/v1` JSON with method `best-of-N timing`, the minimum
  and the spread of the rounds. It never says `perf-ipc`; a perf-ipc reading
  comes from the `perf-ipc` skill on an isolated host and is passed to
  `band` by hand with that method.

## Output (additive)

With `w_cal`, `ecbench.band/v1` JSON gains a top-level `calibrated` object:

```
calibrated: {
  w_cal, method, host,
  field_weights: [0, w_cal, 10*w_cal],
  field_counters: [...],
  pairs: [{ curve, a, b,
            order_at: { "0": ..., "w_cal": ..., "10*w_cal": ... },
            ranked_at_calibrated_price: bool,
            band_verdict }]
}
```

For each of the three field weights the other native counters (everything
priced that is not in the class, plus calibration-priced native work) take
every weight of the band; an order is reported only if it holds at each of
them, otherwise `flips` (or `unrankable` for uncounted work or unverified
runs). `ranked_at_calibrated_price` is true only when the same strict order
holds at all three field weights. The band verdict is repeated beside it and
is not altered. The Markdown output gains a section with the same table.
Without `w_cal` there is no `calibrated` key at all.

## Compatibility evidence

- Unit test `without_w_cal_the_band_output_is_unchanged_on_committed_records`
  serialises `band_with` and `band_with_wcal(.., None)` for three committed
  sessions (`ic_planner_validation_20261020` r24, c24 and r26), under both
  pricings, and requires identical JSON without a `calibrated` key; with a `w_cal`, removing
  `calibrated` gives that same JSON again. [pcc:unverified]
- Five existing tests (four in `band.rs`, one in `compare.rs`) read
  `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`,
  which is an S3-held artifact absent from this checkout; they fail here with
  "No such file" before and after this change, so they could not be used as
  evidence. [pcc:unverified]
- Synthetic cases: an order that flips at the band's 0.1 and 1 but holds at
  0, `w_cal`, `10*w_cal` gives `ranked_at_calibrated_price: true`; one whose
  `10*w_cal` reaches the flip gives `false`; a byte counter changes no
  calibrated order. [pcc:unverified]

## Limits

- The counter class is a reading by name of counters that exist today. A new
  oracle's mulmod counter is in the class only once it is added to the list.
- The genus-3 positive control of `research/ic_positive_control_20261112/`
  used its own counters (`native_mul`, `la_mul`), not ecbench record
  counters; this change does not re-read it.
- The timing helper is a labelled best-of-N wall-clock reading in one process,
  not a perf-ipc measurement, and `w_cal` for a curve family other than
  short-Weierstrass prime curves must come from elsewhere.

## Replay with the built binary

`ecbench band --no-price-table-work` on `curve25519_algebraic_ecbench_scale_20261105`
sessions k28-c0-a1 and k28-c1-a1 equals the committed `analysis/band_k28.json`
in every field except the `table_work_mean_counts` listing, a field added on
2026-11-08 that the committed file predates; prices, ranks and counts are
equal to 1e-9. [pcc:unverified] `ecbench wcal --p 2147483647 --b 7` printed
`w_cal` 0.0285 with method `best-of-N timing` on a shared VM (mul spread 3.6x,
so informational only). [pcc:unverified] `scripts/build_ic_leaderboard.py`
reads `band.json["rows"]` only. Its `--check` already differs here for an
unrelated reason (missing VALIDATION.json artifacts), so it was not usable as
a gate.
