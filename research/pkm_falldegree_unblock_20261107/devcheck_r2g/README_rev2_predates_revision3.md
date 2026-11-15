# devcheck_r2g: local test of `stageb_units_runner.sh` (draft Amendment R2-G)

Date 2026-11-10. Development check only. Every test row here is produced with `--seed 777` (the study seed is the pilot's
default, 1347112276); none is stage B data. Nothing was submitted to the cloud, committed or pushed. All numbers are
`[pcc:unverified]` (local, one host, 4 vCPU, 2 rayon threads).

## Binaries (verified)
Image layer `sha256:a0f7e28e...b1971` of `isolab/tasks` (us-east-2) was downloaded with `aws ecr get-download-url-for-layer`
and untarred into the scratchpad. `sha256sum` of `usr/local/bin/{pkm_tower_pilot,hky_fp_tower_check,prime_fall_degree_tools}`
equals `image/binaries.sha256` for all three (f78f3959..., c36d324e..., 7e8a6703...). The tests use the real frozen pilot and G1 binary.

> **Revision 2 (2026-11-10, runner sha256 773a978d...).** Sections "Files" to "NOT tested" and "Added by the main session"
> below describe runner revision 1 (sha256 143242e2...) and PREDATE this revision; their raw outputs are
> `test_outputs_rev1_predates_revision.txt` and `progress_rev1_predates_revision/`. T1-T6 were re-run on revision 2 (see the end);
> `test_outputs.txt` and `progress/` are the revision 2 outputs.

## Files
- `test_driver.sh T1..T6` (and `T5k`, a kill check): the driver. It runs the runner exactly as a job does,
  `bash -c "$(cat stageb_units_runner.sh)" stageb <PREV...>`, with `S3CP/S3SYNC/S3LS` replaced by `s3shim.sh` (a local
  directory standing in for S3), `RAYON_NUM_THREADS=2`, `PILOT_EXTRA="--seed 777 ..."`, and `UNIT_FILTER` to small unit sets.
- `s3shim.sh`: `cp`, `sync`, `ls` on a local directory.
- `test_outputs.txt`: verbatim output of T1..T6 (paths shortened to `$T`, `$LAYER`).
- `progress/`: the `progress.txt` of each test job (small).

Commands: `devcheck_r2g/test_driver.sh T1` ... `T6` (about 6 minutes in total; T4 and T5 contain two 75 s pilot runs each).

## Outcomes (verbatim excerpts are in `test_outputs.txt`)
- **T1 list: PASS.** `--list` prints 91 lines, 25 distinct ladders in the stated order (B-m3..m6 tower / p0-kummer / p0-isogeny,
  B-m3-null, then DB-...), 49 `B-` and 42 `DB-` units, 13 hky-B ladders; max t per ladder 8,5,5 / 6,4,4 / 6,3,3 / 6,2,2 / null 8.
  The count assertion (exit 11) was not triggered; it was not tested with a deliberately wrong list.
- **T2 happy path: PASS.** Units `B-m3-tower-t2`, `-t3`, `DB-m3-tower-t2`: `start`/`end ... exit 0` for each, rc 0. Each
  `.jsonl` has 2 rows plus one final `{"summary":...}` line (3 lines). All of `runs/<unit>.{jsonl,log,out}` are in the S3 copy; binary.sha256,
  host.txt (with `prev:` line), units_done.txt, ladders_closed.txt (empty) uploaded.
- **T3 ladder stop: PASS.** `--budget 0`: unit t2's log has `every run at t = 2 timed out; stopping`; progress has
  `closed B-m3-tower at t=2` and `skip B-m3-tower-t3..t8 ladder-closed`; the registered-style invocation
  (`--t-min 1 --max-t-m3 8 --max-t-m4 6 --max-t 6 --seed 777 --budget 0`) stops at t = 2 too. `--budget 2` (2 threads):
  units t2, t3 normal, t4 `timed out; stopping`, `closed B-m3-tower at t=4`, t5..t8 skipped; the registered-style invocation
  also stops after t = 4. (Timeout level depends on wall time: budget 2 s sits between the t3 time of ~1.2 s and t4 of
  ~37 s, not near a boundary.) The `timed out; stopping` line goes to stderr, i.e. into `runs/<unit>.log`.
