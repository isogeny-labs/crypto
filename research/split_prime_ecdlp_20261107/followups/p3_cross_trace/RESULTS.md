# P3 results: cross-trace covariate test of the split-prime statistic (run-20261107T2220Z)

**Outcome (frozen rule 2): null at the tested sizes** [pcc:unverified]. Across
2,000 random prime-order curves at each of p24 = 16,777,213 and
p28 = 268,435,399, neither exact m = 2 decomposition coverage nor the measured
rho cost depends on the split-prime statistic z_691, the partial L(1, χ)
ratio, or the ψ3/ψ5 root counts. None of the 14 tests is significant at the
Bonferroni level (α = 0.003125); no covariate reaches |effect| ≥ 1 % with the
same sign at both primes. The positive controls are informative (planted +2 %
recovered on both outcomes, analytic power ≥ 0.995). The seed of this study, the
split-prime deficit of the published curves, is therefore closed for the
coordinate (index-calculus coverage) track at these sizes, as it was closed for
the generic track by the scalar-only obstruction. The A/A control exposed a
defect in this study's rho instrument (below), which limits what the rho half
says.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md) (frozen 535d9da826; Addendum A before any
  claim cell; merged in #3657).
- Card `CARD-p3-cross-trace-20261107-2` (sha256 `2ef981a0…503e`), receipt
  `cards/p3-run-02.receipt.json`, launch log `cards/p3-run-02.launch.log`, director
  assessment `cards/p3-run-02.assessment.md`.
- Card 1 (`CARD-p3-cross-trace-20261107-1`, `runs/run-20261107T214600Z/`) was killed by
  the executor shell's 30-minute background limit after its p24 steps (no receipt);
  card 2 was launched detached. **Replay check:** card 1's p24 curves, covariates,
  rho A, rho B and analysis files are byte-identical to card 2's, and its yield file
  is identical apart from the wall-time field; its p28 curves and covariates are
  identical too.
- Analysis: written by `p3census analyze` inside the card (`runs/.../p24|p28/analysis.json`);
  descriptive statistics by `analysis/p3describe` (Rust) → `analysis/derived/p3_descriptive.txt`.
  Figure: `figures/p3_summary.svg`.

## The 14 tests (effect per covariate SD as % of the outcome mean, 95 % CI)

| prime | outcome | z_691 | logL1_200000 | ψ3 roots | ψ5 roots |
|---|---|---|---|---|---|
| p24 | rho ops/√n | +0.17 [−0.37, +0.71] | +0.37 [−0.17, +0.91] | −0.07 [−0.62, +0.47] | +0.70 [+0.16, +1.25], p = 0.011 |
| p24 | coverage / (2N²/n) | −0.002 [−0.003, +0.000] | −0.000 [−0.002, +0.002] | −0.001 [−0.003, +0.001] | −0.000 [−0.002, +0.002] |
| p28 | rho ops/√n | +0.26 [−0.45, +0.97] | +0.43 [−0.29, +1.15] | +0.61 [−0.11, +1.34] | not tested (degenerate, Addendum A) |
| p28 | coverage / (2N²/n) | −0.000 [−0.000, +0.000] | −0.000 [−0.000, +0.000] | −0.001 [−0.001, −0.000], p = 0.020 | not tested |

All Bonferroni-adjusted p = 1.0 except those printed with an uncorrected p; none
below 0.003125. Coverage effects are exact-count effects of at most 0.002 % of the
mean: the coordinate track sees nothing. Three rho intervals reach past +1 %
(p24 ψ5; p28 logL1, ψ3), so the rho half cannot exclude a +1 % effect everywhere;
the frozen decision (significant and ≥ 1 % at both primes) is not met by any of
them.

## Instrument finding from the A/A control: the rho walk is defective

Two rho runs per curve with disjoint seeds correlate at r = 0.974 (p24) and 0.986
(p28), and the between-curve variance is 41 and 72 times the within-curve
expectation. Per-curve cost is **bimodal**: about 1.6 or 2.6 · √n at p24 and about
2.7 or 5.1 · √n at p28, every curve in one of two tight clusters, with no unsolved
target, no mismatch and identical setup cost. A sound negation-map walk costs about
0.9 · √n and does not grow with n. So outcome Y1 measures this walk, not rho in
general; the null says only that this walk's cost does not track the covariates.
The jump table is seeded per target (not per curve), and the clusters are not
separated by trace parity, t mod 4, n mod 3 or n mod 4, or a mod 2. A likely
ingredient is the walk's 32 small-coefficient jumps (u_j, v_j < 16) under the
negation map, which create many short linear relations among jumps and hence
recurrent fruitless cycles; why that splits curves into two classes is not
established. Recorded as `problems/p3census-rho-walk-bimodal.md`. The coverage
outcome is exact and does not use rho.

## Descriptive (decides nothing)

Percentile of each published curve's z_691 within the census (share of census curves
with z ≤ it), p24 / p28: prime239v1 0.05 % / 0.00 % (below all 2,000 curves at p28),
secp224r1 4.00 % / 2.75 %, secp521r1 5.50 % / 5.45 %, secp384r1 13.95 % / 14.50 %,
the others between 27 % and 54 %. The census is at toy primes, so this places the
published curves against random prime-order curves under the same baseline; it is
not a statement about the curves' own fields.

