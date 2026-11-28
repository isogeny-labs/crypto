# Flat-presentation exponent study at m = 3 and m = 4, with wall time as a co-primary unit (frozen before data, 2026-11-15)

## Question

The exponent studies (`curve25519_exponent_study_20261108`, `_ext_20261110`) left the m = 3 and m = 4 verdicts
"undetermined": their chain presentation censored the series before the preregistered estimator could apply.
Wave-6 lead L3 showed that the flat presentation (one variable y with y^D = 1 per summand, base kind `subflat`)
is 10 to 360x cheaper in counted field operations at m = 3 and reaches D = 36 under 6 GB, and that at m = 2 the
counted unit and wall time disagree. This study measures the exponent s of T(E, m, L) in |V| = D with the flat
presentation at m = 3 and m = 4, in both units, under the parent's frozen estimator and lever rules.

## Instrument

`curve25519_family` sha256 5abc70ba47b1192df71a5e93d697f095e59a0e5f0132cf47db1d0d34d34e3746 (the wave-6 build;
reproduces the frozen dc2c30a4 exactly on control cells, validated in `curve25519_wave6_runs_20261115`), command
`f4sub 20 subflat D 0 mont M sym 6 SEED CAP` on the k = 20 member (p = 1,048,573, A = 926), SEED = 20261108 +
1000·M + D (the parent's rule, so every cell draws the parent's targets), N = 6 targets, CAP = 7,200 s per target,
`ulimit -v 12000000`, RAYON_NUM_THREADS=1, one process at a time on an otherwise idle host (load averages recorded
per cell; any cell whose 1-minute load before or after exceeds 1.5 is flagged and its wall time excluded from the
wall fit). Runner `run_cells.py` (resumable; a chain stops at its first censored cell; censor = timeout, memory
exit, or pairs above the degree cap 400).

Units. T = mean `field_ops` over the 6 targets (counted F_p multiplications of the row reduction; it leaves out
the symbolic preprocessing). W = mean per-target wall time `ms` over the 6 targets. Both are fitted; the lever
rules (below) are stated for T as in the parent, and the wall-time slope is reported beside it as the co-primary
unit; a lever claim would need both.

## Cells (sequential, in this order)

1. m = 4, flat: D = 3, 7, 9, 12, 14, 18, 19, 21, 27, 28, 36, 38, 42 (chain rule).
2. m = 3, flat: D = 9, 12, 14, 18, 19, 21, 27, 28, 36, 38, 42, 54, 57, 63, 73, 76, 84, 108, 114, 126 (chain rule;
   the L3 cells D ≤ 36 are rerun inside this study so that all wall times come from the same idle-host regime).
3. Controls on the faithful k = 24 member (`f4sub p=16777141,A=7842 subflat D 0 mont 3 sym 6 SEED CAP`): D = 10, 12,
   15, 20, 30 (the divisors ≥ 9 of 60), chain rule.

## Estimator (the parent's section 5, unchanged)

For each (m, unit) with at least 4 complete cells with D ≥ 9 spanning at least 4x in D: OLS of ln(unit) on ln D,
slope s with a two-sided 95 % t interval (n − 2 df). Fewer cells or smaller span: no fit, "undetermined", with
two-point local slopes reported. Lever rule on the T slope: at m = 3 a lever needs s < 0 with the upper bound
below 0; at m = 4, s < 0.5 with the upper bound below 0.5. The balanced total exponent 2/(m + 1 − s) is reported as
derived, not measured.

## Predictions (written before data)

- P1. The m = 3 estimator applies (the D ≥ 9 cells reach at least 36) and gives s_T in [4.0, 4.8] with lower bound
  above 3 (L3's non-preregistered reading was 4.40 [4.10, 4.71]). Verdict: no lever at m = 3.
- P2. The m = 3 wall-time slope s_W exceeds s_T by at least 0.5 (per-operation time grew with D in L3), with
  s_W in [4.5, 7].
- P3. The m = 3 chain reaches D = 57 and censors at or before D = 84 (memory or the 7,200 s cap).
- P4. At m = 4 the cells D = 3, 7, 9 complete; D = 12 completes under 12 GB (the chain cell did not); the chain
  censors at or before D = 28, so the m = 4 estimator does not apply (the D ≥ 9 cells span less than 4x) and the
  verdict stays "undetermined"; every two-point local slope among complete m = 4 cells with D ≥ 9 exceeds 2.
- P5. The k = 24 controls agree with the k = 20 cells at the shared D = 12 to four digits in T, and their T values
  lie within a factor 1.5 of the m = 3 power law fitted on the k = 20 cells at every D.
- P6. The binary's sha256 matches the value above at launch and in every cell record.

A failed prediction stays failed. Every number is [pcc:unverified] until a validator report says otherwise. No
Conductor task, EKS run, S3 checkpoint or benchrec record is reachable from this runtime (disclosed as in every
study of this series).
