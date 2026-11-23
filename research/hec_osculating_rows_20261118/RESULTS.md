# Small-place multiples lower anchor counts but raise complete arithmetic cost

Every arm verified every scalar on the 44 matched fixtures. Each inserted startup row passed the group-law and linear-solver audits. Each ratio below divides the comparator median wall time by the candidate median on the same input.

| Genus | Orders | Startup rows per input | Fewer anchors vs prior | Faster vs prior | Prior / candidate wall | Original / candidate wall |
|---|---|---:|---:|---:|---:|---:|
| 2 | g+1 | 2–18 | 14/24 | 0/24 | 0.862–0.980× | 1.153–2.254× |
| 2 | both | 6–37 | 23/24 | 1/24 | 0.808–1.006× | 1.199–2.198× |
| 3 | g+1 | 0–3 | 6/12 | 0/12 | 0.781–0.862× | 1.435–2.583× |
| 3 | both | 1–5 | 8/12 | 0/12 | 0.718–0.786× | 1.286–2.428× |
| 4 | g+1 | 0–1 | 3/8 | 1/8 | 0.817–1.059× | 2.152–4.417× |
| 4 | both | 1 | 7/8 | 1/8 | 0.787–1.052× | 2.026–4.311× |

Both startup schedules increased total field reductions on every fixture. The two-order schedule reduced anchor visits on 38/44 inputs but beat the prior median on only 2/44. Keep startup disabled. A cheaper construction would have to recover the heavy production cost even on the genus-two cells where admissible rows were plentiful. The complete-time target remains open.

The frozen source commit is recorded in RUN.txt. CONTROL.txt holds all 176 counted calls and 2,112 individual timings. COMPARISON.tsv gives the matched comparisons. EXIT.txt records normal completion.
