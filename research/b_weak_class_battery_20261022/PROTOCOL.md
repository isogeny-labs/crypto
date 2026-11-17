# Protocol: a calibrated battery of "weak class recognisable from b" tests

Status: **frozen before any statistic was computed on the real curves**
(2026-10-22). The report is [`README.md`](README.md), written after the run.
This protocol follows the `controlled-experiment` skill.

## 1. Question and the claims each outcome licenses

Hypothesis 1 of `research/p224_hypothesis_ranking_20261008/` says a designer
grinding SHA-1 seeds could have selected `b` from a rare class that is
testable from `(p, a, b)` without counting points. The one test run so far
(KS on `(b mod q)/q`, `q ≤ 10^8`) sees only classes that bias `b mod q`.

This study runs seventeen further cheap statistics on the actual `b` (and
`c`, where stated) of P-192, P-224, P-256, P-384 and P-521, and calibrates
each against a null population generated the way the real curves were.

| outcome for a curve × statistic | claim it licenses |
|:--|:--|
| empirical p at most the corrected threshold, positive control passed | "flagged after correction": the curve is more extreme than the null on this statistic. Not evidence of intent; it needs its own protocol |
| p above the threshold, positive control passed | "within the null distribution (p = …)" on this statistic. A statement about this statistic only |
| positive control failed (or planted instances could not be built) | "inconclusive": this detector cannot see the planted effect on this curve |

Every sentence about a curve in the report is a comparative/statistical claim
(class 3 of the skill). Nothing here is a theorem or an exact computation
except the `evaluate` step on a given `(p, b, c)`, which is verified by unit
tests against brute-force point counts on small primes.

## 2. Targets, null, conditioning

**Targets.** `CurveParams::{p192, p224, p256, p384, p521}` in
`src/ecc/curve.rs`, `a = −3`, with `c` derived from the FIPS 186-4 seed
(`x962_c_from_seed`), checked equal to the published `c` and to
`−27/b² mod p` before scoring. Seeds: `nist_seeded_curves()` in
`src/cryptanalysis/p224_hypotheses.rs`.

**Null population (negative control).** For each curve, draw `i` is built
from seeds `s_{i,k} = SHA-1("b-weak-class-null/v1/" ‖ curve ‖ i ‖ k)` (160
bits), `k = 0, 1, …`. The first `k` that passes every step is draw `i`:

1. `c = x962_c_from_seed(s_{i,k}, ⌈log2 p⌉)`, `c ≠ 0`;
2. `−27/c` is a square mod `p`; `b` is a square root, negated when the low bit
   of `SHA-1(s_{i,k} ‖ "sign")` is 1 (the real sign convention is unknown);
3. **Conditioning on necessary conditions of prime group order**, which the
   NIST curves satisfy and which need no point count:
   - `x³ − 3x + b` has no root in `F_p` (odd order);
   - no root `x₀` of the 3-division polynomial `3x⁴ − 18x² + 12bx − 9` has
     `x₀³ − 3x₀ + b` a square (no rational point of order 3).

   Both are checked on the five real curves before scoring (stop condition,
   §9). Torsion of order 5 and above is **not** conditioned on (power limit).

Panel size `N = 8000` per curve; held-out null `H = 1000` per curve from the
label `…/holdout`; planted instances `100` per statistic and curve.

