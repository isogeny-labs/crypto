# Run 20261018T2208Z (Amendment A1, minimal P2): msolve -t 16 on m3_t7 target 0

Pod `dapncaphacx5w3`, RunPod `cpu3m-16-128` (16 vCPU, 128 GB cgroup), $0.88/h,
booted from `claude/ic-leads-rented-launch-ready` (PR #2225, which registers
Amendment A1). One system only (`MSOLVE_SYSTEMS=pkm_kummer_p2013265921_m3_t7_x221026520`),
`THREADS=16`, `CAP=108,800,000,000` B, `WALL=20,577` s. Records: `../../runs-mt/`,
`../../progress-mt.txt` (transcribed from the pod's file before termination;
the four lines are verbatim), pod STATUS in `../../runs-mt/pod-dapncaphacx5w3-STATUS`.
Spend about $3.20 (22:07Z–01:13Z, terminated after fetching).

## What happened (*recorded*)

| system | outcome | wall | peak RSS | rounds |
|---|---|---|---|---|
| control `m3_t5_x1289903598` (N = 15) | exit 0, `.out` identical to the committed one (one-element basis, no solution) | 245.9 s | 2.0 GiB | as committed |
| `m3_t7_x221026520` (N = 21, target 0) | **exit 139** (segmentation fault) during the first degree-8 round | 7,088 s | 99.6 GiB (VmHWM 104,410,572 kB) under the 108.8 GB cap | 27 completed, 7 of them at degree 7 |

- Control (A1.3) passed. Its wall time against the recorded single-thread
  1,299 s is 5.3× (descriptive; another host).
- The degree-4 to degree-7 rounds equal the first run's round for round (same
  `sel`, `pairs`, matrix sizes, `new`, `zero`): A1.5 prediction 4 holds and
  `-t` did not change the F4 trace.
- Degree 7 ran out of pairs: after two more degree-7 rounds than the first run
  completed (384,152 × 528,099 with 4,040 new; 390,746 × 529,485 with 5,481
  new), msolve selected **398,409 pairs at degree 8** out of 398,687. The
  process died inside that round, before the round reported `new`. The basis
  was not `{1}` at the end of degree 7.

## Reading under the frozen rule (A1.4)

- **Inconclusive.** A1.4 counts H0 only for a *productive* degree ≥ 8 round
  (`new` > 0 in the trace). The degree-8 round did not complete, and an
  exit under the memory cap is "recorded with the last completed round and is
  not evidence". The "finished, D = 7" row needs a finished run on both
  targets; this run did not finish and covers target 0 only.
- What is new and recorded: on target 0 at N = 21, F4 exhausted every pair
  of degree ≤ 7 without reaching the unit ideal, and the next step is a
  degree-8 round of about 398k pairs that does not fit in about 100 GB.
  A1.5 prediction 3 (40–120 GB for the first degree-8 round, memory stop
  possible) was consistent with this; prediction 2 (degree 7 ends within
  10 h) held (1.1 h of round time).
- Exit 139 rather than 134/137: msolve dereferenced a failed allocation
  near the `prlimit --as` cap. It is classed as a memory stop on that ground
  (peak 96 % of the cap); the exit code alone does not prove it.

## What would decide it

The same system (and target 1, `m3_t7_x704078485`, per A1.2) with the
first degree-8 round fitting in memory: a host with 256 GB or more for one
process, at 16 threads. On RunPod `cpu3m-32-256` ($1.76/h) at `CAP` ≈ 217 GB
with the degree-8 round extra time unknown; on AWS spot r7i.8xlarge
(`../../../ic_leads_rented_20261018/aws_spot/`) once credentials exist.
That is a new launch under A1 (A1.2 already names both targets); only the
host size differs from the card.

Graph: `msolve-mt-degree.svg` / `.png` (`plot.py`). PDF: `report.pdf`.
