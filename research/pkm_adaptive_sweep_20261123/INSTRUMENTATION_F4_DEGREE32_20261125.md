# Instrumentation: where the degree-32 F4 saving comes from — 2026-11-25

Read from the three source-pinned equal-18 stores already published
(`degree32_source_pinned_equal18.jsonl`,
`…_20261221.jsonl`, `…_20261276.jsonl`; binary
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`). No new
benchmark was run for this note; every number is a field the stores already
carry. `ops` is the solver's elimination-only F_p multiply-add count;
`steps` and `refutations` are its own counters.

## Per-instance solver split

`int` = `prime-abscissa:below=32`, `cand` = `prime-isogeny:t=5,seed=5`.
Ratios are int / cand.

| Seed | Arm | F4 calls | UNSAT frac | Relations | MA/call | Steps/call | MA/relation | D |
|---:|:--|---:|---:|---:|---:|---:|---:|---:|
| 20261185 | int | 1645 | 0.854 | 219 | 127.4M | 11.73 | 957M | 6 |
| 20261185 | cand | 1610 | 0.871 | 208 | 4.0M | 12.51 | 31M | 5 |
| 20261221 | int | 2185 | 0.890 | 213 | 127.4M | 11.55 | 1307M | 6 |
| 20261221 | cand | 2101 | 0.894 | 222 | 4.0M | 12.42 | 38M | 5 |
| 20261276 | int | 2482 | 0.910 | 189 | 127.3M | 11.45 | 1672M | 6 |
| 20261276 | cand | 2934 | 0.930 | 204 | 4.0M | 12.27 | 57M | 5 |

Per-system multiply-add factor: **32.0×, 32.1×, 32.1×**. Per-relation factor:
**31.1×, 34.8×, 29.3×**. Calls per relation are the same order in both arms
(7.5/7.7, 10.3/9.5, 13.1/14.4). `budget_exceeded` is 0 everywhere; no call in
either arm hit the degree bound 8.

## Matrix shape (seed 20261221, worst row of each arm)

| Arm | Max rows | Max cols | Max nnz | nnz/row |
|:--|---:|---:|---:|---:|
| interval | 1498 | 929 | 282,095 | 188.3 |
| candidate | 821 | 583 | 44,894 | 54.7 |

## What this says about the three hypotheses

1. **UNSAT refutation dominates — supported.** 85–93 % of F4 calls in *both*
   arms are refutations. Total F4 work in these runs is overwhelmingly the work
   of deriving `1 ∈ I` on inconsistent systems. Caveat: the store records only
   aggregate `ops` per row, so the split of multiply-adds *between* refuted and
   consistent systems is not yet measured; the call-count fraction is a lower
   bound on the share only if per-call cost is uniform, which is unverified.
2. **Banded chain vs dense bit-coupled S₃ — supported, in a specific form.** The
   saving is *not* fewer elimination steps: the candidate takes slightly more
   steps per call (12.3–12.5 vs 11.5–11.7) and resolves at a lower solving
   degree (5 vs 6). It is per-system sparsity and width: 1.8× fewer rows, 1.6×
   fewer columns, 3.4× fewer non-zeros per row, 6.3× fewer non-zeros total, and
   32× fewer multiply-adds per system. So the degree-32 tower delivers its win
   inside each Macaulay reduction, exactly where a dense 5-bit expansion of
   `x_i` would inflate rows.
3. **Degree-bound censoring via denominator poles — not supported by the
   recorded data.** The reproducible 13-bit seed-10 failure
   (`scale13_instance_sweep.jsonl`, instance `icv1-fp13-tm43-d65e6f58`) reports
   `exhausted: true`, `targets_tried: 1000`, `relations_found: 6`,
   `budget_exceeded: 0`, `degree_over_bound: null`, `solving_degree_max: 5`,
   `refutations: 900 / 906 calls`. That is trial-cap exhaustion at
   hit rate 0.006 against an expected ≈0.03 — a relation-density failure on that
   instance, not an intermediate regularity blow-up past degree 8.

## Two-factor decomposition of the charged GAE

In these stores `total_gae` equals the solver's `gae` exactly: the walk and
`setup` components price to zero, so the charged ratio is *only* the solver.

| Seed | Arm | MA/call | GAE/call | GAE per MA | median ns/MA |
|---:|:--|---:|---:|---:|---:|
| 20261185 | int | 127.38M | 1708.4k | 0.01341 | 0.82 |
| 20261185 | cand | 3.98M | 1207.4k | 0.30354 | 11.30 |
| 20261221 | int | 127.40M | 1518.0k | 0.01192 | 0.65 |
| 20261221 | cand | 3.97M | 147.2k | 0.03705 | 3.25 |
| 20261276 | int | 127.33M | 894.1k | 0.00702 | 0.45 |
| 20261276 | cand | 3.97M | 212.1k | 0.05344 | 2.46 |

So the ratio factors as (32× fewer multiply-adds per system) × (3–14× more
GAE per multiply-add) = 1.4×, 10.3×, 4.2× per system, which reproduces the
observed combined ratios (0.51, 0.17, 0.24) once calls per relation are
included. The second factor is the pricing model following measured `ns/op`:
the candidate's own operations are individually more expensive in the measured
calibration, and the interval arm's median `ns/op` itself moved 0.45–0.82
across the three runs on the same field and binary.

Two consequences follow, and both weaken earlier wording in this note and in
`RESULT_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md`:

- The residual gap is **not** setup or target-walk cost. It is per-operation
  price. Driving the multiply-add count down further only helps if the
  per-operation calibration stays flat.
- Charged GAE is **not** load-independent here: it is `ops × measured ns/op`,
  and the measured `ns/op` varies by up to 1.8× across runs of the same arm.
  The "GAE is component evidence, timings are not" split in the result doc is
  too strong; GAE inherits part of the timing noise.

## Consequence for the next gate

The 14-bit rung therefore tests two things at once: whether the 32× per-system
multiply-add factor survives, and whether the per-operation price penalty stays
where it was. A 14-bit pass with a *worse* GAE ratio than 13 bits would suggest
the price factor is the size-dependent term, not the matrix shape.

## Claim boundary

Descriptive instrumentation of one selected 13-bit curve family at equal base
size. It explains the direction and constancy of the measured saving; it is not
a new performance claim, not an attribution of cost to individual refuted
systems, and not evidence about 14+ bits or any deployed curve.
