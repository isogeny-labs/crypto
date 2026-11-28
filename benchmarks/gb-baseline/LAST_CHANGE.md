## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 1.06 | unchanged |
| M31 | 3 | ok | 1.000 | 0.92 | unchanged |
| M31 | 4 | ok | 1.000 | 1.00 | unchanged |
| M31 | 5 | error | — | — | still error |
| M31 | 6 | error | — | — | still error |
| M31 | 7 | error | — | — | still error |
| M31 | 8 | error | — | — | still error |
| M31 | 9 | error | — | — | still error |
| N31 | 2 | ok | 1.000 | 0.94 | unchanged |
| N31 | 3 | ok | 1.000 | 0.96 | unchanged |
| N31 | 4 | ok | — | 0.54 | cost unit changed |
| N31 | 5 | ok | — | 0.74 | cost unit changed |
| N31 | 6 | ok | — | 0.75 | cost unit changed |
| N31 | 7 | ok | — | 0.90 | cost unit changed |
| N31 | 8 | ok | — | 0.83 | cost unit changed |
| N31 | 9 | ok | — | 0.69 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
