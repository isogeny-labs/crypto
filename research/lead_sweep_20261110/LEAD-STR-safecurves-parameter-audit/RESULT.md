# RESULT: LEAD-STR-safecurves-parameter-audit

## Outcome
- status: ran; exit code 0; wall 166 s (run.status: exit=0 wall=166); run.stderr empty/no errors.
- Single run of run.py, unmodified.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): the 95% bootstrap CI of insecure-minus-secure D lies inside (-0.2, 0.2); the A/A split of the secure arm also lies inside; size positive control CI above 0. Falsified if CI excludes 0 with |D| > 0.1 while both controls pass.

Measured:
- n_prime_order 600; n_secure 116; n_insecure 270; mid band 214 discarded; anomalous trace-1 curves 13.
- Primary D = -0.0169, CI95 [-0.1619, 0.1281]: inside the margin and includes 0.
- A/A D = +0.1101, CI95 [-0.1307, 0.3652]: upper end EXCEEDS 0.2, so the A/A control FAILED the preregistered requirement.
- Positive control: raw log(steps) diff 0.7486, CI95 [0.627, 0.874], expected sqrtN diff 0.683; detected = true.
- Verdict: inconclusive. Controls: positive passed, A/A failed, so controls_ok = false.

## Reading
- The toy does test the stated claim (twist class vs generic rho cost on E(F_p) at toy size). The point estimate is near 0 and the primary CI is within the margin, which matches the prediction, but the preregistered rule requires the A/A to pass for "consistent".
- The A/A CI is wide (width ~0.5 vs ~0.29 for primary) because the secure arm has only 116 curves, split into 58/58; Floyd rho step counts have a heavy-tailed spread (a single DLP per curve). A/A noise floor thus exceeds the 0.2 margin: the instrument at this n cannot certify equivalence within 22%.
- run.py's verdict field ("inconclusive") agrees with the numbers; no discrepancy. Note the n target of 150 per arm was a cap on the draw, secure arm fell short (116).
- Nothing surprising; the noise in A/A (+0.11 point estimate) is comparable to the whole primary CI, so no twist effect is visible, nor is one excluded rigorously.

## Bearing on the sub-sqrt(p) question
none: toy generic-rho on orders 2^14..2^17 with an inconclusive equivalence test; it neither closes nor opens anything beyond the already recorded closed_known disposition.

## Standing
[pcc:unverified], single local toy run, no independent replay.
