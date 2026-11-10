# RESULT: LEAD-ICB-orbit-linear-algebra

## Outcome
- status: ran (single run, no retry). exit code 0, wall 2 s (run.status: exit=0 wall=2). run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): median over 5 seeds of ops_unfolded/ops_folded at m=36 in [151, 281]
  (0.7x-1.3x of 216 = |Aut|^3). Outside with valid controls: falsified; any control failing: inconclusive.
- Measured (main, m=36): median ratio 18.60 (per-seed 22.75, 17.22, 16.31, 18.60, 22.60). Ratios at
  m=6/12/24/36: 16.6 / 15.1 / 16.7 / 18.6, i.e. roughly flat, nowhere near 216.
- A/A (seeds +1000) m=36 median 16.52; relative difference 0.1118 (> 0.10 limit): A/A control FAILED.
- Positive control (both solves recover all logs on every main seed): FAILED. Folded solve recovered on
  all sizes, but the unfolded solve failed on some seeds for m=12, 24, 36 (all_ok_unf false); only m=6 was clean.
- Null (wrong eigenvalue mu=12345): folded solve failed to recover at all sizes: PASSED.
- controls_ok = {positive_recovery: false, null_fails: true, aa_within_10pct: false}.
- Verdict: inconclusive (run.py "verdict" field), because two controls failed. The number 18.6 is
  far outside [151, 281], but the protocol does not allow calling it falsified with failed controls.

## Reading
- The toy does test the stated arithmetic claim (dense elimination cost folds by n^3), but the measured
  ratio is about 12x smaller than 216. Folded cost (50383 at m=36) is close to m^3 = 46656, as expected.
  The unfolded cost (about 9.4e5) is far below the dense n^3 ~ 1e7 figure.
- Likely cause (not tested, my inference only): weight-4 sparse random rows make unfolded elimination
  cheap through the counter (n-c per row update, with many zero entries skipped/early exits), and the
  unfolded system is frequently rank-deficient or ill-conditioned with n+8 weight-4 rows
  (recovery fails), so the unfolded run is not a clean dense n^3 reference. The folded matrix fills in
  quickly (each folded row touches up to 4 of m columns), so it is effectively dense.
- So the preregistered cubic-gain model does not match this construction, but the harness's own
  validity checks (unfolded recovery, A/A noise 11% vs 10% limit) failed, so no clean falsification.
- Discrepancy check: run.py verdict "inconclusive" agrees with the controls_ok numbers.

## Bearing on the sub-sqrt(p) question
- none: a dense-elimination mulmod count on synthetic relations; it does not touch sparse/Wiedemann
  linear algebra or the 10^6+ column revisit condition, and the exhausted_at_scale disposition stands.

## Standing
[pcc:unverified], single local toy run, no independent replay.
