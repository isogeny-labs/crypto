#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 8pt)
#show table: set par(justify: false)

= Run report: target-free 4-sum system against the chained m = 3 target system (lead L1)

*Study* `research/target_free_relations_20261018/`, pre-registered in
`PREREGISTRATION.md`. *Run* run-20261018T1645Z (2026-10-18 16:45 UTC; queue log
2026-10-18T16:45:52Z to 2026-10-18T18:27:36Z). *Commit* `aa7d718f0`. *Class:* stage
diagnostic, with no `S`, no rho ratio and no speed claim.

== What ran

- *Command.* `run.sh`: per cell `(n, ℓ) ∈ {(16,4), (18,4), (20,5), (24,6)}`,
  basis `gp` (one draw: the progression base is deterministic) and `random`
  (four draws), twists `[0,0,0,0]` and `[0,1,2,3]`, one process per
  (cell, basis, twists):
  `prlimit --as=12000000000 timeout 7200 dreg_target_free --cells n:ℓ --basis B --twists T --draws K --seed 20261018 --budget-secs 1200 --target --no-count`,
  with `F4_F2_MAX_ROWS=F4_F2_MAX_COLS=50000000`, `RAYON_NUM_THREADS=2` and
  the default 1 GiB F4 step-matrix cap.
- *Arms.* `target_free`: `build_target_free_system(basis, b = 1, twists)`,
  `4ℓ + n` unknowns, `2n` equations. `target_m3`:
  `build_decomposition_system(basis, x_R, b = 1, m = 3)` on the same basis
  with a fresh random `x_R` per draw, `3ℓ + n` unknowns, `2n` equations.
  Both solved by `pq_f4_f2::groebner_basis_f4` with the field equations.
- *Measure.* F4 solving degree (highest step degree that added a basis
  element) and degree reached. A record with `oversize` or `timed_out` is
  unmeasured; its `solving_degree` is a lower bound only.
- *Host.* The 4-core, 15 GB container, which is not isolated (`AGENTS.md`
  §10). Wall times are practicality notes only.
- *Scorer.* `dreg_score target_free runs/cell-*.jsonl`, output in
  `score-output.txt`.

== Completion

4 of 4 `runs/cell-*.jsonl` records non-empty;
4 processes logged an exit; the logged statuses are not the
processes' (the script read `$?` after a command substitution, so every
logged status is that of `date`; see the hand note at the end of
`runs/queue.log`, fixed in `run.sh` afterwards). Every record here is
non-empty and its `.log` is empty. A fifth process, `cell-18-4.gp.tw0000`,
was killed within seconds of its start when the queue was stopped at the
registered stop condition (target-free arm unmeasured on every (16, 4)
draw); its empty record was removed and the stop is noted in the queue log. Target-free arm: 10 draws, 0 finished,
10 oversize, 0 timed out. Target arm: 10 draws,
10 finished, 0 oversize, 0 timed out.
Longest single F4 run 168 s; largest recorded step matrix
0.35 GiB. Peak RSS was not recorded by the harness.

== Measured values

#figure(image("target_free_degrees.svg", width: 100%),
  caption: [Recorded values from `runs/cell-*.jsonl`, run run-20261018T1645Z. F4
  solving degree per draw and arm; filled markers are finished runs, open
  markers with an arrow are unfinished runs drawn at their lower bound.
  Source: `plot.py` (layout only).])

#figure(image("target_free_sizes.svg", width: 100%),
  caption: [Recorded `matrix_cols_max` and `secs` per F4 run at (16, 4), run
  run-20261018T1645Z, log scale. Open markers stopped at the 1 GiB step-matrix cap,
  so their values are what was reached before the stop. Wall seconds are
  practicality notes on a non-isolated host. Source: `plot.py`.])

=== Per (cell, basis, twists) readings

