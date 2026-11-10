# Protocol: two engine items on the Curve25519-family miniatures, single-target ecbench (frozen 2026-11-10)

Frozen before any measured data of this study. Nothing below changes after the first session
runs; failed predictions stay failed and are reported as failed. Before freezing, the only
executions were the unit tests of the two options (tiny test curves and the k = 20 curve, seeds
outside this study's), `ecbench plan` (no run) on this study's specs, and the pre-data model
`scripts/model.py`, which reads code-derived counts and the committed
`research/curve25519_tw_jumps_20261108/data/pooled_measured.json` and no data of this study.

## 1. Questions

The current best generic index-calculus arm on this family is the target-in-walk F = 1 `wwalk`
oracle with chained jumps (`twcj`: `wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=16,seed=run` on
`prime-projected-abscissa:size=1,scan_cap=32`), 1.34x to 1.40x behind rho-neg at w = 0 from
k = 20 to 32. The coverage report (`research/curve25519_ic_coverage_20261108/REPORT.md`, section 3,
items 3 and 4) designed two engine changes and left them unrun.

- **(A)** Does dropping the per-collision recomputation in `TargetWalk` (option `cv=0`) lower the
  arm's cost by the collision-verification ledger (0.256 sqrt(r) at k = 20) and nothing else?
- **(B)** Does building rho-neg's r-adding jump table by chained additions
  (`rho.negation_chained`) lower the reference's cost by the saved set-up, and by how much?

(B) changes the reference the programme is measured against, not any arm.

## 2. Implementation under test

- **(A)** `wwalk` parameter `cv` (default 1, unchanged). With `cv=0` (needs `tw=1`)
  `TargetWalk::build_cv` does not recompute the two entries of a key collision. The table key is
  the point's abscissa (`PrimeCurve::key >> 1`), so a key collision is exactly `E_t = ±E_s`; the
  key's sign bit, stored beside the coefficients, gives the sign. `verify_ops` stays empty; the
  rows, the rank-2 stop and the walk are those of `cv=1` from the same seed. The relation loop's
  final `[d]G = Q` check is kept and charged; a failed check ends the run as a failed solve
  (`verification_failures`), never retried. `cv=0` adds the counters `tw_collision_verify = 0`
  and `tw_unverified_collisions`; `cv=1` emits exactly the committed counters.
- **(B)** method id `rho.negation_chained` (a new id, so `rho.negation` and every committed
  session replay unchanged): `rho_walk_with_policy(NegationClasses, RhoWalk::negation(),
  WalkPolicy::chained())`. The first two jumps are joint scalar multiplications
  `[a]G + [b]Q` with coefficients uniform in `[1, 2^9)` as the independent table draws them;
  every later jump is `M_j = M_{j-1} + M_{j-2}`, one charged addition, coefficients added
  modulo r. Walk, starts, distinguished points, look-ahead, cycle window and charged final
  check are `rho.negation`'s. Counter `chained_setup = 1`.
- Unit tests: `ic_boundary::wsum_table_tests::target_walk_without_collision_verify_keeps_rows_and_drops_the_ledger`,
  `ecbench::methods::engine_items_tests::*` (4 tests); every existing test unchanged.
- Binary: `/home/user/ctarget-f1/release/ecbench` built from this study's commit; SHA-256
  recorded per size in `data/binary-k{20,24}.sha256` when the sessions start.

## 3. Design (frozen)

- Curves: k = 20 (`p = 1,048,573`, `r = 131,203`) primary; k = 24 (`p = 16,777,213`,
  `r = 2,097,349`) replicate. The same explicit curves as the earlier studies.
- Sessions: 8 per k, each `targets_per_curve = 1` with its own `target_seed`
  (3026115000 + 100k + s) and measurement `seed` (3026111000 + 100k + s), disjoint from the
  earlier studies' seeds (`scripts/make_specs.py`). Each arm runs 64 measured rounds plus 1
  warm-up, `order = alternate`. Warm-up records are excluded from every pooled figure.
- Arms:
  - `rho-neg` (reference): `rho.negation`.
  - `rho-neg-chained` (candidate reference, item B): `rho.negation_chained`.
  - `twcj` (baseline, the current arm): the pipeline arm above.
  - `twcj-cv0` (candidate, item A): the same with `cv=0`.
- Counted unit: ecbench GAE; `S = cost/sqrt(r)`; every comparison at w = 0, 0.1, 1
  (AGENTS.md section 5a), under the current default band reading (table work priced) and under
  `--no-price-table-work`.
- Audit: `ecbench verify --replay 1` per session.
- Pooling: `scripts/pooled_compare.py`, copied from `research/curve25519_tw_jumps_20261108`
  with one change: a record whose outcome status is not the success status no longer aborts the pooling; its
  cost stays in the sums (the work was spent) and it is counted as a failed solve. Per session,
  sum S over the measured rounds per arm; ratio of sums; linearised t interval with df = 7. One
  session is one target and one cluster.
- Analysis: `scripts/analyse.py` -> `data/pooled_measured.json` (means, ratios under both
  readings, phase breakdown, failures, per-run identity check for A, rho walk counters for B).
- Runtime cap: 30 minutes in total.

## 4. Model (derived, before data; `scripts/model.py` -> `data/model.json`)

- (A) With the same seed `cv=0` walks the same walk and emits the same rows, so per run
  S(cv0) = S(twcj) − tw_verify_adds/sqrt(r). From the committed tw_jumps phase table:
  ratio 0.880 at k = 20 (verify 0.256 of 2.136) and 0.950 at k = 24 (0.079 of 1.583) at w = 0;
  with table work priced 0.888 / 0.928 (k = 20, w = 0.1 / 1) and 0.954 / 0.973 (k = 24).
- (B) `rho_jumps_for(r) = 8` at both sizes with 9-bit coefficients. The expected joint
  double-and-add on two uniform 9-bit coefficients is 13.43 GAE (exact enumeration), so the
  chain saves 6 × 13.43 − 6 = 74.6 GAE: 0.206 sqrt(r) at k = 20 and 0.051 at k = 24. Against
  the committed rho-neg S (1.573, 1.179) that is ratio 0.869 at k = 20 and 0.956 at k = 24 at
  w = 0, if the walk is unchanged. The chained table has small integer relations among its jumps
  (`M_j − M_{j−1} − M_{j−2} = O`), which the independent table avoids on purpose
  (`jump_coefficient_bits`); with the negation map they open short fruitless cycles (an order-of-
  magnitude estimate is a few tenths of a percent of steps), caught by the 16-point cycle window
  and escaped by doubling. The model does not price that; B3 tests it.
- The model's B point (0.869) lies below the band assigned to B1 ([0.91, 0.95], from the
  coverage report's estimate of the saving, 0.08 to 0.14 sqrt(r), which extrapolated from the
  target-in-walk set-up). Both are frozen and scored separately.
