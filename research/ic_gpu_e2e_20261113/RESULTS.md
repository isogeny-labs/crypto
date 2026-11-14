GPU: NVIDIA A100 80GB PCIe
Binary and worker sha256:

```
ad9084a9daf1e26c2fa42daebdb13e99a1a322e0808abb7382d999933678d836  /work/e2e/ic
af17ce2c754d8bf8c49fa6b504072cd432676902a6274fc5f316e397b874e35d  /work/e2e/gpu/fes/fes_cuda
```


## koblitz-n23 (r = 4196903)

| configuration | solver | repeats | verified | hit rate | S mean | S / rho | solver calls | solver wall per call (ms) | solver wall total (s) | row wall (s) | GPU calls / fallback / capped |
|:--|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| `binary-subspace[dimension=12] + mitm[m=2] + — + walk` | `—` | 2 | y | 0.6238 | 3986.20 | 4334.12 | 0 | 0.000 | 0.00 | 5.07 |  |
| `binary-subspace[dimension=12] + descent-algebraic[m=2] + inherited-f4 + walk` | `inherited-f4` | 2 | y | 0.6238 | 1852.45 | 2014.13 | 2487 | 15.466 | 38.46 | 38.68 |  |
| `binary-subspace[dimension=12] + descent-algebraic[m=2] + fes-f2 + walk` | `fes-f2` | 2 | y | 0.6225 | 18444.03 | 20053.83 | 1878 | 14.164 | 26.60 | 26.78 |  |
| `binary-subspace[dimension=12] + descent-algebraic[m=2] + fes-f2-gpu + walk` | `fes-f2-gpu` | 2 | y | 0.6225 | 18444.03 | 20053.83 | 1878 | 1.002 | 1.88 | 2.18 | 1878 / 0 / 0 |

- derived: `fes-f2` → `fes-f2-gpu`: solver wall 26.60 s → 1.88 s (14.1×); row wall 26.78 s → 2.18 s (12.3×); counted S identical (18444.03 vs 18444.03).

- derived: `inherited-f4` → `fes-f2-gpu`: solver wall 38.46 s → 1.88 s (20.4×); row wall 38.68 s → 2.18 s (17.7×); counted S differs (1852.45 vs 18444.03).

## char2-n25 (r = 5593349)

| configuration | solver | repeats | verified | hit rate | S mean | S / rho | solver calls | solver wall per call (ms) | solver wall total (s) | row wall (s) | GPU calls / fallback / capped |
|:--|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| `binary-subspace[dimension=13] + mitm[m=2] + — + walk` | `—` | 1 | y | 0.6396 | 14073.97 | 10935.62 | 0 | 0.000 | 0.00 | 11.18 |  |
| `binary-subspace[dimension=13] + descent-algebraic[m=2] + inherited-f4 + walk` | `inherited-f4` | 1 | y | 0.6396 | 5613.93 | 4362.09 | 1193 | 28.086 | 33.51 | 33.64 |  |
| `binary-subspace[dimension=13] + descent-algebraic[m=2] + fes-f2 + walk` | `fes-f2` | 1 | y | 0.6369 | 297765.58 | 231367.03 | 2338 | 60.522 | 141.50 | 141.94 |  |
| `binary-subspace[dimension=13] + descent-algebraic[m=2] + fes-f2-gpu + walk` | `fes-f2-gpu` | 1 | y | 0.6369 | 297765.58 | 231367.03 | 2338 | 1.183 | 2.77 | 3.22 | 2338 / 0 / 0 |

- derived: `fes-f2` → `fes-f2-gpu`: solver wall 141.50 s → 2.77 s (51.2×); row wall 141.94 s → 3.22 s (44.1×); counted S identical (297765.58 vs 297765.58).

- derived: `inherited-f4` → `fes-f2-gpu`: solver wall 33.51 s → 2.77 s (12.1×); row wall 33.64 s → 3.22 s (10.4×); counted S differs (5613.93 vs 297765.58).

## char2-n29 (r = 134225849)

| configuration | solver | repeats | verified | hit rate | S mean | S / rho | solver calls | solver wall per call (ms) | solver wall total (s) | row wall (s) | GPU calls / fallback / capped |
|:--|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| `binary-subspace[dimension=15] + descent-algebraic[m=2] + inherited-f4 + walk` | `inherited-f4` | 1 | y | 0.6305 | 29580.39 | 30856.36 | 8977 | 99.478 | 893.02 | 895.51 |  |
| `binary-subspace[dimension=15] + descent-algebraic[m=2] + fes-f2-gpu + walk` | `fes-f2-gpu` | 1 | y | 0.6305 | 3135443.12 | 3270693.03 | 8977 | 3.620 | 32.50 | 38.70 | 8977 / 0 / 0 |

- derived: `inherited-f4` → `fes-f2-gpu`: solver wall 893.02 s → 32.50 s (27.5×); row wall 895.51 s → 38.70 s (23.1×); counted S differs (29580.39 vs 3135443.12).
