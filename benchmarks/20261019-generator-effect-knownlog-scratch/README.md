# E1 known-log generator-effect dry runs (scratch)

This browsing copy points to the full scratch evidence in [`research/generator_effect_ic_20261019/`](../../research/generator_effect_ic_20261019/), including the raw ECbench records, session seals, specifications, aggregation JSON, and plot source. **This is explicitly scratch, not a frozen protocol or canonical benchmark result.** The curves are 24-bit toy instances, the host ran at L0 without pinning or isolation, and wall time is not used. No security claim or exponent change is made.

Dry run 1 compared an arithmetic-progression known-log base with the abscissa base on two curves (16 planted targets, two measured rounds). The relation-yield ratio is 0.0097 [0.0074, 0.0130] and 0.0092 [0.0072, 0.0126] (90% bootstrap intervals), and at T = 1 the total-S intervals contain 1.

Dry run 2 (PLAN Amendment 1) adds a random-scalar known-log base, a BSGS comparator at two table sizes, and a 3000-target yield block. The pre-registered sumset hit-rate predictions held within about 3% for both bases. The random base matches the abscissa yield (0.989 [0.958, 1.020] and 0.976 [0.947, 1.005]) and its total S at T = 1 is 11% to 39% below the abscissa baseline across the 0 / 0.1 / 1 GAE pricing band, but it is not below BSGS at equal memory, so the amended decision rule reads "no improvement". Dry run 1's multi-target model was wrong and is withdrawn in the report.

- [Dry run 1 graph (SVG)](dryrun.svg), [PNG](dryrun.png)
- [Dry run 2 graph (SVG)](dryrun2.svg), [PNG](dryrun2.png)
- [Full report](DRYRUN.pdf)
- [Raw report and limitations](../../research/generator_effect_ic_20261019/DRYRUN.md)
