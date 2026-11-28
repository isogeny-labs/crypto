# Gaudry–Diem k = 3 Weil-descent oracle (arm D) against matched rho on binary composite fields

2026-11-27 · branch `claude/binary-weil-descent-20261126` · PR #5261, stacked on PR #5253 · study `bwd-20261126/v1`.
All measured numbers are **[pcc:unverified]**: L0 producer evidence from one shared 4-vCPU / 15 GB container. There is no
second-host replay and no validation gate, and nothing here is a leaderboard row. **Single target only**: every solve
is one cold solve of one public Q.

## Result

- **No parity at any rung, n = 18..48.** The whole cold solve with arm D costs **32× to 50× matched negation-map rho**
  for n = 18..39. It then rises to **541× at n = 48**, because echelon linear algebra takes 98% of D's cost there.
- **D's relation phase alone**, without linear algebra, falls from 34× rho to **10.3× [8.9, 12.2] at n = 48**. Its
  slope in log cost against log ℓ is 0.376 [0.369, 0.382], against rho's fitted 0.430.
- **Against #5253's arms:**
  - D is far cheaper than the pairs-and-solve arm F: D/F = 0.091 at n = 18 and 1.8·10⁻⁴ at n = 33.
  - D overtakes the m = 2 hash MITM arm M2 at n ≥ 33: D/M2 = 0.517 [0.494, 0.540] at n = 33 and 0.291 at n = 39.
  - So D is the best index-calculus arm measured in this lab on these fields, and it is still well above rho.
- **Projected crossover:** this is a model fitted to the measured constants, not a measurement.
  - With the full factor base there is **no crossover**. Linear algebra grows like q′², faster than rho's q′^{3/2}.
  - The relation phase alone would cross rho near **n′ ≈ 23.5, n ≈ 70**.
  - Only a double-large-prime variant, with exponent 4/3, could cross on the whole solve, roughly at n ≳ 130 before its
    own constants. This was not measured.

## Arm D

`harness/src/descent.rs`. Every field operation is counted in #5253's Meq unit through its counted field `CF`.

1. **Symmetric S₄.** The starting point is the symmetric form of the fourth summation polynomial for
   y² + xy = x³ + ax² + b, from crypto `koblitz_sym_s4.rs`. In it, e₁ and e₂ appear only squared, so D works in
   E₁ = e₁², E₂ = e₂² and t = e₃.
2. **Weil descent to K′ = F_{2^{n′}}.** The coordinates of x_R give three Frobenius-conjugate equations. Constant row
   operations, then the resultant of two quadratics in E₂, give a univariate U(t) of degree ≤ 20.
3. **Factor removal.** The structural factors t^v (t + τ₀)^w are removed. What is left, U′, has degree 16, with mean
   15.87–16.00 across rungs.
4. **Subfield roots.** They are found by gcd(U′, t^{q′} + t).
5. **Lifting.** Each root is lifted exactly to (E₁, E₂), with no search. The cubic X³ + e₁X² + e₂X + e₃ must split with
   every root in the factor base.
6. **Group-law check.** Every candidate relation is checked with the group law before it is used.

**Reuse.** #5253's field, gen, ic, rho and seeds modules are included unchanged. So the curves, targets, cycle-free
target stream (A1), group-law relation check, incremental echelon linear algebra (A2), negation-map rho and cost unit
are bitwise #5253's. The #5253 binary rebuilt from this branch has #5253's recorded sha256 (`bf3f431f…`).

## Design

The protocol is in [`PROTOCOL.md`](PROTOCOL.md) and [`protocol.json`](protocol.json). It was frozen at commit
`f08153504` before any D evaluation data.

- **Rungs.** n = 3n′ for n = 18..33, the #5253 panel: 4 curves × 3 targets, or 2 × 2 at n = 33. The extension
  n = 36..48 was declared before its data: 4 × 3 targets, or 2 × 2 at n = 48. F was run only on the panel, and M2 only
  up to n = 39 because of memory.
