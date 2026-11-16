# RESULT: LEAD-EXO-lattes-xline-dynamics

## Outcome
- status: ran (single run, no retry). exit code 0, wall 6 s (run.status: exit=0 wall=6; run.py elapsed_s 6.65, timed_out false).
- 27 null curves (n=1: 12, n=2: 12, n=3: 3). 0 nonsquarefree cases.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): D = 0 and M = 0 over all null curves (n=1,2,3), exact zero; positive controls show dependence (>=80% and >=90%); A/A mean TV <= 0.35.
Measured:
- D (null curves with >1 pattern over sampled k) = 0 of 27
- M (cross-generator mismatch) = 0
- Positive control 1 (2^n | ord(P)): 11 of 12 curves dependent (91.7%), ok
- Positive control 2 (random degree-4 maps): 10 of 10 dependent, ok
- A/A split-half mean TV = 0.168 (<= 0.35), ok
Verdict: consistent_with_prediction. Controls passed: null, positive, A/A.
Observed patterns (identical on every null curve at a given n): n=1 {1:1, 3:1}; n=2 {1:1, 3:1, 6:2}; n=3 {1:1, 3:1, 6:2, 12:4}.

## Reading
- The toy tests the stated claim (degree multiset of phi_2^(n)(x) - x(kP) over F_p is k-independent when r is odd) and agrees with it. The identical pattern across all curves (not only across k) is consistent with Frobenius acting on E[2^n] by a fixed-type element for these odd-order curves; the run did not analyse why.
- run.py's verdict field matches the numbers; no discrepancy found. Controls ran and passed, so the DDF instrument can detect dependence.
- Caveats: n=3 has only 3 curves x 5 k; only d=2, p<4000; degrees only, not the factors themselves. Positive control 1 had one non-dependent curve (11/12), above the threshold. The run was much faster than the estimated 3-8 min.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the factorisation-degree observable is k-independent at these sizes, matching the recorded GGM-simulability argument, so it gives no leverage.

## Standing
[pcc:unverified], single local toy run, no independent replay.
