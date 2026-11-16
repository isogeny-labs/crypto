# Genus 4: the positive-control speed-ups on the genus ladder — results (2026-11-16)

Status: every number is `[pcc:unverified]`. One local run, no independent
replay yet; red-team and validator review are pending (PROTOCOL §7), so
nothing here is a leaderboard or `benchmarks/README.md` row. Protocol and
binary frozen before data (`frozen.sha256`, `binary.sha256`, commit
`bce0eb2148`). Data `run.jsonl` (144 runs), `run.log`; tables `tables.md`,
`analysis.json`; figure `figure.svg`; report `REPORT.pdf`.

**Scope.** Three synthetic genus-4 hyperelliptic Jacobians per prime with
`h ≤ 2` (the instances of `research/genus_ladder_bound_r_20261114`), p =
2053, 4099, 8209, four solver seeds each. Index calculus beating rho on
genus-4 hyperelliptic Jacobians is a known asymptotic effect (Thériault 2003;
Gaudry, Thomé, Thériault, Diem 2007): this is a **positive control** of the
lab's instrument, not a statement about elliptic curves or any deployed curve.

## Gates

| gate | result |
|---|---|
| G1: instances rebuilt with the r-bound orders, h, r; c and R rows for seeds 1–2 equal the r-bound rows | **pass** |
| G2: every log certified (`[t]G = T` twice) and equal to the planted value, all four arms, 144 runs | **pass** |
| G3: unit tests (`ext_default_is_run_ic_count_for_count`, `ext_arms_certify_at_genus_3_and_4`; 11/11 genus_ladder) | pass |

## Components (paired on curve and seed)

| p | LA v/c | LA w/c | relation w/c | relation share of w's native mult. | fallbacks v, w |
|--:|--|--|--|--:|--|
| 2053 | 0.331 | 0.222 [0.219, 0.225] | 0.988 | 0.88 | 0, 0 |
| 4099 | 0.330 | 0.229 [0.226, 0.232] | 0.990 | 0.86 | 0, 0 |
| 8209 | 0.331 | 0.233 [0.230, 0.236] | 0.998 | 0.84 | 0, 0 |

The genus-3 results carry over: coordinate Wiedemann alone cuts the linear
algebra to 0.33×, with the unit-T walk to 0.23×, at every prime and without
a single fallback. Total cost against the baseline arm c: v 0.71–0.75×, w
0.66–0.70× at both w = 0.1 and w = 1 (all intervals below 1).

## Against the matched negation rho (§5a band)

| p | arm | w = 0 | w = 0.1 | w = 1 | FPM |
|--:|--|--|--|--|--|
| 2053 | c | 0.009 | 0.272 | 1.459 [1.148, 2.081] | 0.051 |
| 2053 | w | 0.008 | 0.192 | 1.018 [0.806, 1.436] | 0.021 |
| 4099 | c | 0.005 | 0.195 | 1.047 [0.675, 1.838] | 0.038 |
| 4099 | w | 0.005 | 0.133 | 0.708 [0.456, 1.251] | 0.015 |
| 8209 | c | 0.004 [0.003, 0.005] | 0.148 [0.107, 0.212] | 0.799 [0.578, 1.142] | 0.030 |
| 8209 | v | 0.004 [0.003, 0.005] | 0.105 [0.076, 0.151] | **0.564 [0.408, 0.805]** | 0.014 |
| 8209 | **w** | **0.004 [0.003, 0.005]** | **0.099 [0.072, 0.141]** | **0.528 [0.382, 0.754]** | 0.011 |

- At p = 8209 arm w's upper 95% bound is below 1 at all three prices: by the
  frozen decision rule this cell is reported as **below rho across the §5a
  band for these instances**, pending the reviews. Arm v alone also clears
  w = 1 there (upper end 0.805). The baseline c does not (upper end 1.142).
- At p = 2053 and 4099 w is below rho at w = 0 and w = 0.1, and its w = 1
  interval straddles 1.
- The w = 1 ratio falls with p (w: 1.018 → 0.708 → 0.528), as the exponent gap
  `p^{3/2}` against `p^2` predicts.
- FPM (the genus ladder's field-multiplication reading, which prices a Cantor
  step at its 393 field multiplications and the LA at g² per product): w is
  0.011× rho at p = 8209.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | w/c LA in [0.15, 0.35], v/c in [0.25, 0.45] at every p | **held** (0.222–0.233; 0.330–0.331) |
| P2 | w/R at w = 1, p = 8209, in [0.40, 0.75] | **held** (0.528) |
| P3 | w below R at w = 0 and 0.1 at every p | **held** |
| P4 | at p = 8209 the w = 1 upper bound below 1 | **held** (0.754) |
| P5 | relation multiplications ≥ 75% of w's native multiplications | **held** (0.84–0.88) |

## What is next

- Independent validator replay and red-team review of this cell (in progress)
  before any wording beyond `[pcc:unverified]`.
- The relation phase is now 84–88% of w's native work: a Cardano/Ferrari-type
  field-level splitting test for quartics (the genus-3 lever, generalised) is
  the next component.
- More curves at p = 8209 and a larger prime would tighten the rho-variance
  part of the intervals (3 curves per prime here).
