runs: 18; all correctness gates pass: True

| q | seeds | t_ic baseline s | A/A ratio median [min, max] | lp:0.5 ratio median [min, max] | lp:0.5 faster | lp:0.35 ratio median [min, max] | lp:0.35 faster | unknowns base / lp:0.5 | IC/rho-exp base | IC/rho-exp lp:0.5 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2003 | 4 | 2.26 | 0.975 [0.902, 0.987] | 0.55 [0.52, 0.93] | 0/4 | 0.28 [0.24, 0.48] | 0/4 | 1012 / 506 | 3.88 | 7.36 |

| q | arm | t_rel s | t_LA s | t_descent s | candidates | partials | combined | dropped (>=2 LP) |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|
| 2003 | baseline | 1.13 | 0.76 | 0.28 | 25275 | 0 | 0 | 0 |
| 2003 | lp50 | 3.52 | 0.11 | 0.42 | 70890 | 378 | 337 | 2025 |
| 2003 | lp35 | 7.66 | 0.04 | 0.68 | 153687 | 427 | 265 | 5601 |
