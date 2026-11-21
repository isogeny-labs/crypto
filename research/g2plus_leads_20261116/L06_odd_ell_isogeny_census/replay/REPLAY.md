# L06 replay (mechanical, no judgement)

Scope: re-run of the experiment commands recorded in `raw/COMMANDS.log`, with every output redirected into `replay/` under the same file names. Runner: `replay/replay.sh` (log: `replay/COMMANDS.log`). Runner-only; no code, existing file, or `raw/` file was edited. Everything is [pcc:unverified].

## Step 1: frozen hashes

`sha256sum -c frozen.sha256`: DERIVATION.md OK, PROTOCOL.md OK, SPEC.md OK (exit 0).

## Step 2: commands re-run

| Command (recorded in raw/COMMANDS.log) | Exit | Notes |
|---|---|---|
| `l06 lagr2 --out` | 0 | |
| `l06 gsp --samples 200000` | 0 | |
| `l06 gate-enum --p 101 --count 200` | 0 | |
| `l06 gate-enum --p 257 --count 40` | 0 | |
| `timeout 900 l06 census --p 257` | 0 | timeout raised from recorded 600 s to 900 s in the replay runner; no run stopped by it |
| `timeout 900 l06 census --p 521` | 0 | same note |
| `timeout 900 l06 census --p 1031` | 0 | same note |
| `timeout 900 l06 census --p 2053` | 0 | same note |
| `timeout 900 l06 census --p 4099` | 0 | same note |
| `python3 -I analyze.py replay` | 0 | recorded as `analyze.py raw`; input dir changed to replay/ |

No command ran longer than 15 minutes (longest census about 1.5 min wall by the log timestamps, within its timeout). Not re-run: `cargo build --release`, `cargo test --release`, and the `sha256sum` recording lines. These are build/test/bookkeeping steps, not experiment commands, and re-running cargo would write into `l06/target/`. The existing binary `l06/target/release/l06` (mtime 23:25) was used.

## Step 3: cmp of replay/ against raw/

Identical (cmp exit 0):
- l2_lagrangians.json
- gsp_reference.json
- gsp.log
- gate_enum_p101.jsonl, gate_enum_p101.log
- gate_enum_p257.jsonl, gate_enum_p257.log
- census_p257.jsonl, census_p257.log
- census_p521.jsonl, census_p521.log
- census_p1031.jsonl, census_p1031.log
- census_p2053.jsonl, census_p2053.log
- census_p4099.jsonl, census_p4099.log

Differs:
- summary.json. Two differences, both traceable to the replay input directory, not to the experiment outputs:
  1. Gate G0 (`gates.G0`) reads `test.log` and `build.log`, which are not in replay/ (step 2 did not re-run cargo). Replay gives `"pass": false, "note": "test.log missing"` where raw gives `"pass": true, "failed_lines": 0`. `gate_failures_G0_G7` gains `"G0"`, so it reads `["G0", "G1"]` against raw's `["G1"]`.
  2. `gate_failures_G0_G7` is the only other change (the `"G0"` entry above).
  Before running analyze I did not copy raw build/test logs into replay/, because that would have changed the analyzer's inputs.
  Also: the gate G1 failure appears in both raw and replay (`["G1"]`), so it is not a replay artifact. Its cause is not examined here.

The census JSONL outputs match byte for byte, so the sha256 values recorded in raw/COMMANDS.log for those files reproduce (checked for census_p257.jsonl: d99b7c2d...).

## Unclear / not done

- Whether analyze.py should read raw/ build and test logs from replay/ or from raw/ is not specified by the task. The G0 mismatch follows from that choice. Not resolved here.
- The G1 gate failure is recorded as-is; no judgement on it.
- Build and test were not replayed (see step 2).
- No code was changed and no result was interpreted.
