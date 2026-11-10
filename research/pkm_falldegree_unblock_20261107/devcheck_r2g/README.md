# devcheck_r2g: local test of `stageb_units_runner.sh`, revision 3 (draft Amendment R2-G)

Date 2026-11-10. Development check only. Every row produced here uses `--seed 777` (the study seed is the pilot's default,
1347112276); none is stage B data. Nothing was submitted to the cloud, committed or pushed; no real S3 was used (a local
directory shim stands in). All numbers are `[pcc:unverified]` (local, one host, 4 vCPU, 2 rayon threads, bash 5.2.21, mawk).

Runner under test: `stageb_units_runner.sh` revision 3 (sha256 and size are in `test_outputs.txt`, test T25, and in the
report that accompanied this check). Frozen binaries: `sha256sum` of `pkm_tower_pilot`, `hky_fp_tower_check`,
`prime_fall_degree_tools` in the scratchpad layer equals `image/binaries.sha256` (f78f3959..., c36d324e..., 7e8a6703...).

## Files
- `test_driver.sh Tn`: one test. Starts the runner as a FILE (`/bin/bash <runner> <args>`); with `BOOT_REQUEST=<request.json>`
  exactly as a job does (the request's command words up to and including its last runner piece). Each test prints
  `check PASS|FAIL: ...` lines and ends with `RESULT Tn PASS|FAIL`.
- `run_suite.sh <out> [progress dir]`: runs T1..T22, T24, T25, T26 and writes the verbatim outputs (paths shortened to `$T`,
  `$LAYER`, `$SCRATCH`). T23 is the same script with `BOOT_REQUEST` set.
- `s3shim.sh`: local stand-in for `aws s3 cp|sync` (unchanged from revision 2). `shim_faults.sh`: wrapper that fails the n-th
  progress upload, delays one, or snapshots S3 (from the review's `shim2.sh`). `fail_unit_wrapper.sh`: stands in for the pilot and
  fails one unit by environment variable (from the review's `failwrap.sh`).
- `compare_outputs.sh`: diff of two output files after removing timestamps, ms, elapsed seconds, pids.
- `test_outputs.txt`, `progress/`: verbatim outputs and the `progress.txt` of every test job of the file-form run.
- `test_outputs_jobform.txt`: the same suite started through the request (T23).
- Predating revision 3 (kept as they were): `README_rev2_predates_revision3.md`, `test_outputs_rev2_predates_revision3.txt`,
  `progress_rev2_predates_revision3/`, and the revision 1 files `*_rev1_predates_revision*`.

## Adapted from revision 2 (T1-T15) and why
- Start form: file form instead of `bash -c "<text>" stageb` (spec A1).
- PASS/FAIL is now computed by `check` lines; revision 2 judged by reading.
- Earlier jobs now need `host.txt`, `binary.sha256` and `runs/<unit>.argv` that match: real PREV jobs are unchanged; T14's hand-made
  PREV copies host.txt and binary.sha256 from a real job and gets an argv record; T8 gained argv, host, progress, binary variants
  and an unedited control.
- T6 uses `fail_unit_wrapper.sh` (FAIL_UNIT) instead of an inline wrapper; same behaviour.
- T15: `close:` is accepted only for a lost unit; the two-kill chain makes t4 lost, as before; the four bad words still give 16.
- T13: the two jobs use different UNIT_FILTER; this is accepted because unit_filter is not compared (spec A3 lists three lines).
- T9 also checks the abort lines; T10 adds LOCK_WAIT (refused, and non-numeric refused); T12 also runs a resume with PREV and
  `allow:`, and hand-made lost / `close:` / tool-failure cases under the restricted PATH.

## Tests: what each shows, result, what it does not test
Results are from `test_outputs.txt` (file form); T23 repeats them through the job form.

| Test | Shows | Result | Not tested |
|:--|:--|:--|:--|
| T1 | `--list`: 91 units, 25 ladders, 49 + 42, sha256 0f80105d... unchanged | PASS | exit 11 on a modified list |
| T2 | happy path: 3 units, argv records equal the pilot words, units_done.txt, host.txt lines incl. NODE_NAME, POD_UID, ISOLAB_JOB_ID, runner_sha256 | PASS | /opt/study/build-info.json (path fixed, absent here) |
| T3 | ladder stop (budget 0 closes at t=2, budget 2 at t=4), registered-style run stops at the same level | PASS | production budget 1800 |
| T4 | rows of single-unit jobs equal one registered invocation (B tower, DB tower, P0 isogeny, null chain), all fields except wall times | PASS | m >= 4, other ladders |
| T5 | SIGKILL resume: done-in skips, t4 re-run, two kills then `lost`, exit 75 | PASS | |
| T6 | pilot exit 3: exit 20, files and end line uploaded, no end 0, later unit not started | PASS | |
| T7 | SIGTERM mid-unit: exit 143, no pilot left, stop file, no lock, units_done = t3 only | PASS | |
| T8 | unreadable or incomplete PREV (log, jsonl, summary, argv, host, progress, binary.sha256): exit 12, no unit started; control accepted | PASS | |
| T9 | upload failures: exit 13, abort lines (start, files) | PASS | persistent failure leaves no abort line in S3 (final upload fails too) |
| T10 | test-only variables incl. LOCK_WAIT refused outside test mode (exit 15), fake aws never called | PASS | |
| T11 | an S3 command that reads stdin cannot eat the unit list | PASS | |
| T12 | image tool set only (PATH of symlinks; no flock, python, jq; gawk absent here): runs, resume, 75, close:, tool-failure closure | PASS | gawk, bash 4.2 |
| T13 | trailing-slash PREV: `//` keys read verbatim | PASS | real aws |
| T14 | completeness exit 14 with a hand-made PREV | PASS | |
| T15 | `close:` accepted for a lost unit; bad words exit 16 | PASS | |
| T16 | tool failure twice: third and fourth job close the ladder with no argument and no rerun; failed once then passes: ladder continues; exit 75 still lists stop-line, carried-rule and tool-failure closures | PASS | OOM-killed pod path |
| T17 | units_done.txt vs progress.txt in S3 after a failed end-line upload (retry) and after SIGTERM during that upload; invariant checked in every snapshot | PASS | SIGKILL between the units_done upload and the progress upload (units_done may then list a unit whose end line is not in S3; the next job treats it as not done) |
| T18 | job left out: exit 12; duplicate PREV: 16; unit done in two PREVs: 12 | PASS | |
| T19 | other pilot_extra, other runner (genuine copy), edited testmode, binary hash, argv, missing runner_sha256: exit 12 each; control accepted | PASS | |
| T20 | start upload failing in two jobs: abort lines, third job has no lost unit and runs it | PASS | |
| T21 | `close:` carried forward; `close:` without a lost unit: 16; `allow:` runs the unit, carried to a later job, 3 in flight with allow 3: 75; bad words | PASS | |
| T22 | leftover lock with LOCK_WAIT=3: exit 13 in bounded time; handler waits for a lock released by someone else; syncer stops at the stop file (fake sleep: 2 s period) | PASS | the real 300 s period, an orphaned aws child |
| T23 | whole suite through the request's boot command | PASS | all 25 tests PASS in `test_outputs_jobform.txt` (summary block at its end); `compare_outputs.sh test_outputs.txt test_outputs_jobform.txt` prints 0 differing lines after removing timestamps, wall times and the pids of bash's own "Killed" notices. Checked by the main session, 2026-11-10 18:00Z, on runner sha256 `33bf8d84...`. |
| T24 | argument and URI errors outside test mode (16, 2), PREV regex, `bash -c` form refused (2) | PASS | |
| T25 | sha256, no capital Q, `bash -n` on every script, list hash, grep for constructs newer than bash 4.2 | PASS | bash 4.2 itself (not available; reading and grep only) |
| T26 | `make_request_bu.sh`: pieces, BOOT, exit 17 on a changed piece, arguments, skips, overrides, Q refusal, validator | PASS | the real placeholder fill at the freeze |

## Not tested
Real `aws s3` commands and error output; bash 4.2; gawk; G1 failure (exit 10); exit 11 on a modified list; m >= 4 ladders; the
12-thread 56Gi pod; production budgets; exit 17 inside a real pod; a SIGKILL timed between two uploads.
