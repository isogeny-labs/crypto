# RESULT: LEAD-TRF-odd-prime-ghs-cover-floor

## Outcome
- status: ran (kind runnable_local); exit code 0; wall 192 s (run.status: exit=0 wall=192; run.py reports 192.37 s).
- run.stderr empty. One run only, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): R = f_B / mean decoy f in [0.5, 2.0], at least 5 hits in B at n=5 (N=3000),
A/A halves agree (|f1-f2| <= 0.5 f_B + 0.01); positive control 100% in B with exact divisibility.

Measured, n=5 (primary): 2998 random curves with j not in F_5, B = {0, +-57, +-76, +-82, +-101}.
- hits = 76, f_B = 0.02535; decoy mean = 0.03866 (10 decoy shifts); R = 0.656 (inside [0.5, 2.0]).
- A/A halves: 0.02602 vs 0.02468; difference 0.0013 <= 0.5*0.02535+0.01 = 0.0227: pass.
- Positive control: 40/40 in B, 40/40 divisible by x^2 - s x + 5: pass.
- Null check: every random hit passed exact divisibility (hits_divisible true).
- Verdict (run.py): consistent_with_prediction. Controls: all passed.

n=7 (secondary, descriptive, N=300): 0 hits in B, decoy mean 0.004, ratio 0.0 (uninformative; expected hits
about 1 at ordinary density). Positive control 10/10.

## Reading
- The toy tests the narrow sharpened claim (the hit rate of random E/F_{5^5} in the set of n-th-power
  subfield Frobenius traces is not enriched), not any real GHS cover construction or genus.
- run.py's verdict matches the numbers. R = 0.66 is in band but on the low side: B is somewhat
  depleted relative to decoys (76 vs about 116 expected from the decoy mean). The band is wide
  and no interval on R is computed, so mild depletion is not excluded or established. Likely cause:
  the trace distribution is bell-shaped, and B has mass at +-101 and +-82 in the tails, while decoys
  B+d shift toward larger magnitudes only partly; this was not tested.
- decoy_count is 10 of the 12 shifts (two overlap B and were excluded as per the disjointness rule).
- n=7 arm had too few samples to say anything. The positive control only confirms the instrument,
  and the divisibility check is a consistency check of the algebra (t = u_n(s)).
- Nothing surprising: the "reach" set is ordinary density, so the n-dimensional floor is not
  lowered for a typical curve; only a roughly 2.5% fraction of j-non-subfield curves in B has a
  factorable Weil restriction.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: for typical E/F_{5^5} the Weil restriction is simple at ordinary density (about 2.5% reducible), so the floor stays at dimension n and this toy gives no route below sqrt(p) (and says nothing about prime-field E(F_p)).

## Standing
[pcc:unverified], single local toy run, no independent replay.
