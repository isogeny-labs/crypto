# L12 replay (mechanical re-run of raw/COMMANDS.log)

Replay date: 2026-11-16, 22:17Z to 22:27Z. Lead: L12_bielliptic_quartic_orbits.
Runner: REPLAY runner (Haiku). No code or existing file was edited. Python bytecode writing was disabled (`PYTHONDONTWRITEBYTECODE=1`, `-B`) so no `__pycache__` appeared in `code/`.

## 0. Frozen inputs
`sha256sum -c frozen.sha256`: `PROTOCOL.md: OK`, `SPEC.md: OK` (exit 0).

## 1. Commands re-run (outputs redirected to replay/)
| # | Recorded command | Replay command | Exit |
|---|---|---|---|
| 1 | `python3 --version > raw/python_version.txt` | `python3 --version > replay/python_version.txt` | 0 |
| 2 | `python3 -m unittest discover -s code -p 'test_*.py' -v > raw/unittest.log` | same with `-B` to `replay/unittest.log` | 0 |
| 3 | `python3 code/select_instances.py --out raw/instances.jsonl --rejects raw/rejects.tsv > raw/select.log` | same with `replay/` paths | 0 |
| 4 | `python3 code/run_matrix.py --instances raw/instances.jsonl --outdir raw/runs --log raw/run_matrix.log` | same with `replay/` paths (273 child runs, wall 22:17:47Z to 22:26:50Z, about 9 min, under the 15 min cap) | 0 |
| 5 | `python3 code/analyze.py --instances ... --runs raw/runs --summary raw/summary.tsv --decision raw/decision.json > raw/analyze.log` | same with `replay/` paths (`analyze.py` takes its gate-file directory from the summary path, so it read `replay/unittest.log` and `replay/select.log`) | 0 |
| 6 | `sha256sum raw/instances.jsonl raw/summary.tsv raw/decision.json raw/runs/*.json > raw/outputs.sha256` | same over `replay/` files, written to `replay/outputs.sha256` | 0 |
| 7 | `python3 code/make_results.py > raw/results_tables.stdout` | see deviation D1 | 0 |

Deviation D1 (make_results.py): the script ignores any output argument. It writes `ROOT/plot.svg` and `ROOT/raw/results_tables.md`, which are the lead's existing files. Running it in place would have overwritten them. It was therefore run inside a temporary copy (`code/`, `raw/runs/`, `raw/decision.json` from the replay set) under `replay/_make_results_sandbox/`. Its outputs were copied to `replay/plot.svg`, `replay/results_tables.md` and `replay/results_tables.stdout`. The sandbox was then removed. The lead's `plot.svg` and `raw/results_tables.md` were not touched.

Deviation D2: the recorded `raw/COMMANDS.log` entries for `results_tables` list `plot.svg` and `raw/results_tables.md` hashes. The replay `plot.svg` was compared to the lead's `plot.svg` (see table).

## 2. File-by-file comparison (cmp, raw/ vs replay/)
| File | Result |
|---|---|
| python_version.txt | identical |
| instances.jsonl | identical |
| rejects.tsv | identical |
| select.log | identical |
| analyze.log | identical |
| summary.tsv | identical |
| decision.json | identical |
| results_tables.stdout | identical |
| results_tables.md | identical |
| plot.svg (vs lead root `plot.svg`) | identical |
| unittest.log | differs at line 16 only: `Ran 13 tests in 1.363s` (raw) vs `Ran 13 tests in 1.349s` (replay). Wall-time line; all 13 tests OK in both. |
| run_matrix.log | differs on all 273 lines only by the path prefix `raw/runs/` vs `replay/runs/`. After normalizing that prefix the files are identical (273 lines, status and exit columns equal). |
| runs/*.json (273 files) | byte-different in all 178 files that carry `wall_s`; after removing `wall_s` all 273 are identical. (The byte count shows 178 differing files; the normalized check covered all 273, with 0 differing beyond `wall_s`.) |
| outputs.sha256 | the 3 non-run entries (instances, summary, decision) match. Run-file hashes differ because the run JSON bytes contain `wall_s`. The file also lists `replay/` paths instead of `raw/` paths. |

`wall_s` is the only field that differs. It is a wall-time field written by `ic_run.py`, and the lab's own G8 gate already strips it. Every counted field (FPM counters, relation counts, W, R, N, certification flags, stream digests) is identical.

## 3. Summary
- All 273 run outputs, the instance selection, and the analysis decision reproduce exactly on every counted field.
- Replay is consistent with the raw outputs. No discrepancy in any count, gate or decision.
- Non-identical bytes come only from the wall-time field (`wall_s`, `Ran ... in ...s`) and from the path prefix in logs.
- Replay adds no measurement claim. Per the task, all results remain `[pcc:unverified]`.

## 4. Not run / open items
- Nothing exceeded the 15 minute limit. The matrix run finished in about 9 minutes.
- Nothing was refused by a hook.
