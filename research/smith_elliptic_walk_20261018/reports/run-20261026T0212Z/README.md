# Run report: W3a Q1 exhaustive cells, run-20261026T0212Z

This run report records observations only. Interpretation is in
[`../../README.md`](../../README.md) and [`../../REPORT.pdf`](../../REPORT.pdf).
PDF: [`report.pdf`](report.pdf).

## Run

| Item | Value |
|---|---|
| Command | `scripts/run_exhaustive.sh <bin> data/exhaustive` |
| Subcommand per cell | `smith_elliptic_walk exhaustive --p P --graph3 --graph-json`, plus `--unreduced` at `p = 7` |
| Binary | sha256 `443d1f505f5c41b0…`, built from branch `claude/smith-walk-exhaustive-20261020` on top of `71dd7db5c4` |
| Host | Apple M4 Pro, 14 cores, shared with other jobs, no pinning, `nice 10`, at most 2 cells at once |
| Isolation | none beyond that |

The run is deterministic, so it has no seeds.

Cells requested: `p = 7, 11, 13, 17, 19, 23, 29, 31`, plus the unreduced
`p = 7` control. All 9 completed with exit 0. Wall times were 0.03 s at
`p = 7` and 62.4 s at `p = 31`; the unreduced control took 6.6 s.

## Measured values

Measured values (exhaustive, so no sampling interval):
- `D_H` = 0.5408, 0.6074, 0.5828, 0.5502, 0.5346, 0.5388, 0.5404, 0.5624;
- `D_E` = 0.5385, 0.6000, 0.5969, 0.5512, 0.5312, 0.5368, 0.5431, 0.5589;
- `C_2` = 0.9670, 0.9000, 0.7750, 0.7367, 0.9489, 0.9807, 0.7418, 0.9502;
- `C_t` = 0.9670, 0.9304, 0.8125, 0.7827, 0.9830, 0.9942, 0.8372, 1.0000.

Mean distance in 2-steps, over reachable failing weak curves: 1.46, 1.87,
1.79, 2.51, 2.86, 3.33, 2.72, 3.57. The full table is in `../../README.md`.

## Controls

Every control counter recorded 0 failures in every cell:
- `l_fail`, `velu_trace_fail`, `dual_missing`;
- `jv_not_weak`, `jv_no_models`, `jv_signature_missing`;
- `smith_none`, `roots_mismatch`.

The unreduced `p = 7` run printed the same summary line as the reduced run.

## Records and figures

Raw records are in `../../data/exhaustive/`:
- `exh_p*.json`;
- `graph_p*.json`;
- `exh_unreduced_p7.json.gz`;
- `log_exh_*.txt`.

Graphs: `q1_shares.svg` and `q1_distance.svg`, copies of the study figures
drawn from those records by `../../plot.py`.
