# Protocol: does the `ic` planner pick the best configuration on curves it has not seen?

Frozen 2026-10-20, before any recorded session, with the planner at
`a29bead23` (`ic-planner/1`). Changes after data go under "Deviations" in the
README, never edited in here.

## Question

`ic plan` / `ic bench --auto` (`src/cryptanalysis/ic_framework/planner.rs`)
chooses every `ic.pipeline` stage of a prime-field run from earlier
measurements: the factor-base size, the oracle, the stop rule and the target
source. On three prime-order curves none of that evidence was measured on, is
each of its choices better than the alternative one change away, in counted
`S`?

## Curves

ecbench `prime_search` (`find_prime_order_curve(bits, seed)`, cofactor 1),
none in the planner's evidence:

| spec | bits, seed | log2 r | planned base |
|:--|:--|--:|--:|
| `specs/c24.json` | 24, 3 | 23.47 | 113 abscissae |
| `specs/c28.json` | 28, 1 | 27.69 | 300 |
| `specs/c30.json` | 30, 1 | 29.62 | 469 |

All three sizes are between the planner's measured points (19.1 and 32 bits),
so each plan is `interpolated`.

## Arms (each curve, one session)

| arm | change from the plan |
|:--|:--|
| `rho-neg` (reference) | `rho.negation` |
| `plan` (candidate) | none: `prime-abscissa:size=N`, `mitm-prime-blocked:m=2`, `incremental-gauss`, `walk` |
| `half-base` | `size = ⌈N/2⌉` |
| `double-base` | `size = 2N` |
| `rootfind` | oracle `rootfind-subtract:m=2` |
| `full-rank` | linalg `incremental-gauss-full-rank` |
| `random-targets` | targets `random` |
| `plan-aa` (control) | repeat of `plan` |

8 targets per curve (target seed 20261021), 3 rounds + 1 warm-up, alternate
order, `isolation_required: L2` (only counts are compared), timeout 1,800 s.

## Measures

1. `S` per arm (`ecbench table`), and `ecbench compare` of every alternative
   against `plan`: the `ops` ratio with its cluster 95% interval.
2. `ecbench band` at 0, 0.1 and 1 GAE per unpriced operation (AGENTS.md §5a).
3. The A/A control (`plan-aa` against `plan`).
4. `S_plan / S_rho` against the planner's predicted `ic_over_rho` for the curve
   (11.46, 19.33 and 24.55).

## Predictions

- P1. On each curve `plan` has the lowest mean `S` of the IC arms.
- P2. `half-base / plan` and `double-base / plan` are both above 1. The cost
  model `N² + K/N` puts them at about 1.4 and 1.7 if `N` is the optimum; a
  shallow optimum may put one interval across 1.
- P3. `rootfind / plan` ≥ 10; `full-rank / plan` ≥ 1.5; `random-targets / plan`
  ≥ 1.5, each interval above 1.
- P4. `S_plan / S_rho` is within a factor of 2 of the planner's prediction.
- P5. The order of `plan` against each alternative holds across the band.

## Decision rule

- A choice is **confirmed** on a curve when the alternative's ratio interval
  lies above 1 at every band weight; **not separated** when it contains 1 (the
  plan is then within noise of that alternative); **refuted** when it lies
  below 1, i.e. the alternative is better.
- Any refutation means the rule that made that choice is wrong at that size; it
  is reported, and the planner's rule is changed only in a later, separate
  change with its own evidence.
- The rho outlook (P4) is a calibration check, not a success criterion.
- This is tooling validation (AGENTS.md §3: engineering). It moves no
  leaderboard row: no `S` improves on a measured baseline.
- Failures, timeouts and unverified runs are reported, never dropped.

## Inadmissible

Changing the curves, sizes, arms, targets or seeds after seeing a session;
dropping a run; editing the planner between sessions.
