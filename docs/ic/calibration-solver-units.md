# Pinning the solver and oracle unit keys

**Status (2026-11-12): protocol only, nothing run.** The keys below exist in
the code (`ic_boundary::SOLVER_UNIT_KEYS`, `Calibration::pin`,
`ic_framework::solvers::solver_unit_key`). No row of
[`calibration.json`](calibration.json) carries one. Until a row pins a key,
the work it would price stays a **lower bound** everywhere:

- `ic bench` prices solver blocks at this host's wall time (`measured`);
- `ecbench` takes that term back out and lists `solver_<unit>_uncharged`
  or `<counter>_uncharged`, so the record has `lower_bound: true`;
- the AGENTS.md §5a band charges the count at `w ∈ {0, 0.1, 1}` GAE per
  operation.

Pinning a key changes `S` for every later run on that curve. Pins therefore
follow the same rules as the 2026-09-21 table: frozen inputs, one host, the
median of repeated runs, and a note naming what was repriced. The FRAMEWORK
audit table (§3, "Solver and oracle work units") lists every unit, where it
is emitted and how it is priced today.

## What a key means

Every key is dimensionless, `ns_per_<unit> / ns_per_add`, measured on the
curve of the table row it is written to. `Calibration::pin` stores
`ratio · ns_per_add`, as it does for the older keys.

- **Solver-block keys** (`ns_per_conflict`, `ns_per_monomial_op`,
  `ns_per_monomial_test`, `ns_per_vector_xor_8x32`,
  `ns_per_vector_xor_16x32`, `ns_per_f4_muladd`, and `ns_per_fp_mul` when
  it prices a solver block) are each solver call's **whole wall time per
  counted unit**: `Σ wall_ns / Σ ops` over the calls of a run, divided by
  that run's measured `ns_per_add`. A pinned count is then charged as a
  whole-run proxy. This is why the key may charge a count that does not
  itemise every step, such as a SAT conflict.
- **Native-counter keys** (`ns_per_fp_mul` on the `pkm_*_fp_mults_uncharged`
  counters, `ns_per_hashed_tensor_step`) are per-primitive costs. They are
  measured on the primitive itself, because a phase's wall time also covers
  other work.
- **Refused even when pinned.** `ns_per_word_op` for `crossbred-f2`'s
  `word operations (partial: …)` is refused: the per-point linear solves are
  counted apart in `solve_row_ops`. `ns_per_monomial_op` for `xl-f2`'s
  `monomial operations (modelled)` is refused: the count is `terms × n²`,
  not the work done. Each refused count is priced as if no key were pinned,
  and the solver block's `pricing_qualification` says why. Do not pin these
  two to make the rows look complete. The count must change first.

## Where and with what

**Host.** The calibration host named in `calibration.json` is
`Intel(R) Xeon(R) Processor @ 2.10GHz`. The single Apple M4 Pro row is
marked for re-pinning on that host. Run on that host class through
[`isolab-eks`](../../.claude/skills/isolab-eks/SKILL.md) (CLAUDE.md default
route). Requirements: an isolated node; `.resume/RESUME.md` filled
(`scripts/checkpoint.sh init`); a `bench.record/v1` record
(`python3 tools/benchrec.py init|run|add-raw|metric|close`), with raw
reports in S3 by SHA-256. Record the image and commit in the record.

**Tooling.** [`tools/boundary_pin_calibration.py`](tools/boundary_pin_calibration.py)
exists, but it cannot pin these keys as written. It reads
`ledger.instances[].calibration` from `ic boundary` runs, where no solver
unit is measured, and its `UNITS` list stops at `ns_per_canon`. A pin needs
three things that do not exist yet:

1. an extension of that script, or a sibling script, that reads `ic bench --json`
   reports (`calibration.ns_per_add`;
   `rows[].decomposition.solver.{op_unit, ops, wall_ns, calls}`) and writes
   the median whole-call ratio per `(row, key)` into the existing
   `instances` rows, without touching any other key;
2. a primitive microbenchmark for `ns_per_fp_mul`: the oracle's own `mulm`
   (`u128` product reduced mod `p`) on the row's prime, built in the style of
   `ecbench::full_price::field_and_lookup_ns`;
3. a contraction timer for `ns_per_hashed_tensor_step`: wall time of the
   `hashed-tensor` contraction calls divided by `Meter::used` over the same
   calls, excluding the curve additions and square roots the meter also
   reports.

The committed reports
[`runs/ic-bench-solver-engines-2026-09-22.json`](runs/ic-bench-solver-engines-2026-09-22.json)
and [`…-n17-…`](runs/ic-bench-solver-engines-n17-2026-09-22.json) show the
input format (`buchberger-f2`, `sat-cdcl` and `exhaustive` solver blocks).
They are on `random-binary-n13-b1503` and `random-binary-n17-b1ad26`, which
have no table row, so they cannot be pin sources.

## Frozen inputs per key

A pin is written only to rows that already exist in `calibration.json`.
A curve without a row stays a lower bound. Each configuration below is run
**5 times** on the host, interleaved by key, with seeds `1..5`. The pinned
value is the **median of the 5 per-run ratios**, rounded to 6 decimals like
the existing rows. The director freezes the exact command lines in the task
card before any run. The executor runs them unchanged.

