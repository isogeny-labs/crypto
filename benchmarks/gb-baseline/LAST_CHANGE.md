## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **status timeout -> error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 1.59 | unchanged |
| M31 | 3 | ok | 1.000 | 1.66 | unchanged |
| M31 | 4 | ok | 1.000 | 1.61 | unchanged |
| M31 | 5 | error | — | — | status timeout -> error |
| M31 | 6 | error | — | — | status timeout -> error |
| M31 | 7 | error | — | — | status timeout -> error |
| M31 | 8 | error | — | — | status timeout -> error |
| M31 | 9 | error | — | — | status timeout -> error |
| N31 | 2 | ok | 1.000 | 1.30 | unchanged |
| N31 | 3 | ok | 1.000 | 1.60 | unchanged |
| N31 | 4 | ok | — | 1.63 | cost unit changed |
| N31 | 5 | ok | — | 1.67 | cost unit changed |
| N31 | 6 | ok | — | 1.69 | cost unit changed |
| N31 | 7 | ok | — | 1.62 | cost unit changed |
| N31 | 8 | ok | — | 1.65 | cost unit changed |
| N31 | 9 | ok | — | 1.56 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
