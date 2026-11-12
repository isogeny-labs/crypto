# Assessment of card CARD-coord-spectrum-20261110-F3-RUN-02

Assessor: experiment-director, assess mode. Interpretation of the scientific result: not_assessed.
No claim is made. Nothing was edited except this file.

## Classification: complete (with the caveats listed under "Deviations and caveats")

The 13 commands of the card ran in card order, once each, all with exit 0, and every
receipt output exists on disk with a hash equal to the receipt's recorded hash. The
run-full step ran to its own exit 0 and the tool printed its closing `wrote ... run.json` line.

## Evidence per check

| Check | Evidence | Result |
|---|---|---|
| Receipt status | `F3-RUN-02.receipt.json`: status `completed`, commands_total 13, 13 entries, each `attempts: 1`, `exit_code: 0`, `timed_out: false` | pass |
| Card identity | receipt card_sha256 d0918644...6f3a equals `sha256sum` of `cards/F3-RUN-02.json` today; receipt commit 9465664dbe6ba5757b9cb011192215a00605e8d8 equals the card commit | pass |
| argv equals card | script comparing receipt `commands[i].id`/`argv` with card `commands[i]` for all 13: no difference. run-full argv: `coord_spectrum --protocol .../PROTOCOL.md --sets R16,R20,ANOM --out research/coord_spectrum_20261110/results/run.json`, identical in card, receipt and `11-run-full.spec.json` | pass |
| Output hashes | for all 13 commands, sha256 of the stdout and stderr files on disk equals receipt `stdout_sha256`/`stderr_sha256` (26 of 26 match, 0 mismatches) | pass |
| Receipt file hash | `sha256sum F3-RUN-02.receipt.json` = f1b23496...1900, equal to the `receipt: ... sha256` line in the scratchpad `F3-RUN-02.wait.log` (line 4) | pass |
| verify-freeze | `F3-RUN-02.verify-freeze.stdout`: 4 lines, all `OK` (PROTOCOL.md, tool/src/main.rs, tool/Cargo.toml, tool/Cargo.lock); empty stderr. FREEZE.sha256 lines (4bfd872a..., ecb877ef..., a3f4cf8b..., 3cc4284b...) equal the card binds, and `sha256sum` of those four files today gives the same values | pass |
| FREEZE.sha256 itself | card bind 4891d5aa563392fe32dd9c3035c2d65c43bd749af8efca329853facb8775bf7a equals `sha256sum` today | pass |
| git-head receipt | `F3-RUN-02.git-head.stdout` = 9465664dbe6ba5757b9cb011192215a00605e8d8 = card pin. `git diff --stat 9465664 HEAD` over PROTOCOL.md, FREEZE.sha256 and tool/ is empty (HEAD today 8f468f587e) | pass |
| Toolchain | rustc 1.97.0 (2d8144b78 2026-07-07), cargo 1.97.0 (c980f4866), nproc 4, uname `Linux vm 6.18.44-fc-v378 ... x86_64`, lscpu 3278 bytes, all exit 0 | recorded |
| build | exit 0, 0.57 s; stderr ends `Finished release profile [optimized] target(s) in 0.49s`, preceded by 4 `Blocking waiting for file lock` lines (another cargo process was running). No recompilation | pass; see caveat 3 |
| test receipt | `F3-RUN-02.test.stdout` contains `test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s`; the 7 tests listed are all `ok`. This is the card's stop_when line (`test result: ok. 7 passed; 0 failed`) | pass |
| binary sha256 | `F3-RUN-02.binary-sha256.stdout` 7e08c12ec3865247299923e9f71398260dfe605b261ce66c5a78226496fe1c4e; equals the card bind and `sha256sum` of the file on disk today; also equals the `F3-RUN-01.binary-sha256.stdout` value | pass |
| run-full | exit 0, 15500.901 s (limit 28800), `11-run-full.exit.json` finished_at 2026-11-11T23:57:57Z, timed_out false, one attempt; pid file started_at 2026-11-11T19:39:36Z, pid 3704, boot_id b5ca9da6-...; `results/run-02.stderr.txt` (19 lines) ends `[15500.9s] ANOM_20 done` and records `18 curves built`, R16_0..7, R20_0..7, ANOM_16, ANOM_20 each `done` | pass |
| protocol_sha256 / tool_source_sha256 lines | `results/run-02.stdout.txt` lines 322-323: `protocol_sha256   4bfd872a04d0e7b671446c6016a541f2e7384fdecc411e1c4d05a0810be2ec7f` and `tool_source_sha256 ecb877ef619ea88e80fbe096059cd43bfb4022f67ecc1e5b2f3fc73c208289a6`. Both equal FREEZE.sha256 (PROTOCOL.md, tool/src/main.rs). `run.json` header carries the same two values | pass |
| result sha256 | `F3-RUN-02.result-sha256.stdout` 642bf9cd48c7f11fabc73f29c7563e488c14742148c5aa4f948ea52daa8dc30a; equals `sha256sum results/run.json` today and the receipt run-full `outputs` entry for run.json | pass |
| result table | `results/run-02.stdout.txt` sha256 3c8c10b1...6111, identical to `F3-RUN-02.result-table.stdout` (receipt stdout_sha256 of command 13 and run-full output entry are the same value) | pass |
| run.json consistency (structure only) | top-level keys header/curves/summary; header: quick false, sets [R16,R20,ANOM], curves_per_random_set 8, null_functions_per_curve 20, a_values_per_function 64, energy_null_subsets 50; 18 curves (8 R16 + 8 R20 + 2 ANOM). Tool's final line: `wrote ... run.json (151455 bytes)`; `ls` shows run.json 151455 bytes | pass |
| Supervisor / wait log | `F3-RUN-02.run.log`: `run rc=4` with command 8 DETACHED (pending 5 commands, next `task_card.py wait`); `F3-RUN-02.wait.log`: one `status: completed` block and `final rc=0` (167 lines). rc=4 is the tool's detached-pending status | consistent |
| write_scope | Card outputs are all under `research/coord_spectrum_20261110/`. Receipt dirty_after has 122 paths; outside `coord_spectrum_20261110` they are `research/p2_lift_linear_complexity_20261110/...` (F2-RUN-02), `research/sat_ladder_scaling_20261110/...` (F4-REP-01) and `resume/RESUME.md`. `resume/RESUME.md` is in dirty_before too and `git status` shows ` M .resume/RESUME.md`; no F3 command names it, so the receipt cannot attribute it to this card. No F3 write outside scope found | pass; see caveat 4 |

