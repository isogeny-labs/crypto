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

## Follow-up 2026-11-13: records carry w_cal, every band reader surfaces it

Owner-approved follow-up to the section above. The band verdict still
governs; every addition is optional and absent without a `w_cal`.

### Where a w_cal lives and who reads it

- `session.json` `options.w_cal` `{w_cal, method, host}`, written by
  `ecbench run --w-cal W --w-cal-method M --w-cal-host H`. The key is
  skipped when absent, so a session written before this change reads and
  re-serialises as before.
- The sidecar `<session>/w_cal.json`, written by
  `ecbench wcal --p P --b B --host H --session DIR` (the measured JSON
  `ecbench.wcal/v1`, method `best-of-N timing`). `session.json` is not
  rewritten after it is sealed. The sidecar wins over `options.w_cal`.
  `runner::read_wcal` validates both (positive finite value, method one of
  `perf-ipc` and `best-of-N timing`, host named).
- `ecbench band`: the `--w-cal*` flags override; without them the sessions'
  own value is used. Several sessions must agree (all carry the same
  reading, or none does); otherwise the command refuses and asks for the
  flags (`runner::shared_wcal`).

### Where the calibrated reading appears

- `band`: `calibrated` (as before).
- `compare`: `ops.band.calibrated`, a `CalibratedGrid`: `Σ S_B / Σ S_A` over
  the matched pairs that passed the check [pcc:unverified] with the field class at `0`, `w_cal`,
  `10*w_cal` (rows) and every other native counter at 0, 0.1, 1 (columns),
  `order_at` per field weight (`B < A`, `A < B`, `flips`, `unrankable`) and
  `ranked_at_calibrated_price`. `ops.band.order` is unchanged. Not in the
  pooled multi-session comparison, and not with `--unit bitop/v1`.
- `challenge verdict` (procedure 2): top-level `calibrated`, the same grid
  with candidate as B and incumbent as A. No axis, outcome or `band` field
  changes.
- `bound fit` (fit version 2, not a weighted fit): `sizes[].calibrated`,
  the mean `S` per size on the same 3 x 3 grid. A bound fitted from sessions
  without a `w_cal` has no such key and keeps its id.

### Class membership test

`band::tests::field_multiplication_lookalikes_are_classed_or_excluded`
collects every name in the `cost.unpriced` lists of all committed
`research/**/records.jsonl` and every `"..._uncharged"` literal in `src/`.
A name with a word beginning `mul`, `sqr` or `squar` must be in
`FIELD_ARITHMETIC_COUNTERS` or in the new `FIELD_ARITHMETIC_EXCLUSIONS`
(each with a reason), else the test fails. It found, and this change
settles:

- added to the class: `cm_formula_muls`, `cm_formula_sqrs`,
  `elimination_modular_multiplications`, `large_prime_mulmods`,
  `explicit_x_zero_lift_squarings`, `scalar_muls` and
  `scalar_multiplications` (mod-`r` ring products), `linalg_mulmod`, and
  the solver-block names `solver_F_p_multiplications_(grid)` and the two
  `solver_F_p_multiply-adds_(...)` (a multiply-add counted as one
  multiplication). A phase-qualified label (`linear_algebra.linalg_mulmod_uncharged`)
  is classed by the part after the last dot, and the dynamic
  `complete_route_f2m<n>_{mul,square}_uncharged` is matched by pattern.
- excluded, with reasons: `aps_core_scalar_mults` (curve-point scalar
  multiplication), `aps_core_square_roots`, `sqrt_mod_p_calls`,
  `sqrt_solves` (square roots), `fp2_mul` (extension field),
  `line_field_multiplications` and `line_field_squarings` (trace-line
  extension field).

Adding these names changes only the `calibrated` readings, which exist only
with a `w_cal`; the band's rows and verdicts do not read the class.

### Limits

Evidence is by the tests named in the PR; none has a pcc claim.
[pcc:unverified] The pooled comparison and `--unit bitop/v1` carry no
calibrated block. `compare`, `challenge` and `bound fit` take the w_cal from
the sessions only; the override flags exist on `run` and `band`.

