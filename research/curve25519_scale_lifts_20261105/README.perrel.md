
### Table D: per-relation cost at exact coverage across k (lifts=0 and lifts=1)

Multiply-adds per target divided by the exact coverage `cov/r` (lifts=0: covered points; lifts=1: projected points), full rows, from the JSON counts. The coverage is a property of the factor base: it falls with the subgroup order `r` (2 097 349 at k = 24, 33 557 809 at k = 28) and the base size (columns) changes with k, so this column rises with k while the per-target cost does not. This is the exact-coverage reading of the cost of one relation and is not an observed rate (there are no observed hits).

| cell | lifts | mul-adds/tgt / (cov/r), k=20 | k=24 | k=28 | 24/20 | 28/24 |
|:--|:-:|--:|--:|--:|--:|--:|
| bits t=5 G=3 | 0 | 9.49e+09 | 1.1e+11 | 1.76e+12 | 11.6 | 16 |
| bits t=5 G=3 | 1 | 1.32e+10 | 1.3e+11 | 1.59e+12 | 9.87 | 12.3 |
| bits t=6 G=3 | 0 | 1.4e+11 | 1.27e+12 | 2.21e+13 | 9.12 | 17.3 |
| bits t=6 G=3 | 1 | 1.43e+11 | 1.33e+12 | 2.13e+13 | 9.29 | 16 |
| isogeny t=4 G=0 | 0 | 3.81e+09 | 1.1e+11 | undef (cov 0) | 28.8 | n/a |
| isogeny t=4 G=0 | 1 | 3.39e+09 | 6.85e+10 | 1.75e+13 | 20.2 | 256 |
| isogeny t=5 G=0 | 0 | 7.21e+09 | 1.09e+11 | 6.04e+12 | 15.1 | 55.3 |
| isogeny t=5 G=0 | 1 | 6.41e+09 | 1.15e+11 | 5.31e+12 | 17.9 | 46.2 |
| isogeny t=6 G=0 | 0 | 4.23e+10 | 3.2e+11 | 6.51e+12 | 7.55 | 20.4 |
| isogeny t=6 G=0 | 1 | 4.66e+10 | 3.07e+11 | 6.08e+12 | 6.58 | 19.8 |
