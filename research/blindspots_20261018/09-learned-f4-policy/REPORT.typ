#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: none)
#show table: set text(size: 8pt)

= Item 09: learned F4 pair selection and pivot order, third-pass assessment

Assessed 2026-10-18 from the committed `raw/eval-NN.jsonl` under the frozen rule of `PROTOCOL.md`; instrument `examples/blindspots_09_learned_f4.rs` (sha256 7e46e27e...0e4743, unchanged); analysis `analyze_all.py`. Host L0 (shared 4-vCPU container), toy scale, counted touches (xors + matrix words) per refutation.

== Verdict

*MATCHES (the null), level constant,* for the primary learned arm b50; TRADES also applies (b50 builds 16 to 25% more rows). `d` REGRESSES (ratio 0.84, 0.66, 0.67, intervals below 1 on all three cells). `c` is INCONCLUSIVE (1.00, 1.21, 1.00). The random half r50 is 7.28 and 6.00 against the incumbent at cells 17 and 19 and 0.75 at cell 21, so the random-subset clause is not met. Cell (13,3,4) was trained, not evaluated; it cannot change any arm verdict. No improvement over the index-calculus baseline is claimed. The gap to matched rho is not computed here (component measurement, no cross-method row); policy work, priced at 0, 0.1 and 1 touch-equivalent per operation, moves every ratio by under 0.003, so every ranking holds across the band.

#figure(image("figures/ratios_r3.svg", width: 100%), caption: [Left: sum touches(a) / sum touches(arm) at w = 0 with 95% paired bootstrap intervals, cells (17,4,3) n = 32, (19,4,3) n = 32, (21,5,3) n = 31. Right: matched-null ratios; above 1 the learned arm beats its null. Source: figures/cells.json from raw/eval-NN.jsonl.])

== Measured ratios (w = 0; 95% interval; derived from counted touches)

#table(columns: 4, align: (left, left, left, left),
[arm], [(17,4,3)], [(19,4,3)], [(21,5,3)],
[b50], [1.992 (1.966 to 2.020)], [2.322 (2.270 to 2.369)], [2.537 (2.524 to 2.549)],
[b50 `rows_a / rows_X`], [0.801], [0.827], [0.865],
[c], [1.000 (0.992 to 1.007)], [1.211 (1.185 to 1.244)], [0.999 (0.998 to 1.000)],
[d], [0.842 (0.810 to 0.878)], [0.656 (0.648 to 0.665)], [0.666 (0.662 to 0.671)],
[r50 (null of b50)], [7.278 (7.149 to 7.402)], [5.998 (5.841 to 6.149)], [0.751 (0.712 to 0.792)],
[p (null of c)], [0.946 (0.936 to 0.957)], [0.915 (0.900 to 0.928)], [0.977 (0.974 to 0.981)],
[cost(r50) / cost(b50)], [0.274 (0.269 to 0.279)], [0.387 (0.379 to 0.395)], [3.378 (3.192 to 3.575)],
[cost(p) / cost(c)], [1.057 (1.044 to 1.070)], [1.324 (1.284 to 1.370)], [1.023 (1.018 to 1.026)],
)

== What was measured

Cells 17, 19 and 21 evaluated (every `check` true in 702 records; reduced bases equal to the incumbent's; model digests agree across the cards of a cell). Cell 21 target 6 is satisfiable (6 solutions), censored at the incumbent and excluded. `d` is censored (counted at 1.5x the incumbent) on 0, 31 and 29 targets and `r50` on 0, 0 and 15, so their ratios favour them. Cell (13,3,4): 31 of 32 train targets refuted (mean 2.71e10 touches, cap 6e10), no evaluation card frozen; measured-command seconds in all receipts 5,628.5 of 7,200. Offline labelling (derived): about 118, 112 and 109 targets repay `b50` at cells 17, 19 and 21. Two relays (cards 20-r2, 22-r2) deviated: the `arm p` lines of two targets each omit ``"label_xors":0,``. Caveats: host L0, toy scale, replica engine, one factor base per cell, wall time descriptive.
