# RESULT: LEAD-TRF-mov-small-embedding-degree

## Outcome
- status: ran; exit code 0; wall 2 s (run.status: exit=0 wall=2). Single run, no retries.
- run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): pooled fraction k<=10 <= 0.01 and pooled median k/(r-1) >= 0.1 and |median - null median| <= 0.15 -> consistent; falsified if controls pass but fraction > 0.01 or median deviates.
- Measured: A1 n=163, frac 0.012270 (2/163); A2 n=164, frac 0.012195 (2/164). Pooled n=327, frac 0.012233 (4/327) > 0.01. Pooled median k/(r-1) = 0.5, null median 0.5 (difference 0), >= 0.1.
- Controls: positive (supersingular y^2=x^3+1, 40 curves, all k=2) passed; A/A passed (|diff| ~ 7e-5 <= 0.01); null instrument ran, median matched. Controls ok.
- Verdict (run.py): falsified, solely via the fraction criterion (0.0122 > 0.01).

## Reading
- The numbers support the verdict field mechanically: the threshold 0.01 is exceeded by 4 hits vs 3.27 expected at exactly 0.01. This is a one-count excess, not statistically meaningful (Poisson P(X>=4 | 3.27) ~ 0.35). The median criterion is satisfied exactly.
- Only 327 of 1400 drawn curves passed the r >= p/8 filter, so the sample is small and the 0.01 line is coarse (one hit moves the fraction by 0.003).
- At p ~ 4-9k, k<=10 requires r | p^j - 1 for j<=10 with r >= p/8, which is far more likely at toy size than at cryptographic size, as the protocol itself notes (s6). The falsification is therefore a toy-size threshold artifact, not evidence against the recorded obstruction. The 4 hits are presumably small-order cases (e.g. r | p+1 or p-1 style); not inspected because RESULT.json does not list them.
- The toy does test the stated toy-size hypothesis; it cannot test deployed or pairing-friendly curves.

## Bearing on the sub-sqrt(p) question
none: a one-count excess over a coarse toy threshold carries no information about large random curves, and no MOV solve or rho cost comparison was run.

## Standing
[pcc:unverified], single local toy run, no independent replay.
