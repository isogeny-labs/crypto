# Does the Joux-Vitse cover cross measured rho at p = 1301-1601? (2026-11-01)

**Scope.** This weak class only: the `jv_cover` genus-3 cover of E(F_{p^6}) over F_{p^2}, full rational 2-torsion, never prime order, `generate_spec(p, seed)`. Lower-bound comparison. No statement about prime-order or deployed curves. Verdicts below are the frozen rules of [`PROTOCOL.md`](PROTOCOL.md) (sha256 `6643ce68...b761` and of `run.sh` in [`results/FROZEN.sha`](results/FROZEN.sha), frozen before any main-run data), nothing else.

Labels: **measured** = read from a run record; **derived** = computed from measured values by the Rust tool in [`fit/`](fit/); **model** = extrapolation or prediction.

Files: graph [`scaling.svg`](scaling.svg) / [`scaling.png`](scaling.png), [`REPORT.pdf`](REPORT.pdf), [`table.md`](table.md), `fit.json` / `fit.txt` (all statistics), raw records and logs in [`results/`](results/), `run.sh`.

## Result per prime

S = cover total F_p multiplications / (c_add_e sqrt(l)), group additions on E(F_{p^3}) per sqrt(l) (**measured**, 3 seeds per prime, all `solved`, `correct`, 0 mismatches). rhodp: `run_rho_dp`, S = group ops / sqrt(l), 4 threads, dp_bits 18 (**measured**, 3 runs on each of 3 instances = n 9). rhostrong: `run_strong_rho` with negation, batch 16, r 256, dp_bits 16, `s_harness` (**measured**, 2 runs on each of 3 instances = n 6). S/rho = geometric mean over seeds of S divided by the prime's reference mean, with a 95% percentile bootstrap (20000 resamples of seeds and of reference runs, **derived**).

| p | cover S per seed | geo-mean S | rhodp S mean ± sd (n) | S/rhodp [95% boot] | verdict vs rhodp | rhostrong S mean ± sd (n) | S/rhostrong [95% boot] | verdict vs rhostrong | preregistered prediction (model) |
|---|---|---|---|---|---|---|---|---|---|
| 1301 | 1.494, 1.337, 1.125 | 1.310 | 1.651 ± 0.795 (9) | **0.793** [0.587, 1.119] | not resolved | 0.971 ± 0.347 (6) | **1.349** [1.085, 1.699] | above measured rho at p | 1.04 |
| 1451 | 0.982, 1.036, 1.241 | 1.081 | 1.395 ± 0.671 (9) | **0.775** [0.645, 0.970] | **below measured rho at p** | 1.017 ± 0.477 (6) | **1.063** [0.819, 1.477] | not resolved | 0.84 |
| 1601 | 1.118, 0.716, 0.665 | 0.811 | 1.381 ± 0.385 (9) | **0.587** [0.448, 0.811] | **below measured rho at p** | 0.892 ± 0.382 (6) | **0.910** [0.658, 1.339] | not resolved | 0.68 |

Per-seed ratios (derived): vs rhodp 1301: 0.905 / 0.810 / 0.681; 1451: 0.704 / 0.743 / 0.890; 1601: 0.810 / 0.519 / 0.482. Vs rhostrong 1301: 1.539 / 1.377 / 1.159; 1451: 0.966 / 1.019 / 1.220; 1601: 1.255 / 0.804 / 0.746. The preregistered predictions used the pooled rho 1.344; the measured rhodp means (1.65, 1.40, 1.38) are at or above it, and the cover S is lower than the old fit implies at 1601, so the measured S/rhodp at 1601 (0.59) is below the prediction 0.69 and at 1301 (0.79) below 1.04. The rhostrong mean is 0.89-1.02, about 0.6-0.7 of rhodp, so the two references give different verdicts: the verdicts are reference-dependent. `s_tuned` of rhostrong (information only, a different counting convention) averages about 0.37-0.42.

**Candidate result, needs independent validation.** Two primes (1451, 1601) are "below measured rho at p" against the rhodp reference under the frozen rule. This is a candidate result only. It needs the repository's independent validation and red-team review (`validator-breakthrough` / `red-team-breakthrough` class) before any claim; it has no replay certificate, validator report or VALIDATION.json, so it does not enter the IC leaderboard or `benchmarks/README.md` (not touched). Reasons for caution, all stated here: (a) against the secondary rhostrong reference no prime is below (1301 is "above", the others not resolved); (b) rhodp means exceed the predecessor's measured rho_e mean at 251 (below), and the new-prime rhodp means are 1.38-1.65, so a possible upward bias of rhodp (the calibration difference +0.28 is not significant but is not zero) would move the rhodp ratios up by up to about 25% (derived: 1.395 / (1.395 - 0.28) = 1.25), which would put 1451 at about 0.97, borderline; (c) 9 rhodp runs per prime, below the requested 4 per instance; (d) 3 cover seeds per prime, so the seed bootstrap is coarse.

## Controls (measured / derived)

