# L4-SCALE: the Table B incumbent `sib8-p256` at 28 and 32 bits

Every number below is `[pcc:unverified]`. Frozen protocol: [`PROTOCOL.md`](PROTOCOL.md) (commit `6ff945c64e`, before any measured row). No code change; binary `target/release/ecbench` (sha256 prefix `b4fcf0ecdb3d0a0f`), L0, shared 4-core container with four other agents (`--cpus none --allow-busy`). Counted costs (GAE, bitop/v1) are exact; wall times are contaminated by the other agents and are not used.

## What ran
- Curves: engine generator `prime_orbit_shape`, shape `generic`, prime order (the snc-panel k28/k32 curves have cofactor 8, so not used). 28 bits: `icv1-fp28-tm733-8079a0d1` (log2 r 27.89), `icv1-fp28-tm16321-89b0bb18` (27.74). 32 bits: `icv1-fp32-t65541-3b3f48b1` (31.27), `icv1-fp32-tm95187-43fe2b49` (31.99). None is in `docs/curves/registry.json`; ecdb was not queried (no token).
- Size: the smoke timing (3 s for 32 executions) allowed more than the planned 24 targets and 2 rounds: 96 public targets per curve, 4 rounds + 1 warm-up, 3840 executions per size. Full wall time was longer than 90 minutes only through contention and `--replay-all`.
- Arms: `rho-neg`, `tw2l`, `sib8-p256`, `sib8-p256-aa` (A/A copy).
- Audit: `ecbench verify --replay-all`: 3840 records, 3840 verified, 3072 replays reproduced, 0 problems, at both sizes (`audit-P28.json`, `audit-P32.json`).
- Pooled intervals are 95% linearised-t over 768 pairs on 192 workloads (2 curves); the w = 0.1 and w = 1 values are band point values (no interval).

## Results (counted GAE; ratio = second arm / first arm)
| size | pair | w = 0 [95%] | w = 0.1 | w = 1 | bitop/v1 [95%] |
|---|---|---|---|---|---|
| 28 | sib8-p256 / rho-neg | 0.892 [0.858, 0.929] | 1.054 | 1.792 | 1.053 [1.012, 1.096] |
| 28 | tw2l / rho-neg | 1.861 [1.787, 1.938] | 1.855 | 1.827 | 1.816 [1.744, 1.890] |
| 28 | sib8-p256 / tw2l | 0.479 [0.467, 0.493] | 0.568 | 0.981 | 0.580 [0.564, 0.596] |
| 28 | sib8-p256-aa / sib8-p256 | 1.000 | 1.000 | 1.000 | 1.000 |
| 32 | sib8-p256 / rho-neg | 0.916 [0.877, 0.955] | 1.089 | 1.870 | 1.090 [1.045, 1.137] |
| 32 | tw2l / rho-neg | 1.922 [1.836, 2.012] | 1.920 | 1.911 | 1.876 [1.792, 1.963] |
| 32 | sib8-p256 / tw2l | 0.476 [0.462, 0.491] | 0.567 | 0.979 | 0.581 [0.564, 0.598] |
| 32 | sib8-p256-aa / sib8-p256 | 1.000 | 1.000 | 1.000 | 1.000 |

Per curve, sib8-p256 / rho-neg at w = 0: 0.871 [0.823, 0.922], 0.914 [0.864, 0.967] (28 bits); 0.902 [0.846, 0.962], 0.929 [0.879, 0.982] (32 bits).

`band`: sib8-p256 is below tw2l at every weight at both sizes; sib8-p256 against rho-neg is unresolved inside the band (below rho at w = 0, above it at w = 0.1 and 1). tw2l is above rho-neg at every weight.

Distance remaining to rho-neg: sib8-p256 is 0.89x (28 bits) and 0.92x (32 bits) of rho-neg at w = 0, 1.05x and 1.09x at w = 0.1, 1.8x and 1.9x at w = 1, and 1.05x and 1.09x in bitop/v1. It is below rho-neg only in the w = 0 reading [pcc:unverified].

## Trend (derived statistic, not a preregistered scaling claim)
`trend.py` fits ln(sib8-p256 / rho-neg) on log2 r by weighted least squares over per-curve points (10 points: Table B round 3 and ablation re-run, 20/21.65/23.38 bits, plus the four curves here; SE from the interval on the log scale). Output `trend.json`.
- Table B only (20–24, n = 6): slope -0.052 per bit [-0.081, -0.023]; predicts 0.71 at 28 and 0.58 at 32 bits.
- 28/32 points only (n = 4): slope +0.008 per bit [-0.007, +0.023]. `ecbench trend` (`ecbench-trend.json`, free exponent of r, 28 and 32 bit comparisons only) agrees: beta = 0.011 [-0.010, 0.033] per ln r, chi2/dof 0.79.
- All ten points: slope -0.010 per bit [-0.016, -0.005] (chi2/dof 16.6/8: a straight line fits poorly).
The fall seen on Table B does not continue: from 24 to 32 bits the ratio sits at 0.87–0.93 and is flat to slightly rising. It is consistent with the autolab stage 3 range 0.78–0.91 at 34–40 bits only at the upper end.

## Predictions against outcomes
| id | prediction | outcome |
|---|---|---|
| S-a | all verify; A/A exactly 1 | held (3840/3840 at both sizes; A/A 1.000 in every unit) |
| S-b | sib/rho w = 0: 0.85 at 28 (0.75–0.95), 0.82 at 32 (0.72–0.92); upper 95% < 1 | 0.892 (in range, point 0.04 high), 0.916 (inside range at its edge, point 0.10 high); upper ends 0.929 and 0.955 below 1: held on range, missed on the 32-bit point |
| S-c | tw2l / rho-neg in [1.7, 2.3] | 1.861, 1.922: held |
| S-d | sib / tw2l, w = 0, 0.38–0.50 | 0.479, 0.476: held |
| S-e | sib / rho at w = 1 in [1.4, 1.8] | 1.792 held; 1.870 missed (above range) |
| S-f | bitop sib / rho in [0.8, 1.2] | 1.053, 1.090: held |
| S-g | slope over 20–32 in [-0.03, -0.005] per bit, flatter than 20–24, no extrapolation below 0.7 at 32 | all-points slope -0.010: held; the stronger finding is that 28–32 alone is flat or rising, which the prediction did not state |

Decision (from the protocol) [pcc:unverified]: improvement over rho-neg at w = 0 at both sizes (upper ends below 1, all runs verified): yes, for the w = 0 reading only. Falling trend (slope interval wholly below 0) holds over the pooled ten points but the 28/32-bit subset does not support further decline; no kill condition fired.

## Files
`specs/P28.json`, `specs/P32.json` (+ smoke specs), `run.sh`, `gen_specs.py`, `sessions/`, `audit-P*.json`, `compare-*`, `bitop-*`, `band-P*`, `summary.json`, `trend.py`, `trend.json`, `ecbench-trend.json`, `plot.py`, `scale_l4.{svg,png,pdf}`. Bench record `benchmarks/records/20261119T1721Z-ic-scale-l4-28-32.json`. No leaderboard row (validation gate not run).

**Raw data.** The two `sessions/P*/records.jsonl` files (about 37 MB each) are not in Git. They are packed in `sessions-raw.tgz` (5.6 MB, sha256 `43d96ca6caccb2eb…`), which is committed here. That tarball is not in S3: this container has no store credentials, and the bench record lists it with `uploaded: false`. To restore the sessions, run `tar xzf sessions-raw.tgz` in this directory.
