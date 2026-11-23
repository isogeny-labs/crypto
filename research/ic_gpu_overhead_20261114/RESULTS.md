GPU: NVIDIA A100 80GB PCIe

```
old abecd26901
new 6128d645d65d064938816416a51e61f4140a6ca4
ad9084a9daf1e26c2fa42daebdb13e99a1a322e0808abb7382d999933678d836  /work/ab/old/ic
03d96415596c3d0389f33e47c139b51bc1031a269fb2a4fb6a4c8b7387402d0c  /work/ab/new/ic
e9c7c09a697fbc8ec472aaf5715b7a15b0f9905dc9e23f3be439b59d458e9535  /work/ab/old/gpu/fes/fes_cuda
cde16a9b087d4764db1225aaf944529ee924d21ccfc769d14ee6c057791bc3d0  /work/ab/new/gpu/fes/fes_cuda
30 replies cuda 30 host EQUAL 459050 solutions
```

## koblitz-n23

| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |
|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|
| `fes-f2` | old | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 13.656, 13.669, 13.618, 13.662 | — | — | 11.98, 13.83, 11.94, 13.82 |
| `fes-f2` | new | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 13.448, 13.461, 13.425, 13.400 | — | — | 11.80, 13.62, 11.77, 13.55 |

- derived, `fes-f2`: counted ops, calls and hit rate identical per repeat in every run (S priced `pinned`); solver per call, by repeat: 13.637 → 13.437 ms (1.01×); 13.666 → 13.431 ms (1.02×); whole solve: 11.96 → 11.79 s (1.01×); 13.82 → 13.58 s (1.02×); largest same-arm, same-repeat max/min 1.005.
| `fes-f2-gpu` | old | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 1.053, 0.649, 1.135, 0.657 | 0.568, 0.242, 0.645, 0.252 | 0.485, 0.407, 0.490, 0.404 | 1.04, 0.77, 1.11, 0.78 |
| `fes-f2-gpu` | new | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 0.540, 0.236, 0.620, 0.236 | 0.437, 0.131, 0.517, 0.131 | 0.104, 0.106, 0.103, 0.104 | 0.60, 0.40, 0.67, 0.39 |

- derived, `fes-f2-gpu`: counted ops, calls and hit rate identical per repeat in every run (S priced `pinned`); solver per call, by repeat: 1.094 → 0.580 ms (1.89×); 0.653 → 0.236 ms (2.77×); whole solve: 1.07 → 0.64 s (1.68×); 0.78 → 0.39 s (1.97×); largest same-arm, same-repeat max/min 1.147.

## char2-n25

| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |
|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|
| `fes-f2-gpu` | old | 2 | y | 322196.66, 323610.16 | 2338, 2338 | 1.207, 1.007 | 0.517, 0.490 | 0.691, 0.517 | 3.34, 2.75 |
| `fes-f2-gpu` | new | 2 | y | 320218.67, 316107.81 | 2338, 2338 | 0.479, 0.489 | 0.353, 0.365 | 0.127, 0.124 | 1.60, 1.62 |

- derived, `fes-f2-gpu`: counted ops, calls and hit rate identical per repeat in every run (S priced `measured`); solver per call, by repeat: 1.107 → 0.484 ms (2.29×); whole solve: 3.04 → 1.61 s (1.89×); largest same-arm, same-repeat max/min 1.199.

## char2-n29

| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |
|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|
| `fes-f2-gpu` | old | 2 | y | 3494026.90, 3514875.46 | 8977, 8977 | 2.620, 2.617 | 1.578, 1.581 | 1.041, 1.037 | 27.39, 27.30 |
| `fes-f2-gpu` | new | 2 | y | 3493712.14, 3503811.70 | 8977, 8977 | 1.546, 1.541 | 1.369, 1.366 | 0.177, 0.174 | 17.50, 17.39 |

- derived, `fes-f2-gpu`: counted ops, calls and hit rate identical per repeat in every run (S priced `measured`); solver per call, by repeat: 2.619 → 1.543 ms (1.70×); whole solve: 27.35 → 17.45 s (1.57×); largest same-arm, same-repeat max/min 1.004.
