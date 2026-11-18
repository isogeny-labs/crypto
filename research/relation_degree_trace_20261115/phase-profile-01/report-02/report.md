# Replay phase diagnosis

Known original inputs, generation excluded. One planned call per mode/case; unsigned L0, no speed ratios, confidence intervals or numerical family selection. Every accepted call pays retained exact checking. [pcc:unverified]

| Case / mode | Construction+completion ms | Wire ms | Export ms | Checker ms | Unattributed parent ms | Parent ms |
|---|---:|---:|---:|---:|---:|---:|
| star-first-n10-s7 / compact | 8.953 | 0.663 | 0.770 | 16.810 | 5.328 | 32.523 |
| star-first-n10-s7 / expanded | 41.799 | 3.098 | 4.610 | 22.391 | 5.137 | 77.036 |
| star-first-n10-s7 / compact-sparse | 7.150 | 0.489 | 0.842 | 10.266 | 4.715 | 23.463 |
| star-last-n10-s7 / compact | 7.393 | 0.263 | 0.433 | 9.119 | 4.176 | 21.385 |
| star-last-n10-s7 / expanded | 36.717 | 2.609 | 3.675 | 22.224 | 4.818 | 70.044 |
| star-last-n10-s7 / compact-sparse | 6.669 | 0.328 | 0.494 | 10.181 | 4.982 | 22.654 |
| star-first-n12-s7 / compact | 12.188 | 1.149 | 0.953 | 11.511 | 5.053 | 30.855 |
| star-first-n12-s7 / expanded | 48.657 | 3.780 | 6.281 | 33.880 | 5.148 | 97.747 |
| star-first-n12-s7 / compact-sparse | 8.982 | 0.617 | 0.784 | 11.798 | 5.184 | 27.363 |
| star-last-n12-s7 / compact | 12.572 | 0.528 | 0.704 | 11.242 | 5.022 | 30.068 |
| star-last-n12-s7 / expanded | 48.844 | 3.132 | 4.211 | 23.886 | 5.312 | 85.386 |
| star-last-n12-s7 / compact-sparse | 8.536 | 0.484 | 0.961 | 12.487 | 4.891 | 27.359 |
| star-first-n16-s7 / compact | 19.761 | 1.011 | 1.260 | 12.603 | 5.026 | 39.662 |
| star-first-n16-s7 / expanded | 74.594 | 5.458 | 7.455 | 33.663 | 5.162 | 126.333 |
| star-first-n16-s7 / compact-sparse | 15.853 | 0.949 | 1.146 | 14.822 | 5.023 | 37.794 |
| star-last-n16-s7 / compact | 22.394 | 0.758 | 0.864 | 11.625 | 4.628 | 40.269 |
| star-last-n16-s7 / expanded | 74.144 | 3.860 | 8.582 | 26.567 | 5.126 | 118.278 |
| star-last-n16-s7 / compact-sparse | 14.901 | 0.626 | 0.859 | 12.129 | 6.854 | 35.369 |

Disjoint phases sum to the measured parent interval. Construction includes native construction, membership rows, witness completion and its internal export validation. Unattributed parent includes launch, initial input parsing, output assembly, receipt writing and parent parsing/agreement/hash. Internal timer overhead remains diagnostic overhead. No phase by itself establishes a complete-workflow gain or regularity reduction. [pcc:unverified]
