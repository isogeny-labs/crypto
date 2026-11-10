# DRAFT Amendment R2-G (proposed number 15): stage B as resumable units

**Status: draft (second version, after an independent review), NOT applied, NOT frozen.** Written 2026-11-10 by the main
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
- `--out runs/<ladder>-t<T>.jsonl`, stderr to `runs/<ladder>-t<T>.log`.

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
3. **Unit done.** A unit is done when the pilot exited 0, its files were uploaded, and `progress.txt` has its line
   `end <unit> exit 0`. A done unit is never run again. Its row file is the unit's data, from the first job in which it was done.
4. **Resume.** A job is given the result prefixes of the earlier jobs of this stage. It reads their `progress.txt`, skips their
   done units, and for each done unit fetches its log and row file again; if any of those cannot be read, or a row file does
   not end with the pilot's summary line, the job runs nothing and exits. It then continues from the first unit that is not done.
5. **Interruption.** A Spot interruption loses only the unit in flight; that unit has no `end` line and is run again by the
   next job. Owner decision 3 applies per unit: a unit that was in flight in two earlier jobs without finishing is reported lost
   and goes to the owner. A job that finds such a unit writes `lost_units.txt`, runs no unit and exits 75; stage B waits for the
   owner. "In flight" is read from `progress.txt` (a `start` line with no `end` line). The cause of each job's end is read from
   the EKS audit log and recorded.
   - The two lost `pfd-b` attempts count toward no unit: owner decision 8 replaced the unsplit job, and the count starts with
     the first unit job. That is the main session's reading of decision 8. Their partial files stay unread and unscored.
   - If the owner accepts a lost unit, its ladder is closed at that level: the next job is given `close:<ladder>:<T>`, runs no
     level of that ladder from `T` on, and the ladder is reported as "lost at t = T" with no row there. Other ladders continue.
6. **Non-zero exit and upload failures.** A unit whose pilot exits non-zero is not done; the job stops (exit 20). A failed
   upload also stops the job (exit 13). The cause is diagnosed before any further job. If the cause is outside the pilot (S3,
   the node), the unit is run again by the next job. If the pilot itself exits non-zero again on that rerun, the ladder is closed
   at that level in the same way (`close:<ladder>:<T>`), reported as "tool failure at t = T". In the registered job a failing
   invocation ends that ladder and, for a `P0` invocation, also the isogeny ladder that follows the kummer one in the same
   process; in unit form each ladder is its own sequence, so the sibling ladder runs. That difference is fixed here, before data.
7. **Gates.** G1 runs at the start of every job (6 cases, seconds). The `g3` stage that `pfd-b` runs inside the job (and that
   Amendment 11 refers to) is replaced by one G3-only job after the last unit. Its command is the G3-only command of Amendment 11
   (R2-D section 4.4) with these substitutions only: work directory `/work/g3b`; inputs are the row files of all done stage B
   units, fetched from the jobs that hold them; `gate3 /work/g3b pkm_tower_pilot --max-n 20 --jobs 2 --budget 900`; 12 vCPU,
   56Gi, `seconds` 57,600. Pass is exit 0 and `, 0 disagreements`. The file list is filled mechanically from the jobs'
   `units_done.txt`, and the filled command's SHA-256 is recorded before it is submitted. Every stage B cell has `m <= 6`, so
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
   from it is admitted.
10. **Resources and limits.** 12 vCPU, 56Gi, no pin, job time limit 172,800 s (as `pfd-b`). US$40 per job and US$400 in total
    are unchanged. At most 12 unit jobs are submitted without going back to the owner. Before each submission the main session
    adds up the node-hours used so far by this study, at the Spot prices read, and records the sum.

## 4. What does not change

- Every system (prime, kind, `m`, control, `t`, draw, seed), every engine flag and limit, every budget, the image and binaries.
- The hypotheses, the §6 rules, the scorer, Amendment 10's high-lambda rule (it depends on `(m, t, p)` only) and Amendment 11's
  G3 scope.
- Thread count: `RAYON_NUM_THREADS=12` from the pod, as for `pfd-b`.
- Rows: each unit's rows are the rows the registered invocation would have appended at that level. For scoring, a ladder's
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
- Units lost or failed twice close their ladder at that level (rules 5 and 6); the registered job has no such case because it
  has no resume.

## 6. Disclosure

- No stage B row exists or has been seen. The lost attempts' partial files were not read for results; `STAGE_B_LOST.md` records
  only that `g1` passed and which invocation had started.
- When writing this I had seen all of stage A and stage C and the t = 12, 13 capped rows. None of them enters a rule here.
- The first version of this draft had a clause that would have compared budget-censored repeats on identity fields only. The
  reviewer pointed out that it relaxed the registered repeat check. It is removed (rule 8).

## 7. Runner, tests and review

- The runner is one frozen bash script passed as the job's command (`/bin/bash -c`), like the G3-only job of Amendment 11. It is
  in `stageb_units_runner.sh`; its SHA-256 is recorded here at the freeze. It was written by a Sonnet agent to this draft and
  read line by line by the main session.
- It is tested on a local copy of the frozen binaries (the image layer; SHA-256 equal to `image/binaries.sha256`) with
  `--seed 777`, so no test row is a stage row. The tests and their verbatim outcomes are in `devcheck_r2g/`.
- Review (red-team agent, Opus, read-only, 2026-11-10). It confirmed section 2 against the source, the unit list and every
  ladder's flags against `run.sh`, and that the stop string is printed exactly when the registered loop over `t` ends early. It found, in the
  first runner: errors during resume were swallowed, so a missing log could reopen a closed ladder; an `end` line could reach S3
  before the row files; a failed upload of a `start` line could lose the in-flight count; S3 commands inside the unit loop did
  not have their input closed; the test-only variables were neither refused nor recorded; the result prefix differed from the
  earlier jobs' layout by a slash; and exit 0 was not checked against the list. It also found the items now in rules 5, 6, 8, 9
  and 10 and in section 1. Separately the main session found that the image has no `flock`, which the first runner used.
  The runner was revised for all of these and re-tested before the freeze.
- Runner exits: 0 list accounted for; 2 no work directory; 10 G1 failed; 11 unit list count wrong; 12 an earlier job's files
  cannot be read; 13 an upload failed; 14 list not accounted for at the end; 15 a test-only variable set in production;
  16 a bad `close:` argument; 20 a pilot exited non-zero; 75 a lost unit; 143 terminated by a signal.
- Not tested locally: the real `aws s3` commands (a local-directory stand-in is used), ladders with `m >= 4`, the production
  budgets at large levels, the 12-thread pod. The first job is therefore watched as a test of those: its S3 listing and
  `progress.txt` are checked against the rules above before any second job is submitted.
- Requests: `requests_r2i/pfd-bu-<n>.json`, one per job; each names the earlier jobs' result prefixes.
