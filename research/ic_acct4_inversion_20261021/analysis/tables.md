## Equivalence with the committed study

| genus | p | IC runs | rho runs | identical |
|--:|--:|--:|--:|:--|
| 2 | 41 | 5 | 40 | yes |
| 2 | 61 | 5 | 40 | yes |
| 2 | 101 | 5 | 40 | yes |
| 3 | 23 | 5 | 40 | yes |
| 3 | 31 | 5 | 40 | yes |

## Instructions per counted gate (Callgrind, marginal)

| row | IC | rho | IC/rho after | IC/rho before (recorded) |
|:--|--:|--:|--:|--:|
| genus 2, p = 41 | 1.212 | 1.304 | 0.930 | 1.01 |
| genus 3, p = 23 | 1.611 | 1.681 | 0.958 | 1.16 |

## Mean bitop/v1 per run, with and without operand re-validation

| genus | p | IC total | IC algorithm | rho total | rho algorithm | IC/rho total | IC/rho algorithm |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 41 | 2.3485e7 | 1.1331e7 | 1.1338e8 | 4.6572e7 | 0.207 | 0.243 |
| 2 | 61 | 2.4550e7 | 1.3208e7 | 1.1092e8 | 4.9356e7 | 0.221 | 0.268 |
| 2 | 101 | 5.8260e7 | 3.4612e7 | 1.9504e8 | 8.5186e7 | 0.299 | 0.406 |
| 3 | 23 | 3.2577e7 | 1.9826e7 | 1.6328e8 | 8.7641e7 | 0.200 | 0.226 |
| 3 | 31 | 4.4100e7 | 2.7922e7 | 1.7179e8 | 9.3380e7 | 0.257 | 0.299 |
