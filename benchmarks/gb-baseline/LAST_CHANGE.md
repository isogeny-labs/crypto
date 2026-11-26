## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 1.13 | unchanged |
| M31 | 3 | ok | 1.000 | 1.29 | unchanged |
| M31 | 4 | ok | 1.000 | 1.34 | unchanged |
| M31 | 5 | error | — | — | still error |
| M31 | 6 | error | — | — | still error |
| M31 | 7 | error | — | — | still error |
| M31 | 8 | error | — | — | still error |
| M31 | 9 | error | — | — | still error |
| N31 | 2 | ok | 1.000 | 1.20 | unchanged |
| N31 | 3 | ok | 1.000 | 1.39 | unchanged |
| N31 | 4 | ok | — | 1.38 | cost unit changed |
| N31 | 5 | ok | — | 1.47 | cost unit changed |
| N31 | 6 | ok | — | 1.43 | cost unit changed |
| N31 | 7 | ok | — | 1.36 | cost unit changed |
| N31 | 8 | ok | — | 0.97 | cost unit changed |
| N31 | 9 | ok | — | 0.92 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