- **T4 equivalence: PASS.** Registered-style single invocation versus concatenated units, rows only (the per-file summary line
  is excluded), every field except `ms`, `wall_s`, `closures[].ms` (`cmp` of the sorted-key JSON): B-m3-tower t=2..4 (6 rows)
  identical; DB-m3-tower t=2..3 (F4, 4 rows) identical; extras: B-m3-tower-p0-isogeny t=2..3 (4 rows) and
  B-m3-null t=2..3 (`--null-chain`, 4 rows) identical. No differing field. Not compared: the `summary` line
  (`wall_seconds`, `rows`, `seed`, `budget_seconds`), which is per invocation.
- **T5 resume: PASS.** Job A (4 units `B-m3-tower-t2..4`, `DB-m3-tower-t2`) killed with SIGKILL of the whole process group
  (runner, syncer, pilot) once S3's progress.txt had `start B-m3-tower-t4`; S3 holds t2, t3 files and `start t4` without `end`.
  Job D (PREV = A): `skip B-m3-tower-t2 done-in 0`, `skip ...-t3 done-in 0`, t4 re-run (`end exit 0`), `DB-m3-tower-t2` run;
  rc 0. Job B (PREV = A) killed again during t4. Job C (PREV = A, B): `lost B-m3-tower-t4`, `lost_units.txt` = that unit,
  rc 75, no unit started. (The first T5 attempt had a test-harness bug and one false "pilot still running", caused by a defunct
  zombie of the killed pilot; the driver was fixed and T5 re-run; the shown output is the re-run.)
- **T6 non-zero exit: PASS.** `BIN` wrapper exits 3 for `runs/B-m3-tower-t3.jsonl`: `end B-m3-tower-t3 exit 3`, runner rc 20,
  no `end ... exit 0` for t3 (grep count 0), t4 never started; t2 files and the failed unit's `.log/.out` are in S3.

## NOT tested
SIGTERM handling (`trap ... TERM`; an attempt was abandoned); the real `aws s3` commands (only the shim, so `aws s3 ls`
output parsing uses the last whitespace field of each line and `aws s3 sync` flags are untested); the 300 s background sync
(no test ran 300 s); production flags `--budget 1800` at large levels; any P0/F4 ladder beyond the T4 extras and no m >= 4;
a missing PREV progress.txt (runner exits 12; untested); G1 failure (exit 10; untested); the exit-11 assertion on a modified
list; the real 12 thread / 56Gi pod.

## Added by the main session (2026-11-10)

- **Two-kind P0 invocation against single-kind units: PASS.** Registered-style `--p 786433 ... --kinds kummer,isogeny --m 3
  --controls tower --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 --seed 777` (8 rows) against the four single-kind, single-level
  invocations concatenated kummer t2, t3 then isogeny t2, t3: identical in every field except `ms`, `wall_s`, `closures[].ms`
  (`cmp` of `jq -cS`), in that order.
- **Termination signal mid-unit: PASS.** Runner started with `UNIT_FILTER='^B-m3-tower-t[34]$'`, `--seed 777`, 2 threads;
  `kill -TERM` of the runner about 3 s into unit t4. Runner exit 143; no pilot left; the S3 copy has `end B-m3-tower-t3 exit 0`,
  `start B-m3-tower-t4` and no `end` line for t4; `units_done.txt` lists t3 only; `ladders_closed.txt`, `host.txt`,
  `binary.sha256` uploaded. A first attempt signalled the wrong process (a `setsid` wrapper) and showed nothing; it was repeated.
- Still not tested: everything listed under "NOT tested" above except SIGTERM.

## Revision 2 (runner sha256 773a978d5816730026e91dd596c8541fc3e0faca84585ad89dda3f577775cbe3), tests T1-T15

