# Positive control: unit-T walk and deterministic fallback roots — results (2026-11-16)

Status: every number is `[pcc:unverified]` (one local run, no validator, so no
leaderboard or `benchmarks/README.md` row). Protocol and binary frozen before
data (`frozen.sha256`, `binary.sha256`, commit `551590e3b7`). Data `run.json`,
log `run.log` (`all_certified=true`, 5845 s), tables `tables.md`, figure
`figure.svg`, report `REPORT.pdf`.

## Deviation D1 (read this first): arm V did not run as registered

The protocol registers V as "unit-T walk + deterministic fallback roots". The
binary ran V with the **mixed** walk: an edit that should have switched V's
walk did not apply (rustfmt had reflowed the target line) and nothing caught
it before the freeze. `run.json` records `walk: mixed, oracle:
lucas_roots_fast` for every V row. So V as run is **S + deterministic fallback
roots**. The fallback is therefore compared with S (same walk, same relations
up to the root finder's randomness), which is a cleaner single-variable
comparison than the registered V/U; both are reported. The intended
"unit-T + fallback" arm goes into the next study.

## Gates

| gate | result |
|---|---|
| G1: S and R reproduce the previous study's rows on every shared (p, curve, seed) | **pass** |
| G2: every log certified (S, U, V, R; 592 runs incl. p = 131101) | **pass** |
| G3: unit tests at the frozen commit | pass |

## Finding F1: the unit-T walk needs restarts well inside √r

U fell back to the baseline solver, and kept failing, on small primes (268
fallbacks over 20 runs at p = 131, 13 at 2053, 2 at 4099; none from 8209 up).
Diagnosis on one failing instance (p = 131, curve 1, seed 1001; a temporary
debug build, not committed): the first core is 60 rows × 37 columns of
**rank 17**, and the target column is not determined.

Cause: walk segments are 4096 steps between restarts. When `√r` is not much
larger than that (r = 281357 here, `√r ≈ 530`), a segment falls into a rho
cycle and revisits positions.

- In the **unit-T** walk a revisited position carries the same coefficients,
  so it adds an identical row: no information, hence the rank loss.
- In the **mixed** walk a revisit carries different `(a, b)`, so the
  repeated row minus the first is a rho collision `Δa + Δb·log T = 0` that
  pins `log T` by itself.

So at small r the mixed-walk IC arms of all three positive-control studies
(and the original 11-12 control) partly solve by rho collisions inside their
walk. This affects the small-p rows (p ≲ 521, where `√r` is comparable to
4096), not the headline primes (p ≥ 4099, `√r ≥ 4.6e4`). The fix for unit-T
is a restart length well below `√r`.

## Components (paired on p, curve, seed)

| p | U/S LA mult. | U/S LA incl. additions | V/S relation mult. per trial (D1) |
|--:|--|--|--|
| 8209 | 0.778 [0.724, 0.936] | 0.985 | **0.770** [0.765, 0.776] |
| 16411 | 0.760 [0.750, 0.771] | 0.962 | 1.000 |
| 32771 | 0.772 [0.761, 0.782] | 0.954 | 1.000 |
| 65537 | 0.781 [0.774, 0.790] | 0.971 | **0.823** [0.822, 0.825] |
| 131101 | 0.791 [0.784, 0.797] | 0.965 | 1.000 |

- Unit-T cuts LA multiplications to 0.76–0.79 of S wherever it does not fall
  back. Counting additions too it is 0.95–0.97: most of the gain is in the
  multiplications the band prices; additions barely move.
- The deterministic fallback cuts relation work per trial to 0.77–0.82 at
  every prime where it applies (521, 2053, 8209, 65537) and leaves the
  others unchanged (1.000).

## End to end: distance to the matched negation rho

| p | S w=0.1 | U w=0.1 | V w=0.1 | S w=1 | U w=1 | V w=1 |
|--:|--|--|--|--|--|--|
| 8209 | 5.61 | 5.34 | **4.81** [3.44, 6.80] | 29.4 | 28.0 | 25.0 |
| 16411 | 1.97 | **1.77** [1.03, 3.54] | 1.97 | 10.1 | 8.97 | 10.1 |
| 32771 | 1.92 | **1.72** [0.97, 3.32] | 1.92 | 9.85 | 8.75 | 9.85 |
| 65537 | 2.57 | 2.42 | **2.25** [1.49, 3.62] | 13.6 | 12.8 | 11.9 |
| 131101 | 1.97 | **1.78** [1.15, 4.52] | 1.97 | 10.2 | 9.17 | 10.2 |

- The two changes act on different primes, so neither arm shows both: U is
  about 10% closer to rho at the cube-root primes, V 12–14% closer at the
  fallback primes. The combined arm is the next study.
- **§5a verdict: unresolved for every arm** (below rho at w = 0, above at
  w = 0.1 and w = 1 everywhere). U at p = 32771, w = 0.1: point 1.72,
  interval lower end 0.97 — the first interval to touch 1, not a result
  below rho.
- Trend: the slope of ln(V/R, w = 0.1) on ln p over p ≥ 4099 is −0.165,
  matching the predicted `p^{−1/6}`; the noise between neighbouring primes
  (instance-to-instance variance of rho, fallback primes) is larger than the
  trend over this range.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | U/S LA in [0.60, 0.85] at every p ≥ 4099 | **failed at 4099** (1.24, fallbacks, F1); held at every p ≥ 8209 (0.76–0.79) |
| P2 | V/U relation per trial ≤ 0.90 at fallback primes, [0.95, 1.05] elsewhere ≥ 1031 | **held** (0.77–0.82; 1.000–1.006); see D1 |
| P3 | V/R at w = 0.1, p = 131101, in [0.8, 1.8] | **failed** (1.97). V ran as S + fallback (D1), and 131101 is not a fallback prime; U there is 1.78 |
| P4 | no arm below R at all three prices | **held** |
| P5 | slope of ln(V/R, w=0.1) on ln p negative | **held** (−0.165) |

Decision rule: unit-T component rule (U/S ≤ 0.85 at every p ≥ 4099)
**not met** because of F1 at 4099; fallback rule **met**.

## Next

1. Unit-T with restart length `min(4096, ⌊√r / 16⌋)`, combined with the
   deterministic fallback (the intended V), in a new frozen study.
2. Re-check the small-p rows of the earlier studies for rho collisions inside
   the mixed walk (count repeated positions per run) and mark them.
3. Berlekamp–Massey is now most of the LA; relation work (66–146
   multiplications per trial) is most of the total.
