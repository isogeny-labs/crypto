# Exact slope updates on fresh subgroup generators

All five arms solved all 44 new source/target pairs on the eleven original curves. Direct and rolling place walks agreed on the recovered scalar, visited anchors and accepted relation counts in every fixture. The audit checked individual rolling slopes against direct evaluation. None of the 44 sources passed the short-multiple check (17 of 22 passed in the earlier source construction).

| Genus | Cases | Rolling versus original median wall ratio | Faster cells | Equal-price REDC ratio | Rolling versus table-only wall ratio |
|---|---:|---:|---:|---:|---:|
| 2 | 24 | 0.931–1.258 | 19 | 0.999–1.267 | 0.925–1.035 |
| 3 | 12 | 1.193–1.482 | 12 | 1.403–2.102 | 0.983–1.110 |
| 4 | 8 | 1.313–2.615 | 8 | 1.757–3.010 | 1.116–1.200 |

Compared directly to table-only evaluation, rolling improved wall medians on 23/24, 11/12 and 8/8 cells respectively; its equal-price REDC count improved on all 44. The combined place-walk, inverse-table and rolling method improved 39/44 median times relative to the original solver. All five regressions were genus two. Near-unity differences are descriptive on this single host. The complete change achieved at most 2.615 times the baseline performance on these instances. The recurrence alone contributed at most 1.200 times relative to its table-only control. Ten wall observations per fixture/arm and one independent full16 cost capture are in CONTROL.txt.
