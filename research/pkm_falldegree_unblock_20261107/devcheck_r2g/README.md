# devcheck_r2g: local test of `stageb_units_runner.sh`, revision 4 (draft Amendment R2-G)

Date 2026-11-10. Development check only. Every row produced here uses `--seed 777` (the study seed is the pilot's default,
1347112276); none is stage B data. Nothing was submitted to the cloud, committed or pushed; no real S3 was used (a local
directory shim stands in). All numbers are `[pcc:unverified]` (local, one host, 4 vCPU, 2 rayon threads).

Runner under test: sha256 `1c2229386976e8d6f22e5f243b269a5d23d588d0e105d6cb530c9a5488157a79`, 28966 bytes. Request built by
`make_request_bu.sh` from it (scratch directory): 4 runner pieces, BOOT sha256
`572ce3fb23e57416732a64a7cc25523c5ff84bd8496a7dd59455e07b1b6bcade`. Frozen binaries: `sha256sum` of `pkm_tower_pilot`,
`hky_fp_tower_check`, `prime_fall_degree_tools` equals `image/binaries.sha256`.

Tools. Host: bash 5.2.21(1)-release, mawk 1.3.4 20240123. Job image (measured by the main session from its base layer, Amazon
Linux 2023): bash 5.2.15(1)-release (x86_64-amazon-linux-gnu) and GNU Awk 5.1.0, API 3.0 (GNU MPFR 4.1.0-p13, GNU MP 6.2.1).
The image has bash 5.2.15, not 4.2.

