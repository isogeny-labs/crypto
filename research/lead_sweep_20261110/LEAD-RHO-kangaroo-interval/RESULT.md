# RESULT: LEAD-RHO-kangaroo-interval

## Outcome
- status: ran (run.py completed, single JSON object produced)
- wall seconds: 4 (run.status: exit=0 wall=4)
- exit code: 0; run.stderr empty. One run only, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): R = mean over sizes of kangaroo/rho ops, 95% bootstrap CI inside [1.3, 2.4]
-> consistent; wholly outside -> falsified; controls must pass and <=2% runs may fail, else inconclusive.
- Measured R = 5.930, CI [5.284, 6.561] (wholly above 2.4).
- Per size kang/rho mean ops: 2^14 385.4/171.6 (2.25); 2^16 711.5/354.4 (2.01); 2^18 12537/743 (16.9); 2^20 4149/1598 (2.60).
  Kangaroo ops/sqrt(q): 3.01, 2.78, 24.49, 4.05. Rho ops/sqrt(q): 1.34, 1.38, 1.45, 1.56.
- A/A (rho vs rho): 0.995, CI [0.936, 1.057], contains 1: PASS.
- Positive control (short W=q/256 / full): 0.480, CI [0.461, 0.501], expected [0.03, 0.15]: FAIL.
- Failures to recover x: kang 110 of 1200 (9.2%), short 266 of 1200 (22%), rho 0, rho A/A 0. Above the 2% limit: FAIL.
- Verdict: inconclusive (run.py field). Two controls/validity criteria failed, so the R CI cannot be read against the band.

## Reading
- run.py's verdict field matches the numbers: the CI is outside the band, but the protocol makes the verdict inconclusive when
  the positive control or the 2% failure limit fails, and both fail.
- The kangaroo arm looks defective or badly tuned at this scale, not a measurement of the algorithm. Evidence: the 2^18 mean of
  12537 ops (24.5 sqrt(q)) is non-monotone against its neighbours (2.8 and 4.1 sqrt(q)), which suggests runaway walks (cycles, or
  runs that hit a step cap) feeding the mean; the 9-22% failure rates point the same way. The short-interval arm failing most often
  and costing about half the full cost (expected about 1/16) says the instrument does not see the interval benefit.
  Also, if failed runs are counted at their cap cost, the means are biased; I did not inspect run.py's handling (not edited, not rerun).
- Rho behaves as expected (about 1.3-1.6 sqrt(q), 0 failures), and the A/A null is clean, so the rho side is sound.
- The point ratios at 2^14, 2^16 and 2^20 (2.0-2.6) are near the preregistered band, but they are not evidence, given the failed controls.
  I draw no conclusion from them.

## Bearing on the sub-sqrt(p) question
none: the toy failed its positive control and failure-rate gate, so it neither confirms nor reopens the recorded obstruction
(closed_dominated, 2.203 x floor); the unfixed kangaroo implementation is the limiting factor, not the lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
