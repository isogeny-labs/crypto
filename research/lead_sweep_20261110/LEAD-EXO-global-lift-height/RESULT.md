# RESULT: LEAD-EXO-global-lift-height

## Outcome
- status: ran (single run, no retries). exit code 0, wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- RESULT.json is a single JSON object; run.py verdict = consistent_with_prediction.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent if 1.5<=s_real<=2.5, positive-control slope within 0.1 of 3, both null rates within 0.01 of 1/11, and s_b50>=1.0. Falsified if s_real<1.2 or s_b50<0.5.

Measured:
- s_real (log-log slope of N(B), B in 6..200) = 1.772. N(B) from 66 (B=6) to 31855 (B=200).
- positive control slope = 3.0000 (needs 3 +/- 0.1): pass.
- null rate seed 20261110 = 0.08948; A/A seed 20261111 = 0.09237; expected 1/11 = 0.09091. Deviations 0.0014 and 0.0015 (limit 0.01): pass.
- B50(p): 7:14, 11:31, 13:41, 17:64, 19:78, 23:107 (all 6 primes reached 50% by BMAX=200). s_b50 = 1.707 (needs >=1.0): pass.

Verdict: consistent_with_prediction. Controls (null, A/A, positive) all passed.

## Reading
- The toy does test what PROTOCOL.md says: box-height Q-lift counts and coverage mod small p. It does not test T_section, function-field sections, or Groebner cost (PROTOCOL s6).
- run.py verdict matches the numbers; no discrepancy found.
- s_real=1.77 is below the naive heuristic s=2, probably boundary effects at small B (|b|<=B cuts off large y^2 - x^3 - ax terms); it is inside the preregistered band.
- s_b50=1.71 is above the heuristic p^1 lower bound and near p^1.5. B50 is the same order as p already at p=23 (107 vs 23), so the lift height grows faster than sqrt(p) (23^0.5 ~ 4.8). With 6 tiny primes and a 50% threshold the exponent has wide uncertainty; no CI was computed.
- Nothing surprising. B50 at 7 and 11 may be affected by small-p saturation.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: naive bounded-height Q-lifts need height far above sqrt(p) (slope ~1.7 vs p), matching the recorded closure of the Q-lift branch; the open function-field branch is untouched.

## Standing
[pcc:unverified], single local toy run, no independent replay.
