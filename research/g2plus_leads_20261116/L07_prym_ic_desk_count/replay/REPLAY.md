# REPLAY.md  (lead L07, replay runner; cwd = lead directory)

frozen.sha256: PROTOCOL.md OK, SPEC.md OK (exit 0).

Each command from raw/COMMANDS.log was re-run with output redirected into replay/.
PYTHONDONTWRITEBYTECODE=1 was set so no __pycache__ was written. No code or existing file was edited.
All runs finished well under 15 minutes (split_law.py about 22 s wall, 22 s user).

| File | Replay vs raw | Notes |
|---|---|---|
| tests_spec_command_failed.txt | identical | Spec command `python3 -I -m unittest -v src/test_l07.py` fails with ModuleNotFoundError: No module named 'src', exit 1 (reproduces DEVIATIONS D1). |
| tests.txt | differs at line 12 only | `Ran 9 tests in 1.464s` (raw) vs `Ran 9 tests in 1.519s` (replay). Wall-clock timing only; all 9 tests pass, OK. |
| genus_table.tsv | identical | |
| split_law.jsonl | identical | |
| gates.txt | identical | |
| plot.svg | identical | Replay was generated from a scratch copy of raw/split_law.jsonl and src/plot.py. Copied to replay/plot.svg. |

## Issues found during replay

1. **plot.svg redirect mismatch (my error).** COMMANDS.log records `python3 -I src/plot.py > plot.svg`. In fact src/plot.py writes its output itself to `plot.svg` in the current directory (`open("plot.svg","w")`) and prints nothing to stdout. My first replay used `> replay/plot.svg`, which produced an empty file, and it also re-wrote the lead-root `plot.svg`. The root file's content is unchanged: sha256 be6961efece064f3fb9c52ea1ebae4c719e1321d9b550645fc5f152c65317aba, which matches the value in raw/COMMANDS.log. Only its mtime changed (now 2026-11-16 21:45). The root file is not in frozen.sha256. The empty file was then replaced by a correct replay: the command was re-run from a scratch directory and the output copied to replay/plot.svg. Both the replay and the root file hash to be6961..., the recorded value.
2. **Spec test command fails as recorded (D1).** Reproduced as stated: exit 1, ModuleNotFoundError.
3. **Log ambiguity.** COMMANDS.log says "times not recorded" and gives the plot.svg output path as the lead root, not raw/. The replay follows the file names in the log.

## Summary

Replay matches the raw record for 5 of 6 output files byte for byte. The one difference, tests.txt, is the wall-clock timing line. The spec-form unittest command reproduces its recorded failure.
