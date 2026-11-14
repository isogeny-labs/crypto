GPU: NVIDIA A100 80GB PCIe
Selftests passed (of 3): M 3, M2 3, E 3, H 3, P 3, EP 3, HP 3, EHP 3

## |F| = 65536, 65536 targets

| config | main | main (A/A) | Eytzinger | packed hash | paired | Eytzinger + paired | packed + paired | all three |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| hash/C16 | 576.41 | 576.46 (1.000) | 576.44 (1.000) | 410.85 (0.713) | 593.72 (1.030) | 593.67 (1.030) | 404.26 (0.701) | 404.27 (0.701) |
| hash/C32 | 846.41 | 846.42 (1.000) | 846.41 (1.000) | 574.56 (0.679) | 869.23 (1.027) | 869.19 (1.027) | 560.98 (0.663) | 560.97 (0.663) |
| hash/C64 | 1219.47 | 1219.42 (1.000) | 1219.47 (1.000) | 814.21 (0.668) | 1248.47 (1.024) | 1248.46 (1.024) | 787.24 (0.646) | 787.23 (0.646) |
| sorted/C16 | 401.70 | 401.71 (1.000) | 382.62 (0.952) | 401.75 (1.000) | 401.97 (1.001) | 359.96 (0.896) | 401.90 (1.000) | 360.19 (0.897) |
| sorted/C32 | 625.28 | 625.31 (1.000) | 593.88 (0.950) | 625.33 (1.000) | 497.19 (0.795) | 465.08 (0.744) | 497.22 (0.795) | 465.00 (0.744) |
| sorted/C64 | 619.75 | 619.67 (1.000) | 588.17 (0.949) | 619.65 (1.000) | 510.93 (0.824) | 474.63 (0.766) | 510.98 (0.824) | 474.77 (0.766) |

Best configuration per arm (the comparison that counts):

- main: sorted/C16 401.70 ms (1.000× main's best)
- main (A/A): sorted/C16 401.71 ms (1.000× main's best)
- Eytzinger: sorted/C16 382.62 ms (1.050× main's best)
- packed hash: sorted/C16 401.75 ms (1.000× main's best)
- paired: sorted/C16 401.97 ms (0.999× main's best)
- Eytzinger + paired: sorted/C16 359.96 ms (1.116× main's best)
- packed + paired: sorted/C16 401.90 ms (1.000× main's best)
- all three: sorted/C16 360.19 ms (1.115× main's best)

A/A: largest |M2/M − 1| over configurations 0.0001; largest max/min of one arm's rounds 1.0012.

## |F| = 1024, 262144 targets

| config | main | main (A/A) | Eytzinger | packed hash | paired | Eytzinger + paired | packed + paired | all three |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| hash/C16 | 19.87 | 19.88 (1.000) | 19.87 (1.000) | 17.88 (0.899) | 19.51 (0.981) | 19.50 (0.981) | 17.75 (0.893) | 17.73 (0.892) |
| hash/C32 | 25.61 | 25.61 (1.000) | 25.61 (1.000) | 21.88 (0.855) | 25.22 (0.985) | 25.21 (0.985) | 21.16 (0.826) | 21.16 (0.826) |
| hash/C64 | 24.34 | 24.34 (1.000) | 24.36 (1.001) | 21.70 (0.892) | 23.54 (0.967) | 23.54 (0.967) | 21.10 (0.867) | 21.10 (0.867) |
| sorted/C16 | 21.74 | 21.74 (1.000) | 21.12 (0.971) | 21.74 (1.000) | 21.30 (0.980) | 19.86 (0.913) | 21.30 (0.980) | 19.86 (0.914) |
| sorted/C32 | 31.07 | 31.07 (1.000) | 30.25 (0.974) | 31.07 (1.000) | 25.38 (0.817) | 24.30 (0.782) | 25.37 (0.817) | 24.31 (0.782) |
| sorted/C64 | 30.72 | 30.70 (0.999) | 29.85 (0.972) | 30.71 (1.000) | 26.16 (0.852) | 24.93 (0.812) | 26.16 (0.851) | 24.94 (0.812) |

Best configuration per arm (the comparison that counts):

- main: hash/C16 19.87 ms (1.000× main's best)
- main (A/A): hash/C16 19.88 ms (1.000× main's best)
- Eytzinger: hash/C16 19.87 ms (1.000× main's best)
- packed hash: hash/C16 17.88 ms (1.112× main's best)
- paired: hash/C16 19.51 ms (1.019× main's best)
- Eytzinger + paired: hash/C16 19.50 ms (1.019× main's best)
- packed + paired: hash/C16 17.75 ms (1.120× main's best)
- all three: hash/C16 17.73 ms (1.121× main's best)

A/A: largest |M2/M − 1| over configurations 0.0006; largest max/min of one arm's rounds 1.0036.
