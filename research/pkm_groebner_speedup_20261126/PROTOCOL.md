# PKM Gröbner speedup: radix-16 qualifier

**Frozen:** 2026-11-26, before running the cells below.  All empirical
measurements are `[pcc:unverified]`.

## Question

The committed radix-grading study found a 2--6x counted-operation reduction
from radix 8 on exact Kummer PKM ideals, but explicitly did not run radix 16.
`f4_fp_tower` already implements radix 16 as weights `1,2,4,8` repeated in
each tower block.  Does that untested ordering produce a further drastic
reduction without changing the solution set?

This is an ordering experiment, not a claim that weighted solving degrees are
ordinary total degrees.  Degrees are compared only within the same grading.

## Frozen cells

- Instrument: `examples/pkm_tower_pilot.rs`, `--engine tower`, Kummer,
  shape-matched `tower` refutation systems.
- Prime and random draws: the instrument defaults used by
  `research/pkm_radix_grading_20261104`; seeds and full JSON rows are retained.
- Development: `m=2`, `t=8`, two random refutations, radix 2/8/16.
- Confirmation: `m=2`, `t=9,10`, two random refutations, radix 2/8/16.
- Breadth check: `m=3`, `t=4,5`, two random refutations, radix 2/8/16.
- Fixed execution: one release binary, `RAYON_NUM_THREADS=4`, no wall-clock
  claim.  Primary metric is deterministic `F_p` multiply-adds.  Secondary
  metrics are maximum columns, maximum dense entries, steps, and each arm's
  own weighted solving degree.

## Correctness and admission

The instrument must report the same `inconsistent` result for all three arms.
Any planted controls emitted by the command must remain valid.  Radix-2 and
radix-8 counts at overlapping cells must reproduce the committed study (apart
from its documented <=0.015% thread scheduling drift); otherwise the
experiment is invalid.

At the largest completed `t` in both `m` ladders, call radix 16 a **drastic
qualifier** only if, on every draw:

1. `muladds(radix16) <= 0.5 * muladds(radix8)`, and
2. `muladds(radix16) <= 0.1 * muladds(radix2)`.

If either condition fails, radix 16 is retained as a negative/constant-factor
result and is not integrated as a new PKM default.  A follow-up search over
non-radix admissible weights requires a separately frozen protocol and a
disjoint confirmation cell.

No result here establishes an exponent change, a lower ordinary solving
degree, or competitiveness with Pollard rho.

## Frozen continuation: complete cold target

**Frozen:** 2026-11-27, before running the continuation below. All empirical
measurements remain `[pcc:unverified]`.

### Question

Does the selected degree-32 five-Velu factor base reduce the complete native
pipeline cost, rather than only the F4 multiply-add counter, against the
matched 18-abscissa interval on the already selected 13-bit fixture?

### Fixed panel

- Instrument: `examples/pkm_groebner_drastic.rs` at the protocol commit.
- Instance seed `20261185`, planted scalar `2261`, `m=2`, degree bound `8`,
  1,000 target trials, five-second per-call solver budget, incremental Gauss.
- Arms: `prime-abscissa:below=32` and
  `prime-isogeny:t=5,seed=5,aux=ordinary`; each must expose 18 abscissae and
  the same 10-variable, 11-equation degree vector.
- Execution: one release binary, `RAYON_NUM_THREADS=4`, two serialized balanced
  pairs in order interval / candidate / candidate / interval. Each arm is
  reconstructed cold for every observation. Calibration is measured once
  before the panel and shared by both arms.
- Complete interval: arm construction through verified scalar recovery. The
  framework total includes factor-base construction, oracle setup, failed
  trials and relation collection, F4, relation-matrix work, linear algebra and
  verification. The outer cold wall interval additionally includes oracle
  construction.

### Metrics and gate

The primary metric is the sum of `RunReport.total_gae` over the four
observations per arm. It is a complete charged pipeline figure, but its F4
unit is host-measured because this generated curve has no repository-pinned
`ns_per_f4_muladd`; the receipt must retain that pricing mode. Deterministic
F4 multiply-adds and solving degree remain audit metrics. Summed cold wall is
descriptive only because the shared host is not load-isolated.

Call this a selected-fixture end-to-end speedup only if:

1. all eight runs recover and verify scalar `2261`;
2. cardinality, system-shape and ordinary solving-degree checks remain exact;
3. `sum(total_gae_candidate) <= 0.5 * sum(total_gae_interval)`; and
4. no phase or failed attempt is removed from the candidate accounting.

A wall-time regression does not invalidate the counted result, but it must be
reported and prevents a wall-speedup claim. Failure of the total-GAE gate is
retained as a component-only result. Passing does not establish a random
instance result, an asymptotic improvement or a Pollard-rho crossover.

## Frozen continuation: source-pinned three-fixture replication

**Frozen:** 2026-11-27, after the single-fixture result and before executing
any additional target. All empirical measurements remain
`[pcc:unverified]`.

### Question and fixed fixtures

Does the complete-pipeline reduction survive on all three 13-bit seeds that
the pre-existing size-only scan selected as exact 18-abscissa matches, without
searching for a new curve or target after seeing E2E cost?

- Instance seeds: `20261185`, `20261221`, `20261276`, exactly the panel in
  `RESULT_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md`.
- Planted scalar: `2261` on every fixture (reduced modulo the subgroup order
  by the instrument); no target retry or target selection.
- Arms, system parameters and accounting are unchanged from the complete cold
  target continuation above.
- Each fixture runs two serialized balanced pairs with
  `RAYON_NUM_THREADS=4`. The instrument is built once from the frozen commit;
  each process measures and shares one fixture-local calibration.

### Admission

Every run must pass the existing scalar, base-cardinality, system-shape,
phase-sum and ordinary-degree assertions. Retain every fixture regardless of
outcome. Call this a source-pinned three-fixture E2E replication only if:

1. each fixture has `candidate_total_gae / interval_total_gae <= 0.75`; and
2. the ratio of candidate and interval GAE summed across all three fixtures is
   at most `0.5`.

The wider per-fixture bound is fixed before execution to distinguish a
combined win from one seed carrying a regression. The combined gate remains
the original 2x total-cost threshold. Cold wall is descriptive; F4 pricing is
host-measured and must be disclosed. Passing still does not establish an
unselected random-instance result, an asymptotic improvement, transfer to
larger fields or a rho crossover.
