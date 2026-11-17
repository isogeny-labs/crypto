# Assessment of card CARD-p2-lift-linear-complexity-20261110-F2-RUN-02

Assessor: experiment-director, assess mode, 2026-11-12. Interpretation of the scientific result: not_assessed.
No claim is made. Nothing was edited except this file.

## Classification: complete and executed as specified, with a recorded tier deviation

The 13 commands of the card ran in card order, once each, all with exit 0, and every
receipt output exists on disk with a hash equal to the receipt's recorded hash. The run-full
step (detached, 20761.71 s, limit 28800 s) exited 0 and `results/run.json` carries an empty `fatal` array.

Tier deviation (recorded, not an execution fault): the carrier was the main session's nohup
supervisor running `tools/task_card.py run` and wait loops, not an `executor-mechanical`
subagent as `executor_role` states (Haiku rate-limited until 2026-11-14). Same treatment as
F3-RUN-02 and F4-RUN-02. The argv that ran equal the card's argv (below).

## Evidence per check

| Check | Evidence | Result |
|---|---|---|
| Receipt status | `F2-RUN-02.receipt.json`: schema crypto.autoresearch.task-card-receipt.v1, status `completed`, commands_total 13, 13 entries, each `attempts: 1`, `exit_code: 0`, `timed_out: false`, `succeeded` | pass |
| Card identity | receipt card_sha256 8dbad5d0...bd1a equals `sha256sum` of `cards/F2-RUN-02.json` today; receipt commit 9465664dbe6ba5757b9cb011192215a00605e8d8 equals the card commit | pass |
| argv equals card | script comparing receipt `commands[i].id` and `argv` with card `commands[i]` for all 13: 13 of 13 equal (run-full: `.../tool/target/release/p2_lift_linear_complexity --protocol .../PROTOCOL.md --null 200 --out research/p2_lift_linear_complexity_20261110/results/run.json`, identical in card, receipt and `11-run-full.pid.json`). `--null 200` agrees with PROTOCOL.md lines 86, 161 | pass |
| Output hashes | for all 13 commands, sha256 of the stdout and stderr files on disk equals receipt `stdout_sha256`/`stderr_sha256` (26 of 26 match, 0 mismatches) | pass |
| Receipt file hash | `sha256sum F2-RUN-02.receipt.json` = 28c05081389a8e8e160cfa5972007eaae9d52b1327a85416281d219e2b71f9b6, equal to the `receipt: ... sha256` line of the completed block in the scratchpad `F2-RUN-02.wait.log` (line 4; the earlier detached-status value 709c0761... in `F2-RUN-02.run.log` is the pre-completion receipt) | pass |
| verify-freeze (step 1) | `F2-RUN-02.verify-freeze.stdout`: 4 lines, all `OK` (PROTOCOL.md, tool/src/main.rs, tool/Cargo.toml, tool/Cargo.lock), empty stderr, exit 0, cwd research/p2_lift_linear_complexity_20261110. FREEZE.sha256 values equal the card binds; `sha256sum` of the four files today reproduces them | pass |
| Binds | 6 of 6 `sha256sum` of the bound files today equal the card: PROTOCOL.md a9e727d2...5ebb, FREEZE.sha256 62747ae6...1401 (no longer re-hashed against the receipt, which does not store binds), tool/Cargo.toml 5b16e1f3...e755, Cargo.lock 288c3b68...b416, src/main.rs 72dadcb9...1fe6, binary 172f1f4d...19d2 | pass |
| Pinned HEAD | `F2-RUN-02.git-head.stdout` = 9465664dbe6ba5757b9cb011192215a00605e8d8 = card commit. HEAD today is 945f59bbab (9465664 is its ancestor). `git diff --stat 9465664 HEAD` over PROTOCOL.md, FREEZE.sha256, tool/Cargo.toml, tool/Cargo.lock, tool/src is empty; over the whole experiment directory it shows only `cards/F2-RUN-02.json` (301 lines added, the card itself), so HEAD movement has no effect on the frozen inputs | pass |
| Toolchain | rustc 1.97.0 (2d8144b78 2026-07-07, LLVM 22.1.6), cargo 1.97.0 (c980f4866 2026-06-30), nproc 4, uname `Linux vm 6.18.44-fc-v378 ... x86_64`, lscpu 3278 bytes (same size as F2-RUN-01), all exit 0 | recorded |
| build | exit 0; stderr: 2 `Blocking waiting for file lock on package cache` lines, then `Finished release profile [optimized] target(s) in 0.47s`. No recompilation (no-op build) | pass; see caveat 3 |
| test | `F2-RUN-02.test.stdout` contains `test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`; 7 listed tests all `ok`. This is the card's stop_when test condition | pass |
| binary sha256 | `F2-RUN-02.binary-sha256.stdout` 172f1f4dc2db4e0a0d263ad0de4f4439d6d750fa2dce08e0dfe82ebad2ea19d2 equals the card bind and `sha256sum` of the file today | pass |
| run-full | exit 0, 20761.71 s, `11-run-full.exit.json` finished_at 2026-11-12T01:25:38Z, timed_out false, one attempt; pid file: pid 3703, started_at 2026-11-11T19:39:36Z, boot_id b5ca9da6-817c-4be4-a5b7-f8a0d28cdcc9, timeout_seconds 28800; stdout `results/run-02.stdout.txt`, stderr `results/run-02.stderr.txt` (0 bytes, sha256 e3b0c442... empty-file digest, equals receipt) | pass |
| Exit 2 / fatal stop_when | run-full exit is 0, not 2; `results/run.json` key `fatal` is a list of length 0 | not triggered |
| protocol/tool lines | `results/run-02.stdout.txt` lines 23-24: `protocol_sha256 a9e727d2...5ebb`, `tool_source_sha256 72dadcb9...1fe6`, both equal FREEZE.sha256 (PROTOCOL.md, tool/src/main.rs); `run.json` header carries the same two values | pass |
| result sha256 | `F2-RUN-02.result-sha256.stdout` 21591ebbafaea0477b4aeb7cd2fdb9caae00c6c7f77902373e90e5332340bf1e equals `sha256sum results/run.json` today (523764 bytes) and the receipt run-full `outputs` entry for run.json | pass |
| result table | `results/run-02.stdout.txt` (2961 bytes, 25 lines) sha256 b9fc3993...b03a equals receipt run-full `outputs` entry, run-full `stdout_sha256` and `F2-RUN-02.result-table.stdout` (command 13 `stdout_sha256`) | pass |
| run.json structure only | top-level keys header, rows, null, plant, aa_checks, summary, fatal; header: quick false, sets [H,R12,R14,R16,S], multipliers [1,2,3,5], null_sequences_per_curve 200; rows 336, null 21, plant 2, aa_checks 21, summary 21, fatal 0 | pass |
| Supervisor / wait logs | `F2-RUN-02.run.log` shows status `detached` before the wait; `F2-RUN-02.wait.log` (144 lines) shows `status: completed` and `final rc=0` as its last line | consistent |
| must_not_exist | Every listed path (receipt, `F2-RUN-02.run`, `results/run.json`, `run-02.stdout.txt`, `run-02.stderr.txt`, all 24 per-step stdout/stderr files) is now present as a product of this card. Receipt `dirty_before` (10 paths) lists none of them (it lists only the stale F2-RUN-01 `results/run.stdout.txt`/`run.stderr.txt`, the three sibling cards and their stale logs, and `resume/RESUME.md`); the first command's output files carry mtime 2026-11-11 19:38, after the card commit. Pre-existence is therefore not contradicted by any record | pass (by dirty_before) |
| write_scope | Card write_scope is `research/p2_lift_linear_complexity_20261110/`. Receipt `dirty_after` has 83 paths: 45 under this experiment directory (all F2-RUN-02 receipts, run directory, results/ files, the card, plus stale -01 logs); the rest are sibling-experiment files (F3 and F4 cards, logs and F4-REP-01 outputs) and `resume/RESUME.md` (also in `dirty_before`). No card command names any path outside the scope. See caveat 4 | pass |
| Only card argv ran | 13 receipt entries, `index` order matches card order, no extra entries; the three detached steps (build, test, run-full) each have one pid/exit/spec file in `F2-RUN-02.run/` | pass |