## Follow-up 2026-11-14: pooled compare, overrides, and `--unit bitop/v1`

Closes the three items the 2026-11-13 follow-up left out. Every addition
is optional; a session set without a `w_cal` and without an override
compares, fits and judges with the same bytes and ids as before.

### Pooled multi-session compare (`compare --dir D1 D2 ...`)

- `sessions[].calibrated`: each session that carries a `w_cal` (or every
  session, under an override) gets its own `CalibratedGrid`, in total cost
  (`S·√r`) like the rest of the pooled output.
- `calibrated` (top level, `PooledCalibrated`): the matched pairs of every
  session whose runs both count (`matched_verified`), pooled in total cost and read at `0`, `w_cal` and `10*w_cal`.
  The rule for differing readings is the conservative one:
  - every session carries the same reading: `rule: identical`, one grid;
  - every session carries one and the largest is at most
    `1 + POOLED_WCAL_TOLERANCE` (`0.10`) times the smallest:
    `rule: within tolerance`, one pooled grid at each distinct reading;
    `order_at` gives an order only where every grid gives it (else
    `flips`), and `ranked_at_calibrated_price` needs every grid ranked with
    the same order;
  - a session without a reading, or readings further apart:
    `rule: declined`, no grid and no order, `declined` says why, and the
    per-session grids stand. Such a set is not refused, because pooled
    comparisons that read no `w_cal` before this change must still run.
  - an override: `rule: override`, one grid.
- The sessions' own readings do not enter `comparison_id`.

### `--w-cal` override on `compare`, `challenge verdict` and `bound fit`

- Flags `--w-cal W --w-cal-method M --w-cal-host H`, as on `band` and
  `run`. They take precedence over `options.w_cal` and the `w_cal.json`
  sidecar. A value that is not finite or not above zero is refused (as is an
  unknown method or an empty host).
- The grid (or bound point) then carries `source: "override"`. With the
  sessions' own reading there is no `source` key, so grids written by the
  2026-11-13 change serialise unchanged.
- `compare` (single and pooled): the override enters `comparison_id` as
  `w_cal_override` (v2 identity only; the legacy id is unchanged), so a
  saved override comparison never overwrites the sessions' reading.
  Refused with `--unit bitop/v1`.
- `challenge verdict`: the override is refused for procedure 1, which has
  no calibrated block. It reaches the verdict's `calibrated` block and the
  bounds fitted from the session.
- `bound fit`: the override is kept as `fit_options.w_cal`, which is absent
  without one, so `bound check` fits it again with the same id. It is
  refused for fit version 1, with `--weight`, and with `--unit`.
- Behaviour change on `bound fit --unit bitop/v1|bitop/v2`: the sessions'
  own `w_cal` no longer adds `sizes[].calibrated`. Those records are
  re-priced in gates by `work::bitop_view`, so a GAE calibrated point there
  was mislabelled. No committed record carries a `w_cal`, so no committed
  bound changes.

### `--unit bitop/v1`: not implemented, no specification

`grep -rn "bitop/v1"` finds the unit's definition (`docs/units/BITOPS.md`,
`src/cryptanalysis/bitops.rs`, `ecbench::work`) and its counters. Nothing
in AGENTS.md, BITOPS.md, NOTE.md or PROGRESS_2.md defines a calibrated
reading in that unit. NOTE.md only records its absence. BITOPS.md
("Relation to the unpriced-work band") says a `bitop/v1` comparison
between complete arms has no unpriced counters, so the band has nothing to
weigh. It prices `field_mul` and `field_sqr` at frozen primitive weights
and keeps `--unit` and `--weight` exclusive. `w_cal` is a GAE-per-mulmod
ratio for counters left unpriced in GAE, and BITOPS.md gives no rule for
it in gates. No semantics were invented. The only change is the refusal of
a `--w-cal` override together with `--unit bitop/v1`, with the BITOPS.md
reason.

### Tests

