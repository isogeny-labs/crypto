# Genus-4 exponent from three primes, and genus 5 at matched group size (lead C)

Frozen 2026-11-19 before any measured run of this study (`frozen.sha256`). Binary: `genus_ladder` at
ca72267899 (sha256 493ee294…, `binary.sha256`), the binary of `research/genus4_prime_order_20261118`.
Every number is `[pcc:unverified]`.

## 1. Questions

- **C1, the genus-4 exponent.** The prime-order study measured arm x\* at 0.876 and 0.640 of the
  ideal negation-rho bound at p = 16411 and p = 32771, which is a two-point slope of −0.45. Does a
  third prime, p = 60013 (#J ≈ 1.3·10^19), give a slope and an IC-cost exponent consistent with the
  double-large-prime theory of p^(2 − 2/g) = p^1.5?
- **C2, genus 5 at matched group size.** Genus 5 at p = 2357 (#J ≈ 7.27·10^16) and p = 4099
  (#J ≈ 1.16·10^18) has the same group size as genus 4 at p = 16411 (7.25·10^16) and p = 32771
  (1.15·10^18), so the rho reference is the same.
  - Asymptotically, genus 5 should win: IC cost is about N^0.32 against N^0.375 for genus 4.
  - Its constants are worse: a degree-5 polynomial splits with probability 1/120 against 1/24, and
    a Cantor step costs more.
  - Which effect wins at reachable sizes, and does the trend favour genus 5 as N grows?

## 2. Instances (searched with `search.sh`; selection rule frozen here)

- **Search procedure.** `--max-h 1` with the `f_irreducible` prefilter, which is sound for every
  odd-degree f.
- **Coverage.** Seeds are scanned 1..N in chunks of 28 seeds. Every chunk writes a `cell` row, so a
  stopped command leaves a known covered range. N starts at 1500 and extends in blocks of 500 until
  enough seeds are admitted.
- **Genus 4, p = 60013.** The 3 smallest admitted seeds are the confirmation curves.
- **Genus 5, p = 2357.** Of the 5 smallest admitted seeds, the 3 smallest are confirmation curves
  and the 4th and 5th are tuning curves (held out).
- **Genus 5, p = 4099.** The 3 smallest admitted seeds are the confirmation curves.
- **Keys.** On each confirmation curve, targets k = 0..5 and solver seeds 1–2, which gives 36 keys
  per arm and prime.

## 3. Tuning for genus 5 (rule frozen here)

- **Grid.** Arm x at p = 2357 on the two tuning curves, with targets k = 0..2 and seeds 1–2 (12
  keys). The grid is cc ∈ {1.0, 1.4, 2.0, 2.8} × SGE ∈ {(4, 1.5), (8, 2)}.
- **Eligibility and choice.** A configuration is eligible only if every run is certified and
  correct. (cc5\*, sge5\*) is the eligible configuration with the lowest mean cost
  `group_ops + rel_mul + rel_add + la_mul + la_add`. Ties go to the configuration nearest
  (2.0, (4, 1.5)).
- **No genus-4 re-tuning.** Genus 4 keeps cc 1.4 / SGE (4, 1.5), the choice from the prime-order
  study.

## 4. Arms

- **Genus 4 at p = 60013:** x (cc 2.0, SGE (4, 1.5)) and x\* (cc 1.4, SGE (4, 1.5)).
- **Genus 5 at p = 2357 and p = 4099:** x5 (cc 2.0, SGE (4, 1.5)) and x5\* (cc5\*, sge5\*).
  - The Ferrari quartic test exists only for genus 4. At genus 5 the `+ferrari` label is inert, and
    the arm is the double-large-prime walk with prefilter, coordinate Wiedemann, unit-T walk and SGE.
- **Measured rho sanity check.** Window-free plain rho (arm Q) at genus 5, p = 2357, on the first
  confirmation curve, k = 0, seeds 1–2.
- **No measured rho at p = 60013.** One walk there is about 4.5·10^9 steps.

## 5. Gates

- **G1.** Every IC run is certified and correct, and every rho run is certified.
- **G2.** Every confirmation and tuning curve has h = 1, and the coverage of the search is complete
  (`tried` sums to the scanned range).
- **G3.** The measured rho mean over `sqrt(pi r / 2)` lies in [0.5, 1.8], from 2 runs; this is a
  sanity check only.

## 6. Readings and statistics (`analyze.py`)

- **IC cost.** `G + w · (rel_mul + rel_add + la_mul + la_add)` with w ∈ {0, 0.1, 1}. The reference
  is the ideal negation bound `R = sqrt(pi r / 4)` per curve.
- **Ratio.** The mean IC cost over the mean R on the keys.
- **Bootstrap.** Hierarchical (curves, then targets, then seeds), 2000 resamples, seed 20261126.
  Draws are independent per prime and genus.
- **C1.**
  - Least-squares slope of ln(ratio, w = 1) on ln p over p = 16411, 32771 and 60013, for x and x\*.
    The first two primes come from `research/genus4_prime_order_20261118` (same binary, same arms,
    same keys design).
  - Also the slope of ln(mean IC cost, w = 1) on ln p, which is the cost exponent.
  - Bootstrap intervals for both, plus the residual of the middle point (curvature).
- **C2.**
  - At each matched size, x5\* / x\*: the genus-5 ratio over the genus-4 ratio, both against R at
    w = 1, with an independent-stream bootstrap.
  - The "below at all w" decision for each genus-5 arm.
  - The slope of ln(ratio, w = 1) in ln N, for genus 5 (from p = 2357 and 4099) and for genus 4
    (from p = 16411 and 32771).
- **Decisions.**
  - **C1: "exponent consistent with 1.5"** if the cost-exponent interval for x\* overlaps
    [1.4, 1.6].
  - **C2: "genus 5 ahead at size N"** if the upper bound of x5\* / x\* is below 1; **"behind"** if
    its lower bound is above 1.

## 7. Predictions (before data)

- **P1.** x\* at p = 60013 against the ideal negation bound, w = 1, lies in [0.42, 0.58]. This
  extrapolates the slope of −0.45.
- **P2.** The three-point slope of ln(x\* / R) in ln p lies in [−0.60, −0.35]. The cost exponent of
  x\* lies in [1.40, 1.65].
- **P3.** cc5\* ≤ 1.4.
- **P4.** x5\* / x\* falls from N ≈ 2^56 to N ≈ 2^60 (point estimates): the genus-5 constant hurts
  at small N, and the asymptotics take over as N grows.
- **P5.** G1–G3 pass.

## 8. Scope and deviations known at freezing

- **Hyperelliptic only.** This is a hyperelliptic study, not an elliptic-curve one, and it uses the
  same price convention as the prime-order study. The cost calibration
  (`research/genus4_cost_calibration_20261119`) measures what w really is.
- **Curve count.** 3 confirmation curves per new prime, because per-curve ratios agreed to within
  0.01 in the validated prime-order study.
- **Host.** Local 4-core host, not isolab-eks, no S3. Commands stay under the 2 h background limit,
  and searches are chunked and resumable.
- **Ordering.** This study starts only after the cost calibration's exclusive timing blocks finish.
  Its counts are deterministic and do not depend on timing.
