# DRAFT Amendment R2-G (proposed number 15): stage B as resumable units

**Status: draft, NOT applied, NOT frozen.** Written 2026-11-10 by the main session for owner decision 8 of
`research/open_leads_execution_20261108/DECISIONS.md` ("split stage B into short jobs under a dated amendment, reviewed before
it is frozen; readings and rules stay as registered"). Target when frozen:
`research/prime_fall_degree_20261027/PREREGISTRATION.md`, appended as "Amendment 15". Execution only: it changes how stage B's
registered systems are packed into jobs. Every number is `[pcc:unverified]`; each is marked measured, cited or derived.

## 1. Why (measured)

- `pfd-b` runs `g1 hky-B d-B g3` as one job with a time limit of 172,800 s. Both attempts were killed by Spot interruptions,
  16 and 31 minutes in (`STAGE_B_LOST.md`). No stage B row is admitted.
- Since 2026-11-09 20:00Z the longest uninterrupted run of this study on the pool is 3.2 hours (`SPOT_LOSSES.md`).
- `run.sh hky-B` and `run.sh d-B` start 17 pilot invocations. One invocation walks a whole ladder of `t`; the longest
  (`B-m3-tower`, `t = 2..8`, two draws, 1800 s per closure) can take up to 7 hours by its budgets alone (derived).

## 2. What is the same system (measured, source of `examples/pkm_tower_pilot.rs` at f13a690df0)

- A cell's random stream is seeded by `seed ^ kind<<40 ^ m<<32 ^ t<<16 ^ g` (lines 2204-2209). The tower, the curve and both
  random targets of a cell depend on `(kind, m, t)` and the seed only. They do not depend on which other cells the same process
  runs, on the control, or on the order of `--kinds`.
- `--budget` is applied per system: each `measure_*` call gets its own `Duration::from_secs(a.budget)` (lines 2272, 2287, 2307, 1338).
- For `m >= 3` the pilot starts at `t = max(t_min, 2)` (line 2658).
- **Ladder stop.** Inside one invocation the loop over `t` stops for a `(kind, m, control)` ladder when every run at some `t`
  has `timed_out true` (lines 2238, 2320-2322, 2417, 2666-2675: "every run at t = {t} timed out; stopping"). For the closure
  engine `timed_out` is set whenever the closure is censored (line 1383); for F4 it is `timed_out || oversize` (lines 1506, 1636).
  So the registered job does **not** run the levels above a fully censored level.
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

That is 25 ladders and 91 units (derived: hky-B 13 ladders and 49 units; d-B 12 ladders and 42 units). The runner's unit list
is generated from this table and its count is checked against these numbers before the freeze.

**Rules, all fixed before any stage B row exists:**

1. **Order.** Within a ladder, levels run in increasing `t`. Ladders run in `run.sh`'s order (hky-B first, then d-B).
2. **Ladder stop.** After a unit ends, if its log contains the pilot's own line "every run at t = T timed out; stopping", the
   ladder is closed and no higher level of that ladder runs, in this job or any later one. This is the pilot's decision, read
   from its output; the runner makes no judgment of its own.
3. **Unit done.** A unit is done when the pilot exited 0 and `progress.txt` has its line `end <unit> exit 0`. A done unit is
   never run again. Its row file is the unit's data, from the first job in which it was done.
4. **Resume.** A job starts by listing the done units and closed ladders of the earlier jobs of this stage (it reads their
   `progress.txt` and unit logs from S3), skips them, and continues from the first unit that is not done.
5. **Interruption.** A Spot interruption loses only the unit in flight; that unit has no `end` line and is run again by the
   next job. Owner decision 3 applies per unit: a unit that was in flight at two Spot interruptions is reported lost and goes to
   the owner; the stage does not continue past a lost unit of the same ladder.
6. **Non-zero exit.** A unit whose pilot exits non-zero is not done; the job stops and the cause is diagnosed before any
   further job (as for `pfd-c`'s gate 3).
7. **Gates.** G1 runs at the start of every job (6 cases, seconds). G3 runs once, as a G3-only job over the done units' row
   files after the last unit, with the frozen `gate3 --max-n 20 --jobs 2 --budget 900` (Amendment 11; every stage B cell has
   `m <= 6`). Amendment 11's stop rule on a disagreement applies.
8. **Overlap check with stage C.** The `P1` tower units at `t = 2, 3` for `m = 3..6` are the cells `C-t2-m3` ... `C-t3-m6` of
   stage C (same kind, `m`, `t`, control, prime and seed), and the `P0` units at `t = 2, 3` that stage C ran are the cells
   `C-t*-m*-p0`. For each such unit every row field except the wall-time fields (`ms`, `wall_s`, and the per-closure timing
   fields inside `closures`) must equal stage C's admitted row for the same cell and draw. That includes `x_r`, `curve`, `tower`,
   `falls`, `first_fall_degree`, `censored_at` and `muladds`. Exception: a row that is censored by its 1800 s budget in either
   set is compared on `x_r`, `curve` and `tower` only, because a wall budget is a host effect (section 5). A mismatch
   stops stage B and is reported; no rule here says which side would be wrong. Stage C has no `D` rows, so the d-B units have
   no such check.
9. **Resources.** 12 vCPU, 56Gi, no pin, job time limit 172,800 s (as `pfd-b`). Cost: US$40 per job and US$400 in total are
   unchanged; the number of jobs is not fixed in advance.

## 4. What does not change

- Every system (prime, kind, `m`, control, `t`, draw, seed), every engine flag and limit, every budget, the image and binaries.
- The hypotheses, the §6 rules, Amendment 10's high-lambda rule (it depends on `(m, t, p)` only) and Amendment 11's G3 scope.
- Thread count: `RAYON_NUM_THREADS=12` from the pod, as for `pfd-b`.
- Rows: each unit's rows are the rows the registered invocation would have appended at that level. For scoring, a ladder's
  unit files are concatenated in increasing `t` into the registered file name (`B-m3-tower.jsonl`, and for the `P0` ladders
  the kummer units then the isogeny units into `B-m3-tower-p0.jsonl`, the order the registered invocation writes them).

## 5. What is different, stated plainly

- Each unit is a separate process. Counters that depend on process state do not exist in the rows; wall seconds per system may
  differ from a long-lived process (cache, allocator). Stage B's budgets are wall budgets, so a system near 1800 s could be
  censored in one packing and not in the other. That is a host effect of the same kind as any re-run; it is not controlled.
- Rows of one stage can come from several jobs and nodes. Each unit's `host.txt` line and job are recorded.
- A unit's two draws share a process, as registered; the units of a ladder do not.

## 6. Disclosure

- No stage B row exists or has been seen. The lost attempts' partial files were not read for results; `STAGE_B_LOST.md` records
  only that `g1` passed and which invocation had started.
- When writing this I had seen all of stage A and stage C and the t = 12, 13 capped rows. None of them enters a rule here.

## 7. Runner and requests

- The runner is one frozen bash script passed as the job's command (`/bin/bash -c`), like the G3-only job of Amendment 11. It is
  in `stageb_units_runner.sh`; its SHA-256 is recorded here at the freeze. It is tested before the freeze on a local copy of the
  frozen `pkm_tower_pilot` binary (same SHA-256 as `image/binaries.sha256`) on small levels, including the ladder stop, the
  resume and a killed unit (`devcheck_r2g/`). Test rows are not stage data and are produced with a different `--seed`.
- Requests: `requests_r2i/pfd-bu-<n>.json`, one per job, each naming the earlier jobs' result prefixes to resume from.
