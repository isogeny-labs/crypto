# IC collection: the aimed subset gathered once per unit (2026-10-23)

Aimed collection scans, every trial, a window of a fixed subset of the factor
base. It used to gather that window's coordinates on every trial. Now they are
gathered once per unit (`Targets::Subset`'s `doubled_x` and `doubled_y`), and
the trial's scan (`Scan::Gathered`) hands two slices to the folding kernel. The
same summands go in, in the same order: every count and recovered logarithm is
identical at `icv1-f2m53-tm56619371-dac20a85` (n53) and `icv1-f2m61-t158598901-ab42b6c5` (n61).

| measurement | result |
|:--|:--|
| Callgrind, instructions per `ic price` process | n53 1.023× (−0.73 × 10^9); n61 1.029× (−4.0 × 10^9) |
| perf, share of native samples | gather closure 2.5–2.7% → gone; scan family 20.6–21.6% → 18.3–19.1% |
| native collect phase, 30 paired rounds | median 1.111 [1.065, 1.160], 28/30 above 1 |
| native whole set-up, 30 paired rounds | median 1.037 [1.018, 1.068], 26/30 above 1 |

**Native verdict: not established under the declared rule.** Both intervals
exclude 1. The rule also requires each interval's lower end to clear the A/A
interval's upper end (collect 0.944–1.087, set-up 0.953–1.121, from 5 pairs),
and neither does.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), committed in `8660e8ab4` before any
  measurement.
- Report: [`report/report.pdf`](report/report.pdf).
- Raw data: `runs/`.
- Reduction: `summarise.sh`, which writes `summary.json`.
- Timing: `run_time.sh`.