## Self-containment of the -02 chain w.r.t. the lost -01 run

Yes, for execution. Every step, hash and output of -02 is produced by -02:
- The card pins its own commit (9465664), its own sha256, and its own binds; the receipt, run directory, `results/run.json`, `run-02.stdout.txt` and `run-02.stderr.txt` are listed in `must_not_exist`, so none of them existed before the run and `run.json` is not a leftover of -01.
- The -01 receipt (`F3-RUN-01.receipt.json`) has status `detached` and no exit record for run-full (build 0, test 0, binary-sha256 0 recorded; run-full `exit_code` None); no -01 output is read by any -02 command. -01 git-head was 7de7236cf320e9f1d76260f2ca0d558d3ed5b865, -02 is 9465664; the -01 binary sha256 equals the -02 one (7e08c12e...), so both runs used the same executable bytes.
- Loss record `benchmarks/losses/20261111-card-coord-spectrum-20261110-f3-run-01-11-run-full.json` (crypto.infra-loss/v1, cause container_restart, step 11-run-full of F3-RUN-01 lost, pid 22047 gone without an exit record, started 2026-11-10T19:49:34Z, pcc [pcc:unverified]) now reads `relaunched: true`, `relaunch_ref: research/coord_spectrum_20261110/cards/F3-RUN-02.json (receipt completed 2026-11-11T23:5xZ, 15500.9 s on 4 threads)`. The change was made in commit 8f468f587e (4-line diff, `false/null` to `true/ref`). The `23:5xZ` text is approximate; the exit record says 23:57:57Z.
- Stale -01 partial logs remain in the tree: `results/run.stdout.txt` (204 lines, 14450 bytes, sha256 030f4dad...6a) and `results/run.stderr.txt` (13 lines, ends `[8218.1s] R20_3 done`). They are not -02 outputs. The 204 lines of `run.stdout.txt` are byte-identical to the first 204 lines of `run-02.stdout.txt` (`diff` empty); this is recorded as a fact only. Consumers should read `run-02.*` and `run.json`.

## Deviations and caveats

1. The carrier was the main session's nohup supervisor running `tools/task_card.py run` and wait loops, not an `executor-mechanical` subagent as `executor_role` states. The argv that ran equal the card's argv (comparison above), so this is a role deviation of the carrier, not of the commands. No executor text report exists, so `relay_verbatim` is not established and `check-relay` was not run. `task_card.py lint` now fails on the card (`commands[10].argv`, `commands[11].argv`: run.json; `commands[12].argv`: run-02.stdout.txt, "names repository file ... that binds does not pin"). The card file sha256 equals the receipt's, so the card is unchanged; the lint failure appears only because the card's own outputs now exist on disk. Lint at compile time was not re-checked.
2. The card pins commit 9465664, which is the HEAD recorded by git-head; the frozen-file diff 9465664..HEAD is empty, so HEAD movement has no effect on the inputs.
3. The build step was a no-op (0.49 s, `Finished`); the binary is the build on disk before this card, as card notes state. Its sha256 equals the bound value; test ran on the same package. This assessment did not rebuild.
4. Shared host: file-lock waits on the package cache in build and test stderr; F2 and F4 work ran concurrently; nproc 4 and the tool uses all available parallelism (printed `on 4 threads`). Wall times are descriptive only per the card.
5. Specification errors recorded in the card notes remain open and are not repaired here: the tool has no `--threads` flag and no null-count flag (full-run null count is the constant 20 per class and 8 curves; run.json header shows `null_functions_per_curve` 20, `quick` false), so the requested `--threads 2` and an explicit null count are not in the card.
6. Execution route is local (card notes: no aws CLI, GitHub route takes only ecbench specs). The AGENTS.md section 9a confirmatory EKS run remains open per the card; a route fact, not a deviation.
7. Commit 8f468f587e also committed the stale -01 `run.stdout.txt`/`run.stderr.txt` together with the -02 outputs (see self-containment).

## Outcome fields

```yaml
card_id: CARD-coord-spectrum-20261110-F3-RUN-02
relay_verbatim: not_established   # no executor text; carrier was the main-session supervisor; lint now fails because outputs exist
outcome: executed_as_specified    # 13 of 13 commands, exit 0, hashes, stop_when conditions not triggered
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed
```
