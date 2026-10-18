## E7a. Cofactor-cleared base against the fiber-closed table

### `e7a_h4`

| curve | bits | h | log₂ #E | rho S | best closed (arm : S) | best subgroup (arm : S) | subgroup ÷ closed | subgroup ÷ rho | closed ÷ rho |
|:--|--:|--:|--:|--:|:--|:--|--:|--:|--:|
| `icv1-fp18-t334-97871af7` | 18 | 4 | 17.58 | 2.06 | closed-b32 : 25.0 | subgroup-b16 : 16.2 | 0.647 | 7.84 | 12.11 |
| `icv1-fp18-tm204-54b2fadf` | 18 | 4 | 17.83 | 1.82 | closed-b32 : 24.4 | subgroup-b16 : 14.1 | 0.577 | 7.73 | 13.39 |
| `icv1-fp22-tm1214-827be133` | 22 | 4 | 21.34 | 1.37 | closed-b32 : 14.7 | subgroup-b32 : 14.0 | 0.952 | 10.22 | 10.74 |
| `icv1-fp22-tm2826-0afa05b6` | 22 | 4 | 21.08 | 1.20 | closed-b32 : 16.2 | subgroup-b32 : 14.1 | 0.869 | 11.77 | 13.54 |
| `icv1-fp26-t374-04f2b312` | 26 | 4 | 25.37 | 0.82 | closed-b64 : 19.7 | subgroup-b128 : 15.9 | 0.809 | 19.55 | 24.16 |
| `icv1-fp26-tm7664-a6fbaf53` | 26 | 4 | 25.80 | 1.16 | closed-b64 : 23.4 | subgroup-b128 : 15.3 | 0.651 | 13.10 | 20.13 |
| `icv1-fp30-t8770-57966dac` | 30 | 4 | 29.34 | 0.97 | closed-b256 : 32.4 | subgroup-b256 : 18.9 | 0.582 | 19.50 | 33.47 |
| `icv1-fp30-tm29402-75bd2650` | 30 | 4 | 29.47 | 0.84 | closed-b128 : 29.9 | subgroup-b256 : 19.2 | 0.641 | 22.81 | 35.61 |

Geometric mean of subgroup ÷ closed over 8 curves: **0.705** (prediction h^(−1/3) = 0.630; within a factor 1.5: yes). Slope of log₂(subgroup ÷ rho) on log₂ #E: **0.122** (95 % interval 0.081 to 0.163; prediction 1/6).

### `e7a_h72`

| curve | bits | h | log₂ #E | rho S | best closed (arm : S) | best subgroup (arm : S) | subgroup ÷ closed | subgroup ÷ rho | closed ÷ rho |
|:--|--:|--:|--:|--:|:--|:--|--:|--:|--:|
| `icv1-fp18-t872-49a6dd87` | 18 | 72 | 17.94 | 2.40 | closed-b8 : 106.5 | subgroup-b8 : 23.0 | 0.216 | 9.56 | 44.33 |
| `icv1-fp18-tm22-a5ba8c3f` | 18 | 72 | 17.56 | 2.28 | closed-b8 : 121.0 | subgroup-b8 : 21.7 | 0.180 | 9.51 | 52.96 |
| `icv1-fp22-tm1504-967cbbde` | 22 | 72 | 21.48 | 2.00 | closed-b8 : 44.1 | subgroup-b16 : 14.6 | 0.330 | 7.29 | 22.07 |
| `icv1-fp22-tm2412-15c27732` | 22 | 72 | 21.43 | 1.48 | closed-b8 : 48.3 | subgroup-b16 : 15.4 | 0.319 | 10.40 | 32.60 |
| `icv1-fp26-t8250-b50c04e1` | 26 | 72 | 25.47 | 1.35 | closed-b16 : 45.5 | subgroup-b32 : 15.2 | 0.335 | 11.33 | 33.83 |
| `icv1-fp26-tm1038-caffe525` | 26 | 72 | 25.88 | 1.20 | closed-b16 : 42.3 | subgroup-b32 : 14.6 | 0.345 | 12.14 | 35.17 |
| `icv1-fp30-tm45748-31aed113` | 30 | 72 | 29.92 | 1.05 | closed-b32 : 60.5 | subgroup-b128 : 15.8 | 0.261 | 15.01 | 57.55 |
| `icv1-fp30-tm48354-87c391fd` | 30 | 72 | 29.61 | 0.89 | closed-b32 : 58.6 | subgroup-b128 : 15.4 | 0.262 | 17.31 | 66.01 |

Geometric mean of subgroup ÷ closed over 8 curves: **0.275** (prediction h^(−1/3) = 0.240; within a factor 1.5: yes). Slope of log₂(subgroup ÷ rho) on log₂ #E: **0.068** (95 % interval 0.022 to 0.114; prediction 1/6).

## E7b. Trace-kernel subspace base against the default

| curve | arm | runs (verified) | relations / trials | yield | solver ops per relation | max degree | S | online wall (s) |
|:--|:--|:--|:--|--:|--:|--:|--:|--:|
| `icv1-f2m19-t797-b6cf2467` | f4-blind | 8 (8) | 808 / 3488 | 0.2317 | 4329947 | 3 | 4.93 | 2.93 |
| `icv1-f2m19-t797-b6cf2467` | f4-tracekernel | 8 (8) | 952 / 2777 | 0.3428 | 4964166 | 3 | 4.90 | 5.15 |
| `icv1-f2m19-t797-b6cf2467` | mitm-blind | 8 (8) | 808 / 3488 | 0.2317 | – | – | 195.75 | 0.00 |
| `icv1-f2m19-t797-b6cf2467` | mitm-tracekernel | 8 (8) | 952 / 2777 | 0.3428 | – | – | 160.82 | 0.00 |
| `icv1-f2m19-t797-b6cf2467` | rho-neg | 8 (8) | 0 / 0 | – | – | – | 1.72 | 0.00 |
| `icv1-f2m23-t5197-69e76b73` | f4-blind | 8 (8) | 758 / 47189 | 0.0161 | 4634927 | 3 | 5.43 | 7.63 |
| `icv1-f2m23-t5197-69e76b73` | f4-tracekernel | 8 (8) | 929 / 29782 | 0.0312 | 3329596 | 3 | 3.86 | 6.74 |
| `icv1-f2m23-t5197-69e76b73` | mitm-blind | 8 (8) | 758 / 47189 | 0.0161 | – | – | 53.13 | 0.00 |
| `icv1-f2m23-t5197-69e76b73` | mitm-tracekernel | 8 (8) | 929 / 29782 | 0.0312 | – | – | 49.69 | 0.00 |
| `icv1-f2m23-t5197-69e76b73` | rho-neg | 8 (8) | 0 / 0 | – | – | – | 1.18 | 0.00 |

| curve | oracle | yield, trace-kernel ÷ default | 95 % interval | solver ops per relation, default ÷ trace-kernel | S, default ÷ trace-kernel | max degree (default, trace-kernel) |
|:--|:--|--:|:--|--:|--:|:--|
| `icv1-f2m19-t797-b6cf2467` | f4 | 1.48 | 1.37 to 1.60 | 0.87 | 1.01 | 3, 3 |
| `icv1-f2m19-t797-b6cf2467` | mitm | 1.48 | 1.37 to 1.60 | – | 1.22 | 0, 0 |
| `icv1-f2m23-t5197-69e76b73` | f4 | 1.94 | 1.77 to 2.14 | 1.39 | 1.41 | 3, 3 |
| `icv1-f2m23-t5197-69e76b73` | mitm | 1.94 | 1.77 to 2.14 | – | 1.07 | 0, 0 |
