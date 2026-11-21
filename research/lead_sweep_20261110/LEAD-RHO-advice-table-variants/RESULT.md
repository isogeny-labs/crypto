# RESULT: LEAD-RHO-advice-table-variants

## Outcome
- status: ran; exit code 0; wall 102 s (run.status: exit=0 wall=102). One run, no retries.
- run.stderr empty. RESULT.json is a single JSON object, verdict "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): g = 1 - mean R <= 0.02 for both fractions (likely R > 1), both 1 - upper CI <= 0.03; falsified if 1 - upper CI > 0.03 for either fraction; positive control needs 1 - upper CI > 0.03 at 0.75; A/A CI inside [0.9, 1.1].

Measured (ratio vs uniform table, mean [95% t-CI], 8 reps):
- reselect 0.65: 3.48 [-1.78, 8.74]; gain g = -2.48
- reselect 0.75: 3.43 [-1.77, 8.63]; gain g = -2.43
- null 0.65: 3.85 [-2.57, 10.26]; null 0.75: 4.11 [-2.58, 10.81]
- control_map reselect 0.75: 3.12 [0.47, 5.77]; positive-control gain lower bound -4.77 (needs > 0.03)
- A/A (group, 0.75): 2.78 [-1.61, 7.18]; control_map A/A 1.56 [-0.20, 3.33]. aa_ok = false.

Verdict: inconclusive. Controls: A/A FAILED, positive control FAILED (null control is not interpretable either). Controls did not pass.

## Reading
- The toy did not test the lead's claim. An A/A ratio of about 2.8 (should be about 1) means the paired ratio statistic is itself broken or dominated by noise: the CIs span from negative to 7-10, and ratios of means below zero in the lower bound are impossible for a ratio of positive step counts, so the per-replicate R is heavy-tailed (probably a near-zero denominator mean, i.e. very few table hits within the cap of 10200 steps, or capped/failed walks). I did not investigate this, because run.py may not be edited or re-run.
- run.py's verdict field ("inconclusive") agrees with the numbers and with the PROTOCOL rule (controls fail -> inconclusive). No discrepancy.
- Surprising: every arm, including null and A/A, has ratio far above 1. The point estimates (R about 3.4 for reselect, about 4 for null, about 2.8 for A/A) cannot be read as a loss for reselection. The nominal reselect vs null ordering (3.4 vs 3.9-4.1) is within noise.
- Nothing here confirms or contradicts the recorded -0.064 at T/2.

## Bearing on the sub-sqrt(p) question
none: the toy failed its own A/A and positive controls, so it measures nothing about the lead; and a constant-factor gain could not change the sqrt(p) exponent (Shoup) in any case.

## Standing
[pcc:unverified], single local toy run, no independent replay.
