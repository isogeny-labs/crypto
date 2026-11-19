# Prediction recorded before the 16-bit runs finished (2026-11-19T06:50Z)

Observed on all primes 2^7 <= n < 2^12 (sweep_128_4096.csv): max(R-L) = bits-3 for
bits 8..12, and max R over d with L <= 3 is bits-1; no lead rows; excess independent of
gcd(d, n+1).

Prediction for n in {65497, 65519, 65521} (16 bits): max(R-L) in [12, 14] (point
prediction 13), max R with L <= 3 in [14, 16] (point 15), zero lead rows.
