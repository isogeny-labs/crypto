# RESULT: LEAD-EXO-low-degree-query-lower-bound

## Outcome
- status: ran. exit code 0, wall 6 s (run.status: exit=0 wall=6). run.stderr empty. One run only.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "0 of 12 main-arm cells (p in [2039,4001), D in {8,16,32,48}) show rank != min(D, N-1). A/A, saturation positive control and planted-dependency negative control also show 0 mismatches. Any main mismatch is falsified, any control failure is inconclusive."
- Measured: main mismatches 0/12; A/A 0/12; positive control (p<71, D=N+5) 0/4 mismatches, rank capped at N-1 (e.g. N=40, D=45, rank 39); negative control (planted dependent columns) 0/4, rank 12 of 14 columns.
- Verdict (run.py): consistent_with_prediction. Controls passed: null yes, positive yes, A/A yes.

## Reading
- The toy re-derives the rank fact: x-degree basis evaluation matrix on E(F_p) has full rank min(D, N-1) for D<=48, N~2000-3800. This is the known theorem (nonzero function in L(D*O) has <= D zeros), so it is a sanity check, not new evidence.
- Caveat in protocol: the basis here is x-degree only, not the pole-order basis of the recorded 39/39 check, so it is not that exact instrument.
- No discrepancy between run.py verdict and numbers. No control_ok field in JSON, derived from the four mismatch counts (all 0). Nothing surprising.
- It does not test adaptive queries, GGM bounds, or Delta- or work-charged bounds.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the rank cause is confirmed, so degree alone gives no lower-bound leverage; says nothing about any speedup.

## Standing
[pcc:unverified], single local toy run, no independent replay.
