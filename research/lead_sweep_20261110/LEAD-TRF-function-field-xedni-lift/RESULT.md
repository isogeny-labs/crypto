# RESULT: LEAD-TRF-function-field-xedni-lift

## Outcome
- status: ran (single run, no retries)
- wall seconds: 5
- exit code: 0
- run.stderr: empty

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): alpha_hat in [2.5, 3.5] for both prescribed targets (mean used). Falsified if mean alpha_hat < 0.5. Inconclusive if alpha_hat is in [0.5, 2.5) or above 3.5, or if any control fails.

Measured (p in {5, 7, 11, 13}, exhaustive enumeration of p^6 tuples):
- main_t1 alpha_hat = 3.0000 (2.999999999999999)
- main_t2 alpha_hat = 3.0000 (2.999999999999999)
- P at p = 5, 7, 11, 13 for both targets: 0.008, 0.0029155, 0.00075131, 0.00045517. These equal p^-3 exactly, matching 1/125, 1/343, 1/1331 and 1/2197 up to the y0 != 0 rule. The target values chosen by the seed apparently have y0 != 0.
- planted control alpha_hat = 1.0 (P = 1/5, 1/7, 1/11, 1/13)

Verdict: consistent_with_prediction.

Controls:
- Null (identity check at t=2): 0 violations. Passed.
- Positive (planted, must be 1 +- 0.2): 1.0. Passed.
- A/A (|difference| <= 0.3): gap 0.0. Passed.

## Reading
- The toy tests only what the protocol says it tests. For the linear (a, x, y) section shape, a prescribed fibre datum at t=0 costs three F_p coordinates, so the exponent is 3. The run confirms this exactly.
- The run.py verdict field matches the numbers. I found no discrepancy.
- The two main targets and the A/A gap are identical to machine precision. This is expected, because the count is symmetric in the target. It means the A/A arm cannot detect noise here: the counts are exact, with no sampling variance. It is a consistency check only.
- Nothing surprising. The result is near-tautological, as PROTOCOL.md section 6 states.
- It does not touch the atlas's exact P_lift formula (alpha = 3, 596M slots), higher-degree sections, or dimension-surplus families. These are the real reopen routes.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction. It confirms alpha = 3 for this shape, far from the alpha < 1/2 reopen condition. This is a negative for this shape only and does not close the lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
