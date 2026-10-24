# PKM Kummer tower, m = 3, N = 21: the msolve instrument with a long wall

Registered 2026-10-18 before the run, under the 50-dollar budget of
[`research/dreg_closure_m4l6_20261018/PROTOCOL.md`](../dreg_closure_m4l6_20261018/PROTOCOL.md) §9.

## Question

Does msolve's F4 take a productive step at degree 8 or more on the committed
`m = 3`, `t = 7` (`N = 21`) Kummer systems at `p₁ = 2013265921`, or finish
with `D = 7`? Item 3 of the next-10 campaign
([`03-pkm-degree/README.md`](../ic_next10_20261014/03-pkm-degree/README.md))
could evaluate neither way: both runs were stopped by the 45-minute wall
inside the second degree-7 step at 2.6 GB peak (*recorded*: `progress.txt`
lines 27–49 and `runs/*_m3_t7_*.mem` there).

## Instrument (item 3's, with the wall raised)

- systems: `research/ic_next10_20261014/03-pkm-degree/systems/pkm_kummer_p2013265921_m3_t7_x221026520.ms`
  and `..._m3_t7_x704078485.ms` (the cell's two random targets, committed).
- msolve `-t 1 -v 2`, `OMP_NUM_THREADS=1`, `prlimit --as` at `CAP` = 0.85 × the
  host's cgroup memory limit, `timeout WALL` with `WALL` = half the host's
  remaining lifetime minus 20 minutes per target, both targets one after the
  other; then `m3_t8` (`N = 24`) with whatever remains, if any.
- msolve version: 0.6.5 from the distribution package where available,
  otherwise built from the `v0.6.5` tag; the version and its sha256 are
  recorded in `runs/host.txt`. A different version voids the identity with
  item 3 unless the control below agrees.
- control: `m3_t5` target 0 (`N = 15`, committed `D = 7`, finished by msolve in
  item 3) runs first with a 45-minute wall and must give the same answer.
- `run.sh` is item 3's script with `CAP`, `WALL` and the systems path as
  parameters; `.mem` files record peak VmHWM/VmPeak, wall seconds and exit.

## Decision rule (item 3's, fixed)

| outcome at N = 21 | reading |
|---|---|
| finished, `D = 7`, both targets | the plateau holds one more level (`L = 6`); H1a still needs `N ≥ 27` |
| a productive step at degree ≥ 8 on either target | H0 (growth resumes) from a lower bound |
| wall or memory stop with no degree-8 step | inconclusive; nothing new |

The `f4_fp_tower` cell of `research/pkm_lead1_m3n21_20261016/PROTOCOL.md` is
not replaced by this; it stays blocked on a 192 GB host and budget.

## Host

A RunPod CPU pod (general-purpose flavour, 8 vCPU, 32 GB) at about 0.64
dollars per hour, lifetime 10 h, launched by
`research/dreg_closure_m4l6_20261018/pod/launch.py` with `--job pkm-msolve`.

## Amendment A1 (2026-10-18 21:45Z, before any multithreaded run): msolve `-t`, 22 h walls, both m3_t8 targets

Registered by the commit that adds this section, after the first run's
reading (RESULTS.md: all three N ≥ 21 runs stopped at their walls inside
degree 7, inconclusive) and before any run below. Owner request 2026-10-18
(rented-hardware leads, `research/ic_leads_rented_20261018/pkm_msolve_mt/LAUNCH.md`).

**A1.1 Instrument.** `run_mt.sh`, which is `run.sh` with exactly these
changes: `msolve -t $THREADS` and `OMP_NUM_THREADS=$THREADS` in place of
1; records in `runs-mt/` and `progress-mt.txt`, so the first run's records
are neither touched nor skipped into; wall seconds computed with awk (the
first pod had no `bc`, so its `.mem` files have an empty `wall_s`). msolve
is 0.6.5 as before (Ubuntu 24.04 package, sha256 recorded). Multithreading
parallelises msolve's linear algebra. The F4 pair selection and degree
sequence do not depend on it, which the control checks.

**A1.2 Runs.** Two hosts, each a pair of systems run at once, `THREADS` =
vCPUs / 2, `CAP` = 0.85 × cgroup limit / 2, `WALL` = min(22 h, host
lifetime − 2.5 h):

- **P2a**: `m3_t7_x221026520`, `m3_t7_x704078485` (N = 21, the frozen
  question);
- **P2b**: `m3_t8_x1227422598`, `m3_t8_x1272776806` (N = 24; target 1 was
  never run).

**A1.3 Control (gate).** On each host, `m3_t5_x1289903598` (N = 15) first,
at the measured thread count, wall 45 minutes. Its `.out` must equal the
committed `runs/…m3_t5….out` (one-element basis, no solution), or the
measured systems are not run (`job.sh` checks). Its wall time against the
recorded single-thread 1,299 s gives the `-t` speed-up. That figure is
descriptive, since it comes from another host.

**A1.4 Decision rule.** Unchanged (above), applied per N:

| outcome | N = 21 (P2a) | N = 24 (P2b) |
|---|---|---|
| finished, `D = 7`, both targets | plateau holds one more level (`L = 6`) | plateau holds to N = 24 (`L = 9`); H1a still needs N ≥ 27 |
| a productive step at degree ≥ 8 on either target | H0 from a lower bound | H0 from a lower bound at N = 24 |
| wall or memory stop with no degree-8 step | inconclusive | inconclusive |

A productive step is a degree-8 (or higher) round with `new` > 0 in the
`-v 2` trace. A memory stop (`prlimit`, exit 134/137, or msolve's own
allocation error) is recorded with the last completed round and is not
evidence.

**A1.5 Predictions (before the runs).**
1. The control agrees at `-t 16`, with a 3 to 8× speed-up. Confidence 0.7.
2. Degree 7 ends within 10 h at 16 threads on both t7 targets. Confidence
   0.5. *Derived*: the recorded single-thread degree-7 rounds took 154,
   1,127, 2,499, 4,634 and 5,813 s, with `sel` falling from 33,319 to 7,038,
   so 2 to 4 more rounds of 5,000 to 9,000 s single-thread remain.
3. If degree 8 opens, its first round needs 40 to 120 GB (*derived*: 15 GB
   at 384k × 528k, density 0.75 %; roughly 1M × 1.5M at about 1 % for degree
   8). So a memory stop is possible even at 108 GB per process; confidence
   in reaching a decision on either target is about 0.5.
4. Both targets give the same trace (every recorded cell has).

The `f4_fp_tower` cell (`research/pkm_lead1_m3n21_20261016/PROTOCOL.md`)
stays frozen and separate. Its card is
`research/ic_leads_rented_20261018/pkm_f4_tower/LAUNCH.md`.
