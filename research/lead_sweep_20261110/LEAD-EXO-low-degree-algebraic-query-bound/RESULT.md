# RESULT: LEAD-EXO-low-degree-algebraic-query-bound

## Outcome
- status: ran (single run, no retries). exit code 0, wall 14 s (run.status: exit=0 wall=14; run.py secs 14.1).
- run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): "zero Bezout violations in the univariate check (zeros <= 2D); max R <= 1.5 in all cells; null arm max R <= 1.0; mean R at largest N <= 1.5*mean R at smallest N + 0.5. Falsified if any Bezout violation, or any R > 4, or R grows with N beyond the above. Otherwise inconclusive."

Measured (3 sizes, N = 3191, 14243, 38903; D = 2..5; 8 reps per cell):
- Bezout violations: 0. Univariate max zeros/(2D) = 0.6 to 0.8 (<= 1).
- max fit R = 1.6 (N=14243, D=2) -> exceeds the 1.5 bound. fit_AA max was 1.8 (N=3191, D=2), also above 1.5. Both are at D=2, where M-1 = 5 and one extra zero moves R by 0.2.
- null max R = 0.6 (<= 1.0, passes).
- mean fit R, smallest N = 1.124, largest N = 1.087 (no growth; passes 1.5*1.124+0.5).
- A/A max |mean difference| = 0.10.
- No R > 4.
Verdict (run.py): inconclusive. Controls: null passed (0.6 <= 1.0); positive control passed (fit arm R >= 1 on average, forced zeros visible; Bezout arm within 2D); A/A noise floor 0.10, small.

## Reading
- The run.py verdict "inconclusive" matches the numbers: no falsifier triggered, but the 1.5 cap was missed by one cell (1.6), so the "consistent" outcome was not earned. Note the protocol's own decision rule only defines consistent/falsified/inconclusive; the miss is a small-degree (D=2) integer effect: R takes steps of 0.2 there, and the fit_AA arm shows the same excess (1.8), so it is sampling noise, not an N-dependent effect.
- Mean R is about 1.03 to 1.25 and decreases with D, flat in N. That is, a handful of extra vanishing k beyond the M-1 interpolation points appears, not growing with N.
- Labelling: cells report log2N = 12, 14, 16 (N from the sampled prime-order curves) whereas the protocol names log2 p in {11,13,15}; this is a labelling difference only.
- Does the toy test the lead's claim? Only weakly: it tests vanishing counts of an interpolated bivariate test on two x-coordinates, not adaptive tuples, searched (non-interpolating) tests, or any query lower bound in a generic/algebraic group model. N <= 2^15.3.

## Bearing on the sub-sqrt(p) question
none: the toy only shows vanishing counts stay near the interpolation count at N <= 39k; it says nothing about a query lower bound or a sub-sqrt(p) algorithm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