**Power check (skill §3).** `m = 5 × 17 = 85` tests, family-wise `α = 0.05`,
threshold `α/m = 5.88 × 10^-4`. All statistics are upper-tail, `p =
(1 + #{null ≥ x}) / (N + 1)`, so the smallest attainable `p` is `1/8001 =
1.25 × 10^-4`, a factor 4.7 below the threshold. The skill's bound `N + 1 ≥
4m/α = 6800` holds. A curve is flagged iff at most 3 panel values are `≥` its
value (`4/8001 = 5.0 × 10^-4`; `5/8001 = 6.2 × 10^-4` is not).

## 3. The seventeen statistics

All are computable from `(p, a = −3, b)` and, for the `c` rows, the X9.62
integer `c` (`t − 1` bits). All are oriented so **large is suspicious**;
two-sided quantities are folded with an absolute value. `t = ⌈log2 p⌉`.
`j = −6912/(b² − 4)` and `D = 4a³ + 27b² = 27(b² − 4)`, as integers in
`[0, p)`.

| id | family | definition |
|:--|:--|:--|
| `jac_b`, `jac_c`, `jac_j`, `jac_disc` | A | `|Σ (x \| ℓ)| / √32` over the first 32 odd primes `ℓ` (3 … 137), `x ∈ {b, c, j, D}`: Legendre/Jacobi pattern of the integer |
| `depth_b`, `depth_disc` | B | `log2` of the largest `g` dividing `G` such that `x` is a `g`-th power in `F_p^*`, `x ∈ {b, b² − 4}`. `G` is the `{2, 3, 5, 7, 11, 13, 17}`-part of `p − 1` with the 2-exponent capped at 24. This is the joint quadratic, cubic, quartic, … character. `−b` and `j` differ from these by a fixed character, so they add nothing |
| `hw_b`, `hw_c`, `hw_j` | C, E | `|HW(x) − n/2| / (√n / 2)`, `n` the bit length used (`t`, `t − 1`, `t`) |
| `run_b`, `run_c`, `run_j` | C, E | longest run of equal bits in the `n`-bit representation |
| `naf_b` | C | `|NAF(b) − t/3| / √(2t/27)`, `NAF` the weight of the non-adjacent form (signed-digit weight) |
| `smooth_b`, `smooth_c`, `smooth_j` | D | `log2` of the part of the integer made of primes below `2^16` (with multiplicity) |
| `d3` | F | number of roots of the 3-division polynomial in `F_p`: rational 3-isogeny kernels (given the conditioning, all have non-rational `y`) |

Values are rounded to 6 decimals before ranking. The rational 2-torsion count
and 2-isogeny pattern, and the rational 3-torsion count, are **not**
statistics: the conditioning fixes them at zero on the null, and prime order
fixes them at zero on every real curve. They carry no information about a
prime-order curve.

## 4. Positive controls (detector power)

For every statistic and curve, `100` planted instances are built **by
construction**, pass through the same conditioning and the same `evaluate`
code as the null, and are scored against the same panel.

| statistics | planted class |
|:--|:--|
| `jac_*` | target quantity is a quadratic residue modulo each of the 32 primes (Chinese remaindering; the remaining high part random) |
| `depth_b` | `b = r^g`, `g = 2^12 ·` the available odd parts of `G` |
| `depth_disc` | `b² − 4 = v^g`, `b` a square root of `v^g + 4` |
| `hw_*` | each bit of the target set with probability 3/4 |
| `run_*` | a run of 40 one-bits at a random position |
| `naf_b` | `b` with 24 non-adjacent signed digits |
| `smooth_*` | target quantity a product of primes below `2^16` of full length |
| `d3` | rejection sampling from a fresh null stream (`…/plant-d3`) for `d3 ≥` the panel maximum |

**Pass rule.** A cell passes iff at least 80 of the 100 planted instances have
`p ≤ α/m`. A planted instance that cannot be built within 4000 attempts counts
as not detected. A failed cell is reported "inconclusive" and is never read as
"no effect".

**Predicted from the dry run (§8), before the real run.** `d3` can reach at
most `p ≈ 0.06` (its largest value has about 6% null mass), so its positive
control is expected to fail on every curve. It is constant (0) on P-192 and
P-384, where `p ≡ 2 mod 3`. `depth_disc` cannot be planted on P-192 and
P-384 (`p ≡ 2 mod 3` forces the quadratic character of `b² − 4` under the
conditioning), and `depth_b` has support `{0, 1}` there. These cells are
expected to be inconclusive. They stay in the family of `m = 85`; the
threshold is not relaxed for them.

## 5. Negative-control calibration

The `H = 1000` held-out null draws are scored against the panel with the same
code. **Gate:** for each cell, the count of held-out draws with `p ≤ 0.05`
is at most 70 (the upper 3σ bound of `1000 × 0.05`); a count below 30 on a
coarse discrete statistic is conservative from ties and is not a failure. At
most 1 of the 85 cells may exceed 70. If the gate fails, the p-values are
reported as miscalibrated.

## 6. Decision rule and multiple testing

**Declared in advance.** Bonferroni over the whole family of `m = 85`
(curve × statistic) tests at family-wise `α = 0.05`: flag iff `p ≤ 5.88 ×
10^-4`. The statistics within a curve are dependent, so Bonferroni is
conservative. Also reported, descriptively and not for flagging: the count of
`p ≤ 0.05` against the expected 4.25, and the real curves' ranks.

Verdict wording is fixed: "flagged after correction (p = …)", "within the
null distribution (p = …)", "positive control failed: inconclusive". No other
words ("random", "no effect", "secure", "never") are used for a cell.

## 7. What does not count

Changing `N`, `H`, the statistics, the null, the conditioning, the plants, the
threshold or the pass rule after the real curves are scored. Any such change
is a new, labelled protocol version, and both versions are reported.

## 8. Disclosure of prior computation, and the dry run

- The module's unit tests (`cargo test --release --lib
  cryptanalysis::b_weak_class_battery`) ran the five real curves through the
  conditioning and the statistic code and asserted only that the conditioning
  passes and every statistic is finite. No value was printed or inspected.
- A **dry run** at `N = 400`, `H = 100`, 20 plants per cell was done with the
  real-curve scoring switched off (no `--real`), on scratch output. It showed
  the instrument runs end to end, the phase timings below, and the
  resolution facts in §4. With `N = 400` the smallest `p` (`2.5 × 10^-3`) is
  above the threshold, so no plant could be detected there; that is the
  reason for `N = 8000`.
- Pre-flight runtime (dry run, shared 4-core host under load): null draw
  ≈ 12 ms (P-192), 20 ms (P-224), 10 ms (P-256), 30 ms (P-384), 63 ms (P-521)
  wall; fixed costs (plants) ≈ 1 s per 20 plants per curve at the small sizes.
  Estimate for the full run: `8000 + 1000` draws plus `1700` plants per curve
  ≈ 35 minutes wall in total. The actual time is recorded in `run.json`.

## 9. Command and stop conditions

```bash
cargo build --release --bin b_weak_class_battery
target/release/b_weak_class_battery --real \
  --out research/b_weak_class_battery_20261022/run
```

Defaults: `--nulls 8000 --holdout 1000 --plants 100 --tries 4000 --alpha 0.05
--detect-fraction 0.8 --label b-weak-class-null/v1`. `run/summary.json` has
every number in the report; `run/*.csv` are the raw rows (panel, held-out
draws, plants).

**Stop conditions.** A real curve fails the conditioning, a derived `c` differs
from the published one, or a crash. The failure is reported as it stands.

## 10. Power limits stated in advance

A null here does not show that no weak class exists. The battery sees only
classes that move one of these seventeen statistics by more than a
roughly 10-bit selection effect (the planted classes are the measure of that):
classes defined by a property of the group order, the trace, the CM field, a
larger isogeny degree (5 and above), or any non-linear function of `b` outside
these families are invisible. It cannot see a class unless its members move
the statistic beyond the top 4 of 8000 null draws. The null is conditioned
only on odd order and on no rational 3-torsion, not on prime order itself.