- Distances at w = 0 implied by the model (k = 20): twcj / rho-neg 1.358, twcj / rho-neg-chained
  1.563, cv0 / rho-neg 1.195, cv0 / rho-neg-chained 1.375; (k = 24): 1.343, 1.404, 1.276, 1.334.

## 5. Predictions and thresholds (frozen)

"Ratio" is the pooled ratio of sums with its 95 % interval over 8 sessions.

- **A1 (primary A).** k = 20, w = 0: S(twcj-cv0) / S(twcj) point ≤ 0.95 and upper end < 1.
  Point expected 0.88 (A1b: point in [0.86, 0.90], scored separately).
- **A2.** k = 24, w = 0: point in [0.93, 0.97] and upper end < 1.
- **A3 (identity).** For every paired measured run (same session and round) the two arms have
  equal `tw_steps`, `tw_rows`, `tw_rank` and recovered logarithm, and
  total_gae(twcj) − total_gae(twcj-cv0) = `tw_verify_adds` of the twcj run, exactly.
- **A4.** Both sizes, table work priced, w = 1: upper end < 1.
- **B1 (primary B, assigned).** k = 20, w = 0: S(rho-neg-chained) / S(rho-neg) point in
  [0.91, 0.95].
- **B2 (code model).** k = 20, w = 0: point in [0.84, 0.90]; k = 24: point in [0.93, 0.99].
- **B3 (no material walk penalty).** Both sizes: the pooled ratio of the search phase
  (walk operations, starts, look-ahead and escapes; everything but set-up and the final check)
  of rho-neg-chained to rho-neg is in [0.95, 1.05].
