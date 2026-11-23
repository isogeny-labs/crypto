# RESULT: LEAD-STR-level-structure-torsor

## Outcome
- status: ran (single run of run.py, unmodified). exit code 0, wall 3 s (run.status: exit=0 wall=3). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "0.45 < r(N) < 1.0 for all prime N in 29..47, and (b) holds for every row. consistent_with_prediction if both; falsified if either fails; inconclusive if controls fail."
Measured:
- (a) r(N)=g/(N^2/24) over prime N 29..47: range [0.6278, 0.7605], inside (0.45, 1.0).
- (b) hw_vacuous_for_N_gt_p_half = true. 33 (p,N) rows, p in {31,53,79,101,127}. The Hasse-Weil slack ratio (HW bound / p) is 8.8 at its smallest (p=31,N=37) and 261 at its largest (p=127,N=149).
- (c) Exact #Y_1(N)(F_p) is tiny against the bound. Examples: p=127,N=139 gives 345 points vs bound 16739. p=101,N=113 gives 168 vs 9690.
- Controls: positive (genus table N=5..37) true; negative (no points outside Hasse, p=31 N=5,11) true; A/A repeat equal true. Controls passed.
- Verdict: consistent_with_prediction (run.py field and my reading agree).

## Reading
- The toy checks only what the protocol says: the genus formula, the inequality 2g*sqrt(p) > p, and toy point counts. It tests the size-obstruction arithmetic, not the diamond action, Hecke forced zeros or separating-function degree (PROTOCOL section 6).
- No discrepancy between the verdict field and the numbers.
- Caveat 1: the printed rows start at N=23 for p=31, so the N>p/2 filter was applied. Smaller primes in the Hasse interval (e.g. 17, 19 at p=31) are not shown, consistent with the (b) condition N>p/2.
- Caveat 2: the genus ratio r(N) is a closed-form fact (about 0.63-0.76 at these N, tending to 1), so the (a) part is close to a restatement of the formula. (b) is also nearly forced: 2g*sqrt(p) ~ N^2 sqrt(p)/12 > p for any N>p/2.
- Nothing surprising. The slack is large (at least about 9x), so the result is not marginal at toy size.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: for N ~ p the Hasse-Weil bound on X_1(N) exceeds p by at least 9x at every toy row, so point-count or function-field certificates cannot discriminate diamond translates below cost p. This confirms the "dead by size" note only for Hasse-Weil routes, not for other algorithms.

## Standing
[pcc:unverified], single local toy run, no independent replay.
