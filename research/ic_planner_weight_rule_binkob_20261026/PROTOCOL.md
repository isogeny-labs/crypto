# Protocol: does the weight-aware base size carry to binary and Koblitz curves?

Frozen 2026-10-26, before any recorded session of this study. Planner
`ic-planner/2` (`src/cryptanalysis/ic_framework/planner.rs`, branch head at the
freeze commit). Changes after data go under "Deviations" in the README, never
edited in here.

## Question

`research/ic_planner_weight_rule_20261021` found that on prime curves the cheapest
factor base grows by 1.41× when the unpriced native work (lookups, guard probes,
Legendre symbols, square roots, row operations) is charged at w = 1 GAE per
operation (AGENTS.md §5a). It stayed put at w ≤ 0.3, and that held out on
four curves. `ic-planner/2` applies the weight only on prime curves and warns
elsewhere.

Does the same hold for the planner's two other recipes?
- **Random binary curves:** `binary-subspace:dimension=d₀`, base `2^{d₀}`.
- **Koblitz curves:** `compact-orbit-scan:columns=c₀`.

## Phase A: derivation (this protocol)

Curves, all registered:

| tag | curve (`ecbench` kind) | slug |
|:--|:--|:--|
| b21 | `binary_random` n = 21, seed 827675, max cofactor 8 | `icv1-f2m21-t2845-e2683765` |
| b23 | `binary_random` n = 23, seed 827675 | `icv1-f2m23-tm1957-8891f5a2` |
| b25 | `binary_random` n = 25, seed 827675 | `icv1-f2m25-t4775-4830dca7` |
| k23 | `koblitz` a = 0, n = 23 | `icv1-f2m23-t5197-69e76b73` |
| k31 | `koblitz` a = 0, n = 31 | `icv1-f2m31-tm90707-c95f16f5` |
| k37 | `koblitz` a = 0, n = 37 | `icv1-f2m37-tm534059-32aad96b` |

Arms per curve, one session per curve (`specs/A-<tag>.json` from
`scripts/make_specs.sh A`). Each session has a reference, `plan`, and a set
of base-size alternatives:
- **Reference:** `rho.negation` on binary curves, `rho.signed_frobenius` on
  Koblitz curves (the planner's references).
- **`plan`:** the `ic plan` output (`ic-planner/2`, w = 0).
- **Binary alternatives:** dimension `d₀ + k`, k ∈ {−2, −1, +1, +2}, i.e. base
  multiplier `m = 2^k` ∈ {¼, ½, 2, 4}.
- **Koblitz alternatives:** columns `max(1, round(m·c₀))` for m ∈ {0.5, 0.71,
  1.41, 2, 2.83, 4}. Multipliers that give the same column count are run
  once, under the smallest such m; the arm's m is its columns / c₀.

Every other stage is the plan's. Design per session:
- 8 targets (target seed 20261027);
- 3 rounds + 1 warm-up, `alternate_measured`, measurement seed 20261028;
- `isolation_required: L2` (only counts are compared), timeout 1,800 s.

Measures:
- `ecbench band --weights 0,0.03,0.1,0.3,1` per session;
- `ecbench compare --weight w` of every alternative against `plan`;
- `ecbench verify` with 12 replays.

### Derivation rule (fixed now, per family)

For a family, weight `w ∈ W = {0, 0.03, 0.1, 0.3, 1}` and multiplier `m`:
- `G(m, w)` is the geometric mean, over the family's three curves, of
  `mean S_w(m) / mean S_w(plan)` (band means).
- Koblitz curves enter at the arm whose columns are `max(1, round(m·c₀))`.
- `m*(w) = argmin_m G(m, w)`; ties go to the smaller m.

## Phase B: held out (an amendment, frozen after Phase A, before its sessions)

Curves in none of the planner's evidence and not in Phase A, chosen now:

| tag | curve | slug |
|:--|:--|:--|
| hb19 | `binary_random` n = 19, seed 59297 | `icv1-f2m19-tm649-a9133fe6` |
| hb23 | `binary_random` n = 23, seed 59297 | `icv1-f2m23-tm4129-45500fce` |
| hb25 | `binary_random` n = 25, seed 59297 | `icv1-f2m25-tm489-c50a8fb9` |
| hb27 | `binary_random` n = 27, seed 827675 | `icv1-f2m27-t20811-3d0fd5a1` |
| hk19 | `koblitz` a = 1, n = 19 | `icv1-f2m19-tm797-9c54981b` |
| hk23 | `koblitz` a = 1, n = 23 | `icv1-f2m23-tm5197-1f85e9e1` |
| hk39 | `koblitz` a = 0, n = 39 | `icv1-f2m39-t1481485-829ef1d1` |

Arms: the reference, `plan`, and one alternative per distinct `m*(w) ≠ 1`
of the family, fixed by the amendment. A family whose `m*(w) = 1` at every
weight has no Phase B, and its planner rule stays as it is. Same design,
target seed 20261029, measurement seed 20261030.

## Predictions

- P1. Per family, `m*(w)` is non-decreasing in `w`.
- P2. `m*(0) = 1` in both families: `ic-planner/1`'s counted optimum.
- P3. Held out, at each `w` with `m*(w) ≠ 1`:
  - `S_w(alt) / S_w(plan) < 1`;
  - the paired 95% interval is below 1 on a majority of the family's
    held-out curves: 3 of 4 binary, 2 of 3 Koblitz.

## Decision rule

- A family's weight rule goes into the planner (`ic-planner/3`) only if P3
  holds at every weight where its `m*(w) ≠ 1`. Otherwise that family keeps
  its `w = 0` plan and the existing "not modelled" warning, and the result is
  reported as negative.
- The weight is a declared pricing assumption. A weighted plan is cheaper
  only at that weight, never a band-robust ranking (AGENTS.md §5a).
- Class: engineering. No counted `S` improves on a measured baseline, so no
  row of tables A–C moves. The gap to the matched rho is reported as distance
  remaining.
- Failures, timeouts and unverified runs are reported, never dropped.

## Inadmissible

- Changing curves, grids, weights, targets or seeds after seeing a session of
  the same phase.
- Dropping a run.
- Editing the planner between sessions.
- Fitting Phase A's rule on Phase B data.
