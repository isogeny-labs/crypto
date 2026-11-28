# Results: genus 4 follow-up (54 targets at p = 16411, factor-base balance)

Every number is `[pcc:unverified]` in pcc terms. An independent validator replayed and recomputed it
(`benchmarks/20261118-genus4-followup/validator-report.md`, disposition **validated**; findings
F1–F7 are taken up below and in `ERRATA.md`). The study was run once, on a local 4-core host. The
protocol is `PROTOCOL.md`, frozen at ae36cdc6e4 before any data. Tables come from `analyze.py`
(frozen) and `sensitivity.py` (**post hoc**, written after reading `analysis.json`).

## Headline

At p = 16411, over 54 targets (108 runs per arm), arm x costs the following against window-free
plain rho:

| reading | arm x / rho | 95% interval | status |
|---|---|---|---|
| Q3, w = 1, measured rho | **0.874** | [0.767, 0.9996] | frozen |
| Q4, w = 1, measured rho | 0.218 | [0.192, 0.250] | frozen |
| w = 0.1, measured rho | 0.089 | — | frozen |
| w = 1, rho's theoretical expectation `sqrt(pi r/2)` | 0.877 | [0.838, 0.920] | post hoc |
| w = 1, ideal negation bound `sqrt(pi r/4)` | **1.240** | [1.185, 1.301] | post hoc |

Arm x is Ferrari plus SGE on the double-large-prime walk.

- **Frozen decision rule.** Arm x is "below rho at w = 1" (Q3 and Q4 upper bounds < 1). The Q3
  upper bound is 0.9996.
- **That verdict is not robust.** The same bootstrap with 50 other seeds puts the Q3 upper bound
  between 0.994 and 1.012 (median 1.003), and below 1 for only 36% of seeds. Two preregistered
  sensitivities also include 1:
  - new targets only: 0.873 [0.762, 1.013];
  - curve-level bootstrap over 3 curves: [0.671, 1.150].
- **What does hold.** The measured rho mean is 1.004× `sqrt(pi r/2)`, so that expectation is a sound
  reference here. Against it, with no rho sampling noise, arm x is clearly below plain rho at w = 1:
  0.877 [0.838, 0.920]. That reading is post hoc.
- **Distance remaining.** Arm x is 1.24× the ideal negation-rho bound at w = 1. That bound charges
  nothing for negation canonicalisation or fruitless cycles. The measured negation walk (arm B,
  lean-rho study) was dearer than plain rho, so no measured rho is cheaper than arm x here.
- **At w = 0.1** arm x is 0.126× even the ideal negation bound.
- **Arm w (no Ferrari, no SGE)** is 1.605 [1.407, 1.837] at w = 1, and 2.28× the ideal negation bound.

### Per curve (validator finding F7, the most important reading)

Arm x's cost barely depends on r (coefficient of variation 1.2% over the 108 keys), while rho scales
as `sqrt(r)`. Arm x at w = 1, per curve:

| curve | h | r | arm x / `sqrt(pi r/2)` | arm x / ideal negation `sqrt(pi r/4)` | arm x / measured Q (Q3) |
|---|---|---|---|---|---|
| 172 | 2 | 3.6e16 | 0.998 | 1.412 | 1.150 |
| 180 | 2 | 3.6e16 | 0.998 | 1.412 | 0.907 |
| 2640 | 1 | 7.3e16 | **0.707** | **1.000** | 0.685 |

- **On the two cofactor-2 curves** arm x is at parity with plain rho's expectation.
- **On the one prime-order Jacobian**, the cryptographically relevant case, it is 0.71× plain rho and
  at parity with the ideal negation bound.
- **What the pooled number rests on.** The pooled "below plain rho" rests on that one curve. Telling
  whether arm x beats plain rho on prime-order Jacobians at this size needs more h = 1 curves; that is
  the next study.
- **Reading by key.** The median per-key x/Q is 0.935, the mean per-key ratio is 1.265, and arm x is
  cheaper on 56 of 108 keys.

### Why the 9-target estimate held (corrected)

The 9-target estimate (0.876) held up at 54 targets (0.874). PROTOCOL P1 expected it to regress
toward 1, on the stated ground that the 9-target rho mean was about 14% above `sqrt(pi r/2)`. That
ground was wrong (validator F1): the 9-target rho mean is 1.0009× `sqrt(pi r/2)`. The 14% came from
comparing it with the h = 2 curves' `sqrt(pi r/2)` alone. Nothing needed to regress.

