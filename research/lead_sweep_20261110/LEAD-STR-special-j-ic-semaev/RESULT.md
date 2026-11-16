# RESULT: LEAD-STR-special-j-ic-semaev

## Outcome
- status: ran; exit code 0; wall 41 s (run.status: exit=0 wall=41). One run only. run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): |r_j0 - 1| and |r_j1728 - 1| <= band = max(3*SE(A/A ratio), 0.02, 3*SE(r_k)); positive control normalised ratio < 0.5; outside the band in either direction falsifies.
- Measured (normalised sumset coverage, 12 curves/arm): generic_A 1.00242 (se 0.00072); generic_B_AA 1.00268 (se 0.00060); j0 1.00408 (se 0.00161); j1728 1.00307 (se 0.00177); positive control 0.01375.
- Ratios to generic_A: r_j0 = 1.00166 (se 0.00176), r_j1728 = 1.00065 (se 0.00191); band 0.02 for both. A/A ratio = 1.00026 (within 0.02).
- Positive control ratio 0.0137 (< 0.5, detector sees the effect).
- Verdict: consistent_with_prediction. Controls passed: null (generic), positive, A/A all OK.

## Reading
- The toy tests only the narrow claim: at fixed B, Aut-orbit-closed bases on j=0/1728 give the same two-point sumset coverage as +-closed generic bases. Deviations are ~0.2% and 0.07%, far below the 2% band (floor of the band dominates; 3*SE is about 0.005).
- run.py's verdict field matches the numbers. No discrepancy found.
- Note: all arms are at the random-model value (~1.00), so coverage is essentially that of random sets in exponent space. j1728 curves use cofactor subgroups (N = p/2 or p/4-ish), and orders are not matched, as PROTOCOL s6 states.
- The toy does not test Semaev degrees, first-fall, F4 time, charged S, matched rho, or small-height j. Sizes N ~ 2^15-2^17 only.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: no relation-yield gain from Aut-closed bases at fixed B in this toy, so only the sqrt|Aut| accounting remains.

## Standing
[pcc:unverified], single local toy run, no independent replay.
