GPU: NVIDIA A100 80GB PCIe

```
main 3e432229fc48a566f2edda659f151bd05346b428
merged 40587624bb57982e51ae577b681d3a111b434a56
895e8559740c09d55709d6266d31d8dc184690152f7345b5d915de881a20027d  /work/ab/main/ic
cbd66d7e7a815ef71250af4400c108b62c0105b9b7b802ce3cb619001e1999f6  /work/ab/merged/ic
ac01b6523b7be84c12cf05931fa147ac833d61572e4a4840a06d7bef62ce7800  /work/ab/main/gpu/fes/fes_cuda
34f8d0ae668faf99255360e49ae3b51e3637105f602fdc9fa098c6aee93c536d  /work/ab/merged/gpu/fes/fes_cuda
30 replies cuda 30 host EQUAL 459050 solutions
```

## koblitz-n23

| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |
|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|
| `fes-f2` | main | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 12.320, 12.277, 12.512, 12.897 | — | — | 10.82, 12.44, 10.99, 13.06 |
| `fes-f2` | merged | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 12.548, 13.118, 13.693, 13.564 | — | — | 11.02, 13.28, 12.02, 13.73 |

- derived, `fes-f2`: counted ops, calls and hit rate identical per repeat in every run (S priced `pinned`); solver per call, by repeat: 12.416 → 13.120 ms (0.95×); 12.587 → 13.341 ms (0.94×); whole solve: 10.91 → 11.52 s (0.95×); 12.75 → 13.51 s (0.94×); largest same-arm, same-repeat max/min 1.091.
| `fes-f2-gpu` | main | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 0.537, 0.204, 0.517, 0.204 | 0.526, 0.193, 0.506, 0.193 | 0.011, 0.011, 0.011, 0.011 | 0.54, 0.29, 0.51, 0.28 |
| `fes-f2-gpu` | merged | 4 | y | 17128.04, 19760.03, 17128.04, 19760.03 | 872, 1006, 872, 1006 | 0.612, 0.125, 0.467, 0.132 | 0.600, 0.114, 0.456, 0.121 | 0.012, 0.011, 0.011, 0.011 | 0.60, 0.19, 0.47, 0.20 |

- derived, `fes-f2-gpu`: counted ops, calls and hit rate identical per repeat in every run (S priced `pinned`); solver per call, by repeat: 0.527 → 0.539 ms (0.98×); 0.204 → 0.129 ms (1.59×); whole solve: 0.53 → 0.53 s (0.99×); 0.28 → 0.20 s (1.43×); round trip: 0.516 → 0.528 ms; 0.193 → 0.117 ms; largest same-arm, same-repeat max/min 1.311.

## char2-n25

| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |
|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|
| `fes-f2-gpu` | main | 2 | y | 290081.24, 294219.53 | 2338, 2338 | 0.436, 0.411 | 0.420, 0.398 | 0.016, 0.014 | 1.31, 1.19 |
| `fes-f2-gpu` | merged | 2 | y | 276833.40, 237636.00 | 2338, 2338 | 0.383, 0.352 | 0.368, 0.337 | 0.014, 0.015 | 1.14, 1.09 |

- derived, `fes-f2-gpu`: counted ops, calls and hit rate identical per repeat in every run (S priced `measured`); solver per call, by repeat: 0.423 → 0.367 ms (1.15×); whole solve: 1.25 → 1.11 s (1.12×); round trip: 0.409 → 0.353 ms; largest same-arm, same-repeat max/min 1.088.

## char2-n29

| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |
|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|
| `fes-f2-gpu` | main | 2 | y | 3203799.40, 4349384.51 | 8977, 8977 | 1.547, 1.547 | 1.524, 1.525 | 0.023, 0.022 | 16.44, 16.36 |
| `fes-f2-gpu` | merged | 2 | y | 2665398.77, 2629361.98 | 8977, 8977 | 1.443, 1.442 | 1.421, 1.420 | 0.021, 0.023 | 15.00, 15.35 |

- derived, `fes-f2-gpu`: counted ops, calls and hit rate identical per repeat in every run (S priced `measured`); solver per call, by repeat: 1.547 → 1.443 ms (1.07×); whole solve: 16.40 → 15.17 s (1.08×); round trip: 1.525 → 1.421 ms; largest same-arm, same-repeat max/min 1.000.
