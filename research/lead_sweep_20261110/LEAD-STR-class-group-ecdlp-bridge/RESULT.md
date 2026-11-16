# RESULT: LEAD-STR-class-group-ecdlp-bridge

## Outcome
- Status: ran (single run, no retries). Exit code 0. Wall 3 s (run.status: exit=0 wall=3). run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): lower 95% CI of R = median(cost_B/sqrtN)/median(cost_A/sqrtN) >= 0.95 (expected >= 1), slope_B in [0.4,0.6]. Falsified if upper CI < 0.95. A/A must contain 1 or lie in [0.9,1.1]; positive control (index-8 leak) upper CI < 0.6.
- Measured:
  - R_B/A = 1.511, CI [1.408, 1.624]. Lower bound >= 0.95, so the prediction holds.
  - slope_B = 0.4954 (slope_A = 0.4967), inside [0.4,0.6].
  - A/A ratio = 0.981, CI [0.916, 1.040]. It contains 1, so aa_ok = true.
  - Positive control R_C/A = 0.349, CI [0.325, 0.374]. Upper bound < 0.6, so it was detected.
- Verdict (run.py): consistent_with_prediction. Controls passed: null (arm A), positive (arm C), A/A.

## Reading
- The toy checks only that a free exponent-preserving transport oracle does not lower collision-walk cost. Here it adds about 51% cost, since action calls are charged at 1 each. That matches "pure overhead".
- The verdict field agrees with the numbers. No discrepancies.
- Median cost per sqrtN is about 1.2 for A. Each arm has the same exponent, 1/2.
- Limits (PROTOCOL s6): the model is a relabelling by construction. It has no real isogenies or class-group structure, and it tests one algorithm family. The run does not prove the simulation lemma.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction (at toy level only): the oracle with identity transport gives no gain, and exponent 1/2 is preserved. This is expected by construction.

## Standing
[pcc:unverified], single local toy run, no independent replay.
