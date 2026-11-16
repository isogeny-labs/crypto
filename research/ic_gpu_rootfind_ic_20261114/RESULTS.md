GPU: NVIDIA A100 80GB PCIe
Worker: cuda: NVIDIA A100 80GB PCIe (sm_80, 108 SMs) (+ host emulation)
REV: aeeb30b883

Record status per arm: rf2-cpu-F16384 {'verified': 12}; rf2-cpu-F4096 {'verified': 12}; rf2-gpu-F16384 {'verified': 12}; rf2-gpu-F4096 {'verified': 12}; rho-neg {'verified': 12}; rho-neg-aa {'verified': 12}

CPU/GPU pairs compared: 24; mismatches: 0

## Index calculus: rootfind-subtract (CPU) vs rootfind-subtract-gpu, paired runs

| curve | log2 r | \|F_x\| | runs | verified | mean S | rho mean S | S / rho | CPU solve s | GPU solve s | solve ratio | CPU relations s | GPU relations s | relations ratio | GPU batches / targets swept |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|
| icv1-fp28-t7531-5f8c6474 | 27.48 | 4096 | 4 | 4 | 6259.1 | 0.977 | 6403 | 1.881 | 0.489 | 3.9× | 1.809 | 0.070 | 25.8× | 48 / 49152 |
| icv1-fp28-t7531-5f8c6474 | 27.48 | 16384 | 4 | 4 | 3639.0 | 0.977 | 3723 | 2.718 | 1.073 | 2.5× | 1.958 | 0.091 | 21.4× | 29 / 29696 |
| icv1-fp30-tm15113-0b67ffcf | 29.99 | 4096 | 4 | 4 | 18006.6 | 1.369 | 13154 | 12.392 | 0.507 | 24.5× | 12.292 | 0.146 | 84.4× | 287 / 293887 |
| icv1-fp30-tm15113-0b67ffcf | 29.99 | 16384 | 4 | 4 | 14371.0 | 1.369 | 10498 | 12.094 | 1.386 | 8.7× | 11.101 | 0.143 | 77.8× | 79 / 80896 |
| icv1-fp32-tm9515-ee9bd0a2 | 31.26 | 4096 | 4 | 4 | 26009.6 | 1.132 | 22986 | 28.011 | 0.643 | 43.5× | 27.911 | 0.246 | 113.2× | 636 / 651264 |
| icv1-fp32-tm9515-ee9bd0a2 | 31.26 | 16384 | 4 | 4 | 23264.9 | 1.132 | 20560 | 28.569 | 1.511 | 18.9× | 27.475 | 0.227 | 121.2× | 163 / 166912 |

## Eytzinger prefetch (bench throughput kernel ms, medians of rounds)

Selftests passed (of 3): D 3, D2 3, P3 3, P4 3, P5 3, P4L2 3

### |F| = 65536, 65536 targets

| config | default (no prefetch) | default (A/A) | prefetch 3 levels | prefetch 4 levels | prefetch 5 levels | prefetch 4 levels, L2 |
|---|---:|---:|---:|---:|---:|---:|
| hash/C16 | 407.33 | 406.83 (0.999) | 407.93 (1.001) | 407.54 (1.001) | 407.40 (1.000) | 407.05 (0.999) |
| hash/C32 | 561.62 | 561.76 (1.000) | 561.71 (1.000) | 561.37 (1.000) | 561.19 (0.999) | 561.80 (1.000) |
| hash/C64 | 786.93 | 787.25 (1.000) | 786.86 (1.000) | 786.95 (1.000) | 786.91 (1.000) | 786.74 (1.000) |
| sorted/C16 | 360.27 | 360.54 (1.001) | 691.47 (1.919) | 509.85 (1.415) | 472.59 (1.312) | 514.20 (1.427) |
| sorted/C32 | 465.66 | 465.49 (1.000) | 715.73 (1.537) | 714.76 (1.535) | 713.39 (1.532) | 714.47 (1.534) |
| sorted/C64 | 475.07 | 475.34 (1.001) | 831.58 (1.750) | 754.49 (1.588) | 732.19 (1.541) | 754.48 (1.588) |

Best configuration per arm:

- default (no prefetch): sorted/C16 360.27 ms (1.000× the default's best time)
- default (A/A): sorted/C16 360.54 ms (1.001× the default's best time)
- prefetch 3 levels: hash/C16 407.93 ms (1.132× the default's best time)
- prefetch 4 levels: hash/C16 407.54 ms (1.131× the default's best time)
- prefetch 5 levels: hash/C16 407.40 ms (1.131× the default's best time)
- prefetch 4 levels, L2: hash/C16 407.05 ms (1.130× the default's best time)

A/A: largest |D2/D − 1| 0.0012; largest max/min of one arm's rounds 1.2626.

### |F| = 1024, 262144 targets

| config | default (no prefetch) | default (A/A) | prefetch 3 levels | prefetch 4 levels | prefetch 5 levels | prefetch 4 levels, L2 |
|---|---:|---:|---:|---:|---:|---:|
| hash/C16 | 17.71 | 17.71 (1.000) | 17.71 (1.000) | 17.71 (1.000) | 17.71 (1.000) | 17.71 (1.000) |
| hash/C32 | 21.14 | 21.16 (1.001) | 21.16 (1.001) | 21.15 (1.000) | 21.15 (1.000) | 21.14 (1.000) |
| hash/C64 | 21.08 | 21.11 (1.002) | 21.17 (1.005) | 21.15 (1.003) | 21.15 (1.003) | 21.16 (1.004) |
| sorted/C16 | 19.82 | 19.82 (1.000) | 35.04 (1.768) | 35.91 (1.812) | 23.88 (1.205) | 35.18 (1.775) |
| sorted/C32 | 24.28 | 24.29 (1.000) | 33.89 (1.396) | 33.85 (1.394) | 33.80 (1.392) | 33.85 (1.394) |
| sorted/C64 | 24.90 | 24.89 (1.000) | 38.54 (1.548) | 37.21 (1.495) | 37.76 (1.517) | 37.16 (1.493) |

Best configuration per arm:

- default (no prefetch): hash/C16 17.71 ms (1.000× the default's best time)
- default (A/A): hash/C16 17.71 ms (1.000× the default's best time)
- prefetch 3 levels: hash/C16 17.71 ms (1.000× the default's best time)
- prefetch 4 levels: hash/C16 17.71 ms (1.000× the default's best time)
- prefetch 5 levels: hash/C16 17.71 ms (1.000× the default's best time)
- prefetch 4 levels, L2: hash/C16 17.71 ms (1.000× the default's best time)

A/A: largest |D2/D − 1| 0.0016; largest max/min of one arm's rounds 1.0713.
