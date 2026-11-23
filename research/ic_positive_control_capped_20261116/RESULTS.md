# Positive control: arm W (capped unit-T walk + deterministic fallback roots) — results (2026-11-16)

Status: every number is `[pcc:unverified]` (one local run, no validator, so no
leaderboard or `benchmarks/README.md` row). Protocol and binary frozen before
data (`frozen.sha256`, `binary.sha256`, commit `81ddcdc450`). Data
`run_w.json`, `run_sr.json`; logs `run_w.log`, `run_sr.log` (both
`all_certified=true`); S and R rows at p > 8209 reused from the unit-T study
by determinism (PROTOCOL §2, §8). Tables `tables.md`, figure `figure.svg`,
report `REPORT.pdf`.

## Gates

| gate | result |
|---|---|
| G1: S and R re-run at p ≤ 8209 equal the unit-T study's rows exactly | **pass** |
| G2: every W log certified (148 runs); every W row has S and R partners | **pass** |
| G3: unit tests incl. `capped_unit_t_walk_solves_small_r_without_fallback` | pass (10/10) |

## W against S (same instances, same seeds)

| p | LA mult. W/S | LA incl. additions | relation mult. per trial W/S | **total W/S, w = 0.1** | **total W/S, w = 1** |
|--:|--|--|--|--|--|
| 4099 | 0.742 | 0.958 | 0.999 | **0.898** [0.878, 0.917] | 0.890 |
| 8209 | 0.777 | 0.867 | 0.771 | **0.786** [0.769, 0.819] | 0.776 |
| 16411 | 0.760 | 0.962 | 0.999 | **0.898** [0.888, 0.907] | 0.891 |
| 32771 | 0.772 | 0.954 | 1.000 | **0.895** [0.890, 0.901] | 0.889 |
| 65537 | 0.786 | 0.892 | 0.823 | **0.821** [0.815, 0.827] | 0.815 |
| 131101 | 0.791 | 0.965 | 1.000 | **0.902** [0.899, 0.906] | 0.897 |

- W is cheaper than S at both w = 0.1 and w = 1 at every p ≥ 4099, with every
  interval below 1: by the decision rule, **W advances on S and is the new
  operating point.** The gain is about 10% at the cube-root primes (LA only)
  and 18–22% at the fallback primes (LA and roots).
- The capped walk removed F1: no fallback at any p ≥ 4099 (the uncapped walk
  had 2 at 4099). At p ≤ 2053 W still falls back 3–7 times per prime: on
  matrices of 30–300 columns the random square subset is occasionally
  singular. The fallback is certified and charged; there W is level with S
  (W/S 0.75–1.07, intervals straddle 1).

## End to end: distance to the matched negation rho

| p | original C (11-12) w=0.1 | S w=0.1 | **W w=0.1** | original C w=1 | S w=1 | **W w=1** |
|--:|--:|--:|--|--:|--:|--|
| 16411 | 7.54 | 1.97 | **1.77** [1.03, 3.55] | 40.7 | 10.1 | **8.97** |
| 32771 | 7.44 | 1.92 | **1.72** [0.95, 3.32] | 40.2 | 9.85 | **8.75** |
| 65537 | 5.82 | 2.57 | **2.11** [1.39, 3.41] | 31.5 | 13.6 | **11.1** |
| 131101 | – | 1.97 | **1.78** [1.15, 4.53] | – | 10.2 | **9.17** |

- From the original positive control's arm C to W, the distance to rho at
  p = 32771 has gone from 7.44 to 1.72 at w = 0.1 (4.3× closer) and from 40.2
  to 8.75 at w = 1 (4.6× closer), with identical relation decisions in every
  step but the walk.
- **§5a verdict: still unresolved.** W is below rho at w = 0 from p = 257 on
  (0.11–0.16 at p ≥ 16411) and above it at w = 0.1 and w = 1 everywhere. No
  w = 0.1 interval lies entirely below 1 (the closest, p = 32771, has its lower
  end at 0.95).
- Trend: slope of ln(W/R, w = 0.1) on ln p over p ≥ 4099 is −0.161 (theory
  `p^{−1/6}` ≈ −0.167). Extrapolating the fit, not measured: W/R at w = 0.1
  reaches 1 near p ≈ 2^20–2^21; at w = 1 it does not reach 1 below about
  p ≈ 2^35 in genus 3.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | ≤ 5 LA fallbacks in total | **failed** (26, all at p ≤ 2053: singular square subsets of tiny matrices) |
| P2 | W/S LA in [0.60, 0.90] at every p ≥ 4099 | **held** (0.742–0.791) |
| P3 | W/S relation per trial in [0.70, 0.88] at 2053, 8209, 65537; [0.95, 1.05] at 4099, 16411, 32771, 131101 | **held** |
| P4 | W/R < S/R at every p ≥ 4099; at 32771 and 131101 in [1.4, 1.9] | **held** (1.72, 1.78) |
| P5 | §5a unresolved | **held** |

## What is left, and what follows from it

Native work in W at p = 32771 (means per run): relation phase about 66
multiplications per trial, linear algebra dominated by Berlekamp–Massey
(about 2N²). The remaining distance at w = 0.1 is about 1.7×, which the
`p^{−1/6}` trend closes only near p ≈ 2^20 at w = 0.1 and never in practice
at w = 1. In genus 3 the constant-factor work has paid off as expected; a
result below rho at all three prices needs the larger exponent gap of genus 4
(`p^{1/2}`), where the same instrument (Cardano–Lucas test generalised to
quartics, coordinate Wiedemann, capped unit-T walk) would apply.
