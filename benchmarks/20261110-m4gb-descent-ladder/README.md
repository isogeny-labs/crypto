# M4GB on the Weil-descent curve ladder: one decomposition call per engine (2026-11-10)

Copies for browsing of
[`research/m4gb_descent_ladder_20261110/`](../../research/m4gb_descent_ladder_20261110/README.md)
(source of truth: protocol, per-cell JSON, scripts). Benchmark record
`benchmarks/records/20261110T1513Z-m4gb-descent-ladder.json`.

- `REPORT.pdf`: protocol, results and the full table.
- `results.svg` / `results.png`: median wall per call and cell for `fes-f2`,
  `f4-f2`, `inherited-f4` and `m4gb-f2`, Koblitz and random binary curves,
  12 to 32 unknowns at `m = 2` and 12 to 15 at `m = 3`, log scale.

Headline `[pcc:unverified]`: `m4gb-f2` below the in-house full-basis `f4-f2`
from about 22 unknowns on (0.24× at 24, 0.08× at 28; `f4-f2` over budget at
30 to 32 while M4GB decides every target), never below `inherited-f4` (10 to
90× slower) or the exhaustive reference (7 to 200×); 0 disagreements. A stage
diagnostic of one oracle call (AGENTS.md §2), local container, one thread: no
table row in `benchmarks/README.md`; the record and the progress report list
it.
