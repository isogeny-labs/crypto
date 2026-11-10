# M4GB on the Weil-descent curve ladder: solve time per target (2026-11-10)

Owner's ask: run the decomposition benchmarks on various curves and see whether
solve time is lower with the new external backend. Follow-up to
[`research/m4gb_integration_20261110/`](../m4gb_integration_20261110/README.md)
(backend `m4gb-f2`, patches, kernel A/B).

**Status words.** Everything here is a local development check on the session
container (4 vCPU Intel Xeon 2.10 GHz, 16 GiB, shared with a Rust build at
times), `[pcc:unverified]`. It prices one decomposition-oracle call, so by
`AGENTS.md` §2 it is a stage diagnostic and never a speed row.

## Instrument

`ic descent --solver …` ([`src/bin/ic/descent.rs`](../../src/bin/ic/descent.rs),
`price_engine_cell` in `ic_descent_degrees.rs`): for each cell
`(family, n, n', m)` it builds the Weil descent of the Semaev polynomial over
the same seeded targets, hands the Boolean system (`m·n'` unknowns, `n`
equations) to every listed engine, interleaves the engines per target with the
engine order rotating each repetition, and checks every decided answer against
the reference engine (the strongest exhaustive search listed: `fes-f2` for the
quadratic `m = 2` cells, `exhaustive` otherwise).

- Families: `K` (the Koblitz curve of degree `n`) and `R` (a random binary
  curve of the same degree, points counted exhaustively, which bounds `n`).
- Cells: `m = 2`, `n = 11, 13, …, 31` with `n' = ⌈n/2⌉` (12 to 32 unknowns);
  `m = 3`: `11:4:3`, `13:5:3`, `15:5:3`.
- Engines: `fes-f2` (reference), `f4-f2` (in-house F4, full reduced basis),
  `inherited-f4` (the Koblitz oracle's degree-bounded hybrid), `m4gb-f2`
  (M4GB upstream `01860a52` with patches 0001 + 0002, one thread,
  `M4GB_BINARY` a directory of binaries built for 16, 20, 24, 28, 30, 32, 35,
  36, 40 and 64 variables; the narrowest that fits is used).
- Targets 6 per cell, repeats 2, budget 60 s per target and engine (a first
  attempt with 8 targets, 3 repeats and 300 s was killed by the session's
  two-hour cap before `ic descent` wrote its single output file; nothing of it
  survived, see the session log).
- One `ic descent` invocation per cell and family, each writing its own JSON
  under `results/`; for each family the `m = 2` ladder stops once `m4gb-f2`
  exhausted the budget on every target of a cell.
- Launcher: [`run_ladder.sh`](run_ladder.sh), run under
  `tools/benchrec.py run` (record `20261110T1513Z-m4gb-descent-ladder`).

## Pre-stated expectation

`inherited-f4` and `f4-f2` decide each target below `m4gb-f2` on every cell:
they stop at a degree-bounded decision while M4GB computes the full reduced
basis of the same ideal, and M4GB pays a process start, a file round trip and
its own table initialisation per call (tens of milliseconds). The ratio
M4GB / F4 should grow with the number of unknowns. Falsifier: `m4gb-f2`
median wall below `f4-f2` on any cell with at least 24 unknowns.

## Results

(filled by `summarize.py` from `results/ladder.json`; see `RESULTS.md`)