### Q4 caveat (validator F3)

Q4 charges rho a partition hash and two coefficient additions per step, but does not charge arm x's
identical per-step walk work. Charging it moves arm x's Q4 ratio from 0.2185 to 0.2197; no verdict
changes.

## Part A: factor-base balance (p = 8209, arm x, 36 keys per cc)

| cc | nb | group ops | relation native | LA native | mean cost (w = 1) | / cc 2.0 |
|---|---|---|---|---|---|---|
| 0.5 | 432 | 1.42e6 | 2.96e8 | 8.4e6 | 3.06e8 | 3.37 |
| 0.7 | 604 | 8.05e5 | 1.68e8 | 1.41e7 | 1.82e8 | 2.01 |
| 1.0 | 863 | 4.51e5 | 9.38e7 | 2.36e7 | 1.18e8 | 1.30 |
| 1.4 | 1208 | 2.74e5 | 5.69e7 | 3.67e7 | 9.39e7 | 1.03 |
| 2.0 | 1725 | 1.72e5 | 3.57e7 | 5.50e7 | 9.09e7 | 1.00 |

- **Selection.** The frozen rule picks cc\* = 2.0, the existing setting. Arm x\* is therefore arm x.
- **Why genus 4 differs from genus 3.** In the genus-3 lead `g2plus_leads_20261116/L03_base_rebalance`,
  cc = 1.0 won by 36–40%. At genus 4 the relation phase is much steeper in nb: trials scale as nb^-1.4
  between cc = 1.0 and 2.0 and as nb^-1.66 between cc = 0.5 and 1.0, against about nb^-0.8 at genus 3. So shrinking the base costs more in
  relation search than it saves in linear algebra.
- **Edge of the grid.** The minimum is at the top of the grid. A larger cc was not tested.
- **Shift with p.** At p = 16411 linear algebra is 76% of arm x's native work, against 61% at
  p = 8209 and cc = 2.0. The optimum may move with p. Testing that needs a tuning prime that is not
  the confirmation prime, so it is a separate study.

## Ferrari inside the sieve

Not run. The sieve (`genus_ladder_sieve.rs`) has no splitting test for Ferrari to replace. It counts
table hits for the roots of `R_t`, and at g = 4 the cofactor has degree at most 1 (effective large
primes capped at 1). PROTOCOL.md §1 gives the details.

## Gates

- G1: all 216 IC runs certified and correct; all Q runs certified.
- G2: 36 of 36 overlapping w/x rows are identical to the cheap-IC study's rows.
- G3: the Q re-run is identical in steps and group_ops.
- Resume duplicates: the 2 Q keys re-computed by `resume.sh` are identical.

## Prediction scorecard

| # | prediction | outcome |
|---|---|---|
| P1 | x point in [0.90, 1.10], upper > 1 | **refuted** on the point (0.874). The upper bound is 0.9996 at the frozen seed and above 1 for 64% of seeds (32/50). The stated reason was also wrong (F1) |
| P2 | cc\* in [0.7, 1.4], x\* ≤ 0.85× x at p = 8209 | **refuted** (cc\* = 2.0) |
| P3 | x\* below x at p = 16411 | not testable (x\* = x) |
| P4 | every arm > 1.2× the ideal negation bound at w = 1 | holds (w 2.28, x 1.24) |
| P5 | gates pass | holds |

## Deviations

- **Run killed.** The run wrapper hit the 2 h background-task limit after 62 of 90 Q runs.
  `resume.sh`, written before any Q row was read, re-ran curve 2640's block into `b_q.jsonl.r1`,
  which the frozen glob reads.
- **`sensitivity.py` is post hoc.**
- **Host and storage.** Local host, not isolab-eks. No S3 upload, because there is no transport.
  research-kb is unavailable, so the repository was searched instead (L03 above).

## Sources checked

- `research/g2plus_leads_20261116/L03_base_rebalance/ASSESSMENT.md`: genus-3 factor-base rebalance.
- `research/genus4_lean_rho_20261116`: Q, B and R references; Q is the cheapest at p = 16411.
- `research/genus4_cheap_ic_20261117`: earlier arm-x rows, used for G2.
