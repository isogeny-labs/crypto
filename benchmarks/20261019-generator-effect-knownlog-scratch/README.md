# E1 known-log generator-effect dry run (scratch)

This browsing copy points to the full scratch evidence in [`research/generator_effect_ic_20261019/`](../../research/generator_effect_ic_20261019/), including the raw ECbench records, session seals, specifications, aggregation JSON, and plot source. **This is explicitly scratch, not a frozen protocol or canonical benchmark result.** The curves are 24-bit toy instances, the host ran at L0 without pinning or isolation, and wall time is not used.

The two arms compare an arithmetic-progression known-log base with the abscissa base on two curves, with 16 planted targets and two measured rounds per curve. All 288 records, including warm-up, were verified. The measured known-log/abscissa relation-yield ratios are 0.0097 [0.0074, 0.0130] and 0.0092 [0.0072, 0.0126] (90% bootstrap intervals). At T = 1, the total-S intervals contain 1; this is reported as accounting/no improvement. The model projects a growing disadvantage as T rises. No security claim or exponent change is made.

- [Dry-run graph (SVG)](dryrun.svg)
- [Dry-run graph (PNG)](dryrun.png)
- [Full dry-run report](DRYRUN.pdf)
- [Raw report and limitations](../../research/generator_effect_ic_20261019/DRYRUN.md)
