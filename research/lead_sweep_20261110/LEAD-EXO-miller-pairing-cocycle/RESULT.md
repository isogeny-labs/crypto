# RESULT: LEAD-EXO-miller-pairing-cocycle

## Outcome
- status: ran; exit code 0; wall 4 s (run.status: exit=0 wall=4). Single run, no retries.
- run.stderr empty. RESULT.json is one JSON object, verdict field "consistent_with_prediction", controls_pass true.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): for each of 3 prime sizes, p_M > 0.001, chi-square p > 0.001, homomorphism hits <= max(2, 1% of pairs); null and A/A p_M > 0.001; planted control p < 1e-6.

| p | N | d | max stat | p_M | chisq p | hom hits / pairs | null max | A/A max | planted max (p) |
|---|---|---|---|---|---|---|---|---|---|
| 4099 | 4159 | 3 | 3.324 | 0.99999957 | 0.578 | 2 / 980 | 6.887 | 5.169 | 669.3 (0.0) |
| 12289 | 12479 | 3 | 5.900 | 0.666 | 0.453 | 0 / 1004 | 6.241 | 6.433 | 648.3 (0.0) |
| 32771 | 32719 | 5 | 6.822 | 0.353 | 0.555 | 0 / 991 | 6.566 | 5.458 | 927.9 (0.0) |

Verdict: consistent_with_prediction (hash-like at toy size). Controls: null pass, A/A pass, positive pass (planted detected at p=0).

## Reading
- The toy tests only what PROTOCOL s6 says: the order-d residue class of the Miller value f_{k,P}(Q) (an assumed U) for additive-character structure in k, class uniformity, and an exact homomorphism defect. K1/K2 of IDEA-20261017-e3a6f1 are not tested; U is an assumption.
- Discrepancies/caveats: hom_pairs is 980-1004, not the 2000 the protocol text states (pairs apparently filtered, probably for degenerate or out-of-range k; not investigated, run.py not edited). At p=4099, 2 hom hits in 980 pairs vs about 0.24 expected under 1/(p-1); that is within the preregistered bound (9.8) but nominally high (Poisson tail about 0.02); the other sizes gave 0.
- At p=4099 the max stat 3.32 is unusually low for 400 Exp(1) draws (p_M 0.9999996, i.e. too flat, not structured). n_used (5965) exceeds N (4159), so k wraps past the group order and the same point kP recurs; this makes the frequency sums non-independent. The one-sided criterion cannot flag this, but the smallest size is not a clean test of the Exp(1) null.
- The positive control only shows detection of additive phase structure, not other structure (polynomial in k, elliptic-net recurrences).

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: at p ~ 2^12 to 2^15 the Miller value class behaves like a hash in k, matching the recorded obstruction; this does not close larger p or other Q or normalisations.

## Standing
[pcc:unverified], single local toy run, no independent replay.
