# L10 replay (mechanical re-run of raw/COMMANDS.log)

Runner: replay task, 2026-11-16 (UTC clock of the replay host; times below are the log's own timestamps).
Binary: code/target/release/l10, sha256 bc6f77c38a710196b712ad15df8cb107bccf0028d29fac1743871be1104ee294 (matches the binary hash in raw/COMMANDS.log; not rebuilt).
Each recorded command was re-run unchanged except that every `raw/` output path was redirected to `replay/` (same file names). Per-command stdout/stderr: `replay/logs/NNN.txt`. Per-command machine-readable results: `replay/replay_results.jsonl`.

## Step 1: frozen hashes

`sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK (exit 0).

## Commands replayed

Replayed, in log order (48 entries):
- `l10 tiny` p=13,19,31,37,43 (the 21:06:00 group; the 21:05:05 group is superseded per the NOTE in the log and was not replayed).
- `l10 rhocost` 36 runs (the 21:06:11 frozen loop; the 21:06:04 timing probe is superseded per the NOTE and was not replayed).
- `l10 measure` p=271,523,1033,2053,4099,8209 (21:06:21 to 21:07:14); `measure` p=349,1381,5479 with `--families C` (pc.jsonl); `measure` p=1033 twice (replay_1033_a/b) and `cmp`.
- `l10 measure` p=16411 (the 21:09:55 entry, logged exit 137).
- `analyze.py` (with `--raw replay/`, outputs into `replay/`), and `plot.py` (`replay/measure.jsonl` -> `replay/plot.svg`).
- Not replayed: `cargo build`, `cargo test` (build/test, not experiment commands; the binary was used as built). `sha256sum` of the lab source was replayed.

Every replayed command exited 0 (the p=16411 measure exited 0 in the replay although the logged run exited 137; see below). No command reached the 15-minute limit (run under `timeout 900`); the p=16411 run took roughly 10 minutes of wall time on a shared host.

## Hash checks against the log

- All replayed experiment commands produced output whose sha256 matches the hash logged for the same file in COMMANDS.log, except the three cases listed below.
- `lab_source.sha256`: does NOT match. The logged hash is 3bf45c3a... for `/home/user/crypto/src/cryptanalysis/plane_quartic_pic.rs` per the replayed file; the current file's sha256 is e6cf9d53... The source file was modified at 21:34 (after the 21:05 log entry). Replay output reflects the current source, so `raw/lab_source.sha256` is stale with respect to the current file. Not changed by this replay.
- `measure --p 16411`: the logged run exited 137 and left a partial `raw/measure.jsonl` (167 lines, ends at family C). The replay exited 0 after roughly 10 minutes (under the 15-minute limit), writing 168 lines (families through Q; the last row is Q_full p=16411). The kill was not reproduced. Output hash does not match the logged partial file (logged 802cf24e..., replay 8b8cd3b1...). This is expected if the original was killed mid-write; the cause of the original 137 (OOM or external kill) is not recoverable from the log.
- `analyze.py`: logged analysis.json / tables.md hashes do not match, because the input `measure.jsonl` differs (the extra p=16411 row). The replay's `tables.md` row for p=16411 Q_full differs accordingly. `raw/spec_issues.txt` is listed as an output in the log but no logged command produces it (the replay did not create it). Its hash is therefore not checked.
- `plot.py`: plot.svg differs from `plot.svg` at the lead root (same cause: the extra measure row). The replay's plot.svg hash does not match the logged b6f69165... for the root plot.svg.
- The log's `RESULTS.md`, `DEVIATIONS.md` and `code/target/release/l10` hash lines are not outputs of any command; the replay checks them as read-only and they were not rewritten.

## File comparison (replay/ vs raw/)

- `replay/lab_source.sha256` vs `raw/lab_source.sha256`: DIFFERS (1 replay lines vs 1 raw lines; 1 lines differ)
  - line 1: raw `3bf45c3a116d3c870e5b19742ce151179e654e6c173db0d65b7b7dfee954588a  /home/user/crypto/src/cr` | replay `e6cf9d5337f1ba3d6b2bc22b1d4f29f4093831817721452e1702a055cf3d1c37  /home/user/crypto/src/cr`
- `replay/tiny.jsonl` vs `raw/tiny.jsonl`: identical (14257 bytes, sha256 3cbf2fbb2b5bf5d7)
- `replay/rhocost.jsonl` vs `raw/rhocost.jsonl`: identical (12172 bytes, sha256 86593e42ad692d6a)
- `replay/measure.jsonl` vs `raw/measure.jsonl`: DIFFERS (168 replay lines vs 167 raw lines; 1 lines differ)
  - line 168: raw `<EOF>` | replay `{"schema":"l10.measure/v1","family":"Q","arm":"Q_full","p":16411,"curve_seed":3,"solver_se`
- `replay/pc.jsonl` vs `raw/pc.jsonl`: identical (13371 bytes, sha256 c0cbc236e25d4184)
- `replay/replay_1033_a.jsonl` vs `raw/replay_1033_a.jsonl`: identical (17600 bytes, sha256 441001b4e904f54d)
- `replay/replay_1033_b.jsonl` vs `raw/replay_1033_b.jsonl`: identical (17600 bytes, sha256 441001b4e904f54d)
- `replay/replay_cmp.txt` vs `raw/replay_cmp.txt`: identical (7 bytes, sha256 19eaf43821a7660e)
- `replay/analysis.json` vs `raw/analysis.json`: DIFFERS (1184 replay lines vs 1184 raw lines; 16 lines differ)
  - line 248: raw `   "L_bm": 16165.0,` | replay `   "L_bm": 16142.166666666666,`
  - line 249: raw `   "accepted": 16549.0,` | replay `   "accepted": 16524.666666666668,`
- `replay/tables.md` vs `raw/tables.md`: DIFFERS (138 replay lines vs 138 raw lines; 1 lines differ)
  - line 88: raw `| 16411 | Q_full | 16539.0 | 16539.0 | 32956.2 | 16549.0 | 0.5022 | 12.6 | 0.0 | 16394.6 |` | replay `| 16411 | Q_full | 16514.7 | 16514.7 | 32938.5 | 16524.7 | 0.5017 | 12.7 | 0.0 | 16401.2 |`
- `replay/plot.svg` vs `plot.svg`: DIFFERS (54 replay lines vs 54 raw lines; 2 lines differ)
  - line 18: raw `<path d="M80.0,318.2 L179.3,279.0 L282.2,235.6 L385.9,193.1 L490.4,149.5 L595.3,106.0 L700` | replay `<path d="M80.0,318.2 L179.3,279.0 L282.2,235.6 L385.9,193.1 L490.4,149.5 L595.3,106.0 L700`
  - line 25: raw `<circle cx="700.0" cy="62.4" r="3" fill="#1f77b4"/>` | replay `<circle cx="700.0" cy="62.5" r="3" fill="#1f77b4"/>`

Summary: identical for tiny.jsonl, rhocost.jsonl, pc.jsonl, replay_1033_a/b.jsonl, replay_cmp.txt. Differs for measure.jsonl (replay has one additional line, the p=16411 Q_full row, because the raw file is from a killed run), and for analysis.json, tables.md and plot.svg (all downstream of measure.jsonl). lab_source.sha256 differs because the source file changed after the logged run.

## Not replayed / not comparable

- `raw/COMMANDS.log`, `raw/build.log`, `raw/test.log`, `raw/binary.sha256`: logs and hashes, not experiment outputs.
- `raw/tiny_superseded_v1.jsonl`: superseded first tiny run (per the NOTE in COMMANDS.log).
- `raw/spec_issues.txt`: no logged producing command.

## Unclear items reported, not resolved

1. Source change: plane_quartic_pic.rs changed after the logged run (see above).
2. The p=16411 run: logged exit 137 with a partial output; replay completed. Replay is not evidence that the original kill was reproducible.
3. `raw/spec_issues.txt` has no logged producer.
