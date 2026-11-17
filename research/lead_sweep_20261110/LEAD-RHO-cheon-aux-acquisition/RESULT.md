# RESULT: LEAD-RHO-cheon-aux-acquisition

## Outcome
- status: ran; exit code 0; wall 0 s (run.status: exit=0 wall=0, under 1 s, rounded). run.stderr empty.
- run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): slope S of log2(mean rho mults) vs log2 r in [0.45, 0.55]; (acquire-by-DL + Cheon)/rho > 1 at every size; span null hits no more than binomial expectation + 5 sd + 3; positive control (free-aux Cheon beats rho at some size) and A/A (ratio in [0.7,1.4]) pass.

Measured:
- S = 0.5132 (in band).
- Self-acquired Cheon total / rho: 6.25, 4.85, 4.95, 3.91, 3.72 for 12..20 bits (all > 1).
- Null (span heuristic): 0 hits at every size (expected 0.33, 0.16, 0.086, 0.042, 0.025). Passes.
- Positive control: FAILED. cheon_over_rho (free aux) = 5.09, 3.78, 3.90, 2.88, 2.71; never below 1 at any size.
- A/A: FAILED by the run.py criterion. Per-size half ratios 0.83, 1.435, 0.73, 1.33, 0.98; the 14-bit value is above 1.4. (The protocol says "mean ratio", run.py flags a per-size violation.)
- Verdict: inconclusive (controls not all passed). Neither falsified nor consistent.

## Reading
- Main statistic and null behave as predicted, but the detector's positive control did not show an aux-driven gain, so the toy cannot distinguish "aux is the cost" from "Cheon implementation is not competitive at these sizes".
- Free-aux Cheon cost ratio is falling with size (5.09 -> 2.71), which suggests a crossover beyond 20 bits (Cheon about r^(1/3) vs rho r^(1/2)); at toy size the constants (1.5*log2 r per exponentiation, BSGS over (r-1)/d) dominate. Not tested.
- Rho cost / sqrt(r) is about 1.8-2.5, noisy; S = 0.51 is within the stated +-0.03 noise, but 40 runs with a heavy-tailed rho cost give wide per-size error, and A/A at 14 bits (1.435) shows that.
- The acquire-by-DL arm contains a rho, so ratio > 1 is tautological, as PROTOCOL s6 notes.
- run.py verdict agrees with the numbers; no discrepancy.

## Bearing on the sub-sqrt(p) question
none: the run re-measures the rho r^0.5 scaling and finds no span shortcut, but the failed positive control means it neither confirms nor reopens the recorded obstruction.

## Standing
[pcc:unverified], single local toy run, no independent replay.
