# AMENDMENTS (L4-lifting)

All dated 2026-11-13. The KILL/SURVIVE rule in PROTOCOL.md was not changed; results are reported against it.

1. code/mestre.py, before the final Q2 data: (a) an exact torsion check of the SVP combination and
   (b) a stricter independence tolerance (1e-3) for searched extra points were added after a first partial run
   showed rho ~ 200-550 cases whose lambda_G was of the size of the height error (rank deficiency masked by
   numerical error). The partial run was discarded and rerun from scratch (same seeds). Configurations whose
   Gram matrix is numerically singular are reported as "dropped" (spot check: smallest eigenvalue ~ -1e-3 vs
   others ~1, i.e. genuine extra relations among the small-coefficient Mestre points).
2. code/xedni_grow.py: arm P crashed once at (14 bits, k=5) after 17 trials (an assertion when R_3 coincided with
   another R_j so that P_j was left out of the fit). The assertion became status 'offcurve' (arm H unaffected:
   H never drops points). The cell was rerun with seed 3 (30/30 found). Planted trials with R_3 = O (2 trials,
   status R_is_O) are excluded from the control denominator.
3. code/lift_rho.py (supplement, not in PROTOCOL): rho_G of genuine 7-coefficient xedni lifts (k = 6 centred
   lifts at 128 and 256 bits) was added to anchor the Mestre measurements at crypto size. It is reported
   separately and does not enter the frozen Q2 rule.
4. Q2 extra small-coefficient sample (seed 12, 150 curves) was added because seed 11 gave only 8 (s=8) and
   3 (s=9) configurations; both seeds are reported separately and pooled.
