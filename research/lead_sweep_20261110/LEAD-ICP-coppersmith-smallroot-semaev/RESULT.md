# RESULT: LEAD-ICP-coppersmith-smallroot-semaev

## Outcome
- status: ran; exit code 0; wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- Single run of run.py, no retries, no edits.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): |reach_2 - 0.35355| < 5e-4 and |reach_3 - 0.3293| < 5e-4, both < 8/9.
- Measured: vol Newt(S_3) = 14/3 (n=3), 4 (n=2); reach_3 = 0.329317, reach_2 = 0.353553; demand 8/9 = 0.8889; margins -0.5596 (n=3), -0.5353 (n=2).
- Verdict (run.py): consistent_with_prediction. Both windows satisfied.
- Controls: positive univariate deg 2 = 0.5 (pass); bivariate linear = 1.0 (pass); negative full cube {0,1,2}^3 = 0.27516 < 0.3293 (pass); A/A replay bit-identical (pass); curve validity: 0 bad of 400 for each of 3 seeds (pass).

## Reading
- It re-derives only the closed-form equal-bound formula from the 13-point x-support of S_3. It reproduces the recorded numbers exactly.
- No discrepancy between the verdict field and the numbers.
- It does not run any lattice, so it does not test unequal-bound optima (0.4357/0.4892), a joint three-block (4,2) lattice, non-convex monomial sets, rank-r GAP substitution, or non-Coppersmith solvers. The 8/9 demand is taken from the atlas, not re-derived.
- Nothing surprising. The smoke result matched the full run.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the equal-bound reach is reproduced exactly and sits more than 0.5 below the demand, but only for the restricted formula, leaving the lead's reopen clauses untouched.

## Standing
[pcc:unverified], single local toy run, no independent replay.