Changes A-I: mkdir lock instead of flock; `ISOLAB_RESULTS_URI` and PREV used verbatim (no slash stripping); `< /dev/null` on every S3
command and fd 3 for the unit-list loops; upload failures exit 13; done units' `.log` and `.jsonl` (with final `{"summary"` line)
are fetched from the prev or exit 12 (no `ls`, no S3LS); test-only variables refused with exit 15 outside test mode; completeness
check exit 14; `close:<ladder>:<T>` arguments (exit 16 when invalid). Beyond A-I: the final upload turns rc 0 into 13 when it fails;
host.txt also records `testmode`, `pilot_extra`, `unit_filter`, `closed_by_rule`; a PREV progress.txt is still required (exit 12).
Verbatim outputs: `test_outputs.txt`; progress files: `progress/`. Shim switches added: `SHIM_FAIL_PROGRESS`, `SHIM_FAIL_SYNC_RUNS`,
`SHIM_READS_STDIN`, `SHIM_LOG`. Driver: `test_driver.sh T1..T15` (T7 uses `runner_bg`, no setsid).

| Test | Result | Evidence |
|:--|:--|:--|
| T1 list | PASS | 91 units / 25 ladders, 49 + 42, same order |
| T2 happy path | PASS | rc 0, 3 units `end exit 0` |
| T3 ladder stop | PASS | budget 0 closes at t=2, budget 2 closes at t=4, rest `ladder-closed`; registered-style stops at the same levels |
| T4 equivalence | PASS | B-m3-tower, DB-m3-tower, p0-isogeny, null: IDENTICAL except wall-time fields |
| T5 resume | PASS | `done-in 0` skips, t4 re-run; two kills then C: `lost B-m3-tower-t4`, rc 75 |
| T6 non-zero | PASS | rc 20, no `end t3 exit 0` |
| T7 SIGTERM | PASS | runner rc 143, 0 live pilots, S3 progress `start t4` without `end`, units_done.txt = `B-m3-tower-t3` only |
| T8 missing prev file | PASS | missing `.log`, missing `.jsonl`, truncated `.jsonl` (no summary): rc 12 each, 0 unit starts |
| T9 upload failure | PASS | progress.txt cp fails: rc 13, `runs/` empty (no pilot started); runs sync fails: rc 13, 0 `end ... exit 0` lines in S3 |
| T10 guard | PASS | each of PILOT_EXTRA, UNIT_FILTER, BIN, G1BIN, STUDY_DIR without S3CP/S3SYNC: rc 15; fake `aws` marker file absent |
| T11 stdin hazard | PASS | shim reads stdin; all 3 units ran, rc 0 |
| T12 image tool set | PASS | PATH = symlinks to the listed tools only (31 present; `gawk` is not installed on this host, so the link was not made and `awk` is mawk here; no flock, no python3): rc 0, 3 units, no lock dir left |
| T13 trailing slash | PASS | job 1 at `s3://b/jobS/`: raw URIs `s3://b/jobS//runs`, `//progress.txt`; job 2 PREV `s3://b/jobS/` reads `s3://b/jobS//progress.txt`, `//runs/<unit>.log/.jsonl`; `skip ... done-in 0`, t4 run. The shim file system collapses `//`, so the layout listing shows `b/jobS/...`; the raw keys are in the shim log lines shown |
| T14 completeness | PASS | T2: exit 0 with the filtered list exhausted. Forced exit 14: a hand-made prev with a done t4 whose log has `timed out; stopping` but t2, t3 never done: t2, t3 skipped `ladder-closed`, rc 14 |
| T15 close:<ladder>:<T> | PASS | killed twice in t4: without the rule rc 75; with `close:B-m3-tower:4` rc 0, `skip B-m3-tower-t4 closed-by-rule`, DB-m3-tower-t2 run, `ladders_closed.txt` has `B-m3-tower t=4 by-rule`; `close:NOPE:3`, `:99`, `:1`, `:x` rc 16 |

Not tested on revision 2: real `aws s3` commands and their error output, the 300 s background sync, G1 failure (exit 10),
exit 11 on a modified list, m >= 4 ladders, the 12-thread pod, a lock left by a killed syncer in a live run (the `rmdir` in the exit
handler is covered only through T7's normal path).
