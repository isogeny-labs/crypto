# RESULT: LEAD-ICB-cofactor-torsion-quotients

## Outcome
- status: ran (single run, no retries). exit code 0. wall 20 s (run.status). run.stderr empty.
- run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): for every m, |mean D| + 2SE <= 0.03 (null effect), D = g(structured) - g(random).
Falsified if mean D - 2SE > +0.03 at any m, provided controls pass.

| m | mean D | 2SE | |D|+2SE | D-2SE |
|---|--------|-----|---------|-------|
| 12 | +0.0180 | 0.0133 | 0.0314 (> 0.03) | +0.0047 |
| 14 | +0.0047 | 0.0059 | 0.0106 | -0.0012 |
| 16 | +0.0079 | 0.0042 | 0.0121 | +0.0037 |
| 18 | -0.0006 | 0.0015 | 0.0021 | -0.0021 |

Controls (n=30 per m):
- Positive control (random - T-closed > 0.1 at every m): 0.429, 0.437, 0.440, 0.444 with CI-adjusted margins well above 0.1. PASS.
- A/A (|mean diff| <= 2SE + 0.01): m12 0.0152 vs 0.0306; m14 0.0014 vs 0.0232; m16 0.0127 vs 0.0192; m18 0.0010 vs 0.0141. PASS.
- Null control (random negation-closed FB) is the baseline of D by construction. Fine.

Verdict: inconclusive. The null prediction misses at m=12 only (0.0314 vs 0.03, by 0.0014), and no m is falsified (max D-2SE is +0.0047, far below +0.03). My recomputation from the numbers agrees with run.py's verdict.

## Reading
- The toy tests only the narrow sub-claim: 2-sum decompositions, x-subspace FB, density about 0.11-0.12, exact densities, 2-torsion translation quotient. It does not test the excerpt's actual conditions (large smooth cofactor, non-homomorphic k-tree filters); PROTOCOL s6 says so.
- The "inconclusive" label is a strict-threshold miss at the smallest size (m=12, 30 samples, wide CI), not a signal. D shrinks toward 0 as m grows (0.018, 0.005, 0.008, -0.001) and the CI narrows; m=18 is tightly null. Point estimates are all small and positive at m<=16 (<= 0.018, below the 0.03 tolerance).
- Surprising or notable: g_struct is about 0.94-0.95, i.e. S+T overlaps S little (the quotient nearly doubles labelled yield for two tests); this holds for random FBs as well since D is about 0. The A/A means at m=12 and 16 are also about +0.013-0.015, similar in size to D, so the m=12 D is within the noise floor of the instrument.
- Quoted run.py "tolerance" 0.03 was applied as preregistered; no discrepancy between the verdict field and the numbers.

## Bearing on the sub-sqrt(p) question
none (constant factor only at best is not even indicated): a toy-size null for the x-subspace FB; the quotient gives no structured-vs-random gain, and the lead's real conditions were untested.

## Standing
[pcc:unverified], single local toy run, no independent replay.