| key | engine / counter | configuration (`ic bench`) | table rows |
|:--|:--|:--|:--|
| `ns_per_conflict` | `sat-cdcl` | `binary-subspace:dimension=6` + `descent-algebraic:m=2` + `sat-cdcl`, walk targets | `char2/random-binary-n15-b524b`, `char2/random-binary-n18-b6507`, `char2/random-binary-n21-b1b6f3b`; Koblitz: `koblitz-orbit:divisor=0;1` + the same oracle on `koblitz/K_1 / GF(2^17)` (the ecbench test fixture) |
| `ns_per_monomial_op` | `buchberger-f2` | as above, `buchberger-f2` | same rows |
| `ns_per_monomial_test` | `exhaustive` | as above, `exhaustive` | same rows |
| `ns_per_vector_xor_8x32` / `_16x32` | `fes-f2-wide` | as above, `fes-f2-wide` on a host with AVX2 (8 lanes) or AVX-512 (16 lanes); the lane count is read from `op_unit`, never assumed | same rows; a host without the ISA pins nothing for that key |
| `ns_per_f4_muladd` | `pkm-semaev` F4 block | `prime-kummer:t=2` + `pkm-semaev:m=3` (the configuration of `pkm_semaev_solves_on_a_prime_curve_and_matches_the_grid`) | prime rows `prime/bench-16bit`, `prime/bench-18bit`, `prime/bench-20bit`; confirm with a dry run that the configuration recovers the logarithm on each row before freezing, and drop a row that does not |
| `ns_per_fp_mul` | `pkm_*_fp_mults_uncharged`; the grid block; the line oracles' bare `F_p multiplications` | microbenchmark (tooling item 2), 400,000 dependent `mulm` per rep, operands drawn from a fixed seed | every `prime/*` row |
| `ns_per_hashed_tensor_step` | `hashed_tensor_steps` | `hashed-affine` (its default `slots`, `tau` and `s`; seed frozen in the task card) + `hashed-tensor` with matching parameters on the prime row of matching size; contraction timer (tooling item 3) | `prime/generated-24bit-10935329`, `prime/bench-20bit` |
| `ns_per_word_op` | `crossbred-f2` | **do not pin**: refused until the count covers the per-point solves | — |

## Acceptance checks

These thresholds are proposed. The director freezes or replaces them in the
task card. A key is written to a row only if all checks pass. A failure is
reported with its numbers and pins nothing.

1. **Repeatability.** On each row, max/min of the 5 per-run ratios is at most
   1.25. (The 2026-09-21 drift that motivated pinning had a median of 1.08
   and a maximum of 3.70.)
2. **Proxy stability (solver-block keys).** On one row, the per-run ratio of
   two configurations that differ only in base dimension (for example 5 and
   6) is within a factor 1.5. If not, the count is not a stable proxy for
   the run on that row. `docs/units/BITOPS.md` records why this can fail for
   conflicts: 3,092–3,185 word operations per conflict on one arm and
   4,595–4,712 on another. The key is then not pinned.
3. **No zero-count work.** Every call with `wall_ns > 0` has `ops > 0`. If a
   call does work with a zero count (for example a SAT call decided by
   propagation alone), that work would go uncharged. The key is not pinned
   until the count covers it.
4. **Replay.** After the pin, a re-run of one frozen configuration per key
   shows `priced_by: pinned`, the key in `calibration_pins` (`ic bench`) and
   `calibration_pinned_units` (`ecbench`), and the solver term in `S`. The
   matching `solver_<unit>_uncharged` / `<counter>_uncharged` entry is gone
   from `unpriced`.
5. **No collateral repricing.** Re-running the 2026-09-21 table's sources
   with the new script reproduces every existing key of every row exactly.
6. **Band.** `ecbench::band::calibrated_native` takes the newly pinned GAE
   back out (test `a_pinned_solver_unit_key_is_taken_back_out_by_the_band`),
   so a §5a comparison still prices both arms alike.

After a pin lands, an `S` from before the pin and one from after it are not
comparable on that curve. The same holds between a pinned curve and an
unpinned one: the PR #1774 note on `is_word_xor_op_unit`
(`research/f4_pricing_jump_20261018/`) is the precedent. The PR that pins
must say which rows and keys moved and by how much on a frozen
configuration. Under the success criterion this is accounting, not an
improvement.

## `ns_per_hashed_tensor_step` and the hashed arm

`hashed_tensor_steps` is counted in three places (`ic_framework::hashed`):
the `hashed-affine` base build, the `hashed-tensor` oracle set-up, and every
relation trial's contraction. `run_pipeline` qualifies every hashed run
(`cost_qualification`) with "uncalibrated hashed_tensor_steps and omitted
tensor-curve certification work". For `hashed-tensor` it adds "tensor-
contraction primitive steps are uncalibrated, so S excludes most of the
relation phase".

Once a row pins `ns_per_hashed_tensor_step`:

- the steps are charged in every phase that counts them (`price_phase`,
  `ic_boundary::PINNABLE_NATIVE`);
- the step clauses leave the qualification;
- the run **stays qualified** ("hashed-affine setup omits tensor-curve
  certification work"), and `s_over_rho` stays empty (test
  `a_pinned_hashed_tensor_step_key_charges_the_steps`).

The hashed arm can leave `DiagnosticOnly` only when **both** of these hold:

1. `ns_per_hashed_tensor_step` is pinned under this protocol; and
2. the tensor-curve certification work is counted and charged in the
   `hashed-affine` set-up. That work is not counted today, and no key here
   covers it.

Until both hold, a `hashed-affine + hashed-tensor` row is a stage
diagnostic, as [`FRAMEWORK.md`](FRAMEWORK.md) §9 and
[`ECDLP_RESEARCH_TECHNIQUES.md`](ECDLP_RESEARCH_TECHNIQUES.md) (integration
order, item 6) say. The ecbench ids `ic.tensor_contraction_v1` and
`ic.tensor_contraction_cofactor_v1` (`crates/ecdl-tensor-ic`) are a separate
arm. They count their contraction rows as `*_uncharged`, not as
`hashed_tensor_steps`, and this key does not price them.
