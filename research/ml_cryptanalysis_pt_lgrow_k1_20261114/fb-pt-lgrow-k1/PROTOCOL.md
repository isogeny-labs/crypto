# Protocol (frozen before any measurement): pair-table factor base growing with n vs fixed l = 7, K1 n = 17, 19, 23, 29

Written 2026-11-14 at the owner's "1": let the pair-table factor base grow with n. Frozen by `data/PROTOCOL.frozen.sha256`. Later changes are appended under "Changes after freeze".

## Question
`../fb-pt-l7-k1-n23` (VAL-20261114-1227fa) found that a fixed factor base shifts the gap to rho but, at realistic prices, does not bend its growth. This holds for F4 at l = 3 and for the pair-table at l = 7, both at about +0.7 bits per bit of n.

- Primary question: does choosing the pair-table dimension per n, l*(n), lower the slope of the gap in n compared with fixed l = 7, on the same targets?
- Secondary: at n = 23 and 29, is l* cheaper than l = 7 across the band?

## Prior information (disclosed): sizing in `sizing/`
- Local sizing, 2026-11-14, same binary, L0: 2 targets per n, seeds 99990000 + n (never used below), arms pt-l5 to pt-l11 plus strong rho. The sessions and specs are committed in `sizing/`.
- Best l by total S, the same at w = 0, 0.1 and 1:

  | n | l* | gap at l* (w = 0.1) | gap at l = 7 |
  |---|---|---|---|
  | 17 | 7 | 2.83 | 2.83 |
  | 19 | 7 | 3.83 | 3.83 |
  | 23 | 9 | 5.61 | 6.97 |
  | 29 | 10 | 9.75 | 11.14 |

- The rule is frozen as L_STAR = {17: 7, 19: 7, 23: 9, 29: 10}.
- On the sizing targets, D = slope_l7 - slope_l* is about +0.13 and R_n23, R_n29 about +1.3 to 1.6 bits. Those numbers are optimistic, because l* was picked on them.
- Other observations:
  - At n = 29, pt-l5 exhausted on 1 of 2 sizing targets.
  - K1 n = 21, 25, 31, 37 and 41 have no prime-order subgroup in ecbench's constructor.
  - At n = 43, pt-l9 timed out at 600 s, so n = 43 is out of reach.

Predictions made now:
- D > 0 at w = 0, 0.1 and 1 (POSITIVE), with a point estimate of about +0.05 to +0.13 bits per bit of n. Growing l bends the slope modestly.
- R_n23 and R_n29 are POSITIVE, around 1 to 1.5 bits.
- The gap still grows, by about +0.55 bits per bit of n under l*. This is not a path to beating rho at these sizes.

## Instrument
- ecbench static binary sha256 a0be1b2a3a02f5385c9b9654622b01923f3f54a52904bd157861e0cbe8796d29, checked in the pod.
- Route: a RunPod CPU pod (cpu5c or cpu3c, 8 vCPU) running the identical runner `../fb-oracle-compare-k1-eks/runpod/run2.sh`:
  - jobs run in sequence;
  - each is followed by `ecbench verify --replay-all`;
  - results are uploaded after each job.
- Records are L0; only counted cost is scored.
- Arms per job:
  - `rho-strong` (reference);
  - `ic-pt-l7` (baseline): `koblitz-standard-subspace:dimension=7` with `pdp3-koblitz:m=3,engine=rootfind-negfold`, max_trials 1e7, identical to the earlier cells;
  - at n = 23 and 29 only, `ic-pt-l{l*}` (candidate) with the same params at dimension l*(n). At n = 17 and 19, l* = 7, so the baseline arm serves as l*.
- Layout:
  - K1 n in {17, 19, 23, 29}, two jobs of 4 targets per n: 8 targets per n.
  - Job j uses target_seed = measurement seed = 20262400 + 10 n + j. Checked before the freeze: no seed in 20262400 to 20262999 appears anywhere in this repository's experiments.
  - 5 rounds plus 1 warm-up, order alternate, timeout 1800 s.
  - Specs are written by `scripts/make_specs.py` and all 8 pass `ecbench plan`.

## Statistic and decision rule
- S per record: the validated section 5a band function and bitop/v1, copied verbatim into `scripts/score.py`. A smoke test on the sizing sessions reproduced the numbers above.
- gap(arm, n) = log2(sum S_arm / sum S_rho) over the 8 targets of n.
- slope = OLS of the gap over n in {17, 19, 23, 29}.
- **Primary:** D = slope_l7 - slope_l*.
  - POSITIVE if the 95% bootstrap CI lies above 0 at w = 0, 0.1 and 1; NEGATIVE if below 0 at all three; otherwise NOT_SEPARATED.
  - Bootstrap: 10,000 resamples, each n's 8 targets resampled independently, random.Random(0).
- **Secondary:** R_n = log2(S_l7 / S_l*) at n = 23 and 29, with the same verdict rule.
- Solvability: any non-verified measured record of ic-pt-l7 or of the l* arm makes the whole result DOES_NOT_SOLVE.
- Success criterion (AGENTS.md):
  - A POSITIVE R_n is a measured improvement over the pair-table l = 7 baseline at that n. At w >= 0.1 that baseline is the best validated IC arm. At w = 0 the validated ic-ridge comparison is cross-target only at n = 23, and absent at n = 29.
  - The gap to rho is reported as the distance remaining.
- An independent validator follows before any README row, leaderboard or atlas use.

## Changes after freeze

### 2026-11-14 06:18 UTC: launch (no data yet)
- Pod sbu5nquuh1vxmb: cpu5c, 8 vCPU, $0.28/h. JOBS = n17-j0 ... n29-j1 (8 jobs), run in sequence.
- All 10 presigned inputs were checked from the session (HTTP 206).
- The watcher deletes the pod as soon as exits.txt shows ALL_DONE.

### 2026-11-14 06:25 UTC: data in
- All 8 jobs finished by 06:21 UTC.
  - Exit 0 on every job.
  - The binary check passed.
  - In-pod `verify --replay-all` gave ok=true for all 8 sessions.
- The watcher deleted the pod at once (DELETE returned 204). It ran about 3 minutes.
- `results.json` comes from the frozen scorer, unchanged.

### 2026-11-14 08:00 UTC: corrections from validation (VAL-20261114-d9190c)
- F1: the sizing claim "l* is the same at w = 0, 0.1 and 1" is false at n = 19, w = 1. There l = 8 beats l = 7 by 0.0025 bits. The effect on D is about 0.002 or less.
- F2: at n = 29, pt-l5 exhausted on both sizing targets, not "1 of 2".
- F3, not disclosed before: the x-axis is field degree, not problem size.
  - log2 r is 16.0, 18.0, 22.0 and 15.4 at n = 17, 19, 23 and 29. K1 n = 29 has cofactor 12646.
  - At n = 29, rho is setup-dominated: setup is 80% of its cost at w = 0.
  - Slopes "in n" therefore describe these four curves only.
- F4: with l* = 7 at n = 17 and 19, D = (R_n23 + 7 R_n29) / 84, and 83 to 88% of D comes from n = 29.
  - On 23 -> 29 the l* gap grows faster than the l = 7 gap: segment slope difference -0.071 [-0.109, -0.041] at w = 0, -0.057 [-0.095, -0.027] at w = 0.1.
  - The POSITIVE D therefore reflects a roughly 1.2-bit offset that appears between n = 19 and 23, not a lower growth rate.
- Verdicts and numbers are unchanged; only their interpretation is corrected.
