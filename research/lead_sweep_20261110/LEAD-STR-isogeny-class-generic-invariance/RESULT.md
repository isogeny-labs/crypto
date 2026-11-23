# RESULT: LEAD-STR-isogeny-class-generic-invariance

## Outcome
- Status: timeout. run.py was killed by `timeout 660` (exit=124, wall=660 s). Single run, no retry.
- RESULT.json is empty (0 bytes); run.stderr is empty (0 bytes). No statistic was emitted.
- The protocol estimated 1-2 minutes; actual runtime exceeded 660 s (about 5x or more the upper estimate).
- Note: the Bash tool moved the command to the background at 600 s; the same single process finished at the 660 s timeout. No second copy was started.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): consistent if every class curve has its 90% CI inside [1/1.08, 1.08], the A/A arm is equivalent and the positive control is detected. Falsified if a class curve's 99% CI lies wholly outside the band with valid controls.
- Measured: nothing. No ratios, CIs, null, positive-control or A/A results exist.
- Verdict: not_run. Controls: not evaluated (controls_ok = false).

## Reading
- The toy never produced output, so it neither tests nor says anything about the lead's claim.
- Likely cause (not investigated, since run.py may not be edited): the pure-Python class search (up to 4000 point counts of about 16k Legendre terms each) and/or the rho walks were much slower than estimated on this shared machine. This is a guess, not a verified diagnosis.
- Per the rules the timeout is the result; run.py was not fixed and not re-run.

## Bearing on the sub-sqrt(p) question
none: no data was produced, so the recorded obstruction (closed_obstructed, theorem plus 403,168-run census) is neither confirmed nor challenged by this run.

## Standing
[pcc:unverified], single local toy run, no independent replay.