* **Calibration (positive control), passed on the frozen rule.** `rhodp` at p = 251, seeds 1-2, 16 runs: mean 1.539, sd 0.666. The predecessor's `rho_e` samples on the same instances (32 runs): mean 1.264, sd 0.558. Difference +0.276, Welch df 25.9, 95% interval [-0.123, +0.673] contains 0 (rule met); |difference| < 0.3 also holds (information). Per seed rhodp means 1.83 (seed 1) and 1.25 (seed 2): the seed-1 instance read high. The agreement is statistical, with a wide interval; it does not exclude a bias of about 0.3.
* **Reproduction control, FAILED.** `dlp` p = 1009 seed 1 gave `total_muls` = 429320392173 against the predecessor's 452313823812: **not bit-identical**, ratio 0.949 (derived). Cause (read from the two records): residuals, relations and `f4_muls` are the same; `lin_muls` fell from 113394312494 to 90400879953, i.e. the working tree's linear stage counts fewer multiplications than the predecessor's binary (sha256 `a045cb80...e34d`; this study's binary `4a1284e9...9752` was built from the current tree). So the cover S of this study is not strictly the predecessor's instrument, and mixing the predecessor points into the refit carries a 5% scale difference at p = 1009 (the only point where both exist). Sensitivity, post-hoc and labelled so: fit F multiplies every predecessor S by 0.949.
* Every cover run `solved = true`, `correct = true`, `mismatches = 0`; every rhodp run `correct = true`; every rhostrong run `correct = true`, not capped. No timeouts or OOM. `results/walls.txt` has every exit status.

## Refit (derived; p* is a model extrapolation)

All per-seed points, ln(S/rho) on ln p. Primary variant A: predecessor points with their own measured rho_e (p <= 251), predecessor 503 and 1009 with the pooled 1.344 (**still an assumption**), new primes with their measured rhodp mean.

| variant | n | slope (t 95%) | slope (cluster bootstrap 95%) | p* (bootstrap 95%) |
|---|---|---|---|---|
| predecessor (2026-10-24, pooled rho) | 17 | -2.05 [-2.28, -1.83] | [-2.29, -1.84] | 1329 [1168, 1540] |
| **A primary** | 26 | **-2.10** [-2.21, -1.99] | [-2.23, -1.98] | **1264** [1197, 1348] |
| B: 503, 1009 priced at the new-prime rhodp pooled mean | 26 | -2.11 [-2.22, -2.00] | [-2.24, -1.98] | 1246 [1181, 1328] |
| C: only points with measured rho | 20 | -2.11 [-2.23, -1.98] | [-2.24, -1.98] | 1249 [1171, 1351] |
| D: new primes only, vs rhodp | 9 | -1.43 [-3.24, 0.38] | [-3.64, -0.17] | 1140 [211, 1352] |
| E: new primes only, vs rhostrong | 9 | -1.90 [-3.59, -0.22] | [-3.88, -0.28] | 1515 [1397, 2553] |
| F (post-hoc): A with predecessor S x 0.949 | 26 | -2.08 [-2.19, -1.97] | [-2.22, -1.96] | 1253 [1187, 1337] |

With rhodp as the reference p* moves from 1329 to about 1264 (variants A-C, F: 1246-1264), a shift of about -65, inside the old interval; the slope stays near -2.1, consistent with the cost model's -2. Against rhostrong alone (variant E, three primes, wide) p* is about 1515. The line at the new primes under A is 0.94 / 0.75 / 0.61 (model). The p* intervals do not include the 503 and 1009 rho assumption's uncertainty (variants B and C show its effect is small, 1246-1264).

## Section 5a handling

The cover arm reports only F_p multiplications (`total_muls`, group-addition equivalent via c_add_e); it does not emit its F_p additions, reductions, memory traffic or Wiedemann vector work. rhostrong emits native counts (about 15-17 native operations per sqrt(l) of walk, i.e. 15-17 per unit of S, mostly canonical compares and hashes; **measured**). rhodp does not emit counts; its native work is **derived** as one hash per step plus steps/2^18 DP table operations (about 1.4-1.65 per unit S). No pair of arms has counts on both sides, so **no band ranking is possible**. One-sided sensitivity only (rho arm priced at w = 0 / 0.1 / 1 GAE per native op, cover left at 0, not a valid band; derived), S/rho vs rhodp: 1301: 0.79 / 0.72 / 0.40; 1451: 0.78 / 0.71 / 0.39; 1601: 0.59 / 0.53 / 0.29; vs rhostrong: 1301: 1.35 / 0.50 / 0.08; 1451: 1.06 / 0.39 / 0.06; 1601: 0.91 / 0.34 / 0.05. Pricing only the denominator lowers S/rho by construction, so these are not evidence of anything. Every number above is a w = 0 lower-bound comparison.

## Failures and deviations (recorded)

1. **rhodp runs per instance: 3, not the requested 4** (9 per prime). The projection at 4 per instance was about 8.5 h plus 0.55 h already spent, over 8 h; the cut was applied to runs per instance as instructed, no prime dropped. The 6.5 h launch deadline did not bind (all nine rhodp instances ran). Actual: the run took 24850 s (6.9 h) after the 0.55 h of setup and timing, about 7.5 h in total.
2. Reproduction control not bit-identical (above). The instrument differs from the predecessor's binary in the linear-stage multiplication count.
3. 1301 seed 1 cover run is the pre-freeze timing probe, reused as declared in the protocol (same binary, same command). The pre-freeze rhodp and rhostrong timings were short partial or p = 503 runs and are discarded.
4. After freezing, the analysis tool (not the protocol) was changed three times: an empty-bootstrap guard, an XML escape in the SVG legend, and the added post-hoc variant F. The frozen decision rules were not changed.
5. The PNG was rendered from the Rust-written SVG with LibreOffice headless (no `rsvg-convert`); the PDF was built with pandoc to ODT and LibreOffice headless. No Python computed any number.
6. The rhostrong study's 2026-10-20 review withheld comparative conclusions because of missing numeric control rules; this study states its rules above, but rhostrong has no separate positive control here (the calibration is for rhodp only).

## Reproduce

```text
cargo build --release --example jv_cover        # binary sha256 in results/versions.txt
research/jv_cover_crossover_20261101/run.sh     # ~6.9 h on 4 vCPU
cd research/jv_cover_crossover_20261101/fit && cargo build --release --offline && ./target/release/jv_crossover_fit ../results ../../jv_cover_p_scaling_20261024/results ..
```
