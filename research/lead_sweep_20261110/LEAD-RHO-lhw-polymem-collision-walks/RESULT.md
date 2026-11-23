# RESULT: LEAD-RHO-lhw-polymem-collision-walks

## Outcome
- Status: ran to completion. Exit code 0, wall 4 s (run.status: exit=0 wall=4). run.stderr empty.
- run.py verdict field: "inconclusive". Single run, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent iff all rungs c = mean/sqrt(S) in [2,40] AND slope in [0.35,0.65], with controls passing
(positive: 0 solve failures; null: 0 hits; A/A rel diff < 0.35). Falsified if controls pass but c or slope leave the bands.
Inconclusive if any control fails.

Measured:
| (m,w) | S | mean f-evals | c | solved/480 | null hits | A/A rel diff |
|---|---|---|---|---|---|---|
| (16,4) | 28 | 1029.5 | 194.6 | 391 | 0/20 | 0.048 |
| (18,6) | 84 | 2616.3 | 285.5 | 229 | 0/20 | 0.013 |
| (20,6) | 120 | 3402.0 | 310.6 | 177 | 0/20 | 0.007 |
| (22,8) | 330 | 6225.0 | 342.7 | 82 | 0/20 | 0.013 |
Log-log slope 0.731. positive_control_failures = 1041 of 1920 instances (54%). null_spurious_hits = 0. aa_ok = true.

Controls: null PASSED, A/A PASSED, positive control FAILED (1041 unsolved within 60 restarts).
Verdict: inconclusive (as run.py says).

## Reading
- The toy does test the stated mechanism (Floyd walk over hash-selected jump sums, golden collision check), but the
  positive control fails badly, so the walk as implemented is not a working solver at these sizes. The solve rate
  falls with S (81% -> 48% -> 37% -> 17%), which is itself informative: more restarts or a different walk would be needed.
- The numbers would lie outside the bands anyway (c 195-343 vs [2,40]; slope 0.73 vs [0.35,0.65]), consistent with the
  smoke-test warning (ratio 157). But the protocol forbids calling that falsified when controls fail. Also the means
  are likely computed over a mixture that includes failed instances (cost cap 60 restarts), which biases the slope; I did
  not inspect run.py to confirm how failures enter the mean.
- No discrepancy between the verdict field and the numbers. Plausible cause of failures (my speculation, untested):
  a walk on a group of size ~2^23 with only S ~ 28-330 distinct jump targets per type has a tiny golden-collision
  probability per merge, so cost is dominated by restarts, not by sqrt(S).
- Nothing surprising beyond the high failure rate.

## Bearing on the sub-sqrt(p) question
none: controls failed, so the run says nothing about the lead's sqrt(C(m/2,w/2)) scaling, and the lead is a low-weight-key
class in any case, not a generic-key attack.

## Standing
[pcc:unverified], single local toy run, no independent replay.
