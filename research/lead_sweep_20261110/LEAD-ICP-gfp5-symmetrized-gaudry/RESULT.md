# RESULT: LEAD-ICP-gfp5-symmetrized-gaudry

## Outcome
- Status: ran. Exit code 0. Wall 9 s (run.status; run.py internal elapsed 8.4 s). One run only, no retries.
- run.stderr: empty/no tail output of note.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): r in [0.9, 1.1] (CI contained). Falsified if the CI lies wholly outside while controls pass; inconclusive if the CI straddles an edge or a control fails.
Measured (40 curves, q in {29,31,37,41,43}, 8 per q, n=3):
- r_main = 0.9993, 95% bootstrap CI [0.9467, 1.0608] (inside [0.9, 1.1]).
- Null (uniform balls): r = 1.0072, CI [0.9969, 1.0165]. Passed.
- Positive control (degenerate P,-P,Q kept): r = 1.642, CI [1.536, 1.823]; lower bound > 1.5. Passed.
- A/A halves: r = 0.9709 and 1.0304, diff 0.0595. Passed (run.py aa_ok true).
- Verdict (run.py): consistent_with_prediction. All three controls ok.

## Reading
- The toy tests only conservation of relation yield: distinct non-antipodal 3-subset sums of x-in-F_q points collide as uniform balls into N bins. They do, to within about 6% CI half-width.
- The main CI is not tight: its upper edge is 1.061 and the lower 0.947, so yield structure at the few-percent level cannot be excluded.
- Observed T/N is small (about 0.02 to 0.26), so collision counts are modest per curve; the A/A diff 0.059 is close to the tolerance, consistent with the noise floor.
- No discrepancy between the verdict field and the numbers. I did not re-derive the statistics independently beyond reading them.
- Not tested: n=5, rational 2-torsion symmetry, quotient D=2^16, msolve/F4/Wiedemann finish. The 0.30 to 0.63 bit margin is untouched.

## Bearing on the sub-sqrt(p) question
Confirms the recorded obstruction (at most): no hidden yield non-conservation at toy size, so nothing here reopens the closed_dominated lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
