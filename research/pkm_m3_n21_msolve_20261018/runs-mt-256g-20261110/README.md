# Run 2026-11-11 (A1, 256 GB): msolve -t 32 on m3_t7 target 0, lost to a RunPod balance stop

Pod `h2732jfsnrfu97`, RunPod `cpu3m-32-256` (32 vCPU, 256 GB), $1.76/h, launched from branch
`claude/pkm-n21-256g-20261106` with `MSOLVE_SYSTEMS=pkm_kummer_p2013265921_m3_t7_x221026520`,
`MSOLVE_RUNDIR=runs-mt-256g-20261110`, `CAP=217,600,000,000` B, `WALL=43,200` s.
`mirror_pod.sh` copied these files off the pod every 600 s from a process outside the Claude
session. `mirror.log` has its log, and `snapshots/` has the target log at each copy.

## What happened (*recorded*)

| time (UTC) | event |
|---|---|
| 01:23:33 | boot |
| 01:33:11 | control `m3_t5_x1289903598` (N = 15) agrees: exit 0, 360.6 s, 2.0 GiB |
| 01:33:41 | target `m3_t7_x221026520` (N = 21) starts |
| 02:55–03:05 | first degree-8 round selected: 398,409 of 398,687 pairs (first in the 03:05 snapshot) |
| 03:55–04:05 | that round's matrix built: 1,912,891 x 1,719,019, density 0.87% (first in the 04:05 snapshot) |
| 04:25:41 | last contact: pod RUNNING, the round still open |
| about 04:30 | the RunPod account balance reached -$1.09 and every running pod in the account was deleted, this one included |

- The degree-4 to degree-7 rounds equal the October runs round for round (same `sel`, `pairs`,
  matrix sizes, `new` and `zero`). In particular, both late degree-7 rounds that only the 16-thread
  run had reached (384,152 x 528,099 with 4,040 new; 390,746 x 529,485 with 5,481 new) are reproduced.
  Degree 7 again runs out of pairs without reaching the unit ideal.
- The degree-8 round did not report `new`. No `.mem` or `.out` file exists for the target, because
  the process never exited. The last copy of the log is the same file as the snapshot at 04:25:40Z.

## Reading under the frozen rule (A1.4)

**Inconclusive.** The run produced no productive degree-8 round and did not finish at D = 7, and it
covers target 0 only. The stop was neither a wall stop nor a memory stop: the host disappeared.
That is not evidence either way.

New and recorded: the first degree-8 round at N = 21 builds a 1.9M x 1.7M matrix, and that matrix
fits under the 217 GB cap; the October 128 GB run died before reaching it. The round was still open
at least 20 minutes after the matrix was built. Peak memory is unknown, because `.mem` is written
only at exit.

## What would decide it

The same launch, after the account is funded, with `mirror_pod.sh` running from the start. The
spend for this attempt was about $5 (01:23–04:30).
