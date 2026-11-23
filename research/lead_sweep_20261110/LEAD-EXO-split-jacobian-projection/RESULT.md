# RESULT: LEAD-EXO-split-jacobian-projection

## Outcome
- Status: ran. Exit code 0. Wall 14 s (run.status: exit=0 wall=14). run.stderr empty.
- Single run of run.py, unmodified, no retries.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): m = min over split cases of (#J/|G|)/p >= 0.25 in both seeded arms; falsified if m < 0.25 in either arm. Controls: #J = #E1*#E2, Hasse-Weil band, A/A medians within 0.15.
- Measured (23 primes 37..139, 12 curves/prime, 276 split + 276 generic per arm):
  - Arm 1 (seed 20261110): m = 0.831, median 7.48.
  - Arm 2 / A/A (seed 20261111): m = 0.767, median 7.46.
  - Generic #J/p^2 median 1.006 (arm 1), 1.015 (arm 2); range 0.70..1.46.
- Controls: positive (isogeny #J=#E1*#E2) 0 mismatches both arms; Hasse-Weil 0 violations; null generic band OK; A/A ok (aa_ok true, median diff ~0.009).
- Verdict: consistent_with_prediction. Controls passed.

## Reading
- The toy re-derives Hasse-Weil inflation at toy size only; it does not test the lead's open projection idea (as PROTOCOL s2/s6 state).
- Discrepancies: PROTOCOL says 25 primes; run.py used 23 primes (list in RESULT.json). The run.py verdict matches the numbers. Minimum ratio is 0.77-0.83, above the 0.25 threshold but only ~3x, not far from p/4 scale; the closest cases are small-ratio splits worth noting but not a signal.
- Nothing surprising; expected by theorem.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: ambient order is never O(N) at p<=139 for random split curves, matching the recorded Jacobian-inflation obstruction.

## Standing
[pcc:unverified], single local toy run, no independent replay.
