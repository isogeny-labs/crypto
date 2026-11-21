# Concrete-size evaluation  [pcc:unverified]

Growth fits (least squares over bits 8..16; H and log|Delta_W| in nats; medians of the height subsample):

| r | H(b) = a + c*b | log|Delta_W|(b) = a + c*b | rho_G median over cells (max) |
|---|---|---|---|
| 2 | -2.60 + 0.686 b | -57.4 + 13.94 b | 1.109 (2.677) |
| 3 | -2.82 + 0.793 b | -64.8 + 17.84 b | 1.141 (1.715) |
| 4 | -3.10 + 0.941 b | -69.3 + 22.24 b | 1.148 (2.366) |
| 5 | -5.33 + 1.448 b | -117.7 + 36.09 b | 1.118 (1.899) |
| 6 | -9.75 + 2.748 b | -251.8 + 74.94 b | 1.065 (1.204) |

## Unconditional bound (Theorem 2: Petsche + sigma <= log|Delta|/log 11, Mazur T <= 16, factor 1/2 normalisation slack)

| r | rho at 256 bits | log2 C_sharp at 256 | margin at 256 bits (bits) | smallest b excluded (sharp, T=16) | smallest b excluded (sharp, T=1) | smallest b excluded (lean, T=16) |
|---|---|---|---|---|---|---|
| 2 | 6.634e+35 | 64.5 | -4 | 265 | 248 | 269 |
| 3 | 2.765e+36 | 128.2 | -261 | 842 | 816 | 854 |
| 4 | 1.032e+37 | 194.4 | -655 | 1773 | 1739 | 1798 |
| 5 | 1.908e+38 | 267.6 | -1216 | 3134 | 3092 | 3176 |
| 6 | 1.545e+40 | 350.7 | -1983 | 5015 | 4964 | 5078 |

## Conditional bound (Theorem 1 with a height-ratio certificate rho), margin in bits at 256-bit q, T = 1

Positive margin = the bound excludes beating rho at 256 bits (expected xedni trials > sqrt(q)), with that many bits of slack. r = 7..9 use the r = 6 measured rho (the Weierstrass path stops at r = 6).

| r | rho used | margin sharp | margin lean | rho x4: margin sharp | rho x10: margin sharp |
|---|---|---|---|---|---|
| 2 | 1.109 | 122.4 | 119.6 | 120.7 | 119.6 |
| 3 | 1.141 | 114.7 | 106.0 | 108.0 | 104.0 |
| 4 | 1.148 | 98.0 | 83.1 | 85.8 | 77.9 |
| 5 | 1.118 | 74.0 | 50.3 | 54.0 | 40.7 |
| 6 | 1.065 | 42.1 | 6.8 | 12.1 | -7.7 |
| 7 | 1.065 | 0.2 | -48.0 | -41.8 | -69.6 |
| 8 | 1.065 | -51.6 | -114.5 | -107.6 | -144.6 |
| 9 | 1.065 | -113.5 | -193.2 | -185.5 | -233.1 |
