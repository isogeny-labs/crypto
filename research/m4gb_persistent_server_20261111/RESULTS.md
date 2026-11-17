Median over targets of the per-target median wall, ms, one call. Ratio columns: `m4gb-f2-server` over `m4gb-f2` (the persistent server over the one-shot process), the median over targets of the paired per-target saving in ms with the count of targets on which the median wall of the server is below that of the one-shot process, and the server over the reference engine of the cell. `dec` = targets decided in every repetition / targets; `ob` = targets with a budget verdict; `agree` = every decided answer equals the reference.

| family | n | n' | m | vars | ref | `inherited-f4` ms (dec, ob, agree, Dmax) | `m4gb-f2` ms (dec, ob, agree, Dmax) | `m4gb-f2-server` ms (dec, ob, agree, Dmax) | server / one-shot | paired saving ms (targets server below one-shot) | server / ref |
|:--|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|
| K | 11 | 6 | 2 | 12 | `inherited-f4` | 0.53 (6/6, 0, y, 3) | 8.90 (6/6, 0, y, 3) | 3.22 (6/6, 0, y, -) | 0.36× | 5.7 (6/6) | 6.03× |
| R | 11 | 6 | 2 | 12 | `inherited-f4` | 0.65 (6/6, 0, y, 3) | 9.00 (6/6, 0, y, 3) | 4.12 (6/6, 0, y, -) | 0.46× | 5.0 (6/6) | 6.38× |
| K | 13 | 7 | 2 | 14 | `inherited-f4` | 0.87 (6/6, 0, y, 3) | 15.0 (6/6, 0, y, 3) | 7.52 (6/6, 0, y, -) | 0.50× | 5.7 (6/6) | 8.65× |
| R | 13 | 7 | 2 | 14 | `inherited-f4` | 1.20 (6/6, 0, y, 3) | 15.4 (6/6, 0, y, 3) | 10.9 (6/6, 0, y, -) | 0.71× | 5.9 (5/6) | 9.06× |
| K | 15 | 8 | 2 | 16 | `inherited-f4` | 1.98 (6/6, 0, y, 3) | 20.1 (6/6, 0, y, 4) | 13.9 (6/6, 0, y, -) | 0.69× | 6.1 (6/6) | 6.99× |
| R | 15 | 8 | 2 | 16 | `inherited-f4` | 1.71 (6/6, 0, y, 3) | 22.7 (6/6, 0, y, 3) | 17.0 (6/6, 0, y, -) | 0.75× | 5.0 (6/6) | 9.94× |
| K | 17 | 9 | 2 | 18 | `inherited-f4` | 2.54 (6/6, 0, y, 3) | 35.3 (6/6, 0, y, 3) | 35.3 (6/6, 0, y, -) | 1.00× | 6.1 (4/6) | 13.88× |
| R | 17 | 9 | 2 | 18 | `inherited-f4` | 2.42 (6/6, 0, y, 3) | 27.9 (6/6, 0, y, 3) | 23.9 (6/6, 0, y, -) | 0.85× | 3.8 (5/6) | 9.87× |
| K | 19 | 10 | 2 | 20 | `inherited-f4` | 4.20 (6/6, 0, y, 3) | 33.3 (6/6, 0, y, 3) | 26.5 (6/6, 0, y, -) | 0.80× | 7.9 (6/6) | 6.32× |
| R | 19 | 10 | 2 | 20 | `inherited-f4` | 5.42 (6/6, 0, y, 3) | 45.3 (6/6, 0, y, 3) | 37.2 (6/6, 0, y, -) | 0.82× | 6.7 (5/6) | 6.86× |
| R | 21 | 11 | 2 | 22 | `inherited-f4` | 7.92 (6/6, 0, y, 3) | 196.3 (6/6, 0, y, 4) | 170.2 (6/6, 0, y, -) | 0.87× | 12.5 (5/6) | 21.49× |
| K | 23 | 12 | 2 | 24 | `inherited-f4` | 14.5 (6/6, 0, y, 3) | 421.2 (6/6, 0, y, 4) | 402.0 (6/6, 0, y, -) | 0.95× | 23.4 (5/6) | 27.68× |
| R | 23 | 12 | 2 | 24 | `inherited-f4` | 14.3 (6/6, 0, y, 3) | 289.4 (6/6, 0, y, 4) | 285.7 (6/6, 0, y, -) | 0.99× | 20.4 (4/6) | 19.91× |
| K | 11 | 4 | 3 | 12 | `inherited-f4` | 22.2 (6/6, 0, y, 6) | 6206.8 (6/6, 0, y, 7) | 6364.0 (6/6, 0, y, -) | 1.03× | -187.7 (2/6) | 286.21× |
| R | 11 | 4 | 3 | 12 | `inherited-f4` | 24.0 (6/6, 0, y, 6) | 6123.5 (6/6, 0, y, 7) | 6457.5 (6/6, 0, y, -) | 1.05× | -311.9 (1/6) | 269.49× |
| K | 13 | 5 | 3 | 15 | `inherited-f4` | 186.4 (6/6, 0, y, 6) | 46186.5 (2/6, 4, y, 7) | 45441.3 (2/6, 4, y, -) | 0.98× | 745.2 (1/2) | 243.78× |
| R | 13 | 5 | 3 | 15 | `inherited-f4` | 176.4 (6/6, 0, y, 6) | 46591.1 (2/6, 4, y, 7) | 46297.3 (2/6, 4, y, -) | 0.99× | 293.8 (2/2) | 262.47× |
| K | 15 | 5 | 3 | 15 | `inherited-f4` | 172.0 (6/6, 0, y, 6) | n/a (0/6, 6, y, -) | n/a (0/6, 6, y, -) | n/a | n/a | n/a |
| R | 15 | 5 | 3 | 15 | `inherited-f4` | 174.8 (6/6, 0, y, 6) | n/a (0/6, 6, y, -) | n/a (0/6, 6, y, -) | n/a | n/a | n/a |
