# RESULT: LEAD-TRF-cm-canonical-lift-heights

## Outcome
- Status: ran to completion. Exit code 0, wall 5 s (run.status: exit=0 wall=5). run.stderr empty.
- RESULT.json holds one JSON object with 17 prime rows (6+5+4+2 in the four size bands).

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): "R <= 1.25 and Stouffer z < 4 (no enrichment). Falsified if z > 4 AND R > 1.25. Inconclusive if a control fails."
- Measured: pooled R = 0.9888. Stouffer z (enrichment) = -1.586. Both inside the no-enrichment region.
- Controls:
  - Null (random point set of equal size): per-prime z_null all within about +/-2.04. Not pooled by run.py.
  - Positive (Q drawn from S): hit_pos = 1.0 on all 17 primes. Pass.
  - A/A (two independent Q streams): Stouffer z_AA = -0.20. Pass (|z| <= 4). The largest single-prime |z_AA| is 2.69 (p=4153), unremarkable among 17.
- Verdict: consistent_with_prediction. Controls passed.

## Reading
- run.py's verdict field matches the numbers: R <= 1.25, z < 4, controls OK.
- The CM-set hit rate tracks |S|/N within binomial noise. The point estimate is slightly below 1 (R = 0.989, z = -1.59), which is consistent with noise.
- Discrepancy with the protocol text: the target was |S| ~ 0.1 N, but realised |S|/N is about 0.05 (for example 241/4161). The set has many collisions, or the 6(2k+1)^2 estimate over-counts. This lowers power: with 20000 trials per prime, hits per prime are about 1000. It does not affect the validity of the test against the stated thresholds.
- The null arm was not pooled into a summary z, so there is no single null statistic. Per-prime null z values look fine.
- As PROTOCOL section 6 says, the test is close to a tautology. S is the reduction of a subgroup combination, and E(F_p) is hit uniformly. It checks only that mu_3 orbits create no correlated structure. It does not test canonical heights, Serre-Tate lifts, xedni, or the twelve L9 mechanisms.
- Nothing surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy size (N up to 2^18) coverage of the reduced small-height CM set equals |S|/N, which agrees with the recorded bound Pr[Q in red(S)] <= |S|/N. It is a weak confirmation, since the test is nearly tautological and covers one curve of rank 2.

## Standing
[pcc:unverified], single local toy run, no independent replay.
