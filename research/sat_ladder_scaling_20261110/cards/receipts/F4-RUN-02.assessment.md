# Assessment of card CARD-sat-ladder-scaling-20261110-F4-RUN-02

Assessor: experiment-director, assess mode. Interpretation of the scientific result: not_assessed.
No improvement is claimed. Nothing was edited except this file.

## Classification: complete (with the caveats listed under "Deviations and caveats")

The 13 commands of the card ran in card order, once each, all with exit 0, and every
receipt output exists on disk with a hash equal to the receipt's recorded hash.

## Evidence per check

| Check | Evidence | Result |
|---|---|---|
| Receipt status | `F4-RUN-02.receipt.json`: status `completed`, commands_total 13, 13 entries, each `attempts: 1`, `exit_code: 0`, `timed_out: false` | pass |
| Card identity | receipt card_sha256 89d464ed...a604 equals `sha256sum` of `cards/F4-RUN-02.json` today; receipt commit 9465664dbe6ba5757b9cb011192215a00605e8d8 equals the card commit | pass |
| argv equals card | script comparing receipt `commands[i].id`/`argv` with card `commands[i]` for all 13: no difference. `11-run-full.spec.json` argv: `--n 4,5,6,7,8,9 --targets 10 --budget 4000000 --threads 2 --out research/.../results/run.json` as in the card | pass |
| Output hashes | for all 13 commands, sha256 of the stdout and stderr files on disk equals receipt `stdout_sha256`/`stderr_sha256` (26 of 26 match, 0 mismatches) | pass |
| verify-freeze | `F4-RUN-02.verify-freeze.stdout`: 4 lines, all `OK` (PROTOCOL.md, tool/src/main.rs, tool/Cargo.toml, tool/Cargo.lock); empty stderr. Re-run now: same 4 OK. FREEZE.sha256 lines equal the card binds (156fe810..., 6627a932..., b94869ee..., b4685bb4...) | pass |
| git-head receipt | `F4-RUN-02.git-head.stdout` = 9465664dbe6ba5757b9cb011192215a00605e8d8 = card pin. Current repo HEAD is a10de608f9 (the commit that stored these results, made after the run); `git diff --stat 9465664 HEAD` over PROTOCOL.md, FREEZE.sha256, tool/src, Cargo.toml, Cargo.lock is empty | pass |
| Toolchain | rustc 1.97.0 (2d8144b78 2026-07-07), cargo 1.97.0, nproc 4, uname `Linux vm 6.18.44-fc-v378`, lscpu 3278 bytes recorded, all exit 0 | recorded |
| build | exit 0, 0.92 s; stderr ends `Finished release profile ... in 0.82s` (no recompilation: the binary on disk was up to date, preceded by 4 lines `Blocking waiting for file lock`, i.e. another cargo process was running) | pass; see caveat 3 |
| test receipt | `F4-RUN-02.test.stdout` contains `test result: ok. 7 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 152.70s`, the exact stop_when line; the ignored test is `planted_k_recovered_n6` | pass |
| binary sha256 | `F4-RUN-02.binary-sha256.stdout` 8e6cc0b7945ee2658ec2f92bb4a7a56ec7d2e79edb02316e8e2332792aef0ca0; equals the card bind, and `sha256sum` of the file on disk today | pass |
| run-full | exit 0, 12010.542 s (limit 43200), `11-run-full.exit.json` finished_at 2026-11-11T23:02:16Z, timed_out false; `results/run-02.stderr.txt` last line `wrote research/sat_ladder_scaling_20261110/results/run.json` | pass |
| result sha256 | `F4-RUN-02.result-sha256.stdout` a12f6c56bf42ba816a54149b13eb3cfd6d97751a68225f3331be2c7cd1425079; equals `sha256sum results/run.json` today | pass |
| result table | `results/run-02.stdout.txt` sha256 7ae1007c...b35309d, identical to `F4-RUN-02.result-table.stdout` (receipt stdout_sha256 of command 13 and of command 11 are the same value) | pass |
| run.json consistency (structure only) | header: ns [4..9], targets 10, conflict_budget 4000000, threads 2, quick false, protocol_sha256 156fe810... and source_sha256 6627a932... equal the frozen hashes; 120 rows = 6 n x 2 arms (FRESH, REUSE) x 10 targets, each (n, arm) has exactly 10 rows; solver CaDiCaL 3.0.1, fallback_solver_used false | pass |
| Wait log | `F4-RUN-02.wait.log` (scratchpad): `status: completed`, line 149 `final rc=0`; run log shows the first call reported `status: detached` at command 11, as the card requires for a >540 s timeout | consistent |
| write_scope | Card outputs are all under `research/sat_ladder_scaling_20261110/`. Receipt dirty_after lists 124 paths; outside the F4 directory they are `research/coord_spectrum_20261110/...`, `research/p2_lift_linear_complexity_20261110/...` (concurrent F3-RUN-02 and F2-RUN-02 cards) and `resume/RESUME.md`. `resume/RESUME.md` (shown as ` M .resume/RESUME.md` in git status today) is also in dirty_before, so the receipt cannot attribute it to this card; no F4 command names it | no F4 write outside scope found; see caveat 4 |

