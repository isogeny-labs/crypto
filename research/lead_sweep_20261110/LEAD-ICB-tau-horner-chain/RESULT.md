# RESULT: LEAD-ICB-tau-horner-chain

## Outcome
- status: ran (single run, no retries). exit code 0, wall 1 s (run.status: exit=0 wall=1). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): ratio D_horner/D_plain == 1.0 exactly in closed and A/A arms; adds_per_distinct_horner / adds_per_distinct_plain in [1.5, 2.5]; negative control (random FB, same size) ratio < 0.5.

Measured (GF(2^13), N=8373 points, |F|=78, m=3, 40 targets):
- closed (seed 20261110): D_plain=379, D_horner=379, ratio 1.0; adds/distinct 650.34 plain vs 1284.22 Horner (x1.975).
- A/A (seed 20261111): D_plain=380, D_horner=380, ratio 1.0; adds/distinct 648.63 vs 1280.84 (x1.975).
- control (random FB, seed 20261112): D_plain=429, D_horner=0, ratio 0.0 (< 0.5); adds/distinct Horner undefined.
- per-target pairs are equal in every target of both closed arms.

Verdict: consistent_with_prediction. Controls: null (control) passed, A/A passed, no separate positive control by design (control doubles as sensitivity check). Inconclusive guards not triggered (D_plain 379/380 >= 50, control D_plain > 0 and ratio < 0.5).

## Reading
- The toy tests the identity claim: with a tau-closed F, the Horner chain gives the same relation set as plain sums, at about 2x the additions per distinct relation. Result matches run.py's verdict field; no discrepancy.
- Largely tautological, as PROTOCOL s6 states: exact equality is guaranteed by tau-closure. The control ratio of exactly 0 (not merely < 0.5) is expected since a random 78-point set is essentially never tau-closed (chance hits ~ 78^2*78/8373^... negligible). It shows the detector can see lost relations.
- Not tested: symmetry-broken arm, m > 3, Koblitz sizes n=61..83, real factor bases, Semaev/linear-algebra cost, Section 5a native-work pricing.
- Nothing surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: for tau-closed F the chain is a relabelling with ~2x ordered-enumeration cost, so by itself it is no improvement (relabelling is not an improvement under the owner's success rule); the lead's real gain, if any, must lie elsewhere.

## Standing
[pcc:unverified], single local toy run, no independent replay.