- **Rho.** Rho is the mean of 16 runs on the same Q. Its lane count was tuned on held-out pilot curves.
- **Statistics.** Each ratio is the geometric mean of per-case ratios, with a 95% hierarchical bootstrap (B = 10000)
  over curves and then targets.
- **Seeds.** Seed policy `isogeny-labs/experiment-seeds/v1`; hashes are in [`SEEDS.sha256`](SEEDS.sha256).

## Per-rung results

Full table: [`data/summary_table.md`](data/summary_table.md). Figure: [`figures/gap_to_rho.svg`](figures/gap_to_rho.svg).

| n | n′ | cases | D/rho | D/F | D/M2 | rho A/A | LA share of D |
|---|---|---|---|---|---|---|---|
| 18 | 6 | 12 | 34.2 [26.7, 43.7] | 0.091 [0.076, 0.113] | 9.45 [6.87, 13.6] | 1.020 [0.949, 1.101] | 0.6% |
| 21 | 7 | 12 | 34.6 [27.0, 50.6] | 0.028 [0.022, 0.040] | 6.26 [4.50, 9.29] | 0.961 [0.904, 1.015] | 0.8% |
| 24 | 8 | 12 | 49.5 [44.6, 55.8] | 8.09e-3 [7.88e-3, 8.21e-3] | 3.25 [2.79, 3.76] | 1.015 [0.852, 1.190] | 0.9% |
| 27 | 9 | 12 | 44.3 [37.9, 52.7] | 2.24e-3 [2.04e-3, 2.46e-3] | 2.03 [1.64, 2.64] | 1.026 [0.895, 1.181] | 1.7% |
| 30 | 10 | 12 | 46.4 [39.4, 55.8] | 6.27e-4 [5.88e-4, 6.69e-4] | 1.14 [0.99, 1.30] | 0.953 [0.813, 1.129] | 3.4% |
| 33 | 11 | 4 | 32.1 [28.0, 37.9] | 1.80e-4 [1.78e-4, 1.81e-4] | 0.517 [0.494, 0.540] | 0.820 [0.713, 0.962] | 8.8% |
| 36 | 12 | 12 | 47.1 [42.1, 53.2] | — | 0.381 [0.349, 0.434] | 1.007 [0.847, 1.187] | 24% |
| 39 | 13 | 12 | 49.8 [44.8, 56.4] | — | 0.291 [0.262, 0.328] | 1.138 [0.952, 1.347] | 51% |
| 42 | 14 | 12 | 92.6 [84.5, 103.3] | — | — | 0.974 [0.885, 1.089] | 78% |
| 45 | 15 | 12 | 195 [158, 253] | — | — | 0.984 [0.831, 1.167] | 93% |
| 48 | 16 | 4 | 541 [457, 646] | — | — | 0.854 [0.643, 1.134] | 98% |

- **Relation phase alone, against rho:** 34× at n = 18, 24× at n = 39, then 20×, 13× and 10.3× [8.9, 12.2] at
  n = 42, 45 and 48.
- **Oracle cost per target:** 1.9·10³ Meq at n′ = 6, rising linearly in n′ to 4.4·10³ at n′ = 16.
- **Decomposition rate:** decompositions per target match the predicted 4F³/(3N) ≈ 1/3! to within 1% from n′ = 8 on.
  A solve uses about 2.7–2.9·q′ targets.
- **Small rungs:** the n = 33 and n = 48 rows have 4 cases each and carry about ±20% extra noise.
- **Rho A/A:** the control (seeds 0–7 against 8–15) includes 1 at every rung except n = 33, where it is 0.82
  [0.71, 0.96].

**Slopes** of log₂ cost against log₂ ℓ:

| Arm | Slope |
|---|---|
| D relation phase | 0.376 [0.369, 0.382] |
| D total, all rungs | 0.511 [0.504, 0.517] |
| D total, n ≤ 33 | 0.387 |
| rho, all rungs | 0.430 |
| rho, n ≤ 33 | 0.368 |
| M2 | 0.674 |
| F | 0.980 |

