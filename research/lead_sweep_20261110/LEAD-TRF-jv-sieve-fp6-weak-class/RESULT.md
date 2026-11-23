# RESULT: LEAD-TRF-jv-sieve-fp6-weak-class

## Outcome
- status: ran (single run of run.py, unmodified). exit code 0, wall 15 s (run.status: exit=0 wall=15; run.py elapsed_s 15.02). stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): rate of 3-point decomposition within 3*sigma0 + 0.02 of 1/6 at every p; falsified if any p has dev > 3*sigma0 + 0.06. sigma0 = 0.008333 (n=2000), so consistent bound 0.045, falsified bound 0.085.

| p | q | FB size | rate | dev from 1/6 | A/A z | null rate (x in F_p, FB size) | planted |
|---|---|---|---|---|---|---|---|
| 7 | 49 | 60 | 0.2575 | 0.0908 | 0.66 | 0.0 (4) | 20/20 |
| 11 | 121 | 126 | 0.1635 | 0.0032 | -0.54 | 0.0 (6) | 20/20 |
| 13 | 169 | 162 | 0.1405 | 0.0262 | -2.38 | 0.0 (18) | 20/20 |

Verdict: falsified (run.py field; matches my recomputation: p=7 dev 0.0908 > 0.085). p=11 and p=13 are individually in the consistent band.
Controls: null passed (0 <= 0.02), positive passed (20/20 at every p), A/A passed (max |z| 2.38 <= 3.5). So the falsification is not voided by the control rule.

## Reading
- The falsification comes entirely from p=7 (q=49), where the rate is 0.2575, well above 1/6. The model's own caveat is O(1/q) corrections; at q=49 these are large (1/q about 0.02, and FB size 60 vs group about 2^17 makes sums of repeated/ordered points, P+(-P) cancellations and small-q effects non-negligible). The preregistered tolerance did not allow for this, so the result is real against the protocol but most plausibly a finite-size effect, not a refutation of the 1/n! heuristic.
- Trend: rate 0.2575 -> 0.1635 -> 0.1405 with q = 49, 121, 169. p=13 undershoots 1/6 (dev 0.026, 3.1 sigma0), and the p=13 A/A z of -2.38 is the largest, still within the 3.5 limit. Non-monotone approach to 1/6; single curve per p and one seed, so curve-to-curve variation (group order, 2-torsion) is not separated from the finite-q effect.
- The null FB sizes (4, 6, 18) are small and the null rate is 0 trivially, so the null control has little power to detect a detector bug. Not a discrepancy with run.py, but a weak control.
- The toy tests only the decomposition-rate ingredient on generic curves, not the genus-3 cover, weak class, or isogeny walk (PROTOCOL s6).
- No discrepancy between run.py's verdict field and the numbers.

## Bearing on the sub-sqrt(p) question
none: this is a textbook decomposition-rate sanity check at toy q on generic curves; the protocol-level falsification at p=7 reflects finite-size error in the tolerance, so the tolerance "3 sigma0 + 0.02" cannot be frozen as is for q below about 100, but nothing here bears on sub-sqrt cost.

## Standing
[pcc:unverified], single local toy run, no independent replay.
