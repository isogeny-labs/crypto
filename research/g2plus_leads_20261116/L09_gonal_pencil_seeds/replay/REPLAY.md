# L09 replay (mechanical re-run of raw/COMMANDS.log)

Replay date: 2026-11-16 (UTC, approx. 21:55 to 22:20).
Runner: REPLAY runner for lead L09, no code or existing file edited.

## Integrity of the frozen inputs

`sha256sum -c frozen.sha256` (run in the lead directory): PASS
- PROTOCOL.md: OK
- SPEC.md: OK

## Procedure and deviations

1. Every command recorded in `raw/COMMANDS.log` (cargo test, cargo build, 4 census, 24 cell, 1 g6 rerun, 9 exact) was re-run with the same arguments and arms. Only the output path changed from `raw/` to `replay/`.
2. Experiments used the existing binary `code/target/release/l09`, not a rebuilt one. Nothing was rebuilt in `code/`.
3. The two cargo commands (`cargo test`, `cargo build`) were run with `CARGO_TARGET_DIR` set to a directory in the session scratchpad, so that cargo would not write into `code/target/`. The source, `Cargo.toml` and `Cargo.lock` were not modified (their mtimes are unchanged). This is the only deviation in the command text. Its effect is shown below under build.log and g0_tests.log.
4. stderr of every run went to the session scratchpad (`replay_stderr.log`), not into `replay/`. `raw/stderr.log` was therefore not replayed.
5. No run exceeded 15 minutes. The slowest run was `cell --p 16381 --beta-idx 0` at about 133 s wall (raw: 131.2 s CPU). No overrun occurred.
6. The hook refusal recorded in `raw/hook_refusals.txt` (an `rm -f raw/*` refusal from an earlier session) did not recur, and no command was refused during the replay. No workaround was needed.

## Per-file comparison (`cmp -s raw/<f> replay/<f>`)

Replayed outputs: 40 files. Compared: 40. Identical: 38. Differing: 2.

| File | Result |
|---|---|
| census_p251.jsonl | identical |
| census_p1021.jsonl | identical |
| census_p4093.jsonl | identical |
| census_p16381.jsonl | identical |
| cell_p251_b0.jsonl | identical |
| cell_p251_b1.jsonl | identical |
| cell_p251_b2.jsonl | identical |
| cell_p251_b3.jsonl | identical |
| cell_p251_b4.jsonl | identical |
| cell_p251_b5.jsonl | identical |
| cell_p1021_b0.jsonl | identical |
| cell_p1021_b1.jsonl | identical |
| cell_p1021_b2.jsonl | identical |
| cell_p1021_b3.jsonl | identical |
| cell_p1021_b4.jsonl | identical |
| cell_p1021_b5.jsonl | identical |
| cell_p4093_b0.jsonl | identical |
| cell_p4093_b1.jsonl | identical |
| cell_p4093_b2.jsonl | identical |
| cell_p4093_b3.jsonl | identical |
| cell_p4093_b4.jsonl | identical |
| cell_p4093_b5.jsonl | identical |
| cell_p16381_b0.jsonl | identical |
| cell_p16381_b1.jsonl | identical |
| cell_p16381_b2.jsonl | identical |
| cell_p16381_b3.jsonl | identical |
| cell_p16381_b4.jsonl | identical |
| cell_p16381_b5.jsonl | identical |
| g6_rerun.jsonl | identical |
| exact_p251_b0.jsonl | identical |
| exact_p251_b1.jsonl | identical |
| exact_p251_b2.jsonl | identical |
| exact_p251_b3.jsonl | identical |
| exact_p251_b4.jsonl | identical |
| exact_p251_b5.jsonl | identical |
| exact_p1021_b3.jsonl | identical |
| exact_p1021_b4.jsonl | identical |
| exact_p1021_b5.jsonl | identical |
| build.log | DIFFER (see below) |
| g0_tests.log | DIFFER (see below) |

The sixth `cell_p16381_*` line and the `census` rows are included in the 38 identical files. All 24 cell files and all 9 exact files are byte-identical to their raw counterparts.

### build.log (differs)

- Line 1 in replay only: `   Compiling l09 v0.1.0 (/home/user/crypto/research/g2plus_leads_20261116/L09_gonal_pencil_seeds/code)`. The replay target directory was empty, so cargo compiled.
- raw line 29 `Finished release profile [optimized] target(s) in 0.00s` vs replay `... in 1.19s`.
- raw `Running ... (code/target/release/deps/l09-...)` vs replay `(<scratchpad>/cargo-target/release/deps/l09-...)`, because of the deviation in step 3.
- Build succeeded (exit 0) in both. The difference is in cargo's own output (timing and paths), not in the compiled source.

### g0_tests.log (differs)

- Line 1 in replay only: the same `Compiling l09` line as build.log.
- Test result line is `9 passed; 0 failed` in both, so the same 9 tests pass.
- Two test-result lines are reordered: `tests::t3_line_trials ... ok` and `tests::t6_t7_graph_and_exact ... ok` appear in a different order, which is the normal effect of running tests in parallel threads.
- Timing text differs (`finished in 0.18s` raw vs `0.20s` replay), and the test binary path differs (see step 3).
- The pass/fail content is the same. The byte difference comes from cargo and the test harness's output order, not from a different result.

## Files not replayed (outside the COMMANDS.log set)

- `raw/COMMANDS.log`, `raw/SHA256SUMS`, `raw/hook_refusals.txt`, `raw/summary.json`: these are logs and summaries produced by the driver and hooks. They are not experiment outputs and were not re-created.
- `raw/stderr.log`: stderr capture, see step 4.

## Summary

- Experiment outputs (census, cell, g6 rerun, exact): 38 of 38 byte-identical.
- The 2 non-identical files are cargo build and test logs, where the difference is cargo's own compile line, timings and the target path, plus test-output ordering. None of it changes a computed value.
- No run overran its time limit. No code or existing file was edited.

This is a reproduction check only. Its results are `[pcc:unverified]` until validated under the lab's gate.
