# Pre-check 2 for lead #1: Diem line index calculus on prime-order plane quartics, fixed instrument

Status: DRAFT (not frozen). Every number this study produces is `[pcc:unverified]`.

Parent: `research/quartic_diem_precheck_20261120/` (pre-check 1, STOP under K2 with the
cause traced to the instrument), and `research/hec_breakthrough_search_20261120/REPORT.md`,
rank #1. The owner approved fixing the instrument and re-running the pre-check under a
new frozen protocol (2026-11-21).

## 1. Question

With the two instrument defects of pre-check 1 fixed, how do (a) the counted
linear-algebra work and (b) the total counted cost of Diem's double-large-prime line index
calculus scale with `p` on generic smooth plane quartics over `F_p` whose `Pic^0` has prime
order? And, on the same curves and solver seeds, how much does each fix change the counted
cost relative to the pre-check-1 instrument?

This is a scaling pre-check. It makes no comparison with rho and cannot produce a
below-rho claim.

## 2. Instrument

SECTION FILLED AT FREEZE.

## 3. Metrics (counted units only; wall time recorded, never used)

Unchanged from pre-check 1, per run of an arm (one curve, one solver seed):

- LA-count (primary for K1): `la_mul`. Sensitivity `la_mul + la_add`.
- Total (primary for K2): FPM mid = `cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul`.
  Sensitivity FPM low (`I = 1, kappa = 1`), FPM high (`I = 30, kappa = 9`),
  `GAE_w = group_ops + w (rel_mul + la_mul)`, `w in {0, 0.1, 1}`.
- Descriptors: `rel_mul`, `rel_decomp_mul`, `rel_enum_mul`, `decomp_trials`, `attempts`,
  `la_skipped`, `decomp_fallbacks`, `fb_cols`, core size, LA share.

Only runs with `certified = true` and `ok = true` enter a fit. A prime with fewer than 2
certified curves in the PRIMARY arm is dropped from the fit and disclosed.

## 4. Fit and intervals

Unchanged from pre-check 1, applied to the PRIMARY arm: curve-level geometric mean over
seeds; OLS slope of `ln metric` on `ln p`; primary interval = 95% percentile of 4000
cluster-bootstrap replicates resampling curves within each prime (numpy
`default_rng(20261121)`); secondary = t-interval of OLS on the per-prime means; local slopes.
The fit uses every prime that ran (1031 ... 32771, and 65537 if it ran, section 5). The
six-prime fit (1031 ... 32771, the pre-check-1 range) is reported as a sensitivity reading.

SECTIONS 5-9 FILLED AT FREEZE.
