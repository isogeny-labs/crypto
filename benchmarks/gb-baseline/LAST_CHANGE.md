## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **status timeout -> error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 1.62 | unchanged |
| M31 | 3 | ok | 1.000 | 1.61 | unchanged |
| M31 | 4 | ok | 1.000 | 1.53 | unchanged |
| M31 | 5 | error | — | — | status timeout -> error |
| M31 | 6 | error | — | — | status timeout -> error |
| M31 | 7 | error | — | — | status timeout -> error |
| M31 | 8 | error | — | — | status timeout -> error |
| M31 | 9 | error | — | — | status timeout -> error |
| N31 | 2 | ok | 1.000 | 1.25 | unchanged |
| N31 | 3 | ok | 1.000 | 1.50 | unchanged |
| N31 | 4 | ok | — | 1.50 | cost unit changed |
| N31 | 5 | ok | — | 2.98 | cost unit changed |
| N31 | 6 | ok | — | 1.34 | cost unit changed |
| N31 | 7 | ok | — | 1.48 | cost unit changed |
| N31 | 8 | ok | — | 1.46 | cost unit changed |
| N31 | 9 | ok | — | 1.50 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
