# RESULT: LEAD-ICP-digit-presentation-solving-degree

## Outcome
- status: ran, exit code 0, wall 0 s by the harness clock (run.py reports 0.04 s). run.stderr empty.
- One run only; no retries, no edits to PROTOCOL.md or run.py.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): "D <= A (A = same max of |rank(null1) - rank(null2)|, two independent null draws; the A/A noise floor). Consistent with prediction. Falsified if D > A."
- Measured (p=13, m=2, B=3, seed 20261110, 4 curves, d=3..5):
  - rank real = null1 = null2 = [6, 43, 175] on all four curves (4,10), (2,9), (11,11), (7,11).
  - D = 0, A = 0 (aa_spread), so D <= A holds (equality).
  - planted positive control ranks [6, 42, 168]; positive_control_detect = 7 (> 0).
- Verdict: consistent_with_prediction. Controls: null OK, positive OK (rank changes by 0/1/7 vs null), A/A OK (0, with the caveat below).

## Reading
- run.py's verdict field agrees with the numbers.
- The A/A floor is 0 and every arm gives identical ranks, so the test is an exact-equality test with no noise estimate. Equality at every cell, including across null draws, means rank at d<=5 is fixed by the monomial support and the range polynomials, not by the coefficients. The toy therefore sees only a support-determined pre-fall region.
- The positive control only differs at d=4 and d=5 (by 1 and 7), so detection power is modest; a small genuine curve effect could be below it, though any deficit of the planted kind would have shown.
- The 0.04 s runtime is far below the protocol's "few seconds to 2 minutes" cost estimate. I did not inspect or rerun run.py (forbidden); this is worth a sanity check by someone with permission, e.g. that the matrices are the intended size.
- The toy does not test m=3, CDCL(T), staircase representations or the asymptotic solving degree, as PROTOCOL.md section 6 states.

## Bearing on the sub-sqrt(p) question
none: toy-size, pre-fall ranks only, consistent with the recorded curve-independence; no contact with rho or with any predicted d_ff.

## Standing
[pcc:unverified], single local toy run, no independent replay.