## Self-containment w.r.t. the lost -01 run

- `F2-RUN-01.receipt.json` has status `detached`, commit 6930d073e258b4e62ed0b06e7e9993eae1facf77. The loss record `benchmarks/losses/20261111-card-p2-lift-linear-complexity-20261110-f2-run-01-11-run-full.json` (crypto.infra-loss/v1, cause container_restart, pid 4981 gone without exit record, started 2026-11-10T18:05:27Z, `pcc: [pcc:unverified]`) documents the loss.
- No -02 command reads any -01 output. -02 `results/run.json` did not exist before (absent from `dirty_before`). The stale -01 `results/run.stdout.txt` and `results/run.stderr.txt` remain in place, both 0 bytes; they are not -02 outputs. Consumers read `run-02.*` and `run.json`.
- The loss record's working-tree diff (not committed; `git status` shows ` M`) changes `relaunched` false to true and sets `relaunch_ref` to `research/p2_lift_linear_complexity_20261110/cards/F2-RUN-02.json (receipt completed 2026-11-12T01:2xZ, rc=0)`. The `01:2xZ` text is approximate; the exit record says 01:25:38Z. The edit is outside the card's write_scope and no card command made it; it was made by the main session (file mtime 01:28), recorded as a fact.

## Deviations and caveats

