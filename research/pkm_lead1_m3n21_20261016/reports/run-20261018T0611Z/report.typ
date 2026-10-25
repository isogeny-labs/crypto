#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#show table: set text(size: 8.5pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PKM lead 1, $m = 3$, $N = 21$: run report]\
  Runs `M3-N21-t0`, `M3-N21-t1`, 2026-10-18 06:11–10:29 UTC — observations only, no verdict stated here
]

*Sources.* Card `backlog/XP-20261015-pkm-m3-n21-m4-n20`, cell `m3-n21`. Protocol
`research/pkm_lead1_m3n21_20261016/PROTOCOL.md` (frozen before any $N = 21$ run). Engine
`f4_fp_tower` through `examples/pkm_tower_pilot.rs` (`--engine tower`), release build at commit
`b7cafe044` (main), binary sha256 `d25eadb1…c265d`, rustc 1.99.1. Commands `run.sh host`,
`identity`, `main 0`, `main 1`, `verify`. Records: `runs/host-ip-172-31-35-131….txt`,
`runs/identity-N15-t0-ip-….{jsonl,log}`, `runs/M3-N21-t{0,1}.{jsonl,log}`, `progress.txt`,
`verify.txt`. Every number below is *recorded* unless labelled *derived*.

= Host and caps

#table(columns: (auto, 1fr), align: left,
  [host], [EC2 spot instance `ip-172-31-35-131.us-east-2.compute.internal`, 32 vCPU, MemTotal 129,800,716 KiB (124 GB), no swap; all 32 threads used (`RAYON_NUM_THREADS` unset)],
  [caps (PROTOCOL.md §2, SHARE = 1)], [$A$ = 119,416,658 KiB (`ulimit -v`), $C$ = 20,300,000,000 entries (`--max-dense`), budget 345,600 s per target],
  [deviation, disclosed before stage 2], [§2 requires $>= 192$ GB; this host has 124 GB. The rules for $A$ and $C$ were applied unchanged. Recorded in the host file.],
  [isolation], [none claimed: a shape-search sizing probe (one thread, 64 GB cap) ran on the same host throughout],
)

= Stages

#table(columns: (auto, auto, auto, 1fr), align: left,
  [*stage*], [*start*], [*wall*], [*recorded outcome*],
  [identity ($N = 15$, target 0)], [05:54:06], [52 s], [exit 0. `solving_degree_max` 7, `max_cols` 31,397, `steps` 24, `inconsistent` true, `muladds` 3,185,681,037,100, `max_rows` 52,129, `max_residual_rows` 30,474: every compared field equals round 6's committed row. `pkm_tower_check verify`: 1 row, 1 agrees.],
  [main, target 0], [06:11:43], [6,445 s], [exit 0. `last_productive_degree` 7, `degree_reached` 8, `inconsistent` false, `oversize` true, `timed_out` true, 27 steps, `max_cols` 667,444, `max_rows` 891,980, `max_dense_entries` 79,330,205,871, `max_nnz` 34,785,087,985, `muladds` 1.560×10¹⁴, `x_r` 221,026,520.],
  [main, target 1], [08:42:41], [6,416 s], [exit 0. Identical to target 0 on every field above except `max_nnz` 34,785,087,996, `muladds` 1.560×10¹⁴ (differs in the 8th digit), `x_r` 704,078,485.],
  [confirmation (§3.3)], [—], [—], [not applicable: no target finished with a $D$.],
  [verify (§3.4)], [10:31], [—], [1 tower row (the identity row) checked by exhaustive search, 1 agrees; the two $N = 21$ rows carry no verdict to check.],
)

= The stop

Both targets reduced every degree-7 pair (steps 21–26, `fresh` rows at every step, last finished
step 26: 271,278 columns, 430,250 pairs left, all at degree 8) and were then stopped in step 27,
the first degree-8 step, before any reduction (`muladds` 0, `residual_rows` 0): the step's reducer
block $B'$ would hold 79,330,205,871 entries against $C$ = 20,300,000,000 (`f4_fp_tower` stops as
oversize when `dense_entries > max_dense`; the trace prints the generic `oversize: --max-nnz`
wording). *Derived:* at 4 bytes per entry that $B'$ is $≈ 317$ GB, and $≈ 460$ GB at round 7's
peak-to-$B'$ ratio of 1.45, so the same cap would have stopped a 192 GB host. Peak resident memory
before the stop: 67.1 GB (target 0), 67.1 GB (target 1); address-space peak 68.7 GB. The protocol's
$A$ and the budget were not reached.

#figure(image("steps.svg", width: 100%), caption: [Recorded per-step width and peak memory of both
$N = 21$ targets (the two traces coincide), and the recorded width of the $m = 3$ Kummer line at
$p_1$ for $N = 9$–$18$ (rounds 2–7) beside $N = 21$'s last finished and stopped steps. Runs
`M3-N21-t0`, `M3-N21-t1`; source `plot.py`.])

= Against the predictions (PROTOCOL.md §4)

#table(columns: (1fr, 1.4fr), align: left,
  [*prediction*], [*recorded*],
  [1. identity holds], [holds],
  [2. $D = 7$ on both targets, both refute (low confidence)], [not observed: neither target refuted at degree 7; both entered degree 8 and were stopped there by $C$],
  [3. both targets give the same $D$, width and step count], [holds: identical width, steps and stop],
  [4. width 360,000–475,000, $1.3$–$1.5 × 10^11$ B peak, $2.5$–$7.4 × 10^15$ muladds (too low if $D$ rises)], [last finished width 271,278; stopped step 667,444; peak 67 GB before the stop; $1.56 × 10^14$ muladds before the stop. The degree-8 step's $B'$ alone exceeds the projected peak by $≈ 2×$ (derived)],
  [5. 14–60 h on 32 threads], [1 h 47 min per target to the stop, which is not a completed system],
)

The reading under §5 is not made here. The rows that may apply are "stopped with $D >= 7$ only"
and "stopped after a productive step at degree $>= 8$"; the recorded trace has every degree-7 pair
reduced without refuting and no productive degree-8 reduction. That reading belongs to the result PR.

= Files

`runs/M3-N21-t0.jsonl`, `runs/M3-N21-t1.jsonl` (one row each plus a summary line), their `.log`
step traces, `verify.txt`, `progress.txt`, and `research/large_memory_runs_20261018/EXPERIMENT_LOG.md`
for every run on this host.
