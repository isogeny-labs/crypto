# RESULT: LEAD-TRF-same-field-genus-g-embedding

## Outcome
- status: ran; exit code 0; wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- Single run, no retries, run.py and PROTOCOL.md untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): positive-control identity rate exactly 1.0, zero Hasse
cofactor violations, null (generic sextic) match rate < 0.5; any failure falsifies the arithmetic.
Measured (seed 20261110, 11 primes 23..67, 12 curves each):
- positive_control_identity_rate = 1.0 (n_pos = 132)
- hasse_cofactor_violations = 0
- negative_control_match_rate = 0.0 (n_neg = 128)
- mean log(cofactor)/log(N) = 1.0059
- Verdict (run.py): consistent_with_prediction.
Controls: positive passed, null passed (detector rejects non-split sextics), A/A not applicable
(deterministic exact counting).

## Reading
- The toy tests only the group-order identity #Jac = #E1*#E2 for bielliptic genus-2 curves and
  the Hasse cofactor bound. It does not measure index-calculus cost.
- The cost_model field (exponent ratio 2) is true by construction (q^(2-2/g) taken from Gaudry).
- Protocol says about 12 primes; the run used 11 (primes 23..67, from the listed 23..69 range).
  n_pos = 132 = 11 x 12 matches. n_neg = 128, so 4 null curves were skipped (likely degenerate);
  not investigated, since run.py may not be edited.
- No discrepancy between the verdict field and the numbers. Nothing surprising.
- Limits: genus 2 bielliptic only, p < 70, split Jacobians; non-split covers untested.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the identity holds on all 132 curves, so the group
holding the embedded E1 has order N times a Theta(p) cofactor, which matches the recorded
exponent-1 obstruction (this re-derives a known bound; no improvement).

## Standing
[pcc:unverified], single local toy run, no independent replay.
