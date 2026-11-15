# RESULT: LEAD-ICE-semaev-cover-monodromy

## Outcome
- status: ran (single run, no retries). exit=0, wall=35 s (run.status).
- RESULT.json is one JSON object; run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): D_A and D_B < 1.6 with |log(D_A/D_B)| < 0.5; positive control (planted 0.06 bias on odd curves) D > 3.2; permutation p > 0.01 for corr(split fraction, relation rate). Control or A/A failure -> inconclusive.

Measured (p=4093, K=60, NS=15000, FB=96, NT=150):
- D_A = 1.078 (< 1.6 ok); D_B (A/A) = 0.700 (< 1.6 ok); |log(D_A/D_B)| = 0.432 (< 0.5, A/A ok).
- split_mean_A = 0.4995 (matches 1/2).
- corr(split, relrate) = -0.212, permutation p = 0.111 (> 0.01).
- Positive control D = 0.903 (needs > 3.2): FAILED.
- Null (permutation / binomial) behaved as expected: D near 1, p not significant.
- Verdict (run.py): inconclusive.

## Reading
- The inconclusive verdict is correct per the preregistered rule, but the cause is a defect in the positive control, not weak power at this size. In run.py census(), the planted bias does `if bias and rng.random()<bias: ok = not ok if rng.random()<0.5 else ok`. That re-randomises the outcome with prob 0.06 and flips it with prob 0.5, which pulls the split fraction toward 1/2. It does not shift it. With a true 0.06 shift on half the curves, expected D is about 50 (0.03^2*15000/0.25), so seeing 0.90 means the control cannot see a bias.
- So the detector has not been shown able to detect a per-curve bias, and the null results for A and B carry no demonstrated sensitivity.
- Taken at face value, the numbers favour the hypothesis: D_A and D_B near 1 (binomial), split mean 0.5, correlation not significant (p=0.11; r=-0.21 is a mild negative lean at K=60, not significant, worth noting only if replicated).
- The A/A margin is moderate (0.43 of 0.5 allowed), consistent with noise at K=60.
- The correlation test uses 60 points, so it has low power to rule out weak coupling.
- Scope is m=3 at toy p only. Nothing was fixed or rerun, per the rules.

## Bearing on the sub-sqrt(p) question
none: the run is inconclusive (failed positive control), and even a pass would only re-derive the recorded closed obstruction at toy scale.

## Standing
[pcc:unverified], single local toy run, no independent replay.
