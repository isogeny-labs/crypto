# Weighted pencil relations from checked power anchors

All four arms verified every scalar on all 44 targets. Direct visits to stored generator doubles produced finite rows even though splitting their support was never required. With one unshifted visit per bit, genus two supplied 1,396 admitted finite rows from 368 such visits, genus three 430 from 246, and genus four 84 from 172. Visits include degree skips. Counts describe admissions, not independent rank gains.

| Genus | Source schedule | Faster than continuous paired walk | Fewer anchors | Continuous / candidate time | Original normalized / candidate time |
|---|---|---:|---:|---:|---:|
| 2 | one visit per bit | 15/24 | 18/24 | 0.850–1.192× | 2.250–3.714× |
| 2 | two visits per bit | 13/24 | 16/24 | 0.879–1.327× | 2.118–4.262× |
| 3 | one visit per bit | 1/12 | 7/12 | 0.878–1.014× | 2.126–3.118× |
| 3 | two visits per bit | 3/12 | 7/12 | 0.896–1.046× | 2.351–3.266× |
| 4 | one visit per bit | 2/8 | 6/8 | 0.906–1.114× | 3.242–10.429× |
| 4 | two visits per bit | 3/8 | 5/8 | 0.928–1.140× | 3.554–10.664× |

Both variants increased total field reductions against the continuous paired walk on every genus-three target. Reusing a divisor saves a group step but loses the carried slope update at a table reset. Each table entry still needs a pencil scan. The implementation therefore supplies additional weighted relations but has mixed complete wall results. Only the previously identified genus-four p=31 case 1 exceeded tenfold with either new schedule. The full-panel objective remains open.

RUN.txt identifies the remote freeze. CONTROL.txt contains the 176 counted calls and 2,112 individual timings. summarize.rb regenerates COMPARISON.tsv.
