# RESULT: LEAD-ICE-syzygy-cascade-signatures

## Outcome
- status: ran; exit code 0; wall 1 s (run.status: exit=0 wall=1); run.stderr empty.
- Single run of run.py, no retries, no parameter changes.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "null arms A and B (A/A, independent seeds 1001, 2002) give D_3=0 and D_4=10 in all 20 trials each; planted arm k gives D_3=k in all 20 trials, slope = 1."

Measured:
- null_A (seed 1001): D_3 = 0 in 20/20, D_4 = 10 in 20/20.
- null_B (seed 2002, A/A): D_3 = 0 in 20/20, D_4 = 10 in 20/20; identical to null_A.
- planted k=0..4 (seeds 3000..3004): D_3 = k in 20/20 trials each.
- slope_d3 = 1.0 exactly.
- checks: null_d3_zero true, null_d4_equals_koszul true, unit_slope_exact true.

Verdict: consistent_with_prediction.
Controls: null (generic quadrics) passed; positive control (planted k=1..4) passed; A/A passed (exact agreement).

## Reading
- run.py's verdict field matches the numbers; no discrepancy found.
- The toy tests only what PROTOCOL.md section 3 states: a Macaulay rank-deficit detector on random generic and planted ideals over F_10007 with n=5. It does not test Semaev summation polynomials, F5 criteria, or non-rewritable-signature counts in a real F5/Groebner run.
- The SIG-D6-NULL-RECAL-v2 spec was not available, so this is a reconstruction (PROTOCOL section 6). The unit slope is close to a tautology of linear algebra, so the pass shows the detector is implemented correctly, not that the lead is promising.
- Nothing surprising: no rank loss mod p in 140 trials.

## Bearing on the sub-sqrt(p) question
none: this validates a calibration control for a rank-deficit detector on toy generic ideals and says nothing about the cost of any ECDLP method.

## Standing
[pcc:unverified], single local toy run, no independent replay.
