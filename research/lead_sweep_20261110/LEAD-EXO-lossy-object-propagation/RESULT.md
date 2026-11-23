# RESULT: LEAD-EXO-lossy-object-propagation

## Outcome
- status: ran (single run, no retries). exit code 0, wall 18 s (run.status: exit=0 wall=18).
- RESULT.json is one JSON object; run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "max z <= 4.0 (consistent). Falsified if max z > 4.0 while controls pass."
- Measured: max z = 16.91 (16-bit, p=39119, top3x). 12 curve/projection cells total 36.
- Cells with z > 4: all 9 cells at 16 bits (z 4.9 to 16.9), 1 cell at 18 bits (p=170363, top2x_ypar, z=4.73). None at 20 or 22 bits (z from -2.1 to 2.1).
- Controls: positive (Z_N top3 doubling) z=11342, ok. Negative (independent 8-state pairs) z=2.13, ok (<3.5). A/A mean half-split diff 2.32 null-sd, ok (<4.0). All controls passed.
- Verdict: falsified (run.py "verdict" = falsified), by the preregistered rule.

## Reading
- The numbers support run.py's verdict field as a literal application of the rule. The effect is not uniform: it is strong and consistent in sign only at 16 bits, borderline at 18 bits, and absent at 20 and 22 bits, where z scatters around 0 (including negative values) like a null.
- The size trend suggests a finite-size / small-p artifact rather than information transmitted at cryptographic sizes. At p about 2^16 the 40000 sampled points are a large fraction of the roughly 2^17 curve points, so x values repeat and the sample is far from i.i.d. over a large space. Q and 2Q are also a deterministic function of the same x, and the top-bit and low-bit maps are coarse on a small field. This explanation is a hypothesis; the run did not test it (no extra sizes or sample sizes, by rule).
- A real one-step dependence of this kind is plausible in principle (doubling is a fixed degree-4 rational map on x), but it fades as p grows, which is the regime the lead cares about.
- The toy tests only one doubling step and three fixed 8-state projections. It does not test the lead's actual disjoint projection panel (definition unavailable), multi-step belief propagation, or rigidity. Plug-in MI is biased upward at small samples; the permutation null mostly absorbs that, but not repeated-row structure.
- Note: the falsification criterion as preregistered would trigger on p of 16 bits alone; extending to chains on that basis alone would be unjustified given the decay.

## Bearing on the sub-sqrt(p) question
none: the signal is confined to tiny p (<= about 2^18), vanishes at 2^20 to 2^22, and one-step lossy transmission at toy sizes says nothing about a sub-sqrt(p) algorithm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
