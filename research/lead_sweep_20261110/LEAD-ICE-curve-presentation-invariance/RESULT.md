# RESULT: LEAD-ICE-curve-presentation-invariance

## Outcome
- status: ran (single run, no retries); exit code 0; wall 10 s (run.py self-reports 10.1 s; protocol estimated 2-6 min).
- run.stderr empty/uneventful. run.py verdict field: consistent_with_prediction.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): "consistent_with_prediction (no presentation effect)". Falsified only if the xwindow arm has perm_p < 0.01 and ICC > 0.05 at any prime, with positive control detected and null arm clean.

Measured (k=8 curves x R=6 replicates, 2000 permutations):
| p | N | fb | xwindow ICC (perm_p) | null ICC (perm_p) | positive ICC (perm_p) |
|---|---|----|----|----|----|
| 8209 | 8140 | 135 | 0.0727 (0.203) | 0.1474 (0.077) | 0.9617 (0.0005) |
| 32771 | 32784 | 271 | 0.1339 (0.078) | -0.0388 (0.598) | 0.9889 (0.0005) |

Verdict: consistent_with_prediction. Controls: positive detected at both primes (perm_p 0.0005 < 0.01); null clean at both (perm_p 0.077, 0.598 > 0.01). Controls passed. A/A noise floor is the within-curve replicate variance in the ICC denominator.
Falsification criterion not met: xwindow perm_p is 0.20 and 0.078, both above 0.01, although the point-estimate ICC exceeds 0.05 at both primes.

## Reading
- The toy tests only a set-level proxy (sumset coverage of an x-window factor base across same-order curves), not solving degree, Semaev polynomials, Velu transport or special j. It does not test the lead's actual claim.
- The verdict field matches the numbers under the preregistered rule (the p-value gate decides). Caveat: point ICCs of 0.07 and 0.13 are above 0.05 and p = 0.078 at p=32771 is borderline. The null arm also produced ICC 0.147 (p 0.077) at the smaller prime, so ICC noise at k=8, R=6 is of the same size as the xwindow estimates. Power is low; the null result is weak, not strong evidence of absence.
- The positive control (15% larger base) is a huge effect (ICC ~0.96-0.99), so it shows the pipeline works but not that it can see subtle effects.
- Run was far faster than estimated; no crash or discrepancy.

## Bearing on the sub-sqrt(p) question
none: a low-power set-count proxy at toy sizes shows no detectable presentation effect and says nothing about algebraic decomposition difficulty; the lead stays open.

## Standing
[pcc:unverified], single local toy run, no independent replay.
