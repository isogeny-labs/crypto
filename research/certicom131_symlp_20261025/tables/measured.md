# Measured values

Single thread, one field implementation per family, fixed seed 20261025. Step rates are this host's; the index-calculus decomposition step and the Pollard-rho adding-walk step use the same field code in the same process, so their rates compare directly.

## Reference: batched adding-walk (rho) step rate

| family | n | lanes | Msteps/s |
|---|--:|--:|--:|
| binary | 131 | 1024 | 4.35 |
| prime | 131 | 1024 | 5.61 |

## Oracle step rate: symmetric vs per-point baseline

`sym/base` is the per-step speed-up of the symmetric-coordinate sweep (enumerate `e_1 = x_1 + x_2`, batched inversion) over the per-point baseline (enumerate `x_1`, lift each with its own inversion). `IC/rho step` is the symmetric oracle's step rate over the rho reference for the same family.

| family | curve | n | base Msteps/s | sym Msteps/s | sym/base | IC/rho step |
|---|---|--:|--:|--:|--:|--:|
| binary | cal | 61 | 0.39 | 4.52 | 11.6× | 1.04 |
| binary | cal | 71 | 0.47 | 8.88 | 19.0× | 2.04 |
| binary | cal | 83 | 0.31 | 5.23 | 17.0× | 1.20 |
| binary | cal | 97 | 0.37 | 6.70 | 18.1× | 1.54 |
| binary | cal | 113 | 0.31 | 5.19 | 16.6× | 1.19 |
| binary | cal | 127 | 0.36 | 7.59 | 20.8× | 1.74 |
| binary | ecc2-131 | 131 | 0.18 | 5.24 | 28.7× | 1.20 |
| prime | cal | 61 | 0.37 | 0.19 | 0.5× | 0.03 |
| prime | cal | 71 | 0.30 | 0.16 | 0.5× | 0.03 |
| prime | cal | 83 | 0.28 | 0.14 | 0.5× | 0.03 |
| prime | cal | 97 | 0.22 | 0.13 | 0.6× | 0.02 |
| prime | cal | 113 | 0.23 | 0.11 | 0.5× | 0.02 |
| prime | cal | 127 | 0.16 | 0.08 | 0.5× | 0.01 |
| prime | eccp-131 | 131 | 0.17 | 0.09 | 0.6× | 0.02 |

## Full-subspace decomposition yield (binary calibration)

A full sweep of `e_1 ∈ V_{l'}` over random targets, with the 0/1/2-large-prime split against the factor base `V_l`. `direct` are relations with both abscissae in `V_l` (zero large primes); they are the only ones a pipeline without a large-prime graph can use.

| n | l | l' | decomps/sweep | direct (0 lp) | 1 lp | 2 lp | direct fraction |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 19 | 6 | 18 | 32568 | 1 | 1073 | 2083249 | 4.8e-7 |
| 23 | 8 | 20 | 32634 | 1 | 1105 | 2087438 | 4.8e-7 |
| 29 | 10 | 22 | 8176 | 0 | 254 | 523038 | 0.0e0 |
| 31 | 10 | 24 | 32676 | 1 | 70 | 522748 | 1.9e-6 |
