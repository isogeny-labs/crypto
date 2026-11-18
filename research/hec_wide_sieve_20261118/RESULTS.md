# Wider sieves help when the residual is linear

All five complete solvers verified all 44 target scalars. Ratios below compare each fixed arm to the normalized original solver using ten-sample median times. The carried control repeats the previous implementation. No per-cell arm selection is included.

| Genus | Cases | Carried control | Double width, quadratic tail | Double width, linear tail | Full catalogue, no tail |
|---|---:|---:|---:|---:|---:|
| 2 | 24 | 0.919–1.372 | 0.561–1.278 | 0.724–1.936 | 0.272–1.234 |
| 3 | 12 | 1.493–2.022 | 1.100–1.666 | 1.328–1.820 | 0.921–1.826 |
| 4 | 8 | 2.086–3.743 | 2.168–4.742 | 2.232–5.031 | 2.195–4.020 |

The double-width linear-tail arm improved 42/44 medians versus original and all 44 equal-price REDC comparisons. Compared to the carried control it improved 21/24 genus-two, 2/12 genus-three, and 8/8 genus-four medians. On genus four it improved the carried wall time by factors 1.070–1.490. Its best original comparison, g4 p31 case 3, required 78 anchors against the carried control's 239 and the original's 199. Its wall and equal-price REDC ratios against original were 5.031 and 4.469. Full scanning discarded repeated or overlapping decompositions and cost more per anchor; on genus two it beat the original on only 4/24.

The earlier 6.313 carried-solver peak (g4 p31 case 0) measured 3.743 in this run, with unchanged arithmetic ratio 3.509. This is a same-host repeat comparison, not an independent-machine estimate. No arm met the tenfold target. The raw records include every tested width and threshold, discarded unready rows, all timing observations and the full arithmetic counters.
