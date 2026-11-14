# DRAFT Amendment R2-G (proposed number 15): stage B as resumable units

**Status: draft (fifth version, after three independent reviews and a re-check of the runner: "freeze with edits", the edits are applied here), NOT applied, NOT frozen. The freeze waits for the owner's answer named in rule 5.** Written 2026-11-10 by the main
session for owner decision 8 of `research/open_leads_execution_20261108/DECISIONS.md` ("split stage B into short jobs under a
dated amendment, reviewed before it is frozen; readings and rules stay as registered"). Target when frozen:
`research/prime_fall_degree_20261027/PREREGISTRATION.md`, appended as "Amendment 15". It changes execution and procedure: how
stage B's registered systems are packed into jobs, and what is done when a job is interrupted. No reading changes. Every number
is `[pcc:unverified]`; each is marked measured, cited or derived.

## 1. Why (measured, derived)

- `pfd-b` runs `g1 hky-B d-B g3` as one job with a time limit of 172,800 s. Both attempts were killed by Spot interruptions,
  16 and 31 minutes in (`STAGE_B_LOST.md`). No stage B row is admitted.
- Since 2026-11-09 20:00Z the longest uninterrupted run of this study on the pool is 3.2 hours (`SPOT_LOSSES.md`).
- `run.sh hky-B` and `run.sh d-B` start 17 pilot invocations. One invocation walks a whole ladder of `t`. The closure budget is
  per closure (`--budget 1800`; closures `c = 3..7`), so one closure system can take up to 5 × 1800 s and the longest ladder
  (`B-m3-tower`, 7 levels, two draws) up to 35 hours by its budgets alone (derived). An F4 system has one 1800 s budget.
- A unit as defined below is one level of one ladder. By budgets alone a closure unit can take up to 5 hours and an F4 unit up
  to 1 hour (derived). So a unit is much shorter than the registered job, but a closure unit can still be longer than the
  longest uninterrupted run seen. Rule 5 says what happens then.

## 2. What is the same system (measured, source of `examples/pkm_tower_pilot.rs` at f13a690df0; re-read by the reviewer)

- A cell's random stream is seeded by `seed ^ kind<<40 ^ m<<32 ^ t<<16 ^ g` (lines 2203-2209). The tower, the curve and both
  random targets of a cell depend on `(kind, m, t)` and the seed only. They do not depend on which other cells the same process
  runs, on the control, or on the order of `--kinds`. No state is carried from one level to the next.
- `--budget` is applied per system for F4 and per closure for the closure engine; each measurement gets its own duration.
- For `m >= 3` the pilot starts at `t = max(t_min, 2)` (line 2657). The level limit is `--max-t-m3` for `m = 3` and `--max-t-m4`
  for `m >= 4` (lines 2638-2642).
- **Ladder stop.** Inside one invocation the loop over `t` stops for a `(kind, m, control)` ladder when every run at some `t`
  has `timed_out true` ("every run at t = {t} timed out; stopping", line 2669, the only place that string occurs in the source).
  For the closure engine `timed_out` is set whenever the closure is censored (line 1383); for F4 it is `timed_out || oversize`
  (lines 1506, 1636). So the registered job does **not** run the levels above a fully censored level.
- So a pilot invocation restricted to one `t` computes the same systems, with the same budgets, as the registered invocation
  computes at that `t`, provided the ladder stop is applied between levels by the caller.

## 3. The change (plan)

Stage B's registered invocations are run as **units**. A unit is one pilot invocation for one `(ladder, t)`:

- its flags are the flags of the `run.sh` line of its ladder, with the level pinned by
  `--t-min T --max-t-m3 T --max-t-m4 T --max-t T`, and, for the `P0` ladders, with `--kinds` naming one kind (`run.sh` passes
  `kummer,isogeny`; a ladder is one kind);
- both draws of the level run in the unit (`--random 2`, no `--targets`), as in the registered invocation;
- `--out runs/<ladder>-t<T>.jsonl`, stderr to `runs/<ladder>-t<T>.log`, stdout to `runs/<ladder>-t<T>.out`, and the pilot's
  command line to `runs/<ladder>-t<T>.argv`.

Ladders and levels (derived from `run.sh` and `tmax0`; `t` starts at 2 because `m >= 3`):

| stage | ladder | prime | kind | control | levels `t` |
|:--|:--|:--|:--|:--|:--|
| hky-B | `B-m3-tower` | P1 | kummer | tower | 2..8 |
| hky-B | `B-m4-tower`, `B-m5-tower`, `B-m6-tower` | P1 | kummer | tower | 2..6 |
| hky-B | `B-m3-tower-p0-kummer`, `-isogeny` | P0 | kummer / isogeny | tower | 2..5 |
| hky-B | `B-m4-tower-p0-kummer`, `-isogeny` | P0 | kummer / isogeny | tower | 2..4 |
| hky-B | `B-m5-tower-p0-kummer`, `-isogeny` | P0 | kummer / isogeny | tower | 2..3 |
| hky-B | `B-m6-tower-p0-kummer`, `-isogeny` | P0 | kummer / isogeny | tower | 2 |
| hky-B | `B-m3-null` | P1 | kummer | null (`--null-chain`) | 2..8 |
| d-B | `DB-m3-tower` | P1 | kummer | tower | 2..8 |
| d-B | `DB-m4-tower`, `DB-m5-tower`, `DB-m6-tower` | P1 | kummer | tower | 2..6 |
| d-B | `DB-m{3,4,5,6}-tower-p0-kummer`, `-isogeny` | P0 | kummer / isogeny | tower | as the hky-B `P0` ladders |

That is 25 ladders and 91 units (derived: hky-B 13 ladders and 49 units; d-B 12 ladders and 42 units). The runner generates its
unit list from this table and refuses to start if the counts differ.

**Rules, all fixed before any stage B row exists:**

1. **Order.** Within a ladder, levels run in increasing `t`. Ladders run in `run.sh`'s order (hky-B first, then d-B).
2. **Ladder stop.** After a unit ends, if its log contains the pilot's own line "every run at t = T timed out; stopping", the
   ladder is closed and no higher level of that ladder runs, in this job or any later one. This is the pilot's decision, read
   from its output; the runner makes no judgment of its own.
3. **Unit done.** A unit is done when the pilot exited 0, its files were uploaded, and the job's `progress.txt` in S3 has its
   line `end <unit> exit 0`. A done unit is never run again. Its row file is the unit's data. A unit can be done in one job
   only; a job that finds a unit done in two earlier jobs runs nothing (exit 12), stage B stops and goes to the owner.
4. **Resume.** Unit jobs are numbered 1, 2, 3, ... (job name `pfd-bu-<n>-<stamp>`) and run one at a time: a job is submitted
   only after the one before it is terminal, with a status receipt. Each unit-job request is submitted once (the launch card
   refuses a job name that already has a receipt). A job that must be repeated gets the next number, and the earlier one a
   skip. If two pods ever ran under one job name, stage B stops and goes to the owner. Job `N` is given the result prefix of every earlier job
   (each exactly as that job's `ISOLAB_RESULTS_URI`) or a `skip:<n>:<reason>` for it; the reason is `costguard` (rule 9) or
   `neverran` (the job stopped before it wrote `host.txt`, so its prefix has none and it ran no unit). Each skip has a status
   receipt of the skipped job. The runner checks this itself: the numbers of the jobs given and of the skips, including skips
   carried forward from earlier jobs' `host.txt`, must be exactly 1 to `N - 1`, each once. It cannot check a skip's reason;
   the skip is recorded in every later `host.txt`.
   - The job reads the earlier jobs' `progress.txt`, `host.txt` and `binary.sha256`, skips their done units, and for each done
     unit fetches its log, its row file and its recorded pilot command line again.
   - It runs nothing and exits 12 if any of these cannot be read or lacks a line the runner writes; if a row file does not
     end with the pilot's summary line; if an earlier job's runner hash, binary hashes, test-mode flag or extra pilot flags
     differ from its own; if a done unit's recorded pilot command line differs from the one it would use; if an earlier job
     names an earlier job that this one was not given; if an earlier job's result prefix does not contain the job name its
     `host.txt` records; if the job numbers do not add up (one missing, one given twice, one both given and skipped, or a
     skip carried with another reason); or if a tool-failure closure recorded by an earlier job is not one it computes
     itself.
   - After an exit 12 no unit has run; the main session finds the cause in the listing and submits a corrected job. If a done
     unit's files are really gone from S3, stage B stops and goes to the owner.
   - The runner is not changed once the first unit job has run. If it had to be, the earlier jobs would fail the hash check;
     that needs a new amendment that says how their done units are taken over, and stage B waits for it.
5. **Interruption.** A Spot interruption loses only the unit in flight; that unit has no `end` line and is run again by the
   next job. Owner decision 3 applies per unit: a unit that was in flight in two earlier jobs without finishing is reported lost
   and goes to the owner. A job that finds such a unit writes `lost_units.txt`, runs no unit and exits 75; stage B waits for the
   owner. "In flight in a job" is read from that job's `progress.txt`: a `start <unit>` line with no `end <unit>` line and no
   `abort <unit>` line (rule 6 writes `abort` when an upload failed). The count is mechanical: it does not depend on why the
   job ended. The cause of each job's end is still read from the EKS audit log and recorded.
   - The two lost `pfd-b` attempts count toward no unit: owner decision 8 replaced the unsplit job, and the count starts with
     the first unit job. That is the main session's reading of decision 8. It is put to the owner before the first unit job
     and the owner's answer is recorded here at the freeze; it is fixed after that. Their partial files stay unread and
     unscored.
   - The owner has two answers to a lost unit, and only these. **Accept the loss**: the next job is given `close:<ladder>:<T>`;
     no level of that ladder from `T` on runs, in that job or any later one, and the ladder is reported as "lost at t = T"
     with no row there. **Allow further attempts**: the next job is given `allow:<unit>:<K>` with `K` at least 3; the unit
     then counts as lost only when it was in flight in `K` or more earlier jobs. The runner accepts either argument only for a
     unit that is lost by the count above, `allow:` only with `K` above that count, and not both for one unit. Both are
     carried forward by the runner from each job's `host.txt`, so they hold in every later job without being repeated; where
     two jobs carry different values, the lower `T` and the higher `K` hold. Other ladders continue in either case.
6. **Non-zero exit and upload failures.** These are handled mechanically; no diagnosis changes what happens next.
   - A unit whose pilot exits non-zero is not done; its files and its line `end <unit> exit N` are uploaded and the job stops
     (exit 20). A pilot that cannot be started (126, 127), or that dies from a signal while the runner keeps running, counts the same way. The next job runs that unit once more. If the pilot exits non-zero for that unit in a second job, the ladder
     is closed at that level by the runner itself, in every later job, and reported as "tool failure at t = T".
   - A failed upload stops the job (exit 13). The runner writes `abort <unit> <reason>`; if that line reaches S3 the unit
     does not count as in flight, and it is run again by the next job unless its `end` line did reach S3. With S3 unreachable
     for the rest of the job no `abort` line gets out and the unit counts as in flight. A pilot that exited non-zero and whose
     files then failed to upload has no `end` line; that failure is not counted. The same `abort` is written when the job is
     terminated after a unit's `start` line and before its pilot was started.
   - A pilot killed for memory alone exits non-zero and falls under the first bullet. If the whole container is killed, there
     is no `end` line and rule 5 applies. Which of the two happens depends on the node; the rules above are applied to what
     `progress.txt` holds.
   - In the registered job a failing invocation ends that ladder and, for a `P0` invocation, also the isogeny ladder that
     follows the kummer one in the same process; in unit form each ladder is its own sequence, so the sibling ladder runs.
     That difference is fixed here, before data.
7. **Gates.** G1 runs at the start of every job (6 cases, seconds). The `g3` stage that `pfd-b` runs inside the job (and that
   Amendment 11 refers to) is replaced by one G3-only job after the last unit. Its command is the G3-only command of Amendment 11
   (R2-D section 4.4) with these substitutions only: work directory `/work/g3b`; inputs are the row files of all done stage B
   units, fetched from the jobs that hold them; `gate3 /work/g3b pkm_tower_pilot --max-n 20 --jobs 2 --budget 900`; 12 vCPU,
   56Gi, `seconds` 57,600. Pass is exit 0 and `, 0 disagreements`. The jobs that count are the last unit job (the one that exits 0) and the jobs named in the `prev:` line of its
   `host.txt`; a skipped job never counts. The file list is filled mechanically from the `end <unit> exit 0` lines of those
   jobs' `progress.txt` in S3. `progress.txt` decides: an entry of a job's `units_done.txt` without such a line in that job's
   `progress.txt` is ignored. The filled command's SHA-256 is recorded before it is submitted. Every stage B cell has `m <= 6`, so
   Amendment 11's scope covers all of them, and its stop rule on a disagreement applies.
8. **Repeats of stage C cells.** The `P1` tower units at `t = 2, 3` for `m = 3..6` are the cells `C-t2-m3` ... `C-t3-m6` of
   stage C (same kind, `m`, `t`, control, prime and seed), and so are the `P0` units at `t = 2, 3` that stage C ran.
   - Identity: for each such unit, `x_r`, `curve` and `tower` of every row must equal stage C's admitted row for the same cell
     and draw. These do not depend on any budget. A difference means the unit did not run the registered system; it stops
     stage B and is reported. The main session makes this comparison when a job's units are collected, before any further
     stage B job is submitted.
   - Outcome: the frozen scorer already compares B and C repeats (`falls`, first and last fall degree) and fails on any
     difference. This amendment adds no exception to that and relaxes nothing: a disagreement is a reproducibility failure and
     is handled as the preregistration and Amendment 2 say.
   - Stage C has no `D` rows, so the d-B units have no repeat.
9. **Cost guard.** The unit jobs have `seconds` above 90,000, so Amendment 12's price guard applies to each: the reading at
   start and at every check-in. A job stopped by the cost guard is never passed to a later job as an earlier job, and nothing
   from it is admitted. The runner cannot tell such a job from a Spot loss; it is excluded with `skip:<n>:costguard`
   (rule 4), with the price-guard reading that stopped it as the receipt.
10. **Resources and limits.** 12 vCPU, 56Gi, no pin, job time limit 172,800 s (as `pfd-b`). US$40 per job and US$400 in total
    are unchanged. At most 12 unit jobs are submitted without going back to the owner. Before each submission the main session
    adds up the node-hours used so far by this study, at the Spot prices read, and records the sum.

## 4. What does not change

- Every system (prime, kind, `m`, control, `t`, draw, seed), every engine flag and limit, every budget, the image and binaries.
- The hypotheses, the §6 rules, the scorer, Amendment 10's high-lambda rule (it depends on `(m, t, p)` only) and Amendment 11's
  G3 scope.
- Thread count: `RAYON_NUM_THREADS=12` from the pod, as for `pfd-b`.
- Rows: each unit's rows are the rows the registered invocation would have appended at that level. The units that enter
  scoring are the done units of the jobs that count under rule 7, and no others. For scoring, a ladder's
  unit files are concatenated in increasing `t` into the registered file name (`B-m3-tower.jsonl`, and for the `P0` ladders
  the kummer units then the isogeny units into `B-m3-tower-p0.jsonl`, the order the registered invocation writes them). Each
  unit file ends with the pilot's per-invocation `{"summary": ...}` line; those lines are dropped in the concatenation and none
  is made up for the ladder.

## 5. What is different, stated plainly

- Each unit is a separate process. Wall seconds per system may differ from a long-lived process (cache, allocator). Stage B's
  budgets are wall budgets, so a system near a budget could be censored in one packing and not in the other. That is a host
  effect of the same kind as any re-run on another node; it is not controlled, and rule 8 gives it no special treatment.
- Rows of one stage can come from several jobs and nodes. Each job's `host.txt` and the units it finished are recorded.
- A unit's two draws share a process, as registered; the units of a ladder do not.
- A `P0` isogeny ladder runs even if its kummer sibling ended by a tool failure (rule 6).
- A unit whose pilot fails in two jobs closes its ladder at that level (rule 6); a unit lost twice goes to the owner, who
  closes the ladder or allows further attempts (rule 5). The registered job has no such cases because it has no resume.

## 6. Disclosure

- No stage B row exists or has been seen. The lost attempts' partial files were not read for results; `STAGE_B_LOST.md` records
  only that `g1` passed and which invocation had started.
- When writing this I had seen all of stage A and stage C and the t = 12, 13 capped rows. None of them enters a rule here.
- The first version of this draft had a clause that would have compared budget-censored repeats on identity fields only. The
  reviewer pointed out that it relaxed the registered repeat check. It is removed (rule 8).
- The second version let the main session diagnose a failing unit before deciding whether to run it again, which is a choice
  after data. Rule 6 is now mechanical.

## 7. Runner, tests and review

- The runner is one frozen bash script, `stageb_units_runner.sh` (revision 4), SHA-256
  `1c2229386976e8d6f22e5f243b269a5d23d588d0e105d6cb530c9a5488157a79` (28,966 bytes). The start command built from it by
  `make_request_bu.sh` has SHA-256 `572ce3fb23e57416732a64a7cc25523c5ff84bd8496a7dd59455e07b1b6bcade` (4 pieces). It
  was written by Sonnet agents to this draft and to written specifications of the main session, and read by the main session.
- **How a job carries it.** The lab's request validator refuses a command argument longer than 8192 characters or containing
  a newline, so the runner cannot be one argument. `make_request_bu.sh` writes the runner into the command in pieces, with
  each newline written as the letter Q (the runner contains no Q; the script checks). The command's first words are
  `/bin/bash`, `-c`, a one-line start command, and `stageb`. The start command joins the pieces, restores the newlines, checks
  the file's SHA-256 against the frozen value (exit 17 if it differs; exit 18 if it cannot write the file) and runs the file
  with the remaining arguments. So a job runs the frozen runner byte for byte or runs nothing, provided the start command is
  the frozen one: its SHA-256 is recorded above, next to the runner's, and the launch card of every unit job
  checks the request's first four command words against it before submitting.
- The image's shell and awk, measured from its base layer (Amazon Linux 2023): bash 5.2.15 and GNU Awk 5.1.0.
- It is tested on a local copy of the frozen binaries (the image layer; SHA-256 equal to `image/binaries.sha256`) with
  `--seed 777`, so no test row is a stage row. The tests and their verbatim outcomes are in `devcheck_r2g/`. The whole suite
  is run three ways: as a file, through the command a job uses, and under the image's own bash and gawk.
- First review (red-team agent, Opus, read-only, 2026-11-10), of the first draft and runner. It confirmed section 2 against
  the source, the unit list and every ladder's flags against `run.sh`, and that the stop string is printed exactly when the
  registered loop over `t` ends early. It found, in the first runner: errors during resume were swallowed, so a missing log
  could reopen a closed ladder; an `end` line could reach S3 before the row files; a failed upload of a `start` line could
  lose the in-flight count; S3 commands inside the unit loop did not have their input closed; the test-only variables were
  neither refused nor recorded; the result prefix differed from the earlier jobs' layout by a slash; and exit 0 was not
  checked against the list. It also found the items now in rules 5, 6, 8, 9 and 10 and in section 1. Separately the main
  session found that the image has no `flock`, which the first runner used.
- Second review (2026-11-10), of runner revision 2 and the second draft: do not freeze. It confirmed the fixes above and the
  argv of all 91 units against `run.sh`, and reproduced new defects: the command form of the draft (`/bin/bash -c` with the
  whole runner) is refused by the validator; a ladder closed by rule reopened in a later job that was not given the `close:`
  argument again; `units_done.txt` could omit a unit that `progress.txt` shows as done; the list of earlier jobs was not
  checked (a job left out, or given twice); earlier jobs' provenance was not checked; a failed upload counted as "in flight"
  for a pilot that never ran; the owner had no way to allow another attempt; rule 6 left a choice after data.
- Third review (2026-11-10), of runner revision 3 and the third draft: do not freeze. It found the second review's items
  fixed (ten fully, the rest with a remaining item) and reproduced: leaving out the most recent earlier job was not detected
  by anything frozen (only by the request script); `allow:` was accepted for any unit and carried for good; a termination
  during a `start` upload counted as in flight. It asked for the start command's hash to be recorded, for rule 7 to say which
  jobs count, and for the reading about the lost `pfd-b` attempts to be put to the owner. Revision 4 and this fourth draft
  address each of these (rules 3 to 7 and 9, and the bullets above).
- Re-check of revision 4 (same reviewer, 2026-11-10, `fcb1f0a674`): freeze with edits, no runner change. It replayed its
  reproductions and its attempts on the job-number check and on the `allow:` evidence against revision 4 under the image's
  bash and gawk; all show the corrected outcome; the unit list and all 91 pilot command lines are unchanged; the three
  committed suite outputs show what they claim (it rebuilt the request independently and got the same start-command and
  piece hashes). Its one required text edit (a request is submitted once; rule 4) and its wording notes are applied in this
  version.
- Runner exits: 0 list accounted for; 2 no work directory, no result prefix, the runner not run as a file, or a job name not
  of the form `pfd-bu-<N>-<stamp>`; 10 G1 failed; 11 unit list count wrong; 12 the earlier jobs' state cannot be read or does
  not match (rules 3 and 4); 13 an upload failed or a lock wait ran out; 14 list not accounted for at the end; 15 a test-only
  variable set in production; 16 a bad argument; 20 a pilot exited non-zero; 75 a lost unit; 143 terminated by a signal.
  Exits 17 and 18 are the start command's. A container killed outright shows 137.
- Known limits, stated before any unit job: the runner cannot check a skip's reason (rule 4); it cannot detect an earlier job
  that is still running, or two pods under one job name (rule 4: one job at a time, each request submitted once, by
  receipt); it trusts what it reads from S3 (a result prefix forged consistently, which needs write access to the bucket, is
  accepted); if the exit handler cannot take the upload lock within 5 s it uploads anyway, and an overlap with a periodic
  upload still in progress can drop a line, which can only make a unit count as in flight; test-only settings other than
  the extra pilot flags are not compared between jobs, and are refused in production anyway. The start command was tested
  as a whole under the host's shell (the whole suite) and under the image's shell (the unit list, and exit 17 on a changed
  piece; main session, 2026-11-10).
- Not tested locally: the real `aws s3` commands (a local-directory stand-in is used), ladders with `m >= 4`, the production
  budgets at large levels, the 12-thread pod, exits 17 and 18 inside a pod. The first job is therefore watched as a test of
  those: its S3 listing and `progress.txt` are checked against the rules above before any second job is submitted.
- Requests: `requests_r2i/pfd-bu-<n>.json`, one per job, written by `make_request_bu.sh`.
