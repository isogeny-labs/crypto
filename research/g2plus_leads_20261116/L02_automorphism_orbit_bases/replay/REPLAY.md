# L02 replay (REPLAY runner, mechanical)

Replay of raw/COMMANDS.log, outputs written to replay/ (same file names) and every raw/ path in the commands rewritten to replay/, so downstream inputs came from the replay chain. Commands run from the lead directory with PYTHONDONTWRITEBYTECODE=1 and stdin from /dev/null. No file under code/, raw/ or the frozen set was edited.

- frozen.sha256: PROTOCOL.md OK, SPEC.md OK.
- All 79 logged commands exited 0 (no command exceeded 15 minutes; longest rho_F7_b10_Rneg about 214 s).
- Comparison: cmp for bytes; for .jsonl the rows were parsed and compared with wall_s removed (wall_s is a wall-clock field and is not a counted metric).
- Result: all 79 output files match raw/ (bytes or, for wall_s-only differences, all counted fields). analysis.json, summary.tsv, selftest.txt, gates_instances.jsonl and the four instance files are byte-identical.

## Logged command text that does not run as written

COMMANDS.log line 1 is `bash -c python3 code/selftest.py > raw/selftest.txt 2>&1`. Under bash -c, the string `python3` is the command and `code/selftest.py` becomes $0, so the literal text runs bare python3 reading a program from stdin. Run as logged with empty stdin it exits without running the selftest (output kept in the scratchpad, not here). replay/selftest.txt was produced by the intended command `python3 code/selftest.py`, and it is byte-identical to raw/selftest.txt. The log wrapper likely caused this; the raw file shows the selftest did run in the original session.

## Process note

The first replay attempt stopped after one command because the child python process consumed the loop's stdin; the remaining commands were rerun from command 2 with stdin closed. Command 1 was rerun in the intended form. No output from the aborted attempt was used.

## Per-file comparison

| file | result |
|---|---|
| analysis.json | identical (bytes) |
| gates_instances.jsonl | identical (bytes) |
| ic_C5_b10_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b10_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b11_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b11_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b12_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b12_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b13_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b13_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b14_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C5_b14_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C7_b10_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C7_b10_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C7_b8_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C7_b8_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C7_b9_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_C7_b9_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b10_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b10_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b10_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b11_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b11_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b11_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b12_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b12_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b12_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b13_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b13_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b13_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b14_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b14_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F5_b14_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b10_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b10_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b10_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b8_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b8_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b8_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b9_A.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b9_A0.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| ic_F7_b9_B.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| instances_C5.jsonl | identical (bytes) |
| instances_C7.jsonl | identical (bytes) |
| instances_F5.jsonl | identical (bytes) |
| instances_F7.jsonl | identical (bytes) |
| rho_C5_b10_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b10_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b11_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b11_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b12_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b12_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b13_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b13_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b14_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C5_b14_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C7_b10_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C7_b10_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C7_b8_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C7_b8_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C7_b9_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_C7_b9_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b10_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b10_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b11_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b11_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b12_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b12_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b13_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b13_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b14_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F5_b14_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F7_b10_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F7_b10_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F7_b8_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F7_b8_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F7_b9_Raut.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| rho_F7_b9_Rneg.jsonl | identical except wall_s (first differing field is wall_s, a wall-time value; all counted fields equal, same row order) |
| selftest.txt | identical (bytes) |
| summary.tsv | identical (bytes) |
