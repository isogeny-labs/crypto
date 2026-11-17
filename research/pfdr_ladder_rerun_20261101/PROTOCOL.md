# EXP-PFDR-2d36f9-R2: rerun of the break-even growth ladder with two pre-registered design corrections

Date: 2026-11-01. Written and committed before any R2 data (selftest only was run to check the toolchain). Branch `claude/pfdr-rerun-20261101`.

## Origin and why a rerun
`EXP-PFDR-2d36f9` (crypto-autoresearcher, spec `coordination/design/EXP-PFDR-2d36f9-approval-20261031/specification.approved.yaml`, code commit
41020cfad0, runs RUN-PFDR-f86a2a ... 73276d, report `experiments/EXP-PFDR-2d36f9/reports/run-20261031T2130Z/`) read **O-INVALID**: the three
theta = 0.30 lines missed the powered floor (M = 2333 < 2500 at 16 bits) and the z_cal calibration gate failed (sd 0.660, band [0.70, 1.30]).
The three valid theta = 0.35 lines were SUB-BE (delta_hat -0.0098, -0.0044, +0.0054; upper95 <= 0.0136 against delta* = 1/3) and the planted
slopes were recovered, so the instrument sees growth. The question the lead still needs answered is whether the relation excess kappa of
structured (window / rank-d label-GAP) bases grows with N at or above the break-even rate delta* = 1/3 over 16-32 bits, with valid controls.

## Corrections (the only changes; patch in `implementation.patch`, original run.py kept as `implementation/run.py.original`)
1. **Power margin.** The sizing rule was n = clamp(ceil(1.1 x 2500 / mean_null), 10, 4000) with mean_null from 8 pilot curves; the pilot underestimated
   the census null at 16 bits (1965 curves gave M = 2333). R2 uses a safety factor 1.5 instead of 1.1; everything else in the rule (8 pilot curves,
   20 null replicates, clamp, target 2500) is unchanged.
2. **Calibration gate (C6).** The 30 structured cells share their null replicates across d (verified on the original raw data: for all 4857
   (bits, theta, curve) keys the null list is identical for d = 1, 2, 3), so they carry 10 independent z_cal values, not 30. The original gate
   assumed 30 independent values (sd of sd about 0.13). R2 computes z_cal once per independent null set (d = 1 cells; 5 rungs x 2 thetas = 10),
   uses the sample sd, and gates with the exact 95% two-sided acceptance region for n = 10 iid N(0,1): |mean| <= 1.96/sqrt(10) = 0.62 and
   sd in [sqrt(chi2_{.025,9}/9), sqrt(chi2_{.975,9}/9)] = [0.548, 1.454]. The original-rule numbers are also recorded (reported only). On the
   original data this rule gives mean -0.150, sd 0.695 (would pass), which is why this is stated as a design-flaw correction and fixed here
   **before** any R2 data; the new seeds make the R2 data independent of the original.
3. Fresh seed blocks: pilot 7,000,000; main 7,100,000; replication 7,200,000; selftest 7,900,000 (original 6,x). Experiment id `EXP-PFDR-2d36f9-R2`.

Provenance: `implementation/{check.py, paircount.c, vendor/curve.py}` are byte-identical to the original (sha256 in `PROVENANCE.sha256`;
curve.py equals the hash pinned in the original spec, fcca2beb...). `run.py.original` is the original driver.

## Unchanged from the original frozen specification
Cells, lines, kernels, counting convention (sum over s != O of C(m_s, 2) on symmetric bases of b points, 20 null replicates), bits 16/20/24/28/32,
thetas 0.30/0.35, d = 1, 2, 3, gamma plants {0, 1/6, 1/3, 1/2}, Z/N interval control, A/A lines, the five-rung OLS slope of log2 kappa on rung mean
log2 N with a curve-cluster bootstrap (2000), break-even delta* = 1/3, the line classes (SUB-BE, SUB-BE-GROWING, ALIVE, invalid), the blocking controls
C3 (gamma-slope plants), C4 (Z/N), C5 (A/A, at least 5 of 6 cover zero and none ALIVE), the replication block for any ALIVE line, and the readings
O-INVALID / O-SUB-BE / O-ALIVE / O-MIXED / O-PENDING-REPLICATION. A reading of O-SUB-BE requires: selftest passes, C3-C6 pass, at least 5 of 6
structured lines valid, every valid line SUB-BE or SUB-BE-GROWING.

## Claims
- C1 (confirmatory): the structured lines (window and rank-d label-GAP at theta 0.30 and 0.35) have a five-rung slope below the break-even rate
  delta* = 1/3 (SUB-BE or SUB-BE-GROWING) with all blocking controls valid. Reading O-SUB-BE supports "no structural relation source of this
  family on F_p in 16-32 bits".
- C2: any line classed ALIVE is replicated (block rep) before any further statement; BREAK-EVEN-MET only if it survives.
- Class: negative-or-positive structural-lead measurement; it does not price an end-to-end index calculus and makes no claim about rho.

## Execution
`scripts/run_all.sh` (local, WORKERS = 4): build, selftest, pilot, census main, analyze; replication and final analysis only if an ALIVE line exists.
Counts are deterministic given the seeds; no wall-clock claim. The original census took about 5 minutes on this class of machine.
Outputs under `results/` (raw jobs gzip'd, design.json, analysis.json); large raw files follow the artifact-store policy.

## Decision rule (frozen)
Report analysis.json's reading verbatim. If any blocking control fails again, the reading is O-INVALID and is reported as such (no further
redesign in this study). The lead disposition is the sibling repository's decision: this study supplies the numbers, and states the obstruction
(max upper95(delta) against 1/3) when the reading is O-SUB-BE.
