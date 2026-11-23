# RESULT: LEAD-ICP-construction-normal-form

## Outcome
- status: ran; exit code 0; wall 0 s by run.status (under 1 s, integer-second clock).
- Single run of run.py, no retries. stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): R = var_iso / var_randomFB in [0.5, 2] is consistent; falsified if R < 0.1 or R > 10. A/A ratio in [0.5, 2]; transported-FB variance exactly 0; 25% larger FB raises mean coverage by more than 3 SE.

Measured:
- R = 1.273 (var_iso 0.001152, var_random 0.000905); mean coverage 0.0876 (interval) vs 0.0860 (random).
- A/A ratio = 0.929 (in band).
- Positive control (a): transported-FB variance zero = true.
- Positive control (b): size shift 0.03825, SE 0.00325, about 11.8 SE (> 3).
- Dedup: naive distinct (a,b) = 40 per curve for all 6 curves vs 1 normal-form class.
Verdict: consistent_with_prediction. Null, positive and A/A controls all passed.

## Reading
- run.py's verdict field matches the numbers (R in band, all controls pass); I found no discrepancy.
- The toy tests only a stand-in: an x-interval FB is not isomorphism-invariant and its coverage dispersion across isomorphic copies is about that of random FBs. It does not touch H-PFDR-6189d0 or any PFDR/PATH proposal, which were never read.
- The dedup result (40 vs 1) is true by construction of 40 distinct scalings.
- Only 6 curves, fixed seed; R = 1.27 has no stated uncertainty, and the band [0.5, 2] was set without power analysis. Low coverage (about 0.09) at m=2 and p about 2^11-2^12.

## Bearing on the sub-sqrt(p) question
none: a toy dedup/dispersion check at tiny p says nothing about relation yield or beating rho.

## Standing
[pcc:unverified], single local toy run, no independent replay.
