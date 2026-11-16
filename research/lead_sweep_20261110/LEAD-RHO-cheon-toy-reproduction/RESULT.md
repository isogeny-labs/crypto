# RESULT: LEAD-RHO-cheon-toy-reproduction

## Outcome
- status: ran, one run only. exit code 0, wall 0 s (run.status: exit=0 wall=0; under 1 s at date resolution).
- RESULT.json parsed as a single object. run.stderr was empty.
- run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent_with_prediction if all cells recover a in 100% of trials,
every ratio is in [0.8,1.25], raw_gain >= 0.7 theory_gain, and controls are valid.
Controls: negative control success must be <5%; positive control 100%; A/A deviation <=10%.

Measured (8 cells, 120 trials each):
- Recovery: 100% in all 8 cells (positive control passed).
- ratio_meas_over_pred ranged 0.966 (22,64) to 1.031 (22,256). All are inside [0.8,1.25].
- raw_gain / theory_gain was at least 0.94 in every cell. Examples: (16,16) 3.64 vs 3.77; (22,1024) 21.5 vs 21.4; (22,64) 8.22 vs 7.76. This is above the 0.7 threshold.
- charged_gain (descriptive): 0.12 to 0.65, so below 1 in all 8 cells.
- A/A relative deviations in mean Cheon cost: 0.021, 0.045, 0.016. All are within 10%, so A/A passed.
- Negative control (divisor not dividing r-1, 2 cells): success 0.05 (6/120) for (16,16) and 0.0167 for (16,64).
  The criterion is strictly <5%. 0.05 is not <0.05, so run.py set neg_control_fails=false and the verdict is inconclusive.

Verdict: inconclusive, per the preregistered rule. Controls: null/negative FAILED on the boundary,
positive passed, A/A passed.

## Reading
- The toy reproduces the cost model well. Every cost ratio is within 4% of 1.5(sqrt(r/d)+sqrt d), and raw gain tracks theory within about 6%.
- The inconclusive verdict comes from one cell of the negative control sitting exactly at the 5% threshold. The cause looks like chance:
  with an invalid divisor, a wrong stage structure can still hit a by coincidence at a rate of order 1/d-scale or similar, and 6/120 is within sampling noise of 5%.
  The strict inequality was frozen in advance, so I did not reinterpret it. If the criterion had been <=5%, every check would pass and
  the verdict would have been consistent_with_prediction, but that is not the preregistered result.
- No discrepancy between the verdict field and the numbers: the checks dict matches what I computed from the cells.
- Limits of the toy (PROTOCOL section 6): the group is Z_r with free scalar multiplication, the supplied-input CRT composition was not tested,
  and the 1.5 log2 r charge is a modelling constant. charged_gain<1 follows from that accounting by construction, so the toy does not independently test it.
- The 56-bit / 65536 experiment named in the lead was not run here.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: it re-derives the known Cheon cost bound in a generic model. The gain exists only with free
scalar multiplication and a supplied a^d G, and it vanishes (charged_gain<1) once those costs are priced. There is no new sub-sqrt(p) signal.

## Standing
[pcc:unverified], single local toy run, no independent replay.
