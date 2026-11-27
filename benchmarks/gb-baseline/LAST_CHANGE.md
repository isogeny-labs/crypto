## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 0.64 | unchanged |
| M31 | 3 | ok | 1.000 | 0.59 | unchanged |
| M31 | 4 | ok | 1.000 | 0.62 | unchanged |
| M31 | 5 | error | — | — | still error |
| M31 | 6 | error | — | — | still error |
| M31 | 7 | error | — | — | still error |
| M31 | 8 | error | — | — | still error |
| M31 | 9 | error | — | — | still error |
| N31 | 2 | ok | 1.000 | 0.73 | unchanged |
| N31 | 3 | ok | 1.000 | 0.63 | unchanged |
| N31 | 4 | ok | — | 0.63 | cost unit changed |
| N31 | 5 | ok | — | 0.34 | cost unit changed |
| N31 | 6 | ok | — | 0.69 | cost unit changed |
| N31 | 7 | ok | — | 0.72 | cost unit changed |
| N31 | 8 | ok | — | 0.98 | cost unit changed |
| N31 | 9 | ok | — | 0.88 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
