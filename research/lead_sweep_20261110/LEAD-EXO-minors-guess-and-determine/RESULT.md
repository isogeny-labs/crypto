# RESULT: LEAD-EXO-minors-guess-and-determine

## Outcome
- Status: blocked (no run). kind = blocked in phase1_index.json; run_cmd empty.
- Wall seconds: 0. Exit code: n/a (run.py not executed).
- BLOCKED.md written with the missing inputs.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): falsified if the upper 95% CI bound of the fitted slope b (log2 cost on log2 N, N in 2^20..2^40, >= 6 sizes, 10+ instances) is < 0.5 at all three pricing levels (0, 0.1, 1 GAE); consistent if the lower bound is >= 0.5 at all three; otherwise inconclusive.
- Measured: nothing. No statistic exists.
- Verdict: not_run.
- Controls: null (DL-shuffled), positive (planted leak) and A/A were not run. controls_ok = false.

## Reading
- The construction is defined only in four papers whose full texts are not available here. A stand-in would test my construction, not the lead's.
- No discrepancy between a run.py verdict and numbers, since nothing ran.
- The recorded "no security risk" is the papers' claim relayed second-hand and unchecked.

## Bearing on the sub-sqrt(p) question
none: nothing was run, so this evidence neither confirms nor weakens the recorded obstruction.

## Standing
[pcc:unverified], no run, no independent replay.
