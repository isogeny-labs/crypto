# RESULT: LEAD-ICP-elliptic-code-list-decoding

## Outcome
- Status: ran. Exit code 0. Wall 2 s (run.status: exit=0 wall=2; run.py elapsed_s=1.63).
- run.stderr empty (nothing to tail).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md sec. 3): consistent if pooled R_fb in [0.75,1.33], |R_fb - R_null| <= 0.25 R_null, GS fails for all k, positive control passes. Falsified if R_fb > 1.5 or < 0.5.

Measured:
- R_fb = 1.00049 (pooled observed/expected, factor base).
- R_null = 1.00117 (uniform random labels). |R_fb - R_null| = 0.0007, far inside 0.25 R_null.
- A/A (seeds 11 vs 12): max per-cell-ratio spread 0.0223 (noise floor).
- GS: sqrt(Bk) = 6.93, 8.49, 9.80, 10.95 for k=2..5, all > k, so decoder_reaches_weight_k false for every k (gs_fails_all_k true).
- Positive control (planted 3-subset): found.
- Cells: p in {4001, 16001, 64007} x seeds {11,12}, B=24, k=2..5, 2000 targets. Subgroup orders n = 4067, 2062, 7913, 16179, 64183, 63877.
- Verdict: consistent_with_prediction. Null, positive and A/A controls all passed.

## Reading
- The toy tests only the counting premise (signed k-subset sums hit a target at the Poisson rate C(B,k)2^k/n) and the GS arithmetic. It implements no decoder, so it does not test Zhang-Liu or Cheng.
- run.py's verdict field matches the numbers: R_fb is within the band, close to the null, and the A/A spread (about 2%) is much smaller than the band widths.
- k=2 cells at large p have expected means of 0.014-0.02, so they are noisy. The pooled statistic only uses cells with expected >= 1, so these do not drive it.
- Minor note: for p=4001 seed 12 the cyclic subgroup has n=2062 (about half the curve order), so n varies by cell. The expected mean uses each cell's own n, so this is handled.
- Nothing surprising: the factor base (smallest-x points) shows no density structure relative to random labels.

## Bearing on the sub-sqrt(p) question
Closes or confirms the recorded obstruction: solution counts are indistinguishable from random labels and GS needs agreement sqrt(Bk) > k, so the lead reduces to enumeration at these toy sizes. This says nothing about the full-text Zhang-Liu parameters.

## Standing
[pcc:unverified], single local toy run, no independent replay.
