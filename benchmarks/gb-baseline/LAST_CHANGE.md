## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still timeout**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 0.60 | unchanged |
| M31 | 3 | ok | 1.000 | 0.60 | unchanged |
| M31 | 4 | ok | 1.000 | 0.61 | unchanged |
| M31 | 5 | timeout | — | — | still timeout |
| M31 | 6 | timeout | — | — | still timeout |
| M31 | 7 | timeout | — | — | still timeout |
| M31 | 8 | timeout | — | — | still timeout |
| M31 | 9 | timeout | — | — | still timeout |
| N31 | 2 | ok | 1.000 | 0.70 | unchanged |
| N31 | 3 | ok | 1.000 | 0.61 | unchanged |
| N31 | 4 | ok | — | 0.85 | cost unit changed |
| N31 | 5 | ok | — | 0.78 | cost unit changed |
| N31 | 6 | ok | — | 0.34 | cost unit changed |
| N31 | 7 | ok | — | 0.74 | cost unit changed |
| N31 | 8 | ok | — | 0.67 | cost unit changed |
| N31 | 9 | ok | — | 0.70 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
