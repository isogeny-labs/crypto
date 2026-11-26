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
