# RESULT: LEAD-EXO-ml-learned-models

## Outcome
- Status: ran (single run of run.py, unmodified). Exit code 0, wall 9 s (run.status: exit=0 wall=9). stderr empty/benign.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "no test reaches 4.0 (chance on real curves)"; falsified only if some (curve, label) has |z| >= 4.0 on both split seeds.
- Measured: 24 real tests, max |z| = 3.788 (curve p=5857, N=5801, lsb, seed 11; its seed-22 z = -1.30). flag_score 0. Largest other values: 3.53 (p=5857 bit1 seed 22), 2.86, 2.80, 2.53. No replicated exceedance.
- Negative control (permuted labels): max |z| = 2.035 < 4, passed.
- Positive control (planted 0.65 leak): min z = 5.81 >= 4, passed (below the nominal ~27 because the 2^12 curve has only 2900 test points and the learner is 4-epoch SGD; still above the threshold).
- A/A (different SGD shuffle seed): max |acc diff| = 0.0241 (noise floor, not a gate).
- Verdict: consistent_with_prediction. Controls passed.

## Reading
- The toy tests exactly what PROTOCOL.md s6 says: a linear learner on residue/parity/top-bit features of x,y at N <= 2^18 for lsb, bit1, msb. It does not test deep nets, RL, other feature maps, larger N, or ML as a solver subroutine.
- run.py's verdict field matches the numbers. Caveats: the smallest curve is sample-limited (N-1=5800 so 2900/2900 split, not 8000/8000), and it also produced the near-threshold values (3.79, 3.53), consistent with 24 tests plus an A/A spread of 0.024 (about 2.4 sigma at n=2900), i.e. SGD noise rather than signal. The positive control's weaker-than-expected z (5.8 vs ~27) shows the detector is less sensitive on that curve; sensitivity there is about a 0.54-0.55 accuracy effect.
- Nothing surprising otherwise; accuracies on the three larger curves sit within about 3 z of 0.5.

## Bearing on the sub-sqrt(p) question
none: a null result for linear learners on coordinate-residue features at toy size says nothing about a sub-sqrt(p) algorithm; it only agrees with the recorded "chance on real curves" outcome.

## Standing
[pcc:unverified], single local toy run, no independent replay.
