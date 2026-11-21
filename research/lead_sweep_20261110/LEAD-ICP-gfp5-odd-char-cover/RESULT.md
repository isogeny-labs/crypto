# RESULT: LEAD-ICP-gfp5-odd-char-cover

## Outcome
- status: ran (single run, no retries). exit code 0, wall 21 s (run.py elapsed_s 20.4). Budget not hit; no cells cut.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL s3): |R-1|<0.1 -> consistent_with_prediction; falsified if R>1.25 with a valid positive control; inconclusive if R_ap>0.9 or cells cut.
- Measured: R (subfield FB / random FB, summed over 6 cells) = 1.00201. |R-1| = 0.002.
- Per cell, subfield vs mean of 3 random draws: (61,3,3) seeds 101/102/103: 22516/22231, 24884/24463, 21930/22227; (499,2,2) seeds 201/202/203: 101358/101352, 101787/101542, 93406/93335. All within about 3%.
- A/A relative range among the 3 random draws: 0.025, 0.033, 0.022, 0.009, 0.005, 0.0009. The noise floor is up to 3.3% per cell, below the 10% window, so the window is meaningful.
- Poisson expectation #E(1-e^-lambda) matches observed distinct counts to under 1% in most cells (e.g. 22230 vs 22231, 101074 vs 101358).
- Positive control: R_ap = 0.0095 (structured {kP} FB), far below 0.9, so the detector sees collapse. Null control (random x-classes, same code path) behaves as Poisson. Controls passed.
- Verdict (run.py): consistent_with_prediction.
- n=5 descriptive cell (p=7, m=5, 4 classes): 0 sums, empty; no information.

## Reading
- The toy tests the coverage proxy only: a subfield-x factor base covers distinct sums as a random FB of equal class count does, at n=2,3 and lambda 0.1-0.5. It matches the hypothesis.
- Discrepancy: PROTOCOL says R_ap was 0.34 in smoke; the full run gives 0.0095. Still passes the control (<=0.9), but the full-run structured FB collapses much harder, which is consistent with a larger m-fold collision rate at full size versus the smoke size. Not a verdict problem.
- Caveat: the n=5 cell ran with only 4 classes and produced no sums, so the lead's actual n=5 case was not tested at all. Random curves, not EcGFp5/EcMasFp5. Ordering of subfield versus random FB in individual cells is mixed (sign of difference varies), as expected under no effect.
- Nothing surprising. No genus or Diem bound was computed.

## Bearing on the sub-sqrt(p) question
- none: the toy shows no coverage advantage from the subfield structure at toy n=2,3 and does not touch the cover genus or Diem bound that the lead asks about.

## Standing
[pcc:unverified], single local toy run, no independent replay.
