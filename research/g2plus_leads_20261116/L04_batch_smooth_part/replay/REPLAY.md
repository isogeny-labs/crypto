# L04 replay (batch-smooth partial relations): REPLAY.md

Replay runner: mechanical replay of `raw/COMMANDS.log` into this `replay/` directory. No code or existing file was edited. `raw/` was only read.

## Step 1: frozen hashes

`sha256sum -c frozen.sha256` (run in the lead directory):

- `PROTOCOL.md: OK`
- `SPEC.md: OK`

Exit status 0. The frozen hashes pass.

## Step 2: commands re-run

Every experiment command in `raw/COMMANDS.log` was re-run with `--out` (and the `analyze.py --raw` / `--out` / `--verdict` arguments) redirected from `raw/` to `replay/`. The command text is otherwise identical, including seeds, cells and arm lists. Each run was wrapped in `timeout 900` (15 minutes). No command reached the limit.

Stderr was captured to `replay/stderr_<output name>.log`. `COMMANDS.log` does not record the stderr redirection, so the stderr files are compared on the assumption that they were produced the same way as in `raw/`.

Exit status and elapsed seconds (from the replay run):

| Step | rc | elapsed_s (replay) | elapsed_s (raw, COMMANDS.log) |
|---|---|---|---|
| unittest (`code/test_*.py`, 15 tests) | 0 | n/a | n/a |
| corr_g2_p1009 | 0 | 3 | 3 |
| corr_g3_p1009 | 0 | 4 | 4 |
| pc_g3_p65521_N8 | 0 | 1 | 1 |
| main_g3_p65521 AB | 0 | 8 | 8 |
| main_g3_p65521 C | 0 | 17 | 17 |
| main_g3_p65521 Cpf | 0 | 9 | 9 |
| main_g3_p262139 AB | 0 | 23 | 22 |
| main_g3_p262139 C | 0 | 43 | 42 |
| main_g3_p262139 Cpf | 0 | 23 | 22 |
| main_g3_p1048573 AB | 0 | 26 | 26 |
| main_g3_p1048573 C | 0 | 39 | 39 |
| main_g3_p1048573 Cpf | 0 | 26 | 26 |
| sec_g2_p65521 AB | 0 | 2 | 3 |
| sec_g2_p65521 C | 0 | 4 | 3 |
| sec_g2_p65521 Cpf | 0 | 2 | 3 |
| sec_g2_p1048573 AB | 0 | 2 | 1 |
| sec_g2_p1048573 C | 0 | 3 | 3 |
| sec_g2_p1048573 Cpf | 0 | 1 | 1 |
| analyze.py | 0 | n/a | n/a |

The raw elapsed values are the ones recorded in `COMMANDS.log`. They are wall times, recorded for reference only. They are not compared, and no speed claim is made from them.

Note on the first attempt: a first launch wrapped in a background bash call stalled before writing anything into `replay/`. It was stopped and nothing it produced was kept. The replay was then re-run from a script file. The replay outputs come only from that second run, which was the one that completed.

## Step 3: comparison (`cmp -s` byte comparison, raw/ vs replay/)

| File | Result |
|---|---|
| corr_g2_p1009.jsonl | identical |
| corr_g3_p1009.jsonl | identical |
| pc_g3_p65521_N8.jsonl | identical |
| main_g3_p65521_AB.jsonl | identical |
| main_g3_p65521_C.jsonl | identical |
| main_g3_p65521_Cpf.jsonl | identical |
| main_g3_p262139_AB.jsonl | identical |
| main_g3_p262139_C.jsonl | identical |
| main_g3_p262139_Cpf.jsonl | identical |
| main_g3_p1048573_AB.jsonl | identical |
| main_g3_p1048573_C.jsonl | identical |
| main_g3_p1048573_Cpf.jsonl | identical |
| sec_g2_p65521_AB.jsonl | identical |
| sec_g2_p65521_C.jsonl | identical |
| sec_g2_p65521_Cpf.jsonl | identical |
| sec_g2_p1048573_AB.jsonl | identical |
| sec_g2_p1048573_C.jsonl | identical |
| sec_g2_p1048573_Cpf.jsonl | identical |
| analysis.json | identical (replay analysis run with `--raw replay`) |
| VERDICT.md | identical (replay analysis run with `--raw replay`) |
| stderr_* (18 files) | identical |
| unittest.log | differs on line 18 only: `Ran 15 tests in 0.938s` (raw) vs `Ran 15 tests in 1.002s` (replay). The timing line is the only difference. All 15 tests passed in both runs. |

Summary: 36 of 37 comparable files are byte-identical (18 jsonl, analysis.json, VERDICT.md, 18 stderr logs). The one exception is the unittest wall-time line, which is expected run-to-run variation.

Because the outputs are byte-identical, the sha256 values in `raw/data.sha256` also apply to the replay files (for the jsonl files, the listed hashes match the replay). `raw/data.sha256` itself was not regenerated, since the recorded bookkeeping command writes into `raw/`.

## Not re-run

- `sha256sum raw/*.jsonl raw/unittest.log > raw/data.sha256` (bookkeeping command). It was not re-run because it writes into `raw/`. The byte-identity above covers the hashes it records.

## Open items

None from the replay itself. No command exceeded its time limit, and no output diverged except the unittest timing line.
