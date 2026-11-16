# RESULT: LEAD-ICE-relation-graph-rank-large-prime

## Outcome
- Status: ran to completion. Exit code 0, wall 16 s (run.status), run.py wall_s 15.63.
- 20/20 seeds completed at both sizes. run.stderr empty. One run only, no changes to run.py.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): R2 upper CI <= 0.8 and |Rn - 1| <= 0.05 (or CI of Rn contains 1) at both sizes. Falsified if R2 lower CI >= 0.95, or if the R2 gain holds and Rn is clearly different from 1.

| bits | p | B | L | FB | R1 (1LP/full) [CI] | R2 (2LP/full) [CI] | Rn (real/null) [CI] | A/A | size verdict |
|---|---|---|---|---|---|---|---|---|---|
| 18 | 262147 | 34 | 6740 | 11 | 0.487 [0.400, 0.764] | 0.389 [0.336, 0.517] | 1.068 [0.831, 1.386] | 0.997 | consistent |
| 20 | 1048583 | 49 | 16807 | 15 | 0.546 [0.537, 0.670] | 0.484 [0.463, 0.541] | 1.120 [1.021, 1.352] | 1.057 | falsified |

- R2 upper CI <= 0.8 at both sizes (gain holds). Positive control (R1 upper CI < 1) passed at both sizes.
- Null arm: at 18 bits the Rn CI contains 1, so the prediction holds there. At 20 bits Rn = 1.12 with CI [1.02, 1.35], which excludes 1 and exceeds the 5% tolerance, so the "gain beyond null" falsifier fires.
- A/A ratios 0.997 and 1.057, reported and not gating, near 1.
- Controls: positive passed, A/A near 1, null arm ran. controls_ok = true.
- Verdict (run.py overall field): falsified (driven by the 20-bit size only).

## Reading
- run.py's verdict matches the numbers: the 2LP gain is large (about 0.4 to 0.5 of the full-smooth trial count) and the 20-bit real/null ratio is above 1. The 18-bit size is consistent with the prediction, so the two sizes disagree.
- The sign of Rn needs care. Rn > 1 means the real 2LP arm needed MORE trials than the configuration-model null, so real large-prime coincidences are slightly worse than the null. The null relabelling does not give less enrichment than the real data. Whether that counts as "structure beyond null" in the lead's sense, which hoped for extra gain, is doubtful. The preregistered falsifier is two-sided ("clearly different from 1"), so the verdict is formally correct.
- Rn at 20 bits has a lower CI bound of only 1.02, with 20 seeds and 400 resamples. The 18-bit CI is wide (0.83 to 1.39). Evidence of a real deviation is weak, and the two sizes do not agree.
- The toy is tiny: |FB| is 11 and 15 and median full-smooth-only T0 is about 370 to 490 trials. Rank |FB| is reached after a handful of relations, so small-number effects (degree statistics of a very small graph) dominate. This is a mechanism analogue (integer smoothness in F_p*) and not the Semaev/EC 2LP summation multigraph. It does not test whole-solve cost, linear algebra, or the 2.31x penalty.
- Nothing else surprising. The 2LP gain over full-smooth-only is large and the 1LP positive control behaves as expected.

## Bearing on the sub-sqrt(p) question
constant factor only: 2LP enrichment cuts relation-stage trials by about 2x at toy size, and the small deviation from the null is a ratio near 1.1 in the unfavourable direction. It gives no exponent evidence and says nothing about whole-solve cost.

## Standing
[pcc:unverified], single local toy run, no independent replay.