1. Role deviation: carrier was the main-session supervisor, not `executor-mechanical`. No executor text report exists, so `relay_verbatim` is not established and `check-relay` was not run. `python3 tools/task_card.py lint` on the card now prints `CARD FAIL` (commands[10].argv and commands[11].argv name `results/run.json`; commands[12].argv names `results/run-02.stdout.txt`; "names repository file ... that binds does not pin"). The card sha256 equals the receipt's, so the card is unchanged; the lint failure appears because the card's own outputs now exist on disk. Lint at compile time was not re-checked.
2. HEAD moved from the pinned 9465664 to 945f59bbab after the run was launched; frozen-file diff is empty (above).
3. The build step was a no-op (0.47 s, `Finished`); the binary is the build on disk before this card (card notes). Its sha256 equals the bound value and the test ran on the same package. This assessment did not rebuild.
4. Shared host: file-lock waits on the package cache in build and test stderr; F3 and F4 work ran concurrently on the 4-core host. Wall time is descriptive only.
5. Execution route is local (card notes: no aws CLI, GitHub route takes only ecbench specs). The AGENTS.md section 9a confirmatory EKS run remains open per the card; a route fact, not a deviation.
6. `FREEZE.sha256` is bound by the card (62747ae6...1401) and matches the file on disk; the receipt stores no per-bind hashes, so this check rests on the card and the disk, not the receipt.

## Outcome fields

```yaml
card_id: CARD-p2-lift-linear-complexity-20261110-F2-RUN-02
relay_verbatim: not_established   # no executor text; carrier was the main-session supervisor; lint now fails because outputs exist
outcome: executed_as_specified    # 13 of 13 commands, exit 0, hashes match, stop_when conditions not triggered
stopped_at_step: null
recommendation: proceed_to_analysis
reasons:
  - receipt F2-RUN-02.receipt.json status completed, 13 of 13 exit 0, 26 of 26 output hashes match disk
  - verify-freeze 4 of 4 OK; git-head equals pinned 9465664; frozen files unchanged since the pin
  - test line "7 passed; 0 failed"; run-full exit 0, fatal array empty in results/run.json (sha256 21591ebb...bf1e)
  - tier deviation (main-session supervisor in place of executor-mechanical) recorded, as for F3-RUN-02 and F4-RUN-02
interpretation: not_assessed
```