**Cost band.** Non-field operations were priced at 0, 0.1 and 1, and squarings at 1 and 0.25. The order of the arms
holds across the whole band:

- D > rho everywhere. The lowest value is 10.3 for the relation phase at n = 48, with mod-ℓ operations priced at 0.
- D < F everywhere.
- D < M2 at n ≥ 33 (0.16–0.68) and D > M2 at n ≤ 27.
- No order is stated at n = 30, where the band is 1.14–1.55 and the CI touches 1.

## Projected crossover (model, not measurement)

Fitted inputs:

- Rho costs S·√ℓ with S = 9.5, from the top rung.
- D's relation phase costs 2.76·q′·(399 + 252·n′).
- Linear algebra is measured to grow like F^2.79 with the echelon solver; a Wiedemann model would give 10F².

What the model gives:

- **Full factor base:** no crossover. Linear algebra in F ≈ q′ grows like q′², faster than rho's q′^{3/2}.
- **Relation phase alone:** it crosses near n′ ≈ 23.5, or n′ ≈ 24.25 using n = 45's S = 7.5.
- **Balanced (reduced) factor base:** it reaches rho's exponent but levels off about 59× above rho, so it never
  crosses.
- **Double large prime:** with exponent 4/3 it could cross, roughly at n ≳ 130 before its own constants. It was not
  measured.

## Verification

- **Logs:** 116 of 116 D logs are correct. Each is checked by d·G = Q inside the counted solve and against the true d.
  None of the 2,124 records used has a failure.
- **Decomposition sets:** D's sets equal F's on 20,247 targets and the exhaustive group-law set on 6,846 targets, with
  0 mismatches. No relation was left unresolved by the group law.
- **#5253 reproduction:** D's harness reproduces #5253's F records (d, targets, relations, columns) on 64 of 64 panel
  cases.
- **Replay:** the case n = 30, ci = 0, ti = 0 replays bitwise ([`data/replay_compare.txt`](data/replay_compare.txt)).
- **Self-tests, before the freeze and on separate instances:**
  - the symmetric S₄ equals #5253's quartic;
  - D, F and brute force agree;
  - 24 cold solves are correct.

  Log: [`data/selftest_prefreeze.log`](data/selftest_prefreeze.log).
- **Predictions:** all five frozen hypotheses held.

## Deviations (all kept)

1. **Fallback searches.** Four ran: 2 rank-deficient cases and 2 with U ≡ 0, all at n ≤ 21. The protocol said none were
   expected. They are counted in D's cost, and all those solves are correct.
2. **Analysis change after the freeze.** Only the figure's x-axis in `analysis.rs` changed. `summary.json` is
   byte-identical under the frozen binary. The run binary was rebuilt from the frozen sources before the n = 48 run,
   with the same hash `c704c7b6…`.
3. **Engineering runs before the freeze.** These touched three evaluation cases. They are disclosed in PROTOCOL.md, kept
   in [`data/prefreeze_dev_runs.jsonl`](data/prefreeze_dev_runs.jsonl) and not used in the analysis.
4. **Rho lane count.** W = 64 sits at the edge of the frozen grid, so the extension's D/rho ratios may be slightly
   understated, in D's favour.

## Not done

- **Measurement infrastructure:**
  - isolab/L2 runs;
  - a bench.record;
  - an S3 upload;
  - a `.resume/` checkpoint;
  - ICV1 curve slugs.

  All are the same as #5253.
- **research-kb:** no prior-art check, because the MCP server was not available.
- **Next steps, all single-target:**
  - a cheaper linear algebra, using structured Gaussian elimination plus block Wiedemann. This is now the binding cost
    from n ≈ 39;
  - the double-large-prime variant;
  - native subfield root finding of U′, where the lab's fast root finder could apply.

## Reproduce

```
cd harness && CARGO_BUILD_JOBS=3 cargo build --release
../run.sh               # panel + extension; refuses a manifest that differs from SEEDS.sha256
data/replay_compare.sh  # bitwise replay of n=30, ci=0, ti=0
```
