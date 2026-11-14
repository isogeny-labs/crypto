# Collision floor of the residual collector, and the matched rho at p36 (2026-11-09)

Status: local L0 sandbox, counted units (deterministic), one curve. Not
validated under the validation gate; no leaderboard or `benchmarks/README.md`
row. Every claim below is an unverified assertion [pcc:unverified] until a pcc
claim is filed. PDF: [`REPORT.pdf`](REPORT.pdf); figure: [`ratios.svg`](ratios.svg).

## Question

A residual collector harvests relations from walk collisions. With k
unknown-log base columns it needs k + 1 independent relations to pin the
target. Each relation costs a collision, while rho needs only one.

Against one persistent table, the expected time to the c-th collision is
E[T_c] / E[T_1] = (2c − 1)!! / (2^(c−1) (c − 1)!). That ratio is 1.5 for
c = 2, so `res1` (one column) cannot cost less than about 1.5× ideal rho.

Main's scale study reported `res1eq` at 1.32× matched `rho.negation` at p36,
with 12 targets (`../ic_residual_scale_20261101`). That reading is below the
floor. So either the floor is wrong, or the matched rho reference costs about
1.14× its ideal sqrt(π r / 4). The exhaustive study's rho S of 1.13 at p36,
1.28× of 0.886, pointed the same way. The open atlas lead
`LEAD-RHO-matched-rho-calibration` asks exactly this of the reference.

## Predictions (frozen in commit 35f0720fa6, before the run)

| id | claim | prediction | outcome [95% CI] | result |
|:--|:--|:--|:--|:--|
| F1 | baseline `res1` walk group ops / ideal follow the walk-hazard model of `../ic_residual_fused_steps_20261107` with two collisions | 2.717 | 2.819 [2.679, 2.957] | inside |
| F2 | `res1eq` total GAE / ideal is at least 1.45 | ≥ 1.45 | 1.658 [1.585, 1.732] | met |
| F3 | `rho.negation` GAE / ideal lies in [0.95, 1.15] | 0.95–1.15 | 1.024 [0.961, 1.091] | met |

Every residual solve needed exactly two table hits: 256/256 for each of `res1`
and `res1eq`.

Paired by target (cluster bootstrap over 128 workloads):

| ratio | value [95% CI] |
|:--|:--|
| `res1eq` / `rho.negation` | 1.620 [1.501, 1.755] |
| `res1` / `rho.negation` | 2.762 [2.534, 3.019] |
| `res1` / `res1eq` | 1.705 [1.585, 1.829] |

## Reading

- **The two-collision floor holds.** Its walk alone costs 1.563 [1.492,
  1.636] of ideal rho, consistent with 1.5 plus the detection delay at
  distinguished points. Set-up and restarts bring the total to 1.66.
- **The matched reference is at its ideal here.** At p36 `rho.negation` costs
  1.02× sqrt(π r / 4), so the walk shows no shortfall at this size.
  - The study's 1.32 and the exhaustive study's rho S of 1.13 are therefore
    better read as small-sample readings (12–16 targets) than as a biased
    reference.
  - Rho's single-run cost has a Rayleigh-like spread (coefficient of variation
    about 0.52).
  - The distance of `res1eq` to rho at p36 is 1.62, not 1.32.
- **Consequence for the program.** No tuning of a collision-harvested
  collector reaches rho. It is bounded below by
  g(k) = (2k + 1)!! / (2^k k!) × rho for k unknown columns: 1.5 at k = 1,
  1.875 at k = 2, 2.46 at k = 4, growing like sqrt(k). Only relations that
  cost less than a collision can move the distance to rho. That means
  algebraic decompositions, which `KR-IC-ecebb6` (the ladder rule) bounds
  separately.

## Suggested atlas updates (crypto-autoresearcher, not made here: this session has read-only access)

- **`LEAD-ICP-residual-partial-relations`:** add the g(k) floor and this
  measurement as a constant-factor obstruction alongside the exponent
  obstruction.
- **`LEAD-RHO-matched-rho-calibration`:** one prime medium-size point,
  `rho.negation` at 1.024 [0.961, 1.091] × ideal at log2 r = 36. It is
  consistent with the lead's "within 1.10×" branch, so the toy excess would be
  set-up. A sealed bound (≥ 24 runs per size, replay, §5a band) at 32–48 bits
  is still its next step.

## Run

- **Binary:** `ecbench` built from main cd9256bea0. The residual and ecbench
  code changed between that commit and the base of this branch (0837457b46);
  the sealed session records the binary hash.
- **Session:** ECBS1hb0d5644dc061, 1,152 records, all passing ecbench's own
  check [pcc:unverified]. Spec `spec-p36.json`: the scale study's p36 curve,
  128 targets, seed 20261109, arms `rho`, `res1` and `res1eq`, 2 measured
  rounds after warm-up.
- **Host:** local 4-core sandbox, `--cpus none --allow-busy`, L0. Wall time is
  not used.
- **Files:** records in `session/records.jsonl.gz`. `python3 score.py
  <records.jsonl> PREDICTIONS.json > scored.json 2> paired.json`, then
  `python3 figure.py`.
