# RESULT: LEAD-RHO-grumpy-store-all-schedules

## Outcome
- Status: ran (single run of run.py, unmodified). Exit code 0. Wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- RESULT.json is one JSON object; verdict field: consistent_with_prediction; smoke=false.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): strict ordering single_list < bsgs_opt < two_list at every l, and |grumpy - two_list| <= 5% of two_list. Positive control: single_list within 4 SE + 3% of sqrt(pi/2)=1.2533. A/A within 4 combined SE.

Measured mean cost/sqrt(l), arm A:
| l | single | bsgs | two_list | grumpy | grumpy vs two |
|---|---|---|---|---|---|
| 4093 | 1.279 | 1.432 | 1.809 | 1.797 | -0.7% |
| 65521 | 1.247 | 1.414 | 1.738 | 1.766 | +1.6% |
| 1048573 | 1.285 | 1.384 | 1.783 | 1.829 | +2.6% |
| 4194301 | 1.305 | 1.356 | 1.791 | 1.781 | -0.6% |

Verdict: consistent_with_prediction. Controls: positive control passed (true), A/A consistent (true). No separate null arm (closed-form theory serves as null, per protocol).

## Reading
- The numbers support run.py's verdict. Ordering holds in the means at all four sizes; grumpy is within 2.6% of two_list everywhere.
- Caveat: at l=4194301 (120 trials) single (1.305, SE 0.064) vs bsgs (1.356, SE 0.035) differ by under 1 SE, so strict ordering there is a point-estimate result, not a statistically resolved one. The A/A arm at that size shows grumpy 1.674 vs two_list 1.765 (-5.1%), so the 5% margin is noise-limited there.
- two_list ranges 1.74-1.84 across arms, near sqrt(pi)=1.7725; bsgs near sqrt(2) at small l.
- The toy's "grumpy" is two_list with a deterministic baby sequence; in an idealised random-point model it is expected to share the two-list constant, so the 5% test is weakly informative about the real lead. My reading of "grumpy giants" is an assumption (protocol s6).
- It does not run EXP-GRUMPY-eb8675, tiered/compiled walks, covered-set counters, or real curves, and prices no native/memory cost (no s5a band).

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: in the idealised Z/l model, store-all schedules reproduce the known constants (1.25 / 1.41 / 1.77), with no schedule below the generic sqrt(pi/2) barrier; this is a constant-factor, not exponent, statement.

## Standing
[pcc:unverified], single local toy run, no independent replay.
