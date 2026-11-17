# RESULT: LEAD-EXO-lange-winterhof-interpolation

## Outcome
- status: ran; exit code 0; wall 0 s by run.status (elapsed_s 0.4 inside run.py). run.stderr empty.
- Single run, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "deficit_real = 0 on all curves (a random labelling has deficit >=1 with probability about 1/p)". Falsified if any real deficit >= 2, or more than 1 + floor(#curves/4) curves with deficit >= 1. Inconclusive if the positive control fails.
- Measured: 5 curves (p = 307, 457, 617, 787, 967; M = 138, 249, 315, 405, 468). Real deficits [0,0,0,0,0]. Null and A/A deficits (2 per curve, 10 total) all 0. Planted 3x^5+7x^2+1 recovered with degree 5 on all 5 curves.
- Verdict: consistent_with_prediction. Controls passed: null yes, A/A yes, positive yes.

## Reading
- The toy tests only the near-trivial full-interpolant degree statistic. Any generic labelling gives degree M-1, so real = null = A/A = 0 is the expected outcome. It does not test the Lange-Winterhof theorem, which concerns degree bounds for polynomials agreeing on subsets.
- Discrepancy: PROTOCOL.md s4 says about 12 primes (every 25th prime in 300..1100), but run.py produced only 5 primes. The falsification threshold with 5 curves is 1 + floor(5/4) = 2 curves. The number of curves is therefore smaller than the protocol estimate. The verdict is unaffected, since all deficits are 0. I did not investigate the cause because run.py must not be edited.
- The run.py verdict field matches the numbers.
- Nothing surprising. The paper text was not read, and the theorem scope is not verified.

## Bearing on the sub-sqrt(p) question
none: the full-degree result is generic for any labelling, so it says nothing about index-calculus cost or sub-sqrt(p) shortcuts. It does not show a gap to rho either.

## Standing: [pcc:unverified], single local toy run, no independent replay.
