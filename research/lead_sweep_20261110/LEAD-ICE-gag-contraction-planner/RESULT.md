# RESULT: LEAD-ICE-gag-contraction-planner

## Outcome
- Status: ran. Exit code 0. Wall 0 s (under 1 s). run.stderr empty. One run, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): "mono_over_chain > 1 for all configs and tree_over_chain == 1.0 exactly. Falsified if any mono_over_chain < 1 or tree_over_chain != 1."

Measured (m,n): mono/chain; tree/chain
- (3,3): 2.59e7; 1.0 (D mono 100, chain 23)
- (3,2): 3.45e5; 1.0 (D 107 vs 21)
- (4,3): 4.82e19; 1.0 (D 360 vs 27)
- (3,4): 1.61e9; 1.0 (D 104 vs 26)
- (4,2): 2.16e14; 1.0 (D 382 vs 23)

Verdict (run.py): consistent_with_prediction. All D below cap 400.
Controls: A/A chain vs chain = 1.0 exactly (pass). Tree == chain, topology-blind null (pass). Positive control (dup) ratios 1e-4 to 6e-3, not 1 (pass; detector sees change).

## Reading
- The toy tests only the narrow proxy question. It reproduces the prediction: the counted-Macaulay proxy ranks monolithic far more expensive than chain (opposite of the recorded msolve 0.110x) and cannot separate chain from tree.
- The tree/chain ratio of 1 is by construction (identical equation and variable counts), so it is a tautology of the proxy, not an observation.
- The dup positive control gives ratio < 1 (doubling equations lowers first-D cells), so it shows sensitivity but the direction is odd. This is a quirk of the first rows>=cols rule, not a discrepancy with the verdict field.
- No discrepancy between run.py's verdict and the numbers. Mono D values (100 to 382) are near the cap of 400, so the mono magnitudes are crude.

## Bearing on the sub-sqrt(p) question
none: it shows only that this crude proxy is not a usable planner. It says nothing about real F4 cost or ECDLP cost.

## Standing
[pcc:unverified], single local toy run, no independent replay.