## The three suites (each runs T1..T22, T24, T25, T26, T28..T32 = 30 tests, on the final runner text)
| Form | Output | Started as | Result |
|:--|:--|:--|:--|
| file form | `test_outputs.txt`, `progress/` | `/bin/bash <runner> <args>`, host bash and mawk | 30 of 30 PASS, 0 failed checks |
| job form (T23) | `test_outputs_jobform.txt` | the request's own command words (`/bin/bash -c <BOOT> stageb <4 pieces>`, BOOT sha256 above), host bash and mawk | 30 of 30 PASS, 0 failed checks |
| image shell (T27) | `test_outputs_imgshell.txt` | `IMG_BASH` (the image's bash) running the runner, `IMG_AWK` (`img_gawk_wrapper.sh`, gawk with LD_LIBRARY_PATH set for it only) first on PATH as `awk` and `gawk` | 30 of 30 PASS, 0 failed checks |

Every output starts with a header: the start form (command words before the runner's own arguments; a word over 200 characters is
shown by its sha256), the shell and awk in use. `compare_outputs.sh a b` removes timestamps, ms, elapsed seconds and the pids of
"Killed" notices and diffs. Differences, all of them:
- file form vs job form: only the header (job form shows `-c`, the BOOT hash, `stageb` and the four piece hashes instead of the runner path). No difference in any test line.
- file form vs image shell: the header (shell and awk lines, the shell path); the `bash:` line of host.txt in T2 (5.2.15 vs 5.2.21); T12 counts 32 tools instead of 31 (gawk present). Nothing else.

## Files
- `test_driver.sh Tn|header`, `run_suite.sh <out> [progress dir]` (env `BOOT_REQUEST`, `IMG_BASH`, `IMG_AWK`, `WROOT`), `compare_outputs.sh`.
- `s3shim.sh` (local stand-in for `aws s3 cp|sync`; stores `s3://<bucket>/runs/pfd-bu-<n>-<stamp>/<rest>` at `<bucket>/<rest>`),
  `shim_faults.sh` (fails or delays the n-th progress upload, snapshots S3; from the review's shim2.sh), `fail_unit_wrapper.sh`
  (pilot stand-in that fails one unit by environment variable), `img_gawk_wrapper.sh`.
- Predating revision 4, kept: `README_rev3_predates_revision4.md`, `test_outputs_rev3_predates_revision4.txt`,
  `test_outputs_jobform_rev3_predates_revision4.txt`, `test_outputs_imgshell_rev3_predates_revision4.txt`,
  `progress_rev3_predates_revision4/`; and the revision 2 and 1 files.

## Adapted from revision 3 and why
- The driver gives every job `ISOLAB_JOB_ID=pfd-bu-<n>-777` and the result URI `s3://b/runs/pfd-bu-<n>-777/<label>/`; the number n is 1 + the
  highest PREV or skip number given (or `JN=`). PREVs are given as `$(pv <label>)`. Revision 4 requires this (R1). Hand-made earlier jobs
  (T8, T12, T14, T16c, T18c, T19, T28, T31) are copies of real jobs with their `job:` line set to a distinct number (`fakejob`).
- T12: the resume job no longer carries `allow:` for a done unit (revision 4 refuses that, R2); an `allow:B-m3-tower-t3:3` run for a lost unit (t12f) takes its place.
- T18c: the two jobs that both completed the units are job 1 and a copy of it as job 2 (two independent job-1 prefixes would now fail on the job number first).
- T22: the exit handler waits 5 s, not 20 (R5): LOCK_WAIT=2, limits 7..30 s and under 8 s (release after 5 s).
- T26: the skip word is now passed to the runner; piece count read from the request (4); STAMP check and BOOT exit 18 added.
- T21/T29 close/allow use labels through `pv`; T2 records the job name instead of a test value for `job:`.

## Tests: what each shows, result (all three forms), what it does not test
| Test | Shows | Result | Not tested |
|:--|:--|:--|:--|
| T1 | `--list`: 91 units, 25 ladders, 49 + 42, sha256 0f80105d... | PASS | exit 11 on a modified list |
| T2 | happy path: argv records, units_done.txt, host.txt lines (NODE_NAME, POD_UID, job, runner_sha256, skipped:, closed_tool_failure:) | PASS | /opt/study/build-info.json |
| T3 | ladder stop (budget 0 closes at 2, budget 2 at 4) | PASS | production budget |
| T4 | single-unit rows equal one registered invocation (4 ladders) | PASS | m >= 4 |
| T5 | SIGKILL resume, two kills then `lost`, 75 | PASS | |
| T6 | pilot exit 3: exit 20 | PASS | |
| T7 | SIGTERM mid-unit: 143, no pilot, stop file, no lock | PASS | |
| T8 | unreadable or incomplete PREV (7 variants): 12; control accepted | PASS | |
| T9 | upload failures: 13 and abort lines | PASS | persistent failure leaves no abort in S3 |
| T10 | test-only variables incl. LOCK_WAIT refused (15) | PASS | |
| T11 | an S3 command reading stdin cannot eat the unit list | PASS | |
| T12 | image tool set only: runs, resume, 75, close:, allow:, tool-failure closure | PASS | |
| T13 | `//` keys read verbatim | PASS | real aws |
| T14 | completeness exit 14 | PASS | |
| T15 | `close:` for a lost unit; bad words 16 | PASS | |
| T16 | tool failure twice closes the ladder, no rerun; fail once then pass; 75 lists every closure | PASS | OOM-killed pod |
| T17 | units_done.txt vs progress.txt in S3 after a failed end upload and SIGTERM mid-upload (one-snapshot upload) | PASS | SIGKILL between the two uploads |
| T18 | job left out 12; duplicate PREV 16; unit done in two PREVs 12 | PASS | |
| T19 | provenance mismatches 12 each | PASS | |
| T20 | start upload failing twice: abort lines, no lost unit | PASS | |
| T21 | `close:` carried; `close:` without a lost unit 16; allow chains | PASS | |
| T22 | bounded lock wait, handler waits at most 5 s, syncer stops at the stop file | PASS | the real 300 s period |
| T23 | the whole suite through the request's boot command | PASS (30 of 30) | exit 17/18 in a real pod |
| T24 | argument and URI errors outside test mode; `bash -c` refused | PASS | |
| T25 | sha256, no capital Q, `bash -n` on every script, list hash, construct grep | PASS | |
| T26 | `make_request_bu.sh`: pieces, BOOT, exit 17 (changed piece) and 18 (mktemp), skips passed on, STAMP, overrides, Q refusal | PASS | the real placeholder fill |
| T27 | the whole suite with the image's bash 5.2.15 and gawk 5.1.0 | PASS (30 of 30) | a real pod |
| T28 | R1: most recent job left out (r10) 12; gap 12; PREV and skip for one job 12; valid `skip:2:costguard` accepted, recorded, carried; contradicting carried skip 12; bad ISOLAB_JOB_ID 2; job: line vs URI 12; PREV not earlier 12; bad skip words 16 | PASS | |
| T29 | R2: `allow:` for a never-started unit (e1) 16; done unit 16; close+allow 16; K above the count accepted and runs; K not above the in-flight count 16 | PASS | |
| T30 | R3: TERM during the start upload (e5): `abort ... terminated-before-start` in S3; two such jobs do not make the unit lost | PASS | |
| T31 | R7: a PREV closure the job does not recompute 12; missing line 12; real closure agrees | PASS | |
| T32 | R5: handler uploads units_done.txt, progress.txt, runs, gates, rest, in that order (shim log) | PASS | slow S3 |

## Not tested
Real `aws s3` commands and error output; G1 failure (exit 10); exit 11 on a modified list; m >= 4 ladders; the 12-thread 56Gi pod;
production budgets; exits 17 and 18 inside a real pod; a SIGKILL timed between two uploads; a persistent S3 failure (no abort reaches S3).
