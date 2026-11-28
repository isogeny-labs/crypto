# REPLAY of L01 (mechanical)

Replay of the section-10 commands of `raw/COMMANDS.log`, outputs redirected into `replay/`.
Exit codes: all 76 experiment commands and the unittest run exited 0 (`replay/replay_exec.log`).
`sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
`make_results.py` (not in section 10) and `sha256sum` to `raw/data.sha256` were not replayed.

Comparison rule: `identical` = byte-identical. `timing-only` = differs only in wall-clock fields (keys containing cpu_seconds/seconds/time, and the `Ran N tests in Xs` line), which are not counts. Any other difference would be `DIFF`.

| file | status | first differing line (raw vs replay) |
|---|---|---|
| aa_g2_p4099_c1_s1.jsonl | timing-only | line 1 |
| aa_g2_p4099_c1_s2.jsonl | timing-only | line 1 |
| analysis.json | timing-only | line 697 |
| gate_g2_p131.jsonl | timing-only | line 1 |
| gate_g2_p257.jsonl | timing-only | line 1 |
| gate_g3_p67.jsonl | timing-only | line 1 |
| gate_g3_p97.jsonl | timing-only | line 1 |
| la_g2_p1031_c1.jsonl | identical | - |
| la_g2_p1031_c2.jsonl | identical | - |
| la_g2_p1031_c3.jsonl | identical | - |
| la_g2_p4099_c1.jsonl | identical | - |
| la_g2_p4099_c2.jsonl | identical | - |
| la_g2_p4099_c3.jsonl | identical | - |
| la_g3_p1031_c1.jsonl | identical | - |
| la_g3_p1031_c2.jsonl | identical | - |
| la_g3_p1031_c3.jsonl | identical | - |
| lp2_g2_p16411_c1.jsonl | timing-only | line 1 |
| lp2_g2_p16411_c2.jsonl | timing-only | line 1 |
| lp2_g2_p16411_c3.jsonl | timing-only | line 1 |
| lp2_g2_p4099_c1.jsonl | timing-only | line 1 |
| lp2_g2_p4099_c2.jsonl | timing-only | line 1 |
| lp2_g2_p4099_c3.jsonl | timing-only | line 1 |
| lp2_g3_p16411_c1.jsonl | timing-only | line 1 |
| lp2_g3_p16411_c2.jsonl | timing-only | line 1 |
| lp2_g3_p16411_c3.jsonl | timing-only | line 1 |
| lp2_g3_p4099_c1.jsonl | timing-only | line 1 |
| lp2_g3_p4099_c2.jsonl | timing-only | line 1 |
| lp2_g3_p4099_c3.jsonl | timing-only | line 1 |
| rel_g2_p1031_c1_s1.jsonl | timing-only | line 1 |
| rel_g2_p1031_c1_s2.jsonl | timing-only | line 1 |
| rel_g2_p1031_c2_s1.jsonl | timing-only | line 1 |
| rel_g2_p1031_c2_s2.jsonl | timing-only | line 1 |
| rel_g2_p1031_c3_s1.jsonl | timing-only | line 1 |
| rel_g2_p1031_c3_s2.jsonl | timing-only | line 1 |
| rel_g2_p16411_c1_s1.jsonl | timing-only | line 1 |
| rel_g2_p16411_c1_s2.jsonl | timing-only | line 1 |
| rel_g2_p16411_c2_s1.jsonl | timing-only | line 1 |
| rel_g2_p16411_c2_s2.jsonl | timing-only | line 1 |
| rel_g2_p16411_c3_s1.jsonl | timing-only | line 1 |
| rel_g2_p16411_c3_s2.jsonl | timing-only | line 1 |
| rel_g2_p4099_c1_s1.jsonl | timing-only | line 1 |
| rel_g2_p4099_c1_s2.jsonl | timing-only | line 1 |
| rel_g2_p4099_c2_s1.jsonl | timing-only | line 1 |
| rel_g2_p4099_c2_s2.jsonl | timing-only | line 1 |
| rel_g2_p4099_c3_s1.jsonl | timing-only | line 1 |
| rel_g2_p4099_c3_s2.jsonl | timing-only | line 1 |
| rel_g2_p65537_c1_s1.jsonl | timing-only | line 1 |
| rel_g2_p65537_c1_s2.jsonl | timing-only | line 1 |
| rel_g2_p65537_c2_s1.jsonl | timing-only | line 1 |
| rel_g2_p65537_c2_s2.jsonl | timing-only | line 1 |
| rel_g2_p65537_c3_s1.jsonl | timing-only | line 1 |
| rel_g2_p65537_c3_s2.jsonl | timing-only | line 1 |
| rel_g3_p1031_c1_s1.jsonl | timing-only | line 1 |
| rel_g3_p1031_c1_s2.jsonl | timing-only | line 1 |
| rel_g3_p1031_c2_s1.jsonl | timing-only | line 1 |
| rel_g3_p1031_c2_s2.jsonl | timing-only | line 1 |
| rel_g3_p1031_c3_s1.jsonl | timing-only | line 1 |
| rel_g3_p1031_c3_s2.jsonl | timing-only | line 1 |
| rel_g3_p16411_c1_s1.jsonl | timing-only | line 1 |
| rel_g3_p16411_c1_s2.jsonl | timing-only | line 1 |
| rel_g3_p16411_c2_s1.jsonl | timing-only | line 1 |
| rel_g3_p16411_c2_s2.jsonl | timing-only | line 1 |
| rel_g3_p16411_c3_s1.jsonl | timing-only | line 1 |
| rel_g3_p16411_c3_s2.jsonl | timing-only | line 1 |
| rel_g3_p4099_c1_s1.jsonl | timing-only | line 1 |
| rel_g3_p4099_c1_s2.jsonl | timing-only | line 1 |
| rel_g3_p4099_c2_s1.jsonl | timing-only | line 1 |
| rel_g3_p4099_c2_s2.jsonl | timing-only | line 1 |
| rel_g3_p4099_c3_s1.jsonl | timing-only | line 1 |
| rel_g3_p4099_c3_s2.jsonl | timing-only | line 1 |
| rel_g3_p65537_c1_s1.jsonl | timing-only | line 1 |
| rel_g3_p65537_c1_s2.jsonl | timing-only | line 1 |
| rel_g3_p65537_c2_s1.jsonl | timing-only | line 1 |
| rel_g3_p65537_c2_s2.jsonl | timing-only | line 1 |
| rel_g3_p65537_c3_s1.jsonl | timing-only | line 1 |
| rel_g3_p65537_c3_s2.jsonl | timing-only | line 1 |
| summary.md | identical | - |
| unittest.log | timing-only | line 15 |

rows/ (48 files, `rows_*.txt`): 48 byte-identical with raw/rows/ (checked with cmp).

Summary of file statuses: identical=10, timing-only=68.
