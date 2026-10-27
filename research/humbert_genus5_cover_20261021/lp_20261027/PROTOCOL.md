# Protocol: single large prime on the genus-5 cover (frozen 2026-10-27, before data)

## Question

Does a single-large-prime variant of the iota-folded index calculus on the
descended genus-5 curve C reduce end-to-end wall time per DLP against the
merged iota baseline (PR #2580) on the same instances, and by how much does it
change the distance to matched Pollard rho?

## Arms (same binary, same instance per (q, seed))

| arm | meaning |
|:--|:--|
| `iota` | baseline: merged iota arm (factor base = all rational x, iota-folded) |
| `iota` (repeat, A/A) | the baseline again in a separate process: noise floor of the wall-time ratio |
| `lp:0.5` | small factor base = seeded random half of the x-values; relations whose residual has exactly one point outside it are partials, combined in pairs on the shared large prime; residuals with two or more are dropped |
| `lp:0.35` | the same with 35 % of the x-values |

The binary is the crate in this directory's parent at the commit that adds this
file. The refactored `iota` arm was checked to reproduce a recorded run of PR #2580
exactly (q = 1009, seed 1: identical relations, candidates, rank, descent
candidates, rho steps and recovered log).

## Grid

q ∈ {2003, 3001, 4001, 6007, 8009, 12007}, seeds 1–5, twist +1 for odd seeds and −1
for even ones (as in PR #2580). Arm order is rotated per seed
(seed k starts at arm k mod 4 of the list above) so no arm always runs first.
Rho runs only in the first `iota` arm of each instance (`HUMBERT_SKIP_RHO=1`
elsewhere); its expected-steps reference sqrt(pi r / 2) × measured s/step is the
rho reference for every arm of that instance.

## Metrics

- Primary: per instance, t_ic(iota) / t_ic(lp:0.5) (end-to-end IC wall time, set-up included).
- Secondary: the same for lp:0.35; phase times; unknowns; candidates; partials
  stored, combined and dropped; IC / expected-steps rho for every arm.
- Correctness gates on every run: solved, pushforward 8P, T(P) ≠ 0, iota pairs
  constant over known points, X_oo = 4c.

## Predictions and falsification (fixed now)

- P1: at q = 8009, lp:0.5 is faster than iota on at least 4 of 5 seeds, and the
  median ratio exceeds the A/A ratio's max deviation from 1.
  **Falsified if** fewer than 4 of 5 seeds are faster.
- P2: at q = 2003 the ratio is at most 1 (linear algebra too small to repay the
  costlier relation search). Falsified if the median ratio exceeds 1.2.
- P3: the ratio rises with q.

## Limits stated in advance

Local 4-vCPU shared VM, no isolation (L0); isolab-eks was not reachable from
this session (no AWS CLI), so this deviates from the default route of AGENTS.md
§9a. Wall time only; §5a band as in PR #2580 (IC does no group operations, arms
are not ranked on operation counts).

## Amendment 1 (2026-10-27, after 18 rows, before any analysis was used)

The first campaign run stopped at q = 2003, seed 5, arm lp:0.35 with a panic:
the iota-pair check computed c from x-index 0, which in a large-prime arm need
not be in the small base, so its log can be unknown. Fix: c is computed from the
pinned point (always in the small base). The fix touches only that check; the
iota arm still reproduces the recorded PR #2580 run at q = 1009 seed 1 exactly.
The 18 rows are kept as `results/campaign_v1_superseded.*`, and the whole grid is
rerun with the fixed binary. Predictions and metrics are unchanged.
