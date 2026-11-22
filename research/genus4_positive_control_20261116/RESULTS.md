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

- At p = 8209 arm w's upper 95% bound is below 1 at all three prices **of the
  band as this protocol priced it** (multiplications only, against
  negation-map rho). The red-team review (`redteam/REDTEAM.md`) keeps that
  statement only with qualifiers, and the w = 1 part does not survive a
  symmetric pricing:
  - **Additions unpriced (F2).** Arm w also performs 9.9e7 modular additions
    (1.92× its priced multiplications); rho's per-step partition hash and
    additions are not emitted. Pricing multiplications **and** additions on
    both arms as emitted: w/R at w = 1 is **1.057 [0.765, 1.499]**
    (unresolved). With rho's hashes and additions estimated as well: about
    0.63. AGENTS.md §5a prices every native operation, so the
    multiplications-only band of this protocol is a narrower reading than
    the rule.
  - **Cheapest rho not run (F1).** The lab's reference is the cheapest
    registered rho at each price; only the negation walk was run. Depending
    on how plain rho's per-step work is priced, plain rho can be cheaper for
    w above about 0.39 (IC / plain-rho estimate about 0.76 at w = 1).
  - **No calibrated weight (F3), 3 independent targets (F4)**: the 12 runs are
    3 targets × 4 solver seeds; leave-one-curve-out upper bounds 0.74–0.88
    against negation rho.
  - So: **robust below rho at w = 0 (0.004) and w = 0.1 (0.099 [0.072, 0.141])
    under every pricing tried; w = 1 unresolved** until a plain-rho arm, emitted
    rho addition/hash counters with the symmetric reading, and `w_cal` are in.
  - Arm v alone also clears w = 1 in the multiplications-only reading
    (upper end 0.805). The baseline c does not (upper end 1.142).
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
| P4 | at p = 8209 the w = 1 upper bound below 1 | **held as frozen** (0.754, multiplications only); does not hold with additions priced on both arms (1.057 [0.765, 1.499]; red-team F2) |
| P5 | relation multiplications ≥ 75% of w's native multiplications | **held** (0.84–0.88) |

## What is next

- Red-team review done (`redteam/REDTEAM.md`, findings F1–F7; no blocking
  finding, w = 1 claim qualified as above). Follow-ups it names: a plain-rho
  arm on the p = 8209 curves (≥ 8 seeds), rho's addition and hash counters
  and the symmetric multiplications-plus-additions reading, a measured
  `w_cal`, and more independent targets. Independent validator replay pending.
- Process notes from the review (F6): `analyze.py` was committed 40 s after
  the run started (the review's independent recomputation agrees, so no
  effect); rho certifies once, not twice; the bench record's `n` counts runs
  (12), not independent targets (3), and its problem id is the genus-3
  control's.
- The unit-T walk's LA gain is mostly multiplications becoming additions
  (w/v 0.705 in multiplications, 0.933 counting additions; F5), and the
  restart cap never binds at these r (cap ≥ 1.9e5 > 4096).
- The relation phase is now 84–88% of w's native work: a Cardano/Ferrari-type
  field-level splitting test for quartics (the genus-3 lever, generalised) is
  the next component.
- More curves at p = 8209 and a larger prime would tighten the rho-variance
  part of the intervals (3 curves per prime here).
