# Run report: E6, yield of a pushed-forward factor base (2026-10-12, 02:27–02:30 UTC)

Descriptive record of one run.  Interpretation is in the round's
[README](../../README.md).  PDF: [`report.pdf`](report.pdf).

| item | value |
|:--|:--|
| lead | E6 of [`PROTOCOL.md`](../../PROTOCOL.md) |
| runner | `examples/isogeny_gap_pushforward_yield.rs` at commit `cb33477a1` |
| command | `isogeny_gap_pushforward_yield --pairs research/isogeny_conductor_gap_ic_20261010/pairs/prime_pairs.json --out research/isogeny_conductor_gap_round5_20261012/e6/records.jsonl` (20,000 targets per curve) |
| host | Claude Code cloud container; not an isolated host, and no wall times are reported |
| records | [`../../e6/records.jsonl`](../../e6/records.jsonl): 112 arm records (15 curves × 2 base sizes × direct + 2 to 4 quotient arms) |
| tables | [`../../TABLES.md`](../../TABLES.md), [`../../tables.json`](../../tables.json), from `examples/isogeny_gap_round5_tables.rs` |

**Workloads.**
- Crater, floor and isomorphic floor model of five 22-bit round-1 prime
  pairs: D = −7, 3⁶ (h = 72); D = −7, 5⁴ (h = 200); D = −3, 2⁸ (h = 12);
  D = −8, 3⁶ (h = 18); D = −4, 5⁴ (h = 50).
- Bases of the `c` = 32 and 64 smallest abscissae outside `E[h]`.
- Arms: the direct base, and the quotients by `C2`, `C4`, `E[2]` (where
  rational) and `E(F_p)[h]`.

**Completed.**  Every arm completed.  Every decomposition found was lifted
back to points of `E` and checked: 0 failures.

**Measured.**  Exact counts of decompositions over the same 20,000
targets per curve.  Pooled ratio of quotient to direct, with 95 % Poisson
intervals:

| subgroup | \|H\| | cases | ratio | 95 % interval |
|:--|--:|--:|--:|:--|
| C2 | 2 | 30 | 1.81 | 1.64 to 1.99 |
| C4 | 4 | 16 | 4.08 | 3.52 to 4.75 |
| E[2] | 4 | 6 | 3.57 | 2.76 to 4.60 |
| E[h] | 12 | 6 | 8.43 | 6.00 to 11.86 |
| E[h] | 18 | 6 | 21.87 | 17.70 to 27.01 |
| E[h] | 50 | 6 | 44.07 | 39.24 to 49.50 |
| E[h] | 72 | 6 | 79.57 | 67.54 to 93.73 |
| E[h] | 200 | 6 | 139.39 | 103.62 to 187.50 |

The fit (derived, in `TABLES.md`) is ln(ratio) on ln|H| by weighted least
squares: exponent 1.001 ± 0.036.

**F4.**  One random target per arm:
- On the 106 targets with no solution, cost is the same at matched columns
  (degree 33–34 and 65–66, widest matrix 183–189 and 375–381).
- Multiplications are 136,476–145,630 (c = 32) and 581,788–600,510 (c = 64).
  The spread comes from image collisions that leave 31 or 63 columns.
- On the 6 targets with a solution, cost is 483,561 (c = 32) and 2,991,006
  to 6,313,667 (c = 64).

**Graph.**  [`e6_yield_ratio.svg`](e6_yield_ratio.svg), from
[`plot.py`](plot.py) (plots `tables.json`).
