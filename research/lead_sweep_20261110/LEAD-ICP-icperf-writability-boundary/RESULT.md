# RESULT: LEAD-ICP-icperf-writability-boundary

## Outcome
- status: ran (single run, no retries). exit code 0, wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): slope c of log2(ANF terms) vs n per m; consistent_with_prediction if min over m of c >= 0.6; falsified if max over m of c < 0.5; otherwise inconclusive. Rho budget slope 0.5.
Measured: fits m=2: c = 0.390 (n=6..14, 27 rows); m=3: c = 1.003 (n=6..11, 18 rows).
- min c = 0.390 < 0.6, so not consistent. max c = 1.003 >= 0.5, so not falsified. Verdict: inconclusive (matches run.py).
- Controls: positive (dense cap) passed: every row's log2 terms is below the dense cap (for example m=3 n=6 is 7.0-7.3 against a cap of 8.58, m=2 n=14 is 8.8 against 17.8). Null (random coefficients, same support, n=10, m=2): 7.500 against structured 7.32-7.58, a difference within about 0.1 bit, so the count reflects support rather than algebraic cancellation (reported, not decisive). A/A (seeds 101-103, n=10, m=2): log2 terms 7.443, 7.644, 7.459, spread about 0.20 bit; the structured seed spread at the same size is 7.32-7.58 (0.26 bit). controls_ok: yes.

## Reading
- The two m values split: m=2 grows slowly (0.39, under the rho slope 0.5 and well under 0.6), m=3 grows at about 1.0.
- The m=3 slope is mostly a staircase artifact. The number of Boolean variables is m*ceil(n/m), and the monomial support jumps at those steps: n=6 has 45 monomials, n=7-9 has about 253, n=10-11 has 1061. Within a plateau (n=7,8,9 or n=10,11) log2 terms rises only about 0.1-0.2 per unit n, while the jumps are about 2.5 bits. A straight-line fit over n=6..11 with only 3 plateaus measures the step structure, not an asymptotic exponent. The m=2 fit has the same staircase feature at smaller amplitude.
- Observed terms sit far below the dense cap (3-9 bits), so the system is not saturated here, which means the toy is not simply reading the cap. But the sizes are tiny (n<=14, nvars<=14 for m=2), so the monomial count is limited by the number of variables, not by n.
- The run.py verdict agrees with the numbers. No discrepancy found. The toy tests only a naive expansion count at m=2,3; it does not test the lead's m>=4 or n>=409 claims, nor the +40.8 / -4.9 bit figures.

## Bearing on the sub-sqrt(p) question
none: the two slopes straddle the thresholds and the m=3 value is a step artifact of n<=11, so the toy neither confirms nor closes the recorded writability obstruction.

## Standing
[pcc:unverified], single local toy run, no independent replay.
