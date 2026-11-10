# RESULT: LEAD-ICP-self-pairing-prime-order

## Outcome
- status: ran; exit code 0; wall 7 s (run.status: exit=0 wall=7). Single run, no retries.
- run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): S1 = 0, S2 = 0, S3 = 1.0 with bilinearity and DL transfer holding in every positive-control case. Falsified if S1 > 0 or S2 > 0 or A/A counts differ; inconclusive if positive control fails.

Measured:
- S1 (prime-order curves with gcd(r,p-1) > 1): 0
- S2 (prime-order curves with k <= 2): 0
- S3 (positive-control t(P,P) != 1 rate): 1.0
- Negative control A (seed 101): 108 curves, gcd>1 = 0, k<=2 = 0, min k/(r-1) = 0.0066.
- Negative control B / A/A (seed 202): 108 curves, gcd>1 = 0, k<=2 = 0 (counts equal), min k/(r-1) = 0.00049.
- Positive control A (seed 303): 15 curves; 15 gcd>1 detected, 15 nondegenerate, 15 bilinear, 15 DL transfer.
- Positive control B / A/A (seed 404): 17 curves; 17/17/17/17 likewise.
- Verdict: consistent_with_prediction. Controls: null passed, positive passed, A/A passed (detector_alive = true).

## Reading
The toy tests what the protocol says it tests: the Hasse argument at p <= 30013 (9 primes, 216 prime-order curves) and nondegeneracy of the Tate self-pairing exactly when r | p-1. run.py's verdict field matches the numbers; no discrepancy found. Caveats: tiny sizes, the theorem is already proved, positive-control sample is small (32 curves), and the supersingular k=2 case is excluded by design. Nothing surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: it empirically reproduces the known theorem that prime-order E(F_p) admits no rational nondegenerate self-pairing, so the lead gives no route below sqrt(p).

## Standing
[pcc:unverified], single local toy run, no independent replay.
