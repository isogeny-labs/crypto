# Integer-multiple closure on random binary (and generic prime) curves

**Date.** 2026-10-23. **Status.** Measured in `ic bench` (operation-counted
`S = total group-addition equivalents / sqrt(r)`; every row `correct = yes`). Toy sizes.

## What changed

* New factor base `binary-multiplier:size=K[,mult=M]`
  (`src/cryptanalysis/ic_framework/plugins.rs`, registered in `src/bin/ic/bench.rs`):
  each seed `P` (smallest abscissae, projected into the order-r subgroup)
  contributes `[m]P` for `m = 1..M` on its own column with weight `m`, built by
  repeated addition. No endomorphism is used, so it applies to a **random**
  binary curve. Unit test: every point is `[coef]` times its column seed.
* `cm-multiplier` gains `mult=M` (same closure on prime curves, no
  endomorphism needed).

Run:

    ic bench --char2-degree 28 --seed 5 --factor-base binary-multiplier:size=8,mult=32 \
      --oracle mitm:negation_folded=1,m=3 --targets walk --repeats 4

## Measurements (mean S over 4 targets, 3-summand `mitm`, negation folded)

| curve | best `binary-subspace` | best plain seeds (`mult=1`) | best multiplier base | vs subspace / vs plain |
|---|---:|---:|---:|---:|
| random binary, degree 28, seed 5 | 64.1 (dim 10, 1,012 pts) | 40.5 (384 cols) | 10.5 (8 cols, mult 32, 512 pts) | 6.1x / 3.9x |
| random binary, degree 28, seed 6 | 90.6 (dim 10) | 38.7 (384) | 12.5 (4 cols, mult 64) | 7.2x / 3.1x |
| random binary, degree 26, seed 5 | 54.1 (dim 9) | 32.5 (256) | 9.3 (3 cols, mult 48) | 5.8x / 3.5x |
| generic prime, 20 bit (only size in the roster) | 22.9 (`prime-abscissa` 80) | same | about 10 (4 cols, mult 16) | 2.3x |

Seeds-only collapse to a point: 1-2 columns are far worse (S 98 at 2 columns,
6,053 at 1 column, degree 28), because multiples of one point carry arithmetic
structure that makes many table collisions trivial. The useful range is a few
to a few dozen columns.

## What this is not

* A constant factor on the generic `mitm` table pipeline only, **not** a gain over
  published index calculus and not over rho. The best rows are still about 6x
  to 12x the framework's rho reference (S about 1), and as `K -> 1` the base is a
  baby-step/giant-step table over multiples, which is the generic bound the
  pipeline is approaching, not passing. The exponent is unchanged.
* `binary-subspace` is the base the *algebraic* oracle (`descent-algebraic`, F4)
  needs, because the subspace structure is what makes the Weil-descended system
  small. The multiplier base has no such structure, so it does **not** help the
  algebraic route; the comparison above is `mitm` against `mitm` only. No lead
  for the algebraic route (solving degree, w) was found to beat its baseline.
* Toy sizes (degree 26-28; degree 30 instance generation exceeded the 250 s timeout).
