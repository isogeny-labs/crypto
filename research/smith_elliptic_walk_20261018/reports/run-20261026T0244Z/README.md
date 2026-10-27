# Run report: W3a Q2 sampled walks, run-20261026T0244Z

This run report records observations only. Interpretation is in
[`../../README.md`](../../README.md) and [`../../REPORT.pdf`](../../REPORT.pdf).
PDF: [`report.pdf`](report.pdf).

## Run

| Item | Value |
|---|---|
| Command | `scripts/run_walks.sh <bin> data/walks` (cells run by `scripts/walk_cell.sh`) |
| Arguments | `--seed 20261018 --max-nodes 20000000 --max-secs 3300 --cell-secs 40000` |
| Walk binary | sha256 `5e9444cd96568f67…` |
| Fit binary | `aa0829690ca458a4…` (`fit --dir data/walks`) |
| Calibration | `calib --out data/walks`, run before the walks |
| Start / end | 2026-10-26T02:44Z / 04:01Z |
| Host | Apple M4 Pro, shared, no pinning, 4 cells at once, `nice 10` |

## Arms, primes and walks

- Arm A runs from a failing JV-split weak start to the first passing JV
  model.
- Arm B runs from a random full-2-torsion curve to the first JV-weak node.
- Primes: the first prime above `2^k` for k = 10–16, 18 and 20.
- Walks per arm and prime: 40, 24, 12 or 6, so 250 per arm. All 500
  completed and every cell exited 0.

Censoring, by cause, per row of `fit.json`, arm A then arm B:
- trap rule: 70, 46;
- isolated start: 0, 32;
- step cap: 0, 39;
- node cap: 8, 8;
- wall cap: 0, 0.

## Measured values

All values are in `data/walks/fit.json` and `walk_*.jsonl`. Successes per
walk, arm A:
- 35/40, 33/40, 34/40, 3/40 at k = 10–13;
- 23/24, 23/24, 3/24 at k = 14–16;
- 12/12 at k = 18 and 6/6 at k = 20.

Fits:
- The frozen fit gives an arm A cost exponent of −0.14 [−0.80, 0.95]. Its
  label is `o(p)`.
- Post hoc, over successful walks at `p ≡ 3 (mod 4)`, the arm A nodes
  exponent is 1.07 [0.95, 1.19].

Calibration (measured on the shared host): 5.12 ns per `F_p` multiplication,
and one inversion costs 7.64 multiplications, at `p = 65537`.

## Records and figures

Raw records are in `../../data/walks/`. Pilot walks from intermediate
binaries are in `../../data/pilot/` and are not used.

Graph: `q2_walks.svg`, a copy of the study figure drawn by `../../plot.py`.