## Self-containment of the -02 receipt chain

Yes for execution. The -02 chain does not use any -01 output: every step, the
frozen-file hashes, the binary hash, the test result, the result hash and the table
come from -02 receipts written between 2026-11-11T19:38Z and 23:02Z. The card pins its
own commit, its own sha256, and sends stdout and stderr to `run-02.*` files. Its
`results/run.json` did not exist before the run (must_not_exist), so the result is not
a leftover of -01.
The link to -01 is documentary only: the card notes name the loss record
`benchmarks/losses/20261111-card-sat-ladder-scaling-20261110-f4-run-01-11-run-full.json`
(schema crypto.infra-loss/v1, cause container_restart, step 11-run-full of F4-RUN-01
lost, pid 29413 started 2026-11-10T20:18:48Z, relaunched: false, relaunch_ref: null,
pcc: [pcc:unverified]). That record exists, but its `relaunched`/`relaunch_ref` fields
were not updated to point at F4-RUN-02 (not edited by this assessment).

## Deviations and caveats

1. The executor was the main session's nohup supervisor running `tools/task_card.py run`
   and wait loops, not an `executor-mechanical` subagent as `executor_role` states. The
   argv that ran are the card's argv (receipt vs card comparison above), so this is a
   role deviation of the carrier, not of the commands. No executor text report exists, so
   `relay_verbatim` could not be established: `check-relay` refuses to run because
   `task_card.py lint` now fails on the card (lines: `commands[10].argv`,
   `commands[11].argv`: run.json, `commands[12].argv`: run-02.stdout.txt "names repository
   file ... that binds does not pin"). This failure appears only because the card's own
   outputs now exist on disk; they did not exist when the card ran, and the card is
   unchanged (sha256 matches the receipt). Lint at compile time was not re-checked by
   this assessment.
2. The card was itself committed at ec62139dd3 (`F2/F3/F4: re-run cards -RUN-02 ...`),
   and the run recorded commit 9465664 as HEAD; the card's pinned commit is therefore the
   parent state of the card commit. The receipt's frozen-file check and the empty diff of
   frozen files between 9465664 and today's HEAD show this has no effect on the inputs.
3. The build step was a no-op (0.82 s, binary "up to date"); the "binary this card
   builds" is the pre-existing artifact. Its sha256 equals the bound value, and the
   test step ran against the same package. This assessment did not rebuild.
4. The shared-host condition (file-lock waits on the package cache; F2 and F3 cards
   running concurrently; nproc 4 with `--threads 2`) is recorded in the card notes. Wall
   times in `run.json` are descriptive only per the card.
5. Stale -01 files `results/run.stdout.txt` (0 bytes) and `results/run.stderr.txt`
   (106 lines at commit a10de608, partial log from the lost -01 run) remain in the tree;
   they are listed in the receipt's dirty_before and are not -02 outputs. Consumers
   should read `run-02.*` and `run.json`.
6. Execution route is local (card notes: no aws CLI, GitHub route takes only ecbench
   specs); the AGENTS.md section 9a confirmatory EKS run remains open per the card. This
   is a route fact, not a deviation from the card.

## Outcome fields

```yaml
card_id: CARD-sat-ladder-scaling-20261110-F4-RUN-02
relay_verbatim: not_established   # no executor text; check-relay blocked by post-run lint failure (caveat 1)
outcome: executed_as_specified    # all 13 commands, exit 0, hashes and stop_when conditions met
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed
```