- **B4 (set-up).** Both sizes: the mean set-up of rho-neg-chained is within 10 % of the model
  (61.5 GAE at k = 20, 68.1 at k = 24).
- **C (no failed solve).** Every measured and warm-up record of every arm has outcome status `verified` (the success
  status; this is a prediction, not a claim) [pcc:unverified]; in
  particular no `twcj-cv0` and no `rho-neg-chained` run fails its final check. A failed check is
  counted as a failed solve, never retried.
- Every session's `ecbench verify --replay 1` must reproduce. A session that fails the audit is
  excluded and reported; it is not rerun.

## 6. Reporting

README.md tabulates measured S per arm and weight under both readings, the pooled ratios for
A and B at both sizes and all weights, the status of every prediction, the phase breakdown,
the distance to rho under both references (rho-neg and rho-neg-chained), and the scope
statement: these are constant-factor changes on the generic track; the F = 1 arm is one unknown
from rho and is not decomposition-based index calculus; (B) changes the reference, not any arm,
and lowering the reference raises every arm's distance to rho. Every result sentence ends with
[pcc:unverified].

## Addendum 1 (2026-11-10, after the validator's report; a protocol amendment, not a retune)

Everything above this heading, up to and excluding the blank line before it, is the frozen text
of 2026-11-10 (SHA-256 `36f33d55...29e6`, the first line of `data/frozen.sha256`). It is not
altered. This addendum is appended after the first
16 sessions ran and after the independent validator's report (`validator-report.json`), and before
any session of the amendment runs. Its SHA-256 is recorded in `data/frozen.sha256` (second line,
over the bytes from this heading to the end of the file) before the first amendment session starts.

**Defect E1.** Section 2 says the key's sign bit is "stored beside the coefficients". The measured
code (`TargetWalk::build_cv` at commit `b543ece570`) kept it in a second hash map (`signs`): one extra
hash insert per stored entry and one extra lookup per collision, about one extra hash operation per
walk step (499.2 per run at k = 20 and 1,973.6 at k = 24, the counted `table_insert_attempts`). No
counter recorded that work, so the first 16 sessions under-charge the `twcj-cv0` arm at w > 0 in the
default reading. The w = 0 rows (A1, A1b, A2) and the identity A3 are unaffected; A4 and the
w > 0 (A) and cv0-distance rows of the first sessions are lower bounds for the cv0 arm.

**Fix.** The sign bit now rides in the top bit of the stored entry's `u` word (`TW_SIGN_BIT`,
bit 63; every coefficient is below 2^32 by `wwalk_plan_check`), the `signs` map is removed, and the
walk makes exactly one table access per non-identity step in both modes (`table_probes`, tested
against `insert_attempts`). The default `cv=1` path stores entries unchanged (the bit is never set)
and emits the committed counters; `TargetWalk::probe` masks the bit. Steps, rows, rank, collisions
and the recovered logarithm are those of the measured code (unit test
`target_walk_without_collision_verify_keeps_rows_and_drops_the_ledger`, extended).

**Re-measurement.** The (A) predictions A1, A1b, A2, A3 and A4 are re-decided, with the thresholds of
section 5 unchanged, on fresh sessions with new seeds, on a binary built from the fixed code:

- design of section 3 unchanged (8 single-target sessions per size, 64 measured rounds plus
  1 warm-up per arm, `order = alternate`, k = 20 and k = 24, the same curves), with target seed
  3026125000 + 100k + s and measurement seed 3026121000 + 100k + s (s = 0..7);
- arms `rho-neg`, `twcj` and `twcj-cv0` as in section 3; `rho-neg-chained` is dropped, since (B)
  was rejected and its code is unchanged;
- audit `ecbench verify --replay 1` per session; both band readings; pooling and intervals by the
  frozen `scripts/pooled_compare.py` and a copy of `scripts/analyse.py` restricted to these arms;
- the cv0 distance to rho (`twcj-cv0 / rho-neg`) at w = 0, 0.1 and 1 under both readings is reported
  as the distance remaining, not as a gate;
- files under `amendment1/` (`specs/`, `sessions/`, `data/`, `scripts/`); a benchrec record per size.

The verdicts of the first 16 sessions stay as scored there, including every failed (B)
prediction; the amendment's verdicts are reported beside them, not instead of them. A failed
amendment prediction stays failed.
