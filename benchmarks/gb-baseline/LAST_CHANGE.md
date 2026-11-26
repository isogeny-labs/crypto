## Gröbner baseline re-benchmark

Counts: 6 **cost unit changed**, 5 **still error**, 5 **unchanged**

| curve | m | status | counted cost new/base | wall new/base | verdict |
|:--|--:|:--|--:|--:|:--|
| M31 | 2 | ok | 1.000 | 0.92 | unchanged |
| M31 | 3 | ok | 1.000 | 0.95 | unchanged |
| M31 | 4 | ok | 1.000 | 1.03 | unchanged |
| M31 | 5 | error | — | — | still error |
| M31 | 6 | error | — | — | still error |
| M31 | 7 | error | — | — | still error |
| M31 | 8 | error | — | — | still error |
| M31 | 9 | error | — | — | still error |
| N31 | 2 | ok | 1.000 | 1.04 | unchanged |
| N31 | 3 | ok | 1.000 | 1.01 | unchanged |
| N31 | 4 | ok | — | 0.92 | cost unit changed |
| N31 | 5 | ok | — | 0.44 | cost unit changed |
| N31 | 6 | ok | — | 0.95 | cost unit changed |
| N31 | 7 | ok | — | 0.85 | cost unit changed |
| N31 | 8 | ok | — | 0.99 | cost unit changed |
| N31 | 9 | ok | — | 1.02 | cost unit changed |

_Counted cost is the harness's own unit and decides faster/slower (tolerance 1%); wall time is descriptive (shared runner). Stage diagnostics only, `[pcc:unverified]`._
