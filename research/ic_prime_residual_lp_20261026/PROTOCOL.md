# Protocol: the residual collector (large-prime limit) on prime curves

Frozen 2026-10-26, before any recorded session of this study. Atlas lead
`LEAD-ICP-residual-partial-relations` (crypto-autoresearcher) and hypothesis
`H-PFDR-cb9606` ask for the prime-field measurement nobody made. Changes after
data go under "Deviations" in the README, never edited in here.

## Question

On prime curves, does crediting residuals lower the counted cost of the
`ic.pipeline` baseline? Crediting means storing every residual of a partial
decomposition and turning residual collisions into relations. The collector
is `ic_framework::residual` (`targets=residual`), the `|D| = r` limit of the
large-prime variation. Its Koblitz measurement closed it as a negative above
`2.4·(K r)^{1/4}` base points (`docs/ic/LEAD_LADDER_METHODOLOGY_20261010.md`
§L3, §7 item 4). It has never run on a prime field.

If the cost does fall, is the gain generic? Generic means a property of
collision search, which H-PFDR-cb9606 part E says must then be reported
without ECDLP language.

## Sessions (one per curve, `specs/<tag>.json` from `scripts/make_specs.sh`)

Registered prime-order curves:

| tag | slug | log2 r |
|:--|:--|--:|
| p21 | `icv1-fp21-t377-d7bba3fb` | 20.59 |
| p22 | `icv1-fp22-tm2947-e15fed69` | 21.19 |
| p24 | `icv1-fp24-tm5099-a6f137c6` | 24.00 |
| p26 | `icv1-fp26-tm1775-7e8fb6df` | 25.35 |
| p28 | `icv1-fp28-t28201-65099091` | 28.00 |
| p32 | `icv1-fp32-t7319-90da43e9` | 32.00 |

Arms:
- `rho`: `rho.negation`, the reference.
- `plan`: the `ic plan` output (`ic-planner/2`, w = 0), i.e. `N₀` abscissae,
  `mitm-prime-blocked:m=2`, `incremental-gauss`, `walk`.
- `res4`, `res8`, `res16`, `res32`, `res64`: the plan with
  `targets=residual` and `prime-abscissa:size=B`, for B ∈ {4, 8, 16, 32, 64}.
- `resN0`: the same collector at the plan's own base size `N₀`.

Design:
- 8 targets (target seed 20261031);
- 3 rounds + 1 warm-up, `alternate_measured`, measurement seed 20261101;
- L2, timeout 1,800 s.

## The curve-free null (H-PFDR-cb9606 part E)

`examples/ic_residual_curve_free_null.rs` runs the identical collector, with
identical seeds, on two carriers:
- the curve with its smallest-abscissa base of B points;
- `Z/r` keyed by a fixed random bijection, with B random base elements.

It prints both runs' group operations per solve. Runs: p21, p24, p28 ×
B ∈ {4, 16, 64}, 64 solves each, seed 20261102.

## Measures

- **Band and pairing:** `ecbench band --weights 0,0.1,1` per session, and
  `ecbench compare` (at w = 0 and with `--weight 1`) of every `res*` arm
  against `plan`, and of `plan` and the best `res*` against `rho`.
- **Exponents:** from the six sizes, the fitted slope of ln(total GAE) on
  ln r for `plan`, for `res16` and for each size's cheapest `res*` arm.
- **Null:** per (curve, B), the ratio of mean curve-free to mean curve group
  operations, with a bootstrap 95% interval over the 64 solves.

## Predictions

- P1. At every size the cheapest `res*` arm costs less than `plan` in counted
  `S`, with the paired interval below 1.
- P2. `S(resB)` grows with B at every size (about `√B`). The cheapest arm is
  `res4` or `res8`, i.e. the collector is best where the factor base is
  smallest.
- P3. `resN0 / plan > 1` at every size from p24 up: at the plan's base the
  direct table wins, as the Koblitz break-even says.
- P4. The fitted slope of ln(total GAE) on ln r is about 1/2 for `res16`
  and about 2/3 for `plan`.
- P5. Null: every curve-free/curve ratio interval contains 1. The gain is
  generic collision search.
- P6. Every `res*` arm stays above `rho` in `S`: the distance remaining stays
  above 1.

## Decision rule and class

- P1 and P5 together mean the gain is generic: a better collision search
  inside the generic pair-table pipeline, not a structural relation source.
  It is then reported as such, and it moves no leaderboard row by itself.
  Whether to admit "the cheapest measured `ic.pipeline` arm" with a tiny base
  is left to the owner, with this classification.
- P1 failing (no `res*` arm below `plan`) closes the lead on prime fields as
  a negative.
- A null interval excluding 1 would mean the collector depends on curve
  structure. That is unexpected, and it would be reported before anything
  else.
- Class: measurement (lead disposition). Counts only; band at 0, 0.1, 1.

## Inadmissible

- Changing curves, base sizes, targets or seeds after seeing a session.
- Dropping a run.
- Editing the collector between sessions.
