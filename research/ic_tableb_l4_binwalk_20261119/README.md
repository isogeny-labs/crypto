# L4-BINWALK: the wwalk oracle on binary Table B curves

Status: code landed, unit test on B15 passes (see Test results). End-to-end
ecbench runs on B15-B27 are pending the main session. All claims here are
[pcc:unverified] assertions until a pcc claim is filed.

## What was prime-specific

1. The ecbench plan-time gate `pipeline_components_applicable`
   (`src/cryptanalysis/ecbench/methods.rs`) refused every `wwalk` arm unless
   the instance was `Instance::Prime`.  It also read the subgroup order and
   the curve from the prime instance.
2. `solve_ic_binary` had no `wwalk` entry in its oracle table, so the oracle
   name fell through to the binary plug-ins and was refused there.
3. Nothing inside `WWalkOracle`, `WalkTable` or `TargetWalk`
   (`ic_boundary.rs`, `ic_framework/plugins.rs`) was prime-specific for the
   plain, `tw`, `cj`, `lazy`, `lcheck` and `cv` paths: they are generic over
   `CountedGroup`.  The table key is `CountedGroup::key >> 1`, and
   `BinaryGroup::key` (`FastPoint::pack`) has the same shape as the prime key,
   `2(x+1) + sign`, with `-P` flipping only the low bit.

## What changed

- `methods.rs`: `wwalk` with a binary instance passes the gate, using the
  instance's `r`.  `aut=3` is refused on binary at plan time.
  `wsum` and the `prime-interval` base stay prime-only.  Prime behaviour is
  unchanged.
- `methods.rs`: `solve_ic_binary` builds `WWalkOracle::<FastPoint>`.
- `plugins.rs`: unit test `wwalk_binary_tests::wwalk_solves_b15_plain_and_target_walk`.

## What stays prime-only

- `sib`/`pool` (needs `CountedGroup::sibling_pair`, default `None`, so the
  build returns "needs a group with x-only siblings").
- `aut=3` (needs `order_three_aut`).
- `wsum`, `prime-interval`.

## Accounting

Every addition and doubling goes through `BinaryGroup::add/double`, which
counts `adds` and `doubles` exactly as the prime group does; set-up ladders,
walk, verification and probes are merged into the oracle's ops as on prime.
Caveat: `BinaryGroup` does not tally `field_ops`
(`counts_field_ops` is false), so the field-multiplication breakdown is
absent (unknown, not zero); GAE pricing uses the group-op counts.  The
native counters listed by `relation_counters` and `setup_work_native` are
the same ones as on prime.

## Arm strings (factor base `binary-multiplier:size=2,mult=1`, method `ic.pipeline`)

- `oracle=wwalk:x=2.0,g=1,grow=2,seed=run`
- `oracle=wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`

Run each on B15, B18, B21, B24, B27 (specs in
`research/ic_tableb_binary_ksum_20261115/specs/`) beside the matched rho.
`x` may need tuning at larger n; `r` must satisfy the coefficient packing
limits checked by `wwalk_plan_check_bits`.

## Test results

- `cargo test --release --lib -j 2 wwalk_binary`: 1 passed.  It recovers the log on B15 [pcc:unverified]:
  (r = 16319), with `wwalk:x=2.0,g=1,grow=2` and with
  `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,lazy=1`, 4 seeds each, and checks
  [d]G = Q independently of the pipeline.
- `cargo test --release --lib -j 2 wwalk`: 25 passed, 0 failed (the prime
  wwalk tests and the new one).
- Side repair: HEAD had a duplicated test
  `the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e` in
  `plugins.rs` that stopped the lib tests compiling; the first copy (without
  doc comment) was removed.
- Not run: ecbench end to end (the binary must not be rebuilt now).
