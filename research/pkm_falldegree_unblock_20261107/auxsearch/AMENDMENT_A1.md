# Amendment A1 (2026-11-15): exact finite-p prediction and a confirmatory run

Frozen before any A1 data. Owner: "Keep testing". Every number is `[pcc:unverified]`.

## Why

In the first sweep the means at t = 12, 13, 14 were 1.136, 1.129 and 1.130 times 2^t. All three were inside model H's
99% interval, but all three were high. The protocol noted that finite p could explain this (only 44, 22 and 11 traces
in range) but could not compute the exact value. A1 computes it and tests it on fresh seeds.

## Exact prediction (`exact/hurwitz.c`, `exact/predict_exact.json`)

The pilot accepts E: y^2 = x^3 + ax + b over F_p ((a, b) uniform) when 2^t | #E and E[2] is not all rational, i.e.
the 2-Sylow is cyclic of order >= 2^t. By Deuring and Schoof's count of curves with given trace and rational
n-torsion, the number of accepted (a, b) is

    (p - 1)/2 * sum over s = p + 1 (mod 2^t), |s| <= 2 sqrt p, of [ H(4p - s^2) - H((4p - s^2)/4) ]

(the second term for s even). H is the Hurwitz class number, computed by counting weighted reduced forms.

- **Check.** `hurwitz selftest p` enumerates every nonsingular curve over F_p, with #E and E[2] by brute force, and
  compares the count at every t. Exact match for p = 257, 263, 769 at every t. Small values of H also match:
  H(3) = 1/3, H(4) = 1/2, H(12) = 4/3, H(16) = 3/2, H(23) = 3.
- **Prediction.** P_accept(t) = count / p^2. Singular draws (density 1/p) are counted as tries by the pilot; this is
  ignored, at relative size about 5e-10.

## Run

- t = 12, 13, 14: seeds i = 200..799 (600 new each; seed_i = 0xA0C5_0000_0000 ^ t<<16 ^ i, the same formula as the
  first sweep).
- t = 15, 16: seeds i = 0..199.
- Same binary as the first sweep (sha256 `50c61dbf…`), local, 4 processes.

## Reading (per t, fixed now)

- r_t = mean tries / 2^t on the A1 seeds alone. E_t = 1/(P_exact · 2^t). The 99% interval for r_t is simulated
  from n draws of Geometric(P_exact), as in `analyze.py`.
- **Exact model holds at t** if r_t is in its interval.
- **Pooled.** z_t = (mean_t − 1/P_t) / (sqrt(1 − P_t)/P_t / sqrt(n)), and z = (z_12 + z_13 + z_14)/sqrt(3) over the A1
  seeds. |z| > 2.58 is a miss of the exact model: evidence that the search does not sample (a, b) as modelled.
- The first sweep's t = 12..14 excess reads as chance if the A1 means sit inside their intervals and the pooled
  |z| <= 2.58.

## Exact predictions at P1 (computed before A1 data; `exact/predict_exact.json`)

| t | traces | E[tries] exact | / 2^t |
|--:|--:|--:|--:|
| 8 | 701 | 255.9 | 0.9996 |
| 9 | 351 | 511.0 | 0.9980 |
| 10 | 175 | 1,025.5 | 1.0014 |
| 11 | 87 | 2,031.8 | 0.9921 |
| 12 | 43 | 4,137.5 | 1.0101 |
| 13 | 21 | 8,393.1 | 1.0245 |
| 14 | 11 | 16,333.0 | 0.9969 |
| 15 | 5 | 34,342.0 | 1.0480 |
| 16 | 3 | 60,494.8 | 0.9231 |

Finite p moves the expectation by under 2.5% for t <= 14, so it does **not** explain the first sweep's +13% at
t = 12..14 (0.997–1.025 predicted there). A1 tests whether that excess recurs on fresh seeds.
