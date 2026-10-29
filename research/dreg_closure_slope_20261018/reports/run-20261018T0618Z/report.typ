#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: MutantXL closure degree against m·ℓ, up to m·ℓ = 30

*Study* `research/dreg_closure_slope_20261018/`, pre-registered in
`PREREGISTRATION.md`. *Run* run-20261018T0618Z (2026-10-18 06:18 UTC; queue log
2026-10-18T06:17:56Z to 2026-10-18T13:29:43Z). *Commit* `9a44015c1`. *Class:* stage
diagnostic, with no `S`, no rho ratio and no speed claim.

== What ran

- *Draws.* `run.sh LADDER CLOSURE gen` generated the draws with seed
  20261018 (exact solution count; XL built to degree 3 only), committed
  before any closure ran. Files: `draws/*.jsonl`.
- *Instrument check (P4).* `run.sh … check` replayed committed dense
  degree-4 refutations on the sparse engine: 1 of
  2 `runs/check-*` records non-empty.
- *Measurement.* `run.sh … measure` ran one process per (draw, degree), as
  `prlimit --as=7000000000 timeout 10800 dreg_closure --sparse D --max-entries 350000000 --draw K`,
  degrees 4, 5, 6 in turn (5, 6 for the committed m·ℓ = 21 cells), each
  draw stopping at its first refutation. 18 of
  21 `runs/*.D*.jsonl` measurement records are non-empty;
  0 record a closure refutation and 18 hit the entry
  cap.
- *Control (P5).* `run.sh … control`: 0 of
  0 `runs/control-*` records non-empty.
- *Cells.* The frozen table in `PREREGISTRATION.md`: committed anchors at
  m·ℓ = 12–18, fresh cells at 20, 24, 27, 30, and the two committed
  m·ℓ = 21 cells.
- *Host.* The 4-core, 15 GB container, which is not isolated (`AGENTS.md`
  §10). Wall times are practicality notes only.
- *Scorer.* `cargo run --release --example dreg_score -- slope
  research/dreg_closure_slope_20261018/runs/*.D*.jsonl`, output in
  `score-output.txt`.

== Completion

Processes (from `runs/queue.log`): 16 sparse closure runs on the two
committed m·ℓ = 21 cells, all exit 0 and all `capped` (degree 5 after
168–277 s, degree 6 after 102–121 s); 3 runs on fresh m3 (22, 8) draw 2
(degree 4 exit 0 capped after 8,466 s; degree 5 exit 0 capped after 82 s;
degree 6 exit 134, "memory allocation failed" under the 7 GB address-space
limit); draws 4 and 5 of that cell killed by hand when the queue was
stopped after 155 and 13 CPU-minutes (empty records, unmeasured); draw 7
and every later cell not started. P4: check m3 (18, 6) draw 0 exit 0,
refuted at degree 4 after 1,325 s (rank 64,433, 6,863 falls, 400,372
products); check m4 (13, 4) draw 1 killed by a worker restart after
80 minutes (empty record). Peak RSS was not recorded by the harness for
sparse runs; `ps` during the runs showed 0.4–0.8 GB. Slowest finished
draw: m3 (22, 8) draw 2 at degree 4, 8,466 s. No wall-clock claim.

== Measured values

#figure(image("closure_slope.svg", width: 100%),
  caption: [Recorded values from `runs/*.D*.jsonl`, run run-20261018T0618Z. Closure
  degree per draw against $m dot ell$; squares are the committed anchors,
  circles exact degrees, triangles lower bounds. The solid line has the
  least-squares slope read from `dreg_score slope`, drawn through (18, 4)
  for display (not the fitted intercept); dashed lines are the two
  registered thresholds, drawn the same way. Source: `plot.py` (layout only).])

=== Per-cell, per-draw degrees (measured)

#table(columns: 7, inset: 4pt, stroke: 0.5pt + gray,
  [*m*],
  [*(n, ℓ)*],
  [*m·ℓ*],
  [*N*],
  [*degree per draw*],
  [*cell*],
  [*secs per draw*],
  [4],
  [(12, 3)],
  [12],
  [36],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [4],
  [(14, 3)],
  [12],
  [40],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [3],
  [(11, 5)],
  [15],
  [26],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [3],
  [(13, 5)],
  [15],
  [28],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [3],
  [(15, 5)],
  [15],
  [30],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [4],
  [(13, 4)],
  [16],
  [42],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [3],
  [(16, 6)],
  [18],
  [34],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [3],
  [(18, 6)],
  [18],
  [36],
  [4 4 4 4 (committed, dense)],
  [resolved at 4],
  [—],
  [3],
  [(19, 7)],
  [21],
  [40],
  [unmeasured unmeasured unmeasured unmeasured],
  [unmeasured],
  [277 269 247 264],
  [3],
  [(21, 7)],
  [21],
  [42],
  [unmeasured unmeasured unmeasured unmeasured],
  [unmeasured],
  [220 223 220 231],
  [3],
  [(22, 8)],
  [24],
  [46],
  [unmeasured unmeasured unmeasured],
  [unmeasured],
  [8548 0 0])

=== Registered readings (from `dreg_score slope`)

```
Slope s = 0.0000 degrees per summand unknown (m·ℓ), least squares over 8 cells (8 anchors at degree 4, 0 measured here)
Registered verdict: nothing resolved above the anchors: undecided
```

