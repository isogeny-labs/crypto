# Protocol: where should the planner route large Koblitz curves?

Frozen 2026-10-26, before any recorded session of this study. `ecbench` gained
`ic.koblitz_descent` in b561ebca8: the ledger §23 one-target pipeline, charged in
counted group operations. Its logs and native counts are identical to
`ic price --single-target` on the same seeds. Changes after data go under
"Deviations" in the README, never edited in here.

## Question

For Koblitz curves at log2 r 36–45, the leaderboard's Table A reports the §23
descent at 4.7–11.8× rho. Those figures are in mixed units, so they could not
be ranked against the planner's `ic.pipeline` recipe (`compact-orbit-scan` +
`mitm-frobenius-counted:m=2`, predicted 18× rho at 2^39). In one unit and one
session, which recipe is cheaper, and by how much against the strong rho?

## Sessions (`specs/<tag>.json` from `scripts/make_specs.sh`)

Curves are the §23 sizes at which `ic.koblitz_descent` has its rule v8 defaults:

| tag | curve | log2 r |
|:--|:--|--:|
| k47 | `koblitz` a = 1, n = 47 | 36.6 |
| k41 | `koblitz` a = 0, n = 41 | 39.0 |
| k53 | `koblitz` a = 0, n = 53 | 44.3 |

Arms:
- `rho`: `rho.signed_frobenius_strong`, the §23 rule's strong reference.
- `pipeline`: the `ic plan --koblitz a:n` arm (`ic-planner/2`).
- `descent`: `ic.koblitz_descent` with its `rule_v8` defaults.

Design:
- 8 targets (target seed 20261103);
- 3 rounds + 1 warm-up, `alternate_measured`, measurement seed 20261104;
- L2, timeout 1,800 s per run.

## Measures

- `ecbench table` (mean S per arm).
- `ecbench compare` of `descent` against `pipeline`, and of each against
  `rho`, in counted S (w = 0), with the paired cluster 95% interval.
- `ecbench band --weights 0,0.1,1`.

`ic.koblitz_descent` leaves four native categories without a count:
- field arithmetic outside the group law;
- hashing;
- the descent's modular combination;
- declined AVX-512 lanes.

The band therefore cannot price it above w = 0, and an order at w = 0.1 or 1
is "not ranked". That is reported, not filled in.

## Predictions

- P1. `descent / pipeline < 1` at all three sizes, interval below 1.
- P2. `descent / rho` grows with log2 r, as §23's slope says. It stays above
  1: the distance remaining.
- P3. Every run of every arm verifies.

## Decision rule

- **Routing.** If P1 holds at a size, the planner routes Koblitz curves in
  `descent`'s supported range to `ic.koblitz_descent`, as the counted-S
  (w = 0) choice. It says that the band cannot rank it until its native work
  is counted, and the routing comes with a new planner version.
  - Sizes outside the measured span are `extrapolated`.
  - Below the smallest measured size the planner keeps `ic.pipeline`.
- **Mixed result.** Where P1 fails, the planner keeps `ic.pipeline` at that
  size.
- **Class.** Engineering: a routing choice between existing recipes. If the
  descent is cheaper, the Table A row's unit becomes comparable. Re-pricing
  Table A in this unit is the owner's call, and no leaderboard row moves here.

## Inadmissible

- Changing curves, arms, targets or seeds after seeing a session.
- Dropping a run.
- Editing either method between sessions.
