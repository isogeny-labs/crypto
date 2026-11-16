# PARAMS: N4 v2 grid parameters, set by the pilot rules of PROTOCOL.md

Committed before the first grid unit. Pilot records: `pilot/raw/` (seeds 101, 102; not grid data),
`pilot/calib.jsonl`. Machine-readable copy: `params.sh` (sourced by `run_grid.sh`).

## Pilot facts that set the parameters

* Host load during the pilot: load average 120 to 330 on 14 cores (other jobs); each `nice -n 10`
  process received about 5-10% of a core (`/usr/bin/time`: user / real between 0.05 and 0.12).
  The grid budget is therefore planned in **CPU seconds**: at 4 processes and about 0.1 core each,
  20 h wall is about 29 000 CPU-s; the plan below is about 16 000 CPU-s (estimated).
* Measured user CPU per job (pilot): k = 18: B0 9.4-10.6 s, S family (S, L0, L1, L) 19.6-20.3 s,
  D family (D, DL1, DL) 2.8-5.2 s, one plain + one negation-map rho run 17-39 s; k = 20: S family
  105-109 s, D family 12-19 s. Extrapolated (S family x2.3 per k, D x2.2, B0 x2.3, rho x2.83):
  k = 22 S family about 560 s, D family about 75 s, rho pair about 1800 s; k = 21 B0 about 130 s.
* **`c_D` = 2.0.** Mean D cold M-eq (w = 0, robust calibration ratios below) over the pilot units
  (seeds 101, 102): k = 16: c 1.5 -> 3.87e7 (one failed attempt each, charged), 2.0 -> 3.34e7,
  2.5 -> 3.97e7, 3.0 -> 5.07e7; k = 18: 1.5 -> 2.09e8, 2.0 -> 1.81e8, 2.5 -> 2.15e8, 3.0 -> 2.70e8.
  All pilot D units verified.
* **`gamma` = 4** (L1 verified at the k = 18 pilot). Block width `b = 4` (protocol).
* **Measurement deviation found in the pilot (recorded here and in the README):** N4's `ratios()`
  (min of 5 tight loops of 2e6 operations) is unusable on this host: at the same `q` it returned
  `r_inv` from 3 to 325 and `r_lmul` from 0.2 to 38 (`pilot/raw/*`, record `ratios`), because the
  process is preempted and migrates between performance and efficiency cores. A robust variant
  `v2 calib` (600 interleaved triples of short blocks, ratio formed within each triple, median)
  returned `r_inv` 15.00 / 15.01 (q = 4051), 22.97 / 22.94 (262111), 25.88 / 25.88 (2097091) and
  `r_lmul` 1.55 / 1.55, 2.19 / 2.19, 2.00 / 2.04 on repeated calls. **The grid prices M-eq with
  the median of three `v2 calib` records per size** (the per-unit `ratios` records are still written
  and kept). Sensitivity: the summary is also produced with N4's Xeon constants
  (`r_inv = 15`, `r_lmul = 1.1`, `N4V2_FIXED=15,1.1`).

## Grid

* Sizes k = 8..22: q = 223, 499, 991, 2011, 4051, 8191, 16363, 32719, 65479, 131071, 262111,
  524287, 1048507, 2097091, 4194271 (`v2 qlist`). Seeds 1..5 for k <= 16, 1..3 for k >= 17.
* IC arms: S family and D family on every unit; B0 for k <= 21 (k = 21 B0 units in a one-at-a-time
  memory lane, scheduled last).
* rho runs per instance (each = one plain and one negation-map run on the same target and seed):
  k <= 12: 30; 13, 14: 24; 15, 16: 16; 17: 8; 18: 6; 19: 4; 20: 3; 21: 2; **22: 2 runs on seed 1
  only** (protocol rule: k = 22 rho at full count, 3 instances x 2 runs x about 1800 CPU-s, would
  alone take about 10 800 CPU-s and push the estimate past the 20 h limit).
* Replay controls (arm c): seed 1 re-run for S and D families at every k <= 18 and at k = 20; B0
  replays at k <= 18.
* Instance control: N4's `n4 check q 1` at every size. Calibration: `v2 calib q c`, c = 1, 2, 3.
* A pre-grid replay test at q = 8191 (seed 1, scratch, not grid data) gave identical S/L0/L1/L/D/
  DL1/DL records and differing B0 records (`linear_algebra.lmul` 10274418 vs 10274420).
