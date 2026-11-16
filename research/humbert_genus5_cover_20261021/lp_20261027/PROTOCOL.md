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

## Amendment 1 (2026-10-27, after 18 rows; those rows, all q = 2003, were laid out once with `analyze.py` before the fix)

The first campaign run stopped at q = 2003, seed 5, arm lp:0.35 with a panic:
the iota-pair check computed c from x-index 0, which in a large-prime arm need
not be in the small base, so its log can be unknown. Fix: c is computed from the
pinned point (always in the small base). The fix touches only that check; the
iota arm still reproduces the recorded PR #2580 run at q = 1009 seed 1 exactly.
The 18 rows are kept as `results/campaign_v1_superseded.*`, and the whole grid is
rerun with the fixed binary. Predictions and metrics are unchanged.

## Amendment 2 (2026-10-27, after the q = 2003–8009 rows)

- **Confound found in the baseline.** Every lp:0.5 win at q ≥ 4001 is on an
  instance where the iota baseline needed 3–10 linear-algebra rounds. Each round
  collects about 5 % more relations and reruns the whole Wiedemann solve, mostly
  because some factor-base column appears in no relation yet. On every instance
  where the baseline needed one round, lp:0.5 was slower (0.66–0.95). The A/A arm
  repeats the same rounds, so this is deterministic, not noise. P1 is falsified
  as preregistered (3/5 at q = 8009).
- **q = 12007 rerun.** The first q = 12007 attempt overlapped a separate test run
  on the same host, so its 3 rows are set aside as
  `results/campaign_12007_contaminated.jsonl`, and q = 12007 is rerun alone with the
  same frozen binary (`bin/humbert-v1`, sha256 prefix e5d73aa5683c8f0f).
- **Follow-up protocol.** The baseline fix (no Wiedemann solve while a column is
  empty, `HUMBERT_LA_PRECHECK=1`) and a re-test of lp:0.5 against it are a
  separate protocol, `PROTOCOL_v2.md`, frozen before its data.
