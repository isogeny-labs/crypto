# RESULT: LEAD-ICB-decomposition-encoding-and-sat

## Outcome
- Status: ran to completion. Exit code 0, wall 16 s (run.status: exit=0 wall=16). run.stderr empty.
- run.py verdict field: consistent_with_prediction.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): expected slopes about m-1 (direct) and about ceil(m/2) (MITM), both above m/2-1 (0.5 for m=3, 1.0 for m=4). Falsified if, on subspace_A, the smaller solver slope is below m/2-1-0.25 for any m>=3. Consistent if no such case, the positive control passes and the A/A slope difference is <= 0.3.

Measured slopes of log2(additions) on log2|F| (subspace_A):
- m=2: direct 1.00, MITM 1.00 (5 points)
- m=3: direct 1.78, MITM 1.54 (4 points; l=6 had no UNSAT target)
- m=4: direct 2.43, MITM 1.72 (3 points; only l=2..4 had an UNSAT target)
Best (smaller) slopes: m=3 1.538 vs threshold 0.5; m=4 1.723 vs threshold 1.0. Not falsified.
A/A (subspace_B_AA): m=3 direct 1.75, MITM 1.50; m=4 direct 2.44, MITM 1.73. Max A/A slope difference 0.039 (limit 0.3), passed.
Controls: positive control passed (planted instances found by direct and MITM for m=2,3,4). A/A passed. Null (random_null) slopes are identical to subspace_A to every digit (see Reading).
Verdict: consistent_with_prediction.

## Reading
- Discrepancy with the prediction text: direct m=3 is 1.78 (predicted about 2) and m=4 is 2.43 (predicted about 3). MITM m=4 is 1.72 (predicted about 2). All measured slopes sit below the stated expectations but well above the falsification line. The shortfall is probably the small-l regime, where lower-order terms and small |F| dominate. That is not tested here.
- The null arm is not an independent test. random_null reproduces subspace_A's log2F and log2cost lists exactly. Cost of an exhaustive enumerator with counted additions depends only on |F| and m, not on linear structure. The null therefore confirms nothing beyond the code path being structure-blind, which is true by construction.
- The slopes are counting identities of the enumerators (about |F|^(m-1) for direct, |F|^ceil(m/2) for MITM). The toy measures the enumeration cost it implements. It does not test a SAT/XOR solver, Karabina, Nagao, Groebner, or any solver whose cost could be sub-enumerative. A threshold of m/2-1 was never in reach for these two enumerators.
- Fit sizes are small: 4 points for m=3 and 3 points for m=4, fewer than the 5 the protocol assumed. run.py's verdict does not mention this.
- run.py's verdict matches the numbers (all best slopes exceed the threshold, A/A diff 0.039).

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: for this toy family only, the enumeration solvers' slopes exceed m/2-1, consistent with "encoding levers are constants". It says nothing about real SAT/XOR or algebraic solvers, so the evidence is weak.

## Standing
[pcc:unverified], single local toy run, no independent replay.
