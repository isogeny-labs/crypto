# Genus 4 on prime-order Jacobians: arm x against the ideal negation-rho bound at p = 16411 and 32771

Frozen 2026-11-18 before any measured run of this study (`frozen.sha256`, `binary.sha256`). Every
number is `[pcc:unverified]` until a validator replays it.

## 1. Questions

1. **Prime-order comparison.** On prime-order (h = 1) genus-4 Jacobians, how does arm x (double
   large prime, prefilter, coordinate Wiedemann, unit-T walk, Ferrari, SGE) compare with the ideal
   negation-rho bound `sqrt(pi r / 4)` at p = 16411 and p = 32771?
   - In `research/genus4_followup_20261118`, the one h = 1 curve (2640) gave exactly 1.000 at
     w = 1 (validator finding F7).
   - IC cost barely depends on r, while rho grows as `sqrt(r)`. So the comparison that matters
     cryptographically is on prime-order curves.
2. **Exponent.** Theory (double large prime, genus g) gives IC cost about p^(2 − 2/g) = p^1.5, and
   rho costs about p^2. So the ratio should fall as about p^-0.5. Does it fall between the two
   primes?
3. **Linear algebra (step 2).** Linear algebra is 76% of arm x's native work at p = 16411. Does a
   factor-base size and SGE setting tuned on held-out curves lower the cost on the prime-order
   curves?

## 2. Instances

- **Confirmation curves.** At each prime: the 5 smallest curve seeds s ≥ 1 that `genus_ladder`
  admits with `--max-h 1` (#J prime) and `--genera 4`. At p = 16411, seed 2640 is excluded, because
  it was measured in the earlier study.
- **Search.** `search.sh` scans seeds 1..N, with N = 2000 at p = 16411 and N = 1800 at p = 32771.
  Four processes take the seed residues mod 4, emit instance rows only (arm `n` runs nothing), and
  use the prefilter `f_irreducible`. That prefilter skips seeds with reducible f, which have even
  #J; it is proved in the code comment and tested in `reducible_f_gives_even_jacobian_order`.
- **Too few curves.** If fewer than 5 seeds are admitted, the scan extends in blocks of 400 seeds
  under the same rule.
- **Targets and seeds.** On each curve, targets k = 0..5 (target seed `cs + k · 1000003`) and solver
  seeds 1–2. That gives 12 keys per curve and 60 keys per prime.
- **Tuning curves (part A).** p = 16411, curve seeds 172 and 180 (h = 2, earlier studies), targets
  k = 0..2, seeds 1–2: 12 keys. IC cost does not depend on h, so these curves are a fair tuning set,
  and they are disjoint from the confirmation curves.

## 3. Part A: tuning (selection rule frozen here)

- **Grid.** Arm x over cc ∈ {1.4, 2.0, 2.8} × SGE (w_max, fill_max) ∈ {(4, 1.5), (8, 2), (16, 3),
  (32, 4)}: 12 configurations on the 12 tuning keys.
- **Eligibility.** A configuration is eligible only if every run is certified and correct.
- **Selection.** (cc\*, sge\*) is the eligible configuration with the lowest mean cost
  `group_ops + rel_mul + rel_add + la_mul + la_add` (w = 1). A tie goes to the configuration
  nearest the baseline (2.0, (4, 1.5)).
- **Use.** `tune.py` writes the choice and `run.sh` passes it on. Arm x\* is arm x at (cc\*, sge\*).
  If the choice is the baseline, x\* is x and is reported once.

## 4. Part B: confirmation

- **IC arms.** Arm x at the baseline (cc = 2.0, SGE (4, 1.5)) and arm x\* run on the 60 keys of
  each prime.
- **Measured rho check, p = 16411 only.** Window-free plain rho (arm Q) on each confirmation
  curve, target k = 0, seed 1: 5 runs.
- **Measured rho at p = 32771: none.** One run there costs about `sqrt(pi r/2)` ≈ 1.3e9 group
  operations.

## 5. Gates

- **G1:** every IC run is certified and correct.
- **G2:** every confirmation curve has h = 1 and r prime, as emitted by the binary. The validator
  rechecks this independently.
- **G3:** the measured Q mean over sqrt(pi r/2) lies in [0.6, 1.6] at p = 16411. Five runs are
  noisy, so this is a sanity bound and no correction is applied.

## 6. Readings and statistics

- **IC cost.** `G + w · (rel_mul + rel_add + la_mul + la_add)` with w ∈ {0, 0.1, 1}.
- **References.** Per curve, `R_neg = sqrt(pi r / 4)` (ideal negation rho, nothing charged; the
  strongest available reference) and `R_plain = sqrt(pi r / 2)`.
- **Ratio.** Per prime and arm, mean IC cost over mean R, on the 60 keys.
- **Bootstrap.** Hierarchical: curves, then targets within curve, then seeds within target. 2000
  resamples, 95%, seed 20261124.
- **D1, below the strongest rho.** An arm is "below ideal negation rho" at a prime iff the upper
  bound is < 1 at all three of w = 0, 0.1 and 1, so that the §5a order holds across the band.
- **D2, exponent.** The slope is `ln(ratio_32771 / ratio_16411) / ln(32771 / 16411)` against
  R_neg at w = 1, with the same bootstrap applied independently per prime. Also reported: the slope
  of the mean IC cost in p.
- **D3, tuning.** The x\* / x mean cost ratio at w = 1, paired by key, per prime, with the same
  bootstrap.

## 7. Predictions (before data)

- **P1.** At p = 16411, x against R_neg at w = 1 lies in [0.90, 1.10].
- **P2.** At p = 32771, x against R_neg at w = 1 lies in [0.60, 0.85], and D1 holds there (upper
  bound < 1 at every w).
- **P3.** The D2 slope (against R_neg, w = 1) lies in [−0.8, −0.3].
- **P4.** Part A picks a non-baseline configuration, and x\* / x ≤ 0.95 at both primes (upper
  bound < 1).
- **P5.** G1–G3 pass.

## 8. Deviations known at freezing

- **Host.** Local 4-core host with no AWS identity, not isolab-eks. Counts are deterministic; wall
  time is not a measurement. The session is kept active, and every background command stays under
  the 2 h limit, which is why the work is split into search, part A and part B.
- **No S3.** Raw data is committed gzipped.
- **Thin rho check.** The reference is a closed form. The earlier study measured plain rho at
  1.004× `sqrt(pi r/2)` over 108 runs (validated), and part B's 5 runs are a sanity check only.
