# L11 replay (mechanical)

Frozen check: `sha256sum -c frozen.sha256` on PROTOCOL.md and SPEC.md: both OK.

Method: a copy of l11.py and test_l11.py was placed in replay/ (copies only; no existing file was edited). l11.py fixes RAW to HERE/raw, so the replay root's raw/ is replay/raw/. Each command in raw/COMMANDS.log was re-run with the same arguments and the same output file names; outputs went to replay/raw/. Log: replay/replay_commands.log (22 commands, all exit=0, none exceeded 15 min).

Deviations: (a) analyze was run with --in 'replay/raw/runs_k*.jsonl' (the replayed run files) instead of 'raw/runs_k*.jsonl', so that the chain uses replayed inputs; the analysis output still names its inputs in its files field, and that field compares equal. (b) raw/SHA256SUMS was not regenerated (bookkeeping step, not an experiment output).

Result: 70 files compared; 69 identical (cmp); 1 differs.

| File | Status |
|---|---|
| `analysis.json` | IDENTICAL |
| `curves_k10.jsonl` | IDENTICAL |
| `curves_k12.jsonl` | IDENTICAL |
| `curves_k14.jsonl` | IDENTICAL |
| `curves_k8.jsonl` | IDENTICAL |
| `gate_pic_p103.json` | IDENTICAL |
| `gate_pic_p127.json` | IDENTICAL |
| `points_k10_0.json` | IDENTICAL |
| `points_k10_1.json` | IDENTICAL |
| `points_k10_10.json` | IDENTICAL |
| `points_k10_11.json` | IDENTICAL |
| `points_k10_2.json` | IDENTICAL |
| `points_k10_3.json` | IDENTICAL |
| `points_k10_4.json` | IDENTICAL |
| `points_k10_5.json` | IDENTICAL |
| `points_k10_6.json` | IDENTICAL |
| `points_k10_7.json` | IDENTICAL |
| `points_k10_8.json` | IDENTICAL |
| `points_k10_9.json` | IDENTICAL |
| `points_k12_0.json` | IDENTICAL |
| `points_k12_1.json` | IDENTICAL |
| `points_k12_10.json` | IDENTICAL |
| `points_k12_11.json` | IDENTICAL |
| `points_k12_2.json` | IDENTICAL |
| `points_k12_3.json` | IDENTICAL |
| `points_k12_4.json` | IDENTICAL |
| `points_k12_5.json` | IDENTICAL |
| `points_k12_6.json` | IDENTICAL |
| `points_k12_7.json` | IDENTICAL |
| `points_k12_8.json` | IDENTICAL |
| `points_k12_9.json` | IDENTICAL |
| `points_k14_0.json` | IDENTICAL |
| `points_k14_1.json` | IDENTICAL |
| `points_k14_10.json` | IDENTICAL |
| `points_k14_11.json` | IDENTICAL |
| `points_k14_2.json` | IDENTICAL |
| `points_k14_3.json` | IDENTICAL |
| `points_k14_4.json` | IDENTICAL |
| `points_k14_5.json` | IDENTICAL |
| `points_k14_6.json` | IDENTICAL |
| `points_k14_7.json` | IDENTICAL |
| `points_k14_8.json` | IDENTICAL |
| `points_k14_9.json` | IDENTICAL |
| `points_k8_0.json` | IDENTICAL |
| `points_k8_1.json` | IDENTICAL |
| `points_k8_10.json` | IDENTICAL |
| `points_k8_11.json` | IDENTICAL |
| `points_k8_2.json` | IDENTICAL |
| `points_k8_3.json` | IDENTICAL |
| `points_k8_4.json` | IDENTICAL |
| `points_k8_5.json` | IDENTICAL |
| `points_k8_6.json` | IDENTICAL |
| `points_k8_7.json` | IDENTICAL |
| `points_k8_8.json` | IDENTICAL |
| `points_k8_9.json` | IDENTICAL |
| `runs_k10.jsonl` | IDENTICAL |
| `runs_k12_a.jsonl` | IDENTICAL |
| `runs_k12_b.jsonl` | IDENTICAL |
| `runs_k12_c.jsonl` | IDENTICAL |
| `runs_k14_a0.jsonl` | IDENTICAL |
| `runs_k14_a1.jsonl` | IDENTICAL |
| `runs_k14_b0.jsonl` | IDENTICAL |
| `runs_k14_b1.jsonl` | IDENTICAL |
| `runs_k14_c.jsonl` | IDENTICAL |
| `runs_k14_d0.jsonl` | IDENTICAL |
| `runs_k14_d1.jsonl` | IDENTICAL |
| `runs_k14_s.jsonl` | IDENTICAL |
| `runs_k8.jsonl` | IDENTICAL |
| `summary.tsv` | IDENTICAL |
| `unit_tests.txt` | DIFFER |

## Differing file

- `unit_tests.txt`: only line 3 differs: `Ran 12 tests in 0.561s` (raw) vs `Ran 12 tests in 0.577s` (replay). Wall-time line; all 12 tests report OK in both.
