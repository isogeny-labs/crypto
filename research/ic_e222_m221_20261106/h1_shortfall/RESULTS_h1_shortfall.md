# Why H1 recovers a little less than 1/h of the trials (local, unvalidated)

**Status:** unvalidated (pre-gate). Python reconstruction of the repo's factor-base and pair-sum-table code; no cargo, no changes to `src/`. No replay certificate, validator report or matched-rho band. Not a leaderboard row.

**Question.** With the current defaults (pair-sum table, fb 32, extra 2), H1 (`--project-cofactor`) cut trials to 0.277 (Edwards, h = 4, CI 0.258 to 0.300) and 0.153 (Montgomery, h = 8, CI 0.143 to 0.164) of the unprojected count, against a registered prediction of 1/h (0.25, 0.125) (`../RESULTS_h1_defaults.md`, `../results/h1_defaults/rows.jsonl`). Why the excess?

## Model (read from `src/cryptanalysis/ec_index_calculus.rs`)

- A trial draws R = aG + bQ, uniform on <G>. It succeeds iff R is a point of the pair-sum table: the set of **distinct** affine points ±F_i ±F_j (i <= j; i = j gives ±2F_i; the identity is not stored). 2,048 points at most for 32 entries (4·C(32,2) + 2·32).
- Only table points that lie in <G> can match R. With H1 on every entry is [h]F in <G>, so the whole table is usable. With H1 off, F_i ± F_j lies in <G> only if the torsion components cancel (t_i = ∓t_j, t = the component of F in the order-h subgroup T).
- Per-trial success probability P = |table ∩ <G>| / n, E[trials per relation] = 1/P, and both arms collect fb + 2 = 34 relations (checked: every one of the 288 rows has relations − fb = 2, so no rank-deficiency retry).
- **Exact model:** E[on/off trials] = |T_off ∩ <G>| / |T_on|, computed per instance from the actual 32-entry factor bases (scan x = 1, 2, 3, … as `build_factor_base`; entries [h]F with the repo's dedupe by x as `build_factor_base_projected`).
- **Uniform-class model U:** classes t_i iid uniform in T, all points distinct. Usable fraction f_U = (4·C(m,2)/h + 2m·|T[2]|/h) / (4·C(m,2) + 2m), m = 32. The off-diagonal points are usable with probability 1/h, the diagonal ±2F_i with probability |T[2]|/h (2t = 0), which is larger.
- Assumptions: R uniform on <G> (it is, up to O), independent trials, geometric trial counts per relation.

## Checks that the reconstruction agrees with the repo

- Regenerated instances (`gen_instances.py … --seed e222m221-h1default-v1`) match `rows.jsonl` on (p, cofactor, r_bits) for all 72 names.
- Class-based membership (t_i ± t_j = O) and the explicit test [n]P = O give the same set of usable table points on all 72 instances; every point of the projected table passes [n]P = O (72 of 72).
- The reproduced median paired ratios are E 0.2774 and M 0.1525 (the note's 0.277 and 0.153).

## Results

Torsion groups (the order-h component T of E(F_p)): Edwards analogs T cyclic of order 4 (|T[2]| = 2) in all 24 instances; Montgomery analogs order 8, |T[2]| = 2 in 21 and 4 in 3 instances.

| | Edwards (h = 4) | Montgomery (h = 8) |
|---|---|---|
| observed median of paired ratios (the note's statistic) | 0.2774 | 0.1525 |
| observed pooled ratio (sum on / sum off trials) | 0.2473 | 0.1422 |
| 1/h (registered) | 0.2500 | 0.1250 |
| U (diagonal term) | 0.2578 | 0.1289 |
| exact model, median over instances | 0.2716 | 0.1362 |
| exact model, mean over instances | 0.2737 | 0.1424 |
| exact model with random-start factor bases (20 per instance), median | 0.2607 | 0.1333 |
| Monte Carlo of the median paired ratio under the exact model (2.5 / 50 / 97.5%) | 0.252 / 0.273 / 0.299 | 0.131 / 0.142 / 0.154 |
| observed median's quantile in that Monte Carlo | 0.65 | 0.96 |

By generator rung (median over instances; obs = median paired ratio):

| family | rung | obs | exact | U | random start | table points off / on (mean of 2,048) | obs quantile in MC |
|---|---|---|---|---|---|---|---|
| E | 12 | 0.313 | 0.287 | 0.258 | 0.306 | 1927 / 1635 | 0.82 |
| E | 16 | 0.258 | 0.272 | 0.258 | 0.258 | 2041 / 2018 | 0.19 |
| E | 20 | 0.254 | 0.253 | 0.258 | 0.257 | 2048 / 2048 | 0.54 |
| M | 12 | 0.172 | 0.157 | 0.129 | 0.155 | 1986 / 1618 | 0.83 |
| M | 16 | 0.151 | 0.128 | 0.129 | 0.133 | 2038 / 2017 | 0.94 |
| M | 20 | 0.137 | 0.131 | 0.129 | 0.129 | 2047 / 2046 | 0.72 |

Log residuals log(obs_pooled_i / pred_i), mean over the 24 instances of a family (bootstrap 95% CI) and sum of squares:

| predictor | Edwards mean (CI) | RSS | Montgomery mean (CI) | RSS |
|---|---|---|---|---|
| 1/h | +0.097 (+0.008, +0.186) | 1.410 | +0.191 (+0.137, +0.243) | 1.286 |
| U | +0.066 (−0.024, +0.156) | 1.290 | +0.153 (+0.098, +0.205) | 0.997 |
| exact (actual factor bases) | +0.010 (−0.064, +0.084) | 0.815 | +0.070 (+0.024, +0.118) | 0.448 |
| exact, random-start factor bases | +0.007 (−0.076, +0.084) | 0.944 | +0.082 (+0.035, +0.129) | 0.486 |

Decomposition of (observed median − 1/h), medians: Edwards 0.0274 = 0.0078 (diagonal ±2F_i term) + 0.0138 (actual classes of the 32 points and table collisions) + 0.0058 (statistic and sampling); Montgomery 0.0275 = 0.0039 + 0.0073 + 0.0163. The Monte Carlo puts the median of the paired ratio itself at 0.2731 (E) and 0.1417 (M) when the exact model is true, i.e. +0.0015 and +0.0055 above the exact model's median: the "median of a ratio" statistic sits slightly above the ratio of expectations.

## Verdict per hypothesis

- **H-a (torsion classes not uniform) — no evidence for non-uniformity; a related systematic term is real.** The class-order distribution over the 32 small-x points matches the uniform expectation: Edwards orders 1/2/4: 0.238/0.250/0.512 against 0.25/0.25/0.5; Montgomery orders 1/2/4/8: 0.122/0.158/0.270/0.451 against 0.125/0.156/0.281/0.438 (768 points per family; sampling sd about 0.016). What does shift the ratio is the diagonal: the 64 points ±2F_i are usable with probability |T[2]|/h instead of 1/h (Edwards 1/2, Montgomery 1/4 for most instances), which adds +0.0078 (E) and +0.0039 (M) to the expected ratio. With only 32 entries the realised class mix also varies by instance (sd of the exact prediction within a rung 0.004 to 0.026 for E, 0.009 to 0.018 for M).
- **H-b (collisions after the repo's dedupe) — entries: none; table points: yes at the smallest rung.** The projection never skipped an entry in any of the 48 cases (0 repeated x, 0 identity), so the factor base has 32 distinct entries in both arms. But the table's distinct points fall below the 2,048 nominal when the group is small: at rung 12 (n about 2^11 to 2^12) the projected table has 1,635 (E) and 1,618 (M) distinct points against 1,927 and 1,986 unprojected, a birthday effect that raises the ratio by about 0.03 there. It vanishes by rung 20 (2,048 and 2,046 distinct).
- **H-c (small-x scan order) — no effect found.** Factor bases starting at 20 random x per instance give exact predictions 0.2607 (E) and 0.1333 (M), not different from the scan-from-1 predictions beyond the instance-to-instance noise (mean log residual +0.007 against +0.010 for E, +0.082 against +0.070 for M).

## Which accounts for the gap

For **Edwards** the exact model (actual factor bases) is enough: mean log residual +0.010 (CI −0.064 to +0.084), RSS 0.815 against 1.410 for 1/h, and the observed median sits at the 65th percentile of the model's own sampling spread. The excess over 1/h is the diagonal term plus small-group table collisions plus class-composition noise, and the "median of ratios" statistic.

For **Montgomery** the exact model removes about 63% of the log gap (+0.191 to +0.070, RSS 1.286 to 0.448) but a residual of about +7% remains (CI +2.4% to +11.8%); the observed median is at the 96th percentile of the model's spread (94th at rung 16). It is not caused by a retry (relations − fb = 2 in all 288 rows). Observed total trials against the model's expectation Σ relations/P: Edwards off 1.043, on 1.020; Montgomery off 0.982, on 1.072. So the Montgomery on-arm is 7% above expectation and its off-arm 2% below. No hypothesis tested here explains that remainder; it is about two sampling standard errors by the Monte Carlo and may be noise. Cause not identified.

At the largest rung (n about 10^6, no table collisions) the observed medians are 0.254 (E) and 0.137 (M) against exact 0.253 and 0.131, i.e. within about 0.01 of 1/h plus the diagonal term.

## Reproduce

```
cd research/ic_e222_m221_20261106
python3 -I gen_instances.py --out h1_shortfall/instances --families E,M,W --bits 12,16,20 --per 8 --seed e222m221-h1default-v1
cd h1_shortfall
python3 -I h1_shortfall.py > run.log      # about 15 s; writes results_h1_shortfall.json
python3 -I residuals.py > residuals.log    # about 3 s; writes results_residuals.json
```
Inputs: `../results/h1_defaults/rows.jsonl` (sha256 e0c80167f800fcabee3073b529f327c598f3bcd1820cfe10d0d5ea9b23a60695). Random seeds are fixed strings inside the scripts.

## Limits

Python model of the repo's code, not the Rust itself; the agreement checks above (membership tests, the reproduced medians, 72 of 72 instances) are the only link. The model assumes R uniform on <G> and independent geometric trials; it does not model the Gaussian elimination or retries (none occurred). 24 instances per family, 8 per rung, 2 seeds: the Monte Carlo intervals are wide (E rung 16: 0.238 to 0.317). The Montgomery residual is borderline and unexplained. Toy sizes only (n up to about 2^21); at native E-222 / M-221 sizes the table would be a vanishing fraction of the group, so collisions disappear and, for a 32-entry factor base, the diagonal term (about 3% of the table) is the only systematic departure from 1/h that this model predicts; the value (about 0.258 for |T[2]| = 2 and h = 4, 0.129 for |T[2]| = 2 and h = 8) is an extrapolation, since the torsion structure of the native curves was not computed here.
