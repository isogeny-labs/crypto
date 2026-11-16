# Run report: PKM Kummer tower, m = 3, N = 21 with msolve, run 20261018T0627Z

Observations only. The reading under PROTOCOL.md's decision rule is in the
study's `RESULTS.md`.

## What ran

- Study: `research/pkm_m3_n21_msolve_20261018/` (PROTOCOL.md). Command:
  `pod/boot.sh pkm-msolve` of `research/dreg_closure_m4l6_20261018/`, which
  runs this directory's `run.sh` (item 3's script) on the committed systems
  of `research/ic_next10_20261014/03-pkm-degree/systems/`.
- Commit on the host: `6667c066` (`runs/host.txt`). msolve 0.6.5 from the
  Ubuntu package, binary sha256 `c2722288…` (`runs/host.txt`); item 3 built
  its own 0.6.5, so the control below is the identity check.
- Host: RunPod pod `zdec0w9xwevehz`, 8 vCPU, cgroup limit 32,000,000,000
  bytes, `prlimit --as` CAP = 27,200,000,000 bytes (0.85 × limit),
  `OMP_NUM_THREADS=1`, `msolve -t 1 -v 2`, `nice`. MemTotal 792 GB on the
  machine (192 CPUs); no isolation level is claimed.
- Walls (`progress.txt`): control 2,700 s; `m3_t7` target 0 15,458 s (half
  the remaining lifetime minus 20 min); `m3_t7` target 1 7,728 s; `m3_t8`
  target 0 8,328 s (what remained).

## Runs requested and completed

| system | N | role | wall allowed | outcome | rounds | max degree | degree-7 rounds | peak VmHWM | record |
|---|--:|---|--:|---|--:|--:|--:|--:|---|
| `m3_t5_x1289903598` | 15 | control (committed `D = 7`) | 2,700 s | **finished**, exit 0, 1,299 s; output `[-1]:` ("Grobner basis has a single element / No solution") | 25 | 7 | 2 | 2,354,460 kB | `runs/…m3_t5….{log,out,mem}` |
| `m3_t7_x221026520` | 21 | target 0 | 15,458 s | **wall stop** (exit 124) inside the 6th degree-7 round | 25 | 7 | 6 (5 completed) | 15,333,820 kB | `runs/…m3_t7_x221026520.{log,mem}` |
| `m3_t7_x704078485` | 21 | target 1 | 7,728 s | **wall stop** (exit 124) inside the 4th degree-7 round | 23 | 7 | 4 (3 completed) | 8,875,168 kB | `runs/…m3_t7_x704078485.{log,mem}` |
| `m3_t8_x1227422598` | 24 | extra, if time remained | 8,328 s | **wall stop** (exit 124) inside the 5th degree-7 round | 24 | 7 | 5 (4 completed) | 13,028,264 kB | `runs/…m3_t8….{log,mem}` |

"Rounds" and "max degree" are read from msolve's `-v 2` round table in each
`.log` (first column: degree of the pairs selected in the round). Peak VmHWM
is sampled from `/proc` once a second by `run.sh` (measured; a 1 s sampler
can miss a short spike). No `.out` was written by the three stopped runs.

## Recorded degree-7 rounds at N = 21, target 0 (the longest run)

| round | pairs selected | matrix | density | new | zero | round s |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 20,631 | 98,282 × 101,435 | 1.06 % | 2,354 | 18,321 | 154.36 |
| 2 | 29,876 | 170,445 × 194,842 | 1.22 % | 4,269 | 25,607 | 1,127.36 |
| 3 | 33,319 | 203,174 × 228,589 | 1.45 % | 5,421 | 28,044 | 2,498.79 |
| 4 | 28,015 | 255,388 × 307,477 | 1.45 % | 5,043 | 23,096 | 4,633.73 |
| 5 | 20,804 | 323,437 × 431,738 | 1.06 % | 5,502 | 15,486 | 5,812.61 |
| 6 | 7,038 | 384,152 × 528,099 | 0.75 % | (stopped) | | |

(One degree-6 round of 16 pairs sits between rounds 1 and 2.) Every
completed degree-7 round produced thousands of new basis elements; none
produced zero, so no run reached the end of degree 7 and none selected a
degree-8 pair. The N = 21 target 1 and N = 24 traces are the same shape
(`.log` files).

## Control against item 3

The N = 15 control finished with maximum round degree 7 and a one-element
Gröbner basis, as item 3's run of the same system did (committed `D = 7`,
03-pkm-degree README). Wall 1,299 s single-threaded here against item 3's
run on a 19 %-of-a-core share; peak 2.35 GB against item 3's 2.6 GB.

## Graph

`msolve-degree.svg` / `.png` (source `plot.py`, which reads the `.log` round
tables and `.mem` files): left, the degree of each round against cumulative
round time per system, with the stop marked; right, peak VmHWM and wall per
run against the 27.2 GB cap.

## Missing

- No run at N = 21 or 24 finished; no `D` is recorded for them.
- `m3_t8` target 1 was not run (no time left).
- The `f4_fp_tower` cell of `research/pkm_lead1_m3n21_20261016/` was not
  run (PROTOCOL.md: stays blocked on a 192 GB host).

Raw records: `runs/`, `progress.txt`.
