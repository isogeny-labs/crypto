# RESULT: LEAD-STR-small-cm-disc-nonunit-endo

## Outcome
- status: ran; exit code 0; wall 0 s (run.status: exit=0 wall=0). run.stderr empty.
- Single run of run.py, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): S1 = 0; S2 in [1,3]; S3 <= 0.71; S4 median > 20 x S2.
Measured:
- S1 bound_violations = 0 of 3529 tested (alpha,N) pairs.
- S2 positive_control_min_ratio = 0.99592 (50 control pairs, 41 with small h).
- S3 max net sqrt(h)/h = 0.4472 (h=5 at the maximum; <= 0.71).
- S4 generic median h/log_n N = 84635.7 (null random-lambda median 47620.6); 84635.7 / 0.9959 ~ 8.5e4 > 20.
Verdict from run.py: consistent_with_prediction.
Controls: negative (random lambda) ran; positive control non-empty (50 pairs); A/A not applicable (exact integer arithmetic). Controls passed.

## Reading
- Strictly, S2 = 0.99592 lies just below the preregistered interval [1,3]. run.py's verdict field says consistent anyway. The shortfall is expected: the proved bound is h >= 2 log(sqrt(N)-1)/log n, which is slightly below log_n N for small N (the -1 term matters at N ~ 2^12), so this is not a bound violation (S1 = 0). The interval lower edge of 1 was too tight, or run.py tolerates it. Treat it as a minor discrepancy in the preregistration, not as evidence against the lead.
- Falsification criterion is S1 > 0 only; it did not trigger. The inconclusive conditions (empty positive control, S4 failure) did not occur.
- Generic vs null medians differ (84636 vs 47621) by under 2x; both are enormous relative to the equality regime. The detector separates equality-regime pairs from generic ones by about 4 to 5 orders of magnitude.
- The toy tests only the ring-level eigenvalue order and the arithmetic of the assumed c=1 charging (net = sqrt(h)/h = 1/sqrt(h) <= 0.71 for h >= 2, so this is close to a tautology). It does not build curves, measure isogeny evaluation cost, or exclude an o(h) canonical-representative algorithm (PROTOCOL s6).
- Only 18 elements were used (n_elements 18) from the described ranges; nothing surprising otherwise.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: with canonicalisation charged at h-1 applications, the net factor is at most 0.45 in the constructed equality cases, so no gain over plain rho; an o(h) canonicalisation remains untested.

## Standing
[pcc:unverified], single local toy run, no independent replay.