## Next steps

1. Re-run Y1 with the lab's `ic_wide::rho_negation` (the main crate's dependencies
   could not be resolved offline here) or with a fixed walk (r = 32 random
   full-size jump coefficients), with the same frozen covariates and seeds.
2. Diagnose the bimodality on 50 curves at p ≤ 2^20 (cheapest: per-curve fruitless-cycle
   escapes and cycle lengths against the two clusters).

## Round 2: rho outcome rerun with the corrected walk (run-r2-20261108T0544Z)

**Outcome (PROTOCOL-R2 rule 3): null, with an informative instrument** [pcc:unverified].
Card `CARD-p3-cross-trace-20261108-r2` (`cards/p3-run-r2.assessment.md`), walk v2, the same
2,000 curves per prime, covariates and seeds as round 1.

- **Instrument (rule 1 not triggered).** A/A correlation between disjoint-seed runs fell
  from 0.974 / 0.986 (walk v1) to **0.027 / 0.009**; between-curve variance ratios are
  1.016 / 0.992 (p24) and 0.965 / 0.948 (p28), against 41 and 72 with v1. Every one of the
  4,000 per-curve means lies between 0.850 and 1.091·√n (medians 0.97 and 0.95), inside the
  frozen 0.7 to 1.3 band. The positive control recovers the planted +2 % (1.91 % and 1.96 %),
  with a minimum detectable effect of 0.19 % at 80 % power.
- **Tests.** All 7 rho tests are null. The largest effect is −0.087 % per SD (p24, z_691,
  95 % CI [−0.19 %, +0.01 %], p = 0.083); every interval lies within ±0.19 % of zero, far below
  the 1 % decision line.

| prime | z_691 | logL1_200000 | ψ3 roots | ψ5 roots |
|---|---|---|---|---|
| p24 | −0.087 [−0.186, +0.011] | +0.013 [−0.086, +0.113] | −0.022 [−0.122, +0.077] | −0.024 [−0.124, +0.075] |
| p28 | −0.036 [−0.133, +0.061] | +0.025 [−0.073, +0.123] | −0.036 [−0.134, +0.062] | not tested |

(rho ops/√n, effect per covariate SD as % of the mean, 95 % CI.)

**Reading.** With a sound walk, the split-prime statistic, the partial L(1, χ) ratio and the
ψ3/ψ5 root counts change rho cost by less than 0.2 % per SD at both primes. Together with the
round-1 coverage null, the seed of this study is closed on both outcomes at 2^24 and 2^28.
The round-1 rho intervals that reached past +1 % came from the defective walk.
`problems/p3census-rho-walk-bimodal.md` is resolved by this run.
