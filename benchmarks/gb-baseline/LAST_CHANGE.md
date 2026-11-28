## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 1.25 | unchanged |
| M31 | 3 | ok | 1.000 | 1.36 | unchanged |
| M31 | 4 | ok | 1.000 | 1.27 | unchanged |
| M31 | 5 | error | — | — | still error |
| M31 | 6 | error | — | — | still error |
| M31 | 7 | error | — | — | still error |
| M31 | 8 | error | — | — | still error |
| M31 | 9 | error | — | — | still error |
| N31 | 2 | ok | 1.000 | 1.20 | unchanged |
| N31 | 3 | ok | 1.000 | 1.23 | unchanged |
| N31 | 4 | ok | — | 2.27 | cost unit changed |
| N31 | 5 | ok | — | 1.67 | cost unit changed |
| N31 | 6 | ok | — | 1.30 | cost unit changed |
| N31 | 7 | ok | — | 1.48 | cost unit changed |
| N31 | 8 | ok | — | 1.53 | cost unit changed |
| N31 | 9 | ok | — | 1.22 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
