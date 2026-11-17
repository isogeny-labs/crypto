# RESULT: LEAD-RHO-generic-sqrt-lower-bound

## Outcome
- status: ran (single run, no retries). exit code 0, wall 3 s (run.status: exit=0 wall=3).
- RESULT.json is one JSON object; run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): both generic arms give alpha CI containing 0.5 and no size below 0.5*sqrt(N); positive control alpha CI upper < 0.3; BSGS A/A |alpha_A - alpha_B| < 0.1. Falsified if either generic arm CI upper < 0.45 or any mean ratio < 0.5; inconclusive if controls fail. "Consistent" requires both CIs to contain 0.5.

Measured:
- bsgs: alpha 0.4786, CI95 [0.4674, 0.4911]; ratio ops/sqrtN 1.44..1.76 (min 1.445 at 2^20); 0 failures.
- rho: alpha 0.5575, CI95 [0.5268, 0.5877]; ratio 2.95..9.69 (3.51, 3.51, 4.57, 8.49, 9.69, 2.95); 0 failures.
- bsgs_AA: alpha 0.4819, CI [0.4666, 0.4946]; |diff| from bsgs = 0.0033 (< 0.1).
- clear (positive control): alpha 0.0, CI [0,0]; ratios <= 0.016 (detects sub-sqrt).
- Falsification: not triggered (no CI upper < 0.45; no ratio < 0.5).
- Controls: positive passed, A/A passed. Null is the Feistel encoding itself (no separate null).
- Verdict (run.py): inconclusive. Reason: neither generic CI contains 0.5 (bsgs CI sits just below, rho CI above), so the "consistent" condition is not met, and falsification is not met.

## Reading
- The toy does test the claim at the level it can: counted generic-oracle BSGS and DP rho, with a cleartext solver showing the detector sees sub-sqrt cost. No arm used fewer than 0.5*sqrt(N) calls; the smallest ratio is about 1.44 (BSGS).
- The verdict field matches the numbers and the preregistered rule. "Inconclusive" arises from tight CIs missing 0.5, not from control failure or a sub-sqrt signal.
- BSGS slope slightly below 0.5 is a lower-order effect (ratio declines from 1.76 to 1.45 then ticks up to 1.58; the giant-step setup cost ~log N and the expected half-table search are a constant plus a lower-order term); it is far from 0.45 and ratios are > 1. Rho slope 0.56 is driven by noisy per-size ratios (2.95 to 9.69, non-monotone; the 2^22 point is low), consistent with the protocol's warned heavy-tailed trails and DP overhead at toy sizes; 24 seeds is small for rho. The bootstrap CI likely understates this variability.
- The CI-contains-0.5 criterion is strict for finite-size slopes with known lower-order bias; this is a protocol design weakness, not evidence for or against a sub-sqrt algorithm.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction (weakly): no generic arm beat sqrt(N), all ratios >= 1.4, and the positive control shows leaks would be visible; it does not prove the lower bound and says nothing about curve-specific methods.

## Standing
[pcc:unverified], single local toy run, no independent replay.