#table(columns: 10, inset: 3pt, stroke: 0.5pt + gray,
  [*(n, ℓ)*],
  [*basis*],
  [*twists*],
  [*target-free measured / draws*],
  [*target-free solving (measured; bounds ≥)*],
  [*target m = 3 measured / draws*],
  [*target solving (measured)*],
  [*unsat. target draws*],
  [*registered reading*],
  [*single-arm reading (fewer than 3 draws)*],
  [(16, 4)],
  [gp],
  [\[0,0,0,0\]],
  [0 / 1],
  [bounds ≥ 4],
  [1 / 1],
  [4],
  [1],
  [unmeasured],
  [not lower (bounded: every target-free draw ≥ target median)],
  [(16, 4)],
  [gp],
  [\[0,1,2,3\]],
  [0 / 1],
  [bounds ≥ 4],
  [1 / 1],
  [4],
  [1],
  [unmeasured],
  [not lower (bounded: every target-free draw ≥ target median)],
  [(16, 4)],
  [random],
  [\[0,0,0,0\]],
  [0 / 4],
  [bounds ≥ 4, 4, 4, 4],
  [4 / 4],
  [4, 4, 4, 4],
  [4],
  [unmeasured],
  [not lower (bounded: every target-free draw ≥ target median)],
  [(16, 4)],
  [random],
  [\[0,1,2,3\]],
  [0 / 4],
  [bounds ≥ 4, 4, 4, 4],
  [4 / 4],
  [4, 4, 4, 4],
  [4],
  [unmeasured],
  [not lower (bounded: every target-free draw ≥ target median)])

=== Registered readings (from `dreg_score target_free`)

```
Registered verdict: killed on the bounded reading (no cell reads lower for gp on any draw, measured or bounded); undecided by the registered rule (fewer than 3 measured draws per arm). gp cells reading lower: []; random cells reading lower: []; gp cells with a registered reading: 0 of 1; gp cells bounded not-lower on every twist: 1 of 1.
P1 (gp plain reads higher or equal at every measured cell): unmeasured — (16,4)[0000]: unmeasured, bounded not lower
P2 (gp twisted reads equal at every measured cell): unmeasured — (16,4)[0123]: unmeasured, bounded not lower
P3 (random never reads lower): unmeasured — (16,4)[0000]: unmeasured, bounded not lower; (16,4)[0123]: unmeasured, bounded not lower
P4 (target arm F4 solving degree = 4 on unsatisfiable draws at (16,4), (18,4)): holds — 10 of 10 unsatisfiable measured target draws read 4
```

== Deviation: raised-cap pass on (16, 4)

Not registered (see `RESULTS.md`, "Deviations"). `run_highcap.sh`: the same
arms at (16, 4), one draw each (gp plain, gp twisted, random plain, seed
20261018, the first draw of `runs/`), with `F4_F2_MAX_MATRIX_WORDS = 2^30`
(8 GiB step matrices), a 3,600 s budget and the same 12 GB address-space
limit. Records: `runs_highcap/`; scored separately, never pooled with
`runs/` (`score-output-highcap.txt`). Stars in the figure.

#table(columns: 10, inset: 3pt, stroke: 0.5pt + gray,
  [*(n, ℓ)*],
  [*basis*],
  [*twists*],
  [*target-free measured / draws*],
  [*target-free solving (measured; bounds ≥)*],
  [*target m = 3 measured / draws*],
  [*target solving (measured)*],
  [*unsat. target draws*],
  [*registered reading*],
  [*single-arm reading (fewer than 3 draws)*],
  [(16, 4)],
  [gp],
  [\[0,0,0,0\]],
  [1 / 1],
  [4],
  [1 / 1],
  [4],
  [1],
  [unmeasured],
  [equal (1 vs 1 measured draws)],
  [(16, 4)],
  [random],
  [\[0,0,0,0\]],
  [1 / 1],
  [4],
  [1 / 1],
  [4],
  [1],
  [unmeasured],
  [equal (1 vs 1 measured draws)])

Queue log (`runs_highcap/queue.log`):

```
2026-10-18T17:16:02Z start research/target_free_relations_20261018/runs_highcap/cell-16-4.gp.tw0000
2026-10-18T17:49:49Z exit 0 research/target_free_relations_20261018/runs_highcap/cell-16-4.gp.tw0000
2026-10-18T17:49:49Z start research/target_free_relations_20261018/runs_highcap/cell-16-4.gp.tw0123
2026-10-18T17:53:13Z exit 0 research/target_free_relations_20261018/runs_highcap/cell-16-4.gp.tw0123
2026-10-18T17:53:13Z start research/target_free_relations_20261018/runs_highcap/cell-16-4.random.tw0000
2026-10-18T18:27:15Z exit 0 research/target_free_relations_20261018/runs_highcap/cell-16-4.random.tw0000
2026-10-18T18:27:36Z note written by hand: the 'exit' statuses above are date's, not the runs' (run_highcap.sh read $? after a command substitution; fixed afterwards). cell-16-4.gp.tw0123 aborted on allocation under the 12 GB address-space limit (its .log; empty record); the other two finished.
```

== Observations (no interpretation)

See `RESULTS.md` for the reading against the registered verdict rules and
the predictions; this report records what ran and what was measured.
