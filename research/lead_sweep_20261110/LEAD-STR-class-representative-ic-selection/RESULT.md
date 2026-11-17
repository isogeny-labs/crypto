# RESULT: LEAD-STR-class-representative-ic-selection

## Outcome
- status: ran; exit code 0; wall 10 s (run.status: exit=0 wall=10). Single run, no retries. run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): S_obs <= null q99 -> consistent_with_prediction; S_obs > q99 -> falsified; inconclusive if positive control not detected.
- Measured: p=2027, orders [2011,2011,2003,2039,2113,1997], K=12, B=126.
  S_obs = 0.28950; null q95 = q99 = 0.28610 (20 reps; q99 is effectively the null max); null mean 0.28308.
  A/A mean 0.28337, range [0.27997, 0.28568]; overlaps the null band.
- Positive control detected (multiples-of-G yield 1.63e-5 vs random 1.36e-4, ratio 0.12 < 0.5).
- Verdict (run.py): falsified. Controls: null ok, positive ok, A/A ok.

## Reading
- S_obs exceeds q99 by 0.0034, about 1.2 percent of S_obs, and the null mean is 0.2831. With only 20 null replicates, q99 is the sample max, so exceeding it is roughly a 1-in-21 event. The A/A range reaches 0.2857, close to the threshold. The "falsified" verdict is real by the preregistered rule but marginal.
- The best-of-K gain is about 0.28-0.29 in all arms, null included. This gain is dominated by size/sampling fluctuation with small FBs (n about 124), not by structure. S_obs is only about 2 percent above the null mean in relative terms.
- Yields are about 1e-4 per pair, so the normalized yield is not near 1; the statistic is noisy.
- The toy tests only x -> u^2 x relabelling of one curve (changes which points count as small), not isogeny transport or Semaev m=3 at 24-32 bits (PROTOCOL s6). It does not test the lead's 4 percent S claim.

## Bearing on the sub-sqrt(p) question
possible signal needs replication: marginal exceedance of a 20-replicate null max, in a toy that cannot measure real index-calculus cost; at best it motivates, not justifies, the ecbench three-arm study. Constant factor only at most.

## Standing
[pcc:unverified], single local toy run, no independent replay.
