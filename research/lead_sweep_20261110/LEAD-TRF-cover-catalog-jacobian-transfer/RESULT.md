# RESULT: LEAD-TRF-cover-catalog-jacobian-transfer

## Outcome
- status: ran; wall 2 s; exit code 0 (run.status: exit=0 wall=2). run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): split arm = 1.0; A/A = 1.0; null <= 0.2. Falsified if any split/A/A curve violates #J = #E1*#E2.
- Measured (60 curves/arm, 10 primes 59..139, p=3 mod 4): split 1.0, A/A 1.0, null 0.0.
- Verdict (run.py): consistent_with_prediction.
- Controls: positive (split arm) passed; A/A passed; null passed (0.0 <= 0.2, so detector discriminates).

## Reading
- The toy tests only the order identity for the bielliptic family y^2=x^6+ax^4+bx^2+c. It does not test the catalog pullbacks, Mumford arithmetic, subgroup recovery or any DLP speedup.
- Verdict field matches the numbers; no discrepancy. Null at 0.0 shows the check is not vacuous. Small primes and small arms only.
- Positive control is theory-guaranteed, so it mainly validates the counting code.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the pushforward lands in a same-size isogeny factor (#J = #E1*#E2), so no group-order reduction.

## Standing
[pcc:unverified], single local toy run, no independent replay.
