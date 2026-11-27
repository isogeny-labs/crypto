# Curve25519 projective split qualification

[pcc:unverified]

Four of eight frozen cells completed. Both t=5 cells timed out before one target; both t=6 cells were killed for memory before one target. All 32+8 requests in the completed t=3/t=4 cells passed the exact relation checks, including measured projective identity-chart hits.

The descriptive matched t>=4 candidate/baseline geometric-mean ratio on the 32 available t=4 pairs is 1.000463984 with a deterministic 95% bootstrap interval [1.000463984, 1.000463985]. This does not meet the frozen upper-bound target below 0.80. Because t=5 and t=6 have no complete means, both four-point slope intervals are undefined. The projective arm is **not qualified for m=4**.

This is a component qualification result, not a total-work subexponential ECDLP result. See `analysis.json` and `../raw/attempts.json` for exact flow and censoring.
