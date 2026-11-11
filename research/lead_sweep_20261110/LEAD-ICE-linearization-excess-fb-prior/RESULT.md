# RESULT: LEAD-ICE-linearization-excess-fb-prior

## Outcome
- Status: ran to completion (single run, no retries). Exit code 0. Wall 26 s (run.py wall_s 25.8).
- Files: RESULT.json, run.status, run.stderr (empty tail).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent_with_prediction if pooled permutation p <= 0.01 (1000 shuffles), observed mean residual >= 0.01, and >= 75% of units positive. Falsified if p > 0.05 or observed <= 0. Inconclusive if the positive control fails.

Measured:
- observed pooled mean residual = -0.00232 (threshold +0.01)
- perm_p = 0.901 (1000 permutations)
- units positive = 3 of 8 (62.5% needed 75%)
- per-unit residuals: (5,6,3) +0.00108, +0.00062; (5,6,4) -0.00411, +0.00225; (7,4,3) -0.01352, -0.00489; (7,4,4) 0.0, 0.0
- aa_abs_diff_max = 0.00397 (noise floor; larger than the observed pooled effect)
- Null control (label permutation): ran; observed sits well inside the null (p = 0.90).
- Positive control (Spearman of |S| vs coverage > 0.3 in every unit): FAILED. Spearman = 0.99, 0.99, 0.99, 0.99 for p=5; 0.62, 0.75 for (7,4,3); exactly 0.0 for both (7,4,4) units. pos_control_ok = false.
- Verdict (run.py): inconclusive. Controls did not all pass (positive control failed; null and A/A ran).

## Reading
- The numbers lean against the claim: the pooled effect is slightly negative, p = 0.90, only 3 of 8 units positive, and the effect is below the A/A noise floor. Taken alone, those numbers meet the falsified criteria (p > 0.05 and observed <= 0).
- The protocol makes a failed positive control override that, so the verdict "inconclusive" is correct as preregistered, and I do not upgrade it to falsified.
- Cause of the failed control: the two (7,4,4) units have Spearman exactly 0.0 and residual exactly 0.0, which is what saturated coverage looks like (m=4 sumsets over q=2401 cover essentially the whole group for every subspace, so there is no variation to detect). I did not inspect the coverage values, and run.py was not modified, so this is a likely explanation, not verified. The (7,4,3) units are also weakly sensitive (q small).
- The informative units are the four p=5 cells (q=15625), where the positive control passes (about 0.99) and the residuals are tiny (-0.004 to +0.002), so e>=1 shows no visible gain there.
- Caveats from PROTOCOL.md section 6 apply: the toy's e is a proxy of mine, not the atlas definition; structured e>=1 subspaces are all subfield multiples (confounded with e); coverage is sumset yield, not relation cost; toy sizes only.
- Nothing surprising beyond the saturation of the p=7 m=4 cells.

## Bearing on the sub-sqrt(p) question
none: the toy tests a proxy of a factor-base design prior at tiny sizes, the result is inconclusive by its own control, and the point estimate is null to slightly negative, so it neither confirms nor refutes the recorded lead or any sub-sqrt(p) route.

## Standing
[pcc:unverified], single local toy run, no independent replay.
