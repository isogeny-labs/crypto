# Derived cost model (labelled derived)

All rows are computed, not measured. The step counts use the balancing factor-base dimension `l = round(n/3)` for the subspace `m = 2` method; the per-step cost is taken from the measured symmetric-oracle rate and the measured rho rate on this host. `r` is the prime-subgroup order (`2^{n-1}` used for the calibration curves, which are not point-counted).

steps(IC, m=2) ≈ 2^{2n/3}  (2^{d-1} relations ÷ 2^{2d-n-2} success, d = (n+1)/3)

steps(rho) ≈ sqrt(π·r/4),  r ≈ 2^{n}·(cofactor factor)

distance(steps) = steps(IC) / steps(rho) ≈ 2^{n/6} · constant

| curve | n | log2 steps(IC m=2) | log2 steps(rho) | log2 distance (steps) | this matches family optimum Θ(r^{1/6})·rho? |
|---|--:|--:|--:|--:|---|
| ECC2-131 | 131 | 87.3 | 64.8 | 22.5 | the step gap 2^{23} is far above the r^{1/6} ≈ 2^{22} family optimum |
| ECCp-131 | 131 | 87.3 | 65.3 | 22.0 | the step gap 2^{22} is far above the r^{1/6} ≈ 2^{22} family optimum |

The `m = 2` exponent `2n/3 ≈ 0.667 n` agrees with the merged algebraic ladder's measured trial slope 0.647 [0.639, 0.655] (PR #2493); the enumeration and the linear solve share the `2^{2n/3}` optimum. What this study adds over that baseline is measured and constant-factor: the symmetric decomposition step costs about the same as a rho step on this host (so the distance is the step-count gap, not a per-step penalty), and the double-large-prime graph recovers full relations from the 2-lp decompositions that dominate the measured yield. Neither changes the `2n/3` vs `n/2` exponent gap, so the distance to rho at n = 131 stays about `2^{n/6} ≈ 2^{22}` in steps before the linear-algebra floor, which the program gates place above rho for m ≤ 3 on both curves.
