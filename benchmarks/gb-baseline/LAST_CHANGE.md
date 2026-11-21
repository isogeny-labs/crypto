## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still timeout**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 0.71 | unchanged |
| M31 | 3 | ok | 1.000 | 0.79 | unchanged |
| M31 | 4 | ok | 1.000 | 0.80 | unchanged |
| M31 | 5 | timeout | — | — | still timeout |
| M31 | 6 | timeout | — | — | still timeout |
| M31 | 7 | timeout | — | — | still timeout |
| M31 | 8 | timeout | — | — | still timeout |
| M31 | 9 | timeout | — | — | still timeout |
| N31 | 2 | ok | 1.000 | 0.94 | unchanged |
| N31 | 3 | ok | 1.000 | 0.86 | unchanged |
| N31 | 4 | ok | — | 1.05 | cost unit changed |
| N31 | 5 | ok | — | 0.73 | cost unit changed |
| N31 | 6 | ok | — | 0.43 | cost unit changed |
| N31 | 7 | ok | — | 0.89 | cost unit changed |
| N31 | 8 | ok | — | 0.77 | cost unit changed |
| N31 | 9 | ok | — | 0.76 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