=== Prediction outcomes P1 to P5

```
P1 (every resolved draw at m·ℓ = 20, 21 reads 5): 0 of 0 → unmeasured
P2 (a resolved cell with m·ℓ ≤ 30 reads 6): refuted on the resolved cells (or unmeasured)
P3 (s ≥ 0.10): unmeasured
P4 (sparse degree-4 replays of committed dense refutations refute): 1 of 1 measured (2 runs) → incomplete (capped or killed runs are unmeasured)
P5 (no satisfiable draw refuted): 0 control runs, 0 refuted, 0 unmeasured
```

=== Distance remaining (derived)

Derived (cost model, w = 1; extrapolated degree 4 + s·(m·ℓ − 18) at ℓ = ⌈n/m⌉, read from the fit over m·ℓ ≤ 30):

#table(columns: 8, inset: 4pt, stroke: 0.5pt + gray,
  [*n*],
  [*m*],
  [*ℓ*],
  [*log₂ budget per solve*],
  [*admissible D (chained N)*],
  [*admissible D (N = m·ℓ)*],
  [*extrapolated degree*],
  [*gap, degrees (chained)*],
  [73],
  [3],
  [25],
  [13.3],
  [none],
  [2],
  [4.0],
  [—],
  [73],
  [4],
  [19],
  [18.3],
  [2],
  [3],
  [4.0],
  [+2.0],
  [73],
  [5],
  [15],
  [19.0],
  [2],
  [3],
  [4.0],
  [+2.0],
  [73],
  [6],
  [13],
  [21.4],
  [2],
  [4],
  [4.0],
  [+2.0],
  [109],
  [3],
  [37],
  [19.6],
  [2],
  [3],
  [4.0],
  [+2.0],
  [109],
  [4],
  [28],
  [27.6],
  [3],
  [5],
  [4.0],
  [+1.0],
  [109],
  [5],
  [22],
  [29.3],
  [3],
  [5],
  [4.0],
  [+1.0],
  [109],
  [6],
  [19],
  [33.7],
  [4],
  [6],
  [4.0],
  [+0.0],
  [127],
  [3],
  [43],
  [22.7],
  [3],
  [3],
  [4.0],
  [+1.0],
  [127],
  [4],
  [32],
  [30.7],
  [4],
  [5],
  [4.0],
  [+0.0],
  [127],
  [5],
  [26],
  [36.4],
  [4],
  [6],
  [4.0],
  [+0.0],
  [127],
  [6],
  [22],
  [39.8],
  [5],
  [7],
  [4.0],
  [-1.0],
  [131],
  [3],
  [44],
  [22.8],
  [3],
  [3],
  [4.0],
  [+1.0],
  [131],
  [4],
  [33],
  [31.8],
  [4],
  [5],
  [4.0],
  [+0.0],
  [131],
  [5],
  [27],
  [38.4],
  [5],
  [7],
  [4.0],
  [-1.0],
  [131],
  [6],
  [22],
  [37.9],
  [4],
  [7],
  [4.0],
  [+0.0],
  [163],
  [3],
  [55],
  [29.4],
  [4],
  [4],
  [4.0],
  [+0.0],
  [163],
  [4],
  [41],
  [40.4],
  [5],
  [7],
  [4.0],
  [-1.0],
  [163],
  [5],
  [33],
  [47.1],
  [6],
  [8],
  [4.0],
  [-2.0],
  [163],
  [6],
  [28],
  [52.5],
  [6],
  [10],
  [4.0],
  [-2.0],
  [233],
  [3],
  [78],
  [40.2],
  [5],
  [6],
  [4.0],
  [-1.0],
  [233],
  [4],
  [59],
  [59.2],
  [7],
  [10],
  [4.0],
  [-3.0],
  [233],
  [5],
  [47],
  [67.9],
  [8],
  [12],
  [4.0],
  [-4.0],
  [233],
  [6],
  [39],
  [72.3],
  [8],
  [13],
  [4.0],
  [-4.0])

This table is *derived* from the stated cost model and the fitted slope. It
is not a measurement, and it is not extrapolated beyond m·ℓ = 30 as a law.

== Control

Soundness, as registered: the first satisfiable draw of each fresh cell,
replayed with `--satisfiable --sparse 4` and `--sparse 5`. Outcome is the P5
line above. Records: `runs/control-*.d*.D*.{jsonl,log}`.

No control was run. Every registered control (first satisfiable draw of a
fresh cell) is at N ≥ 46, where the degree-4 sparse closure caps after more
than two hours, so a control would be unmeasured by construction; the
previous study's 345-draw control covers the dense engine and the sparse
engine's agreement test covers the sparse one.

== Raw records

- `runs/*.D*.jsonl`: one line per (draw, degree), with rank, falls,
  products, maximum row weight, seconds and the `capped` flag.
- `runs/check-*`: P4 instrument check. `runs/control-*`: P5 soundness control.
- `runs/*.log`: peak VmHWM per process; `runs/queue.log`; `draws/*.jsonl`.
- `score-output.txt` (native `dreg_score slope`); `plot.py` and the graph
  sources beside this report.
- The interpretation is in `../../RESULTS.md`.