- `compare::pooled::tests::pooled_calibrated_is_absent_without_w_cal_and_additive_with_one`
- `compare::pooled::tests::pooled_calibrated_is_the_hand_computed_ratio_of_total_cost`
- `compare::pooled::tests::differing_w_cal_pools_within_tolerance_and_declines_beyond_it`
- `compare::pooled::tests::a_w_cal_override_wins_is_recorded_and_bad_ones_are_refused`
- `band::tests::sidecar_and_override_reach_single_and_pooled_compare_from_directories`
- `bounds::tests::a_w_cal_override_is_recorded_re_derives_and_is_refused_where_meaningless`
- `challenge::tests::a_w_cal_override_needs_procedure_2_and_a_positive_finite_value`
  covers only the refusals. No test fixture builds a procedure-2
  verdict, so the override's path into a sealed verdict is covered by code
  reading and by the bounds test of the same option, not by its own test.

Evidence is by these tests; none has a pcc claim. [pcc:unverified]

## Follow-up 2026-11-15: validator findings B (research/validation_20261114/REPORT.md)

Three open findings from the independent validation are fixed in
`src/cryptanalysis/ecbench/challenge.rs`.

1. **Procedure 1 no longer reads `w_cal.json`.** `verdict` called
   `runner::read_wcal` for every procedure, so a malformed or invalid
   sidecar made a procedure-1 verdict fail although procedure 1 has no
   calibrated block. The verdict's paired comparisons (`compare`, for ops
   and the A/A control) also read it. Procedure 1 now reads no `w_cal`
   anywhere: its comparisons run on the session the verdict loaded, with
   `options.w_cal` cleared (`procedure_1_compare`).
   Procedure 2 is unchanged: it still reads, validates and refuses a bad
   sidecar. Test:
   `challenge::tests::an_invalid_w_cal_sidecar_leaves_a_procedure_1_verdict_unchanged`
   copies the `prime-toy-reference-e1` session to a temporary root at the
   same relative path, writes an invalid sidecar, and checks that the
   procedure-1 verdict text is identical to the one without it and that
   procedure 2 refuses the sidecar.
2. **Procedure-2 calibrated block, end to end.** Test
   `challenge::tests::a_procedure_2_verdict_carries_the_calibrated_block_only_with_w_cal`
   builds procedure-2 verdicts of the same committed session. Each uses a
   synthetic commitment and beacon, so the verdict is inadmissible: the
   spec does not match, and the block is computed anyway. The test covers
   three cases: no `w_cal` (no block), a `--w-cal` override (source
   `override`) and a sidecar (no source). With `w_cal = 0.1` the field
   weights are `[0, 0.1, 1]`, equal to the band weights. The grid's
   diagonal is then checked against a hand computation over pairs matched
   by workload and round: `Σ S_B / Σ S_A` and `priced_s` at 0.1 and 1.
   The block changes no axis or outcome.
3. **The committed procedure-1 verdicts re-derive again.** Commit
   `c7ae83ee81` (2026-10-23, "honest intervals ... in compare") moved
   `compare` to `ecbench.comparison/v2`. That commit made the linearised
   `t` interval primary (`ci_method` `linearised_t`) and kept the old
   percentile two-stage bootstrap beside it as `ci95_bootstrap`, with the
   same statistic, resamples and seed. It did not touch `challenge.rs`,
   so procedure-1 verdicts silently took the new ops interval and
   per-curve intervals. The re-derived id for `prime-toy-reference-e1`
   became `ECVD1he794a2187563` instead of the committed
   `ECVD1h0b0f5e0d7f67`. A procedure-1 verdict is frozen, so the fix reads
   the comparison's bootstrap interval back for procedure 1
   (`procedure_1_compare`, for the ops and control comparisons).
   Procedure 2 keeps the linearised interval. The committed records are
   not edited. Test
   `challenge::tests::the_committed_procedure_1_verdicts_re_derive_byte_for_byte`
   re-derives both committed verdicts (`prime-toy-reference-e1` and
   `koblitz-toy-reference-e1`) with every run replayed and `--bounds
   docs/bounds/records`, as `docs/bounds/refit.sh` does. It checks each
   verdict file and the bound it wrote byte for byte.

Evidence is by these tests; none has a pcc claim. [pcc:unverified]
