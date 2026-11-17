# Protocol: ECDLP cost across the levels of a prime-field ℓ-volcano

**Frozen 2026-10-18, before any session below was run.** Native tools only:
`crypto volcano explore` builds and verifies the volcano and writes the specs,
`ecbench` measures (AGENTS.md §12, "Implementation language: no Python").
This replaces the pure-Python rho experiment of
`research/volcano_descent_dataset_20261011/`, whose counted unit was not
ecbench's and whose confidence intervals ignored clustering (see that study's
addendum).

## Question

In an ℓ-isogeny volcano of ordinary curves over `F_p`, all curves have the same
`#E`. Does the level (crater, middle, floor) change the cost of the ECDLP in the
prime-order subgroup, for the generic solvers?

## Curves

`D = −7`, `ℓ = 2`, depth 8 (256 curves: 1, 1, 2, 4, 8, 16, 32, 64, 128 per level,
equal to the class numbers `h(O_{2^i})`; checked by `explore`). Two volcanoes:

| id | p | #E | r (prime subgroup) | cofactor | group structure |
|---|--:|---|--:|--:|---|
| `m0` | 40327618377097 (46 bits) | 2² · 11 · 916536492643 | 916536492643 (2^40) | 44 | `n1 = 2` at levels 0–7, 1 at level 8 |
| `m5` | 57628051254530177 (56 bits) | 2¹² · 14069348333597 | 14069348333597 (2^44) | 4096 | `n1 = 64, 64, 64, 32, 16, 8, 4, 2, 1` by level |

Two vertices per level (one at level 0 and one at level 1), each a curve of
trace `t` with a generator of the order-`r` subgroup (the specs list them:
`specs/*/layers.json`, `vertices.json` gives level and group structure). Six
targets per curve, two measured rounds and one warm-up.

## Methods

`rho.negation` (reference), `rho.plain`, and `rho.negation` repeated as a control.
Counts only (`isolation_required: L0`, run on a busy host).

## Prediction (H1)

Within each volcano and for each solver, mean counted operations are equal across
levels. Operationally: with `crater` = level 0, `middle` = levels 1–7, `floor` =
level 8, the 95% hierarchical-bootstrap interval (curve, then target) of each of
the ratios middle/crater, floor/crater and floor/middle contains 1.
H1 is falsified for a (volcano, solver, ratio) whose interval excludes 1.

## Why a difference could exist, stated in advance

Pohlig–Hellman removes the smooth part, so the group-structure variation in `m5`
should not matter; an effect would have to come from something the solver sees
in the curve. Nothing in the solvers uses the endomorphism ring. A failure of H1
would therefore be reported as an unexplained difference, not explained away.

## Rules carried over from the binary study

The `rho-neg-aa` control is bit-identical to `rho-neg` (deterministic seeds) and
carries no sampling information; the unit of replication is the workload, and
rounds are collapsed first. Twelve intervals are reported (2 volcanoes × 2 solvers ×
3 ratios); at 95% about 0.6 are expected to exclude 1 by chance, which is noted and
not used to discard a failure.

## Not claimed

Nothing about index calculus or any non-generic attack. Automorphism-folded rho
on the `j = 0` / `j = 1728` craters is not implemented in ecbench for prime curves
(round 2 of the conductor-gap study says the same); it was measured in Python in
the legacy study and is not repeated here. No claim about deployed curves.
