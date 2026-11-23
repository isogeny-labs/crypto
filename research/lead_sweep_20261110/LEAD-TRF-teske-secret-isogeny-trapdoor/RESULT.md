# RESULT: LEAD-TRF-teske-secret-isogeny-trapdoor

## Outcome
- status: ran (single run, no retries). exit code 0, wall 4 s (run.status: exit=0 wall=4).
- 5000 curves sampled, 120 matched pairs, 10 trials per curve, 0 failures.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): CI of D (mean log of rank-2 over rank-1 rho steps over matched pairs, same p and N) lies inside [-0.15, +0.15] -> consistent_with_prediction; falsified if CI wholly outside; inconclusive if a control fails.

Measured:
- D = 0.00444, bootstrap 95% CI [-0.0445, +0.0530]. Inside the margin.
- A/A: mean 0.0147, CI [-0.0294, +0.0561]; contains 0 (pass).
- Positive control: 5 points, pass.
- Null control: matched pairs share N by construction (D=0 null); D is consistent with it.
- Verdict (run.py): consistent_with_prediction. Controls passed.

## Reading
- The toy tests only the narrow, sharpened claim: whether 2-torsion rank (proxy for crater vs lower level) changes generic rho cost on the shared prime subgroup at p=4093. It does not, to within about +/-0.05 in log steps.
- run.py's verdict field matches the numbers (CI well inside +/-0.15). No discrepancy.
- Caveats: the positive control has only 5 points; wall time of 4 s is far below the 1-3 min estimate, which is plausible at l ~ 10^3 but unremarked. The CI width (~0.1) is comfortably narrower than the margin, so the test has power for effects of the margin size. It does not reproduce the 40-bit panel, a Teske construction, or any isogeny walk, and says nothing about non-generic attacks.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: generic rho cost is invariant to this shape proxy within a class at toy size, as Tate's equal-N argument predicts, so no rho-visible weakness appears.

## Standing
[pcc:unverified], single local toy run, no independent replay.
