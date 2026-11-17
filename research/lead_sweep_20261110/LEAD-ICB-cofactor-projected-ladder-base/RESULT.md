# RESULT: LEAD-ICB-cofactor-projected-ladder-base

## Outcome
- status: ran (single run, no retries); exit code 0; wall 0 s (run.status: exit=0 wall=0, i.e. under 1 s at date granularity).
- run.stderr empty. RESULT.json is one JSON object; verdict field = consistent_with_prediction.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): pooled R_h = reps(proj)/reps(full) equals h for h in {1,2,4}; consistent if
|R_h - h| <= 3 * R_h * relSE for every h and the A/A check passes; counts >= 30 in every arm.

Measured (pooled over 2 primes x 2 curves, 300 targets each):
- h=1: R=1.000, relSE 0.0269 (counts 2767 / 2767; A/A ratio 1.041, SE 0.0272).
- h=2: R=1.918, relSE 0.0335, |R-2|=0.082 <= 0.193 (counts full 1358, proj 2605; A/A 1.030, SE 0.0279).
- h=4: R=4.218, relSE 0.0452, |R-4|=0.218 <= 0.572 (counts full 606, proj 2556; A/A 0.996, SE 0.0279).
- All arm counts >> 30. A/A ratios all within 3 SE of 1 (largest deviation 0.041 vs 3 SE ~0.082).
Verdict: consistent_with_prediction. Controls: null (h=1) passed, positive (h=2,4 detect R=h) passed, A/A passed.

## Reading
- The toy tests only the counting identity: projecting a random base by [h] into the order-n subgroup raises 2-term relation hits on subgroup targets by h at fixed base size. The numbers agree with that identity, and the h=2 and h=4 ratios are within about 0.4 SE of h (h=2) and 0.5 SE (h=4).
- Caveat on the null: at h=1, proj and full are the same point set, so reps_proj == reps_full exactly (R=1 with zero real variance); the h=1 control is vacuous for the ratio, though the independent A/A arm at h=1 does show noise-level agreement (1.041).
- Per-curve noise is visible (h=2, p=6007: full 275 vs 387 on two curves) but pooled values are stable.
- The projected base has the same size B but is a fixed-size comparison; projection cost (one scalar mult per base point) and the loss of cofactor/coset information for h>1 are not counted, so the h-fold gain is not an accounting-neutral speedup. No verdict-field discrepancy found.

## Bearing on the sub-sqrt(p) question
none: this confirms a counting identity on prime-field toys and says nothing about binary Jacobians, Weil descent, linear algebra, solving the DLP from projected relations, or the gap to matched rho.

## Standing
[pcc:unverified], single local toy run, no independent replay.
