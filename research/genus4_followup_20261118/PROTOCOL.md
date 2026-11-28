# Genus 4 follow-up: 54 targets at p = 16411, factor-base balance, rho reference

Frozen 2026-11-18 before any measured run of this study (`frozen.sha256`; binary hash in
`binary.sha256`). Every number is `[pcc:unverified]` until a validator replays it.

## 1. Questions

1. **More targets.** In `research/genus4_cheap_ic_20261117`, arm x (double large prime + prefilter +
   coordinate Wiedemann + unit-T walk + Ferrari + SGE) measured 0.876 [0.645, 1.284] of
   window-free plain rho (arm Q) at p = 16411, reading Q3, w = 1, on 9 targets. What is the ratio on
   54 targets (the 9 plus 45 new ones) of the same 3 curves?
2. **Factor-base balance.** Linear algebra is about 75% of arm x's native work at p = 16411. Does a
   smaller factor base (constant `cc` in `nb = ceil(cc · p^(3/4))`), tuned on p = 8209 only, lower
   arm x's cost on the held-out p = 16411 targets?
3. **Reference strength.** How far is each arm from the strongest rho bound, ideal negation rho
   `sqrt(pi r / 4)` group operations with no native work charged?

### Why not "Ferrari inside the sieve"

It was the plan in PR #4693. The sieve (`genus_ladder_sieve.rs`) has no splitting test to replace.
It finds roots of `R_t` by counting table hits and reads the cofactor of degree at most 1 from two
coefficients, so Ferrari's quartic test has nothing to act on. At g = 4 the sieve's effective large
primes are capped at 1, and its relations carry `2g + 1` places through a family vertex. In the
previous study its linear algebra was 1.4–1.9× arm w's. The lever in every arm is linear algebra,
which question 2 addresses.

## 2. Instances, targets, seeds

- **p = 16411:** curve seeds 172, 180 and 2640 (those of the earlier studies). Targets k = 0..17,
  target seed `cs + k · 1000003`. Solver seeds 1–2. That gives 54 targets and 108 keys per arm.
  Targets k = 0..2 are the earlier studies' targets.
- **p = 8209 (tuning only):** curve seeds 363, 393, 407, 457, 474 and 519. Targets k = 0..2, solver
  seeds 1–2. That gives 18 targets and 36 keys.

Binary: `genus_ladder` built from this PR's source (adds `--target-from`, nothing else in this
binary). `--max-h 2 --genera 4`.

## 3. Part A: tuning on p = 8209 (selection rule frozen here)

Arm x at `cc` ∈ {0.5, 0.7, 1.0, 1.4, 2.0} on the 36 keys. A `cc` is eligible only if every run is
certified and correct. **cc\*** is the eligible `cc` with the lowest mean IC cost
`group_ops + rel_mul + rel_add + la_mul + la_add` (Q3, w = 1). Ties go to the larger `cc`.
`select.py` computes it, and `run.sh` passes it to part B unchanged. p = 16411 data plays no part
in the choice.

## 4. Part B: confirmation on p = 16411

- **Arms w and x at cc = 2.0:** k = 0..17 (108 keys each).
- **Arm x\* (x at cc\*):** k = 0..17. If cc\* = 2.0, x\* is x and is reported once.
- **Arm Q (plain rho, no cycle window):** new runs for k = 3..17 (90 keys). For k = 0..2 the
  lean-rho study's rows are used (`research/genus4_lean_rho_20261116`).
- **Determinism check of Q:** curve 172, target k = 0, seed 1 is re-run.

Q was the cheapest rho at p = 16411 in every reading and weight of the lean-rho study (B and R were
dearer), so it is the only measured reference. That removes the minimum-over-references selection.

## 5. Gates

- **G1:** every IC run is certified and correct, and every Q run is certified.
- **G2:** arm w and arm x rows for k = 0..2 equal the earlier studies' rows in every count field
  (`group_ops`, `rel_mul`, `rel_add`, `la_mul`, `la_add`, `trials`, `relations`).
- **G3:** the re-run Q row equals the lean-rho row in `steps` and `group_ops`.

If G2 or G3 fails, the instruments differ, and parts B and C are reported as a new instrument without
pooling.

## 6. Readings and statistics

- **Costs:** Q3 and Q4 as in the lean-rho and cheap-IC protocols. IC cost is
  `G + w · (rel_mul + rel_add + la_mul + la_add)`. Q's cost is `G + w · K` (Q3) and
  `G + w · (K + H + D)` (Q4), with its canonicalisation count 0. w ∈ {0, 0.1, 1}.
- **Statistic:** per arm, the ratio of mean IC cost to mean Q cost over the 108 keys, paired by key.
- **Interval:** bootstrap over targets, then seeds within a target. 2000 resamples, 95%, seed
  20261123.
- **Arm comparison:** x\* against x as the ratio of mean IC cost, paired, same bootstrap.

Sensitivities, reported and not decisive:
- **(s1) References:** the theoretical plain-rho expectation `sqrt(pi r / 2)` and the ideal negation
  bound `sqrt(pi r / 4)`, in group operations, with nothing else charged.
- **(s2) Curve-level bootstrap:** curves, then targets, then seeds. Only 3 curves, so this is weak.
- **(s3) New targets only:** k = 3..17.
- **Part A:** the full cc curve on p = 8209, with each phase's share.

## 7. Predictions (before data)

- **P1:** arm x Q3 w = 1 at p = 16411 has a point estimate in [0.90, 1.10] and an upper bound > 1.
  Reason: the 9-target Q mean was about 14% above `sqrt(pi r / 2)` and should regress toward it.
- **P2:** cc\* < 2.0 and cc\* ∈ [0.7, 1.4]. At p = 8209, x at cc\* costs ≤ 0.85× x at cc = 2.0.
- **P3:** at p = 16411, x\* / x cost ratio (Q3, w = 1) < 1 with an upper bound < 1. x\* Q3 w = 1 point
  estimate ∈ [0.70, 0.95].
- **P4:** against the ideal negation bound (s1) at w = 1, every arm's ratio is > 1.2.
- **P5:** G1, G2 and G3 pass.

## 8. Decision rule

An arm is "below rho at w = 1" at p = 16411 only when its upper bound is < 1 in both Q3 and Q4
against the measured Q. x\* becomes the new baseline candidate only if P3's interval excludes 1.
Either outcome is reported. Nothing is promoted to the leaderboard without the validator replay
(`VALIDATION.json`) and the rho-reference band, including the ideal negation bound.

## 9. Deviations known at freezing

- Local host with 4 cores and no AWS identity, so not isolab-eks. Counts are deterministic, and wall
  time is not a measurement here. The session is kept active during the run.
- No S3 upload of raw data. It is committed gzipped, as in the previous study.
- Part A tunes on the curves of the earlier p = 8209 evaluation, which is acceptable because p = 8209
  is not the confirmation prime.
