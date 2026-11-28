# Positive control with the Cardano–Lucas splitting test: results (2026-11-16)

Status: every number is `[pcc:unverified]`. One local run on a shared 4-vCPU VM
(PROTOCOL.md §8), no independent validator yet, so no IC-leaderboard or
`benchmarks/README.md` row (validation gate). Protocol frozen before data in
commit `4feb188f47` (`frozen.sha256`). Data `run.json`, log `run.log`
(`all_certified=true`, 583 s wall), tables `tables.md` (from `analyze.py`),
figure `figure.svg`/`figure.png`, report `REPORT.pdf`.

## Gates

| gate | result |
|---|---|
| G1: C and R rows equal the 11-12 control's (group ops, relation and LA multiplications, trials, relations, certification) | **pass** |
| G2: L equals C in trials, splits, relations, group ops, LA work and answer, with fewer relation multiplications, on every run | **pass** |
| G3: every log certified (C, L, M, R; 696 runs) | **pass** |
| exhaustive oracle tests (every monic cubic, 12 primes) and trajectory test | pass (6/6) |

## Component result: the relation phase halves (arm L, exact pairing)

Native `F_p` multiplications per relation trial, and the paired ratio:

| p | C | L | M | L/C [95%] | M/C [95%] |
|--:|--:|--:|--:|--|--|
| 1031 | 240.5 | 115.2 | 53.5 | 0.479 [0.466, 0.491] | 0.223 [0.217, 0.229] |
| 4099 | 273.5 | 127.7 | 57.7 | 0.467 [0.460, 0.473] | 0.210 [0.207, 0.214] |
| 16411 | 334.6 | 154.6 | 67.0 | 0.462 [0.457, 0.468] | 0.202 [0.200, 0.205] |
| 32771 | 335.9 | 152.4 | 66.2 | 0.454 [0.450, 0.457] | 0.196 [0.194, 0.197] |
| 65537 | 362.1 | 177.6 | 177.6 | 0.490 [0.487, 0.494] | 0.490 [0.487, 0.494] |

- The test alone (L) is 0.45–0.52× the baseline's relation work at every
  `p ≥ 257`, with the same relations. The decision rule (≤ 0.8 at every
  `p ≥ 1031`) holds: **the component advances**.
- The test plus Cardano roots (M) is 0.20–0.22× where a single-exponent cube
  root exists. Where `3 | n/3` (p = 521, 2053, 8209, 65537) M falls back to
  Cantor–Zassenhaus roots and equals L. At the headline prime 65537 that is
  the case, so M = L there.

## End to end: distance to the matched negation rho (§5a band)

Arm total ÷ R total, two-stage bootstrap 95%:

| p | C w=0.1 | best new w=0.1 | C w=1 | best new w=1 |
|--:|--|--|--|--|
| 4099 | 10.43 [5.09, 30.84] | M 5.10 [2.48, 15.11] | 56.24 | M 26.82 [13.04, 79.90] |
| 16411 | 7.54 [4.41, 15.15] | M 3.69 [2.15, 7.41] | 40.73 | M 19.54 [11.41, 39.25] |
| 32771 | 7.44 [4.21, 14.33] | M 3.69 [2.08, 7.12] | 40.24 | M 19.57 [11.02, 37.84] |
| 65537 | 5.82 [3.84, 9.36] | L = M 3.97 [2.62, 6.38] | 31.49 | L = M 21.30 [14.06, 34.27] |

- The distance to rho halves at `w = 0.1` and `w = 1` where the cube root
  applies (p = 16411, 32771: 7.5 → 3.7 and 40 → 19.6), and falls by a third at
  65537 (5.82 → 3.97, 31.5 → 21.3).
- At `w = 0` (group operations only) nothing changes: the relations are the
  same, 0.11× rho at 65537.
- **§5a verdict: unresolved, as before.** Every arm is below rho at `w = 0`
  from p = 257 and above it at `w = 0.1` and `w = 1` at every p. Nothing is
  below rho at all three prices.

## The next bottleneck is the linear algebra

Share of native multiplications in the linear algebra: C 0.36, L 0.54, M 0.74
(p = 32771). With the relation phase at about 66 multiplications per trial,
Wiedemann on the peeled core is now most of the native work. At p = 32771 the
LA alone is about 2.6× rho at `w = 0.1`; no relation-phase change can close
the gap at this p without it.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | per trial at 65537: C ≈ 362, L in [140, 230], M in [70, 150] | C 362.1 and L 177.6 **held**; M 177.6 **failed**: 65537 has `3 | n/3`, so M fell back to L (this was in round 4's notes; the prediction missed it). Where the cube root applies, M is 66–67 at 16411/32771, just under the band. |
| P2 | L/C ≤ 0.65 at every p ≥ 1031 | **held** (max 0.490) |
| P3 | M/R at 65537 in [2.5, 4.5] at w = 0.1, [12, 25] at w = 1 | **held** (3.97, 21.30) |
| P4 | no arm below R at all three prices | **held** (verdict unresolved) |
| P5 | LA ≥ 45% of M's native multiplications at 65537 | **held** (54%) |

## Reading and next levers

- This is a **component** improvement to the positive control's instrument,
  ported from round 4 item 01, which first built and measured it without a
  rho arm. What is new here is the matched-rho reading: the port halves the
  distance to rho at `w ≥ 0.1` but does not change the §5a verdict.
- Distance remaining at the best measured point (p = 16411–32771, arm M):
  3.7× rho at `w = 0.1`, 19.6× at `w = 1`.
- Next, in order of the measured shares:
  1. Linear algebra (54–74% of native work): structured Gaussian elimination
     before Wiedemann, block Wiedemann, and rebalancing the factor base
     against large primes. Round 4's C3 (oversampling, clique removal, SGE)
     cut LA to 0.78× in its instrument.
  2. Cube roots when `3 | n/3` (Adleman–Manders–Miller or a Tonelli-style
     3-adic loop) so M's 0.2× applies at every p, 65537 included.
  3. Genus 4, where the asymptotic gap to rho is `p^{1/2}` instead of `p^{1/6}`.
