# RESULT: LEAD-ICP-representation-subset-sum

## Outcome
- Status: timeout. run.py hit the 660 s `timeout` wrapper. Exit code 124, wall 660 s (run.status).
- RESULT.json is 0 bytes and run.stderr is 0 bytes. run.py printed nothing, so there are no measured numbers.
- One run only. No retry, no edits to run.py or PROTOCOL.md. The timeout is the result, as the frozen rules require.
- The PROTOCOL.md estimate was "a few seconds" (phase1_index est_minutes = 1). The actual run was at least 660 s.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "all three generic arms (rho, BSGS, representation two-list sampler) have slope >= 0.40 and min ops/sqrt(N) >= 0.5." Falsified if any generic arm has CI upper bound < 0.45 or min ops/sqrt(N) < 0.3. Positive control (Pohlig-Hellman) must give CI upper bound < 0.45. The rho A/A slope difference must be < 0.1.

Measured: nothing. No slopes, no ops/sqrt(N), no CIs.
Verdict: not_run (timeout). Controls: not evaluated, so controls_ok = false (null, positive and A/A all unobserved).

## Reading
- The toy never finished, so it tested nothing about the lead's claim.
- Cause is not diagnosed by running anything. A static look at run.py (grep only) shows the structure. It has Python-level loops with per-trial caps of about 8*sqrt(N) (rho) and 40*sqrt(N) (rep). Pure-Python rho restarts are also possible. Trials run at up to N near 2^22 (sqrt N about 2048), 60 trials per size, several arms, bootstrap with B=200. The "few seconds" cost estimate in the protocol looks wrong by orders of magnitude on this shared machine. I did not confirm which arm was slow.
- The program prints only at the end, so there is no partial output to salvage. The shared machine may also have added load. I did not measure that.
- No discrepancy between run.py's verdict field and the numbers can be assessed, because no verdict was emitted.

## Bearing on the sub-sqrt(p) question
none. No data was produced, so the recorded obstruction (generic model, Shoup bound) is neither confirmed nor closed by this run.

## Standing
[pcc:unverified], single local toy run that timed out, no independent replay.
