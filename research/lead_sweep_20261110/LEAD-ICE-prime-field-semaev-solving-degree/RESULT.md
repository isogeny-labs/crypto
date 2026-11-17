# RESULT: LEAD-ICE-prime-field-semaev-solving-degree

## Outcome
- Status: ran (single run, no retries). Exit code 0. Wall 1 s (run.status: exit=0 wall=1); run.py elapsed_s 0.1.
- run.stderr empty. RESULT.json is one JSON object, verdict "consistent_with_prediction".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): pooled fraction f of instances with D_sol >= v. Consistent if f_semaev >= 0.80,
falsified if < 0.50. Valid only if controls pass (positive control f_pos < 0.5 gating; A/A |f - f_AA| <= 0.15 gating;
null reported, not gating).
Measured (n=48 per arm, 12 per v in 3..6):
- S_3 arm: f = 1.000 (per v: 1.0, 1.0, 1.0, 1.0). mean D_sol/v = 1.056 (v=3), 1.0 (v=4), 1.033 (v=5), 1.0 (v=6).
- A/A (second seed set): f = 1.000, |diff| = 0 <= 0.15: pass.
- Null (random bidegree (2,2) bivariate): f = 1.000 (as expected near 1; not gating).
- Positive control (x2 factor base planted to one point): f = 0.000 (mean D_sol/v = 1/v: 0.33, 0.25, 0.20, 0.167),
  < 0.5: pass.
Verdict: consistent_with_prediction. Controls passed (null, positive, A/A).

## Reading
- The toy tests a narrow thing: whether, for m=2 direct S_3 plus F(x1)=F(x2)=0, the Macaulay degree at which the
  x1-eliminant appears is at least v. It is: D_sol is v in nearly all cases and v+1 in a few (v=3, v=5).
  The detector can see savings, since the planted single-point arm drops to D_sol=1.
- Matches run.py's verdict field; no discrepancy between verdict and numbers.
- Caveats: the S_3 arm is indistinguishable from the random bidegree-(2,2) null, so the result says the degree is set by
  the factor-base equations F1,F2 (degree v each), not that S_3 behaves in any special way. Hardly a test of
  structure in S_3. Ceiling effect: f is 1.0 in three arms, so the 0.80 / 0.50 thresholds were not stressed.
- Runtime was 0.1 s versus the protocol's estimate of "a few minutes"; not a defect in the numbers I can see, but I did
  not audit run.py, and the large gap to the estimate is worth a replay check. The v+2 window and Macaulay (not F4/F5)
  degree are as preregistered; with D capped at v+2 a failure to find the eliminant would be recorded as >= v+1.
- Nothing surprising otherwise.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy size (m=2, p~100-127, v<=6) no degree saving below v appears, consistent with D_sol >= v (A3); it says nothing asymptotic.

## Standing
[pcc:unverified], single local toy run, no independent replay.
