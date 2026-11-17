# E11 native tools (Python helpers replaced)

`examples/fall_degree_e11_tools.rs` replaces `compare.py`, `count_columns.py`,
`derived.py`, `diff_rerun.py`, `project.py`, `scaling_table.py`, `summ.py` and
`maxrss.py` (AGENTS.md "Implementation language: no Python"). `plot.py` stays
(layout of recorded values only). Build:
`cargo build -j1 --release --example fall_degree_e11_tools`.

Equivalence against the committed outputs, run 2026-10-27 from the repo root with
`T=target/release/examples/fall_degree_e11_tools`, `E=research/fall_degree_bounds_20261008/e11`,
`LC_ALL=C` (the glob order the committed `validation_summary.json` rows use):

| subcommand | command | result |
|---|---|---|
| `compare` | `$T compare --json out.json $E/validate/*.jsonl >out.txt` | `cmp` identical to `validation_summary.txt` and `validation_summary.json` (112 joined, 0 mismatches, 0 violations, recall supp 33/90, w1x 90/90, w2x 90/90, wpx 6/90, all per-cell rows) |
| `derived` | `$T derived $E/validate/*.jsonl` | identical to `derived.txt` |
| `diff-rerun` | `$T diff-rerun $E/validate $E/validate_final` | identical to `diff_rerun.txt` (`SAME 112 DIFF 0`) |
| `project` | `$T project` | identical to `projection.json`; stderr `basis_bytes / (cols^2/16): min 0.514 max 1.036 over 98 certified runs` |
| `scaling-table` | `$T scaling-table` | identical to `scaling_table.json`; stderr `agreement problems: 0` |
| `columns` | `$T columns 4:28:7 3:21:7 4:12:3 4:16:4 5:25:5 4:32:8 3:48:16 3:5:2 5:13:3` | identical to `count_columns.py` on the same cells (e.g. 4:28:7 W1_cols 817860, dense_cols 2028356; 3:21:7 W1_cols 79234) |
| `summ` | `$T summ $E/validate/*.jsonl $E/scaling/window_*.jsonl` | identical to `summ.py` on the same files (124 lines) |
| `maxrss` | `$T maxrss RSS dd if=/dev/zero of=/dev/null bs=64M count=1` | 67312 kB vs `maxrss.py` 67532 kB (same `getrusage(RUSAGE_CHILDREN).ru_maxrss`; 0.3% apart, cause not investigated); exit codes equal (3 → 3, SIGKILL → 247) |

`columns` and `summ` have no committed output; they were compared against one
last run of the Python scripts before removal. The committed outputs stay as
evidence. `run_scaling.sh` now calls `maxrss` and `scaling-table`;
`run_validate.sh` prints the `compare` summary of its output directory.
