#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 8pt)
#show table: set par(justify: false)

= Run report: eliminated target-free S₄ against the S₄ target system (lead L1, lemma presentation)

*Study* `research/target_free_s4_20261018/`, pre-registered in
`PREREGISTRATION.md`. *Run* run-20261018T1853Z (2026-10-18 18:53 UTC; queue log
2026-10-18T18:53:28Z to 2026-10-18T20:33:22Z). *Commit* `368d90f34`. *Class:* stage
diagnostic, with no `S`, no rho ratio and no speed claim.

== What ran

- *Command.* `run.sh`: cells (13,3), (16,4), (17,4), (18,4), (19,4), (20,5),
  (24,6) in that order, arms gp `[0,0,0,0]`, gp `[0,1,2,3]` and random
  `[0,0,0,0]`, one process each (below), with `F4_F2_MAX_MATRIX_WORDS = 2^29` (4 GiB step matrices) and 3 rayon
  threads. The queue stops at the first cell with no finished target-free
  record.
  ```
  prlimit --as=12000000000 timeout 10800 dreg_eliminated_s4 --cells n:l \
    --basis B --twists T --draws 4 --seed 20261018 --budget-secs 1800
  ```
- *Arms.* `target_free_s4`: `build_eliminated_s4_system(basis, 1, None, twists)`,
  $4 ell$ unknowns, `n` equations, solved once on gp (deterministic) and on
  4 draws on random. `target_s4`: the same with $x_4 = x_R$ (random per
  draw), $3 ell$ unknowns, 4 draws.
- *Measure.* F4 solving degree (field equations included), degree reached,
  and the top-form deficiency `top_forms − top_rank`, computed before F4.
- *Host.* The 4-core, 15 GB container, which is not isolated (`AGENTS.md`
  §10). Wall times are practicality notes only.
- *Scorer.* `dreg_score s4_free runs/cell-*.jsonl`, output in
  `score-output.txt`.

== Completion

15 of 18 `runs/cell-*.jsonl` records non-empty;
18 processes exited, 3 with a non-zero status
(exit 134 research/target\_free\_s4\_20261018/runs/cell-20-5.gp.tw0000; exit 134 research/target\_free\_s4\_20261018/runs/cell-20-5.gp.tw0123; exit 134 research/target\_free\_s4\_20261018/runs/cell-20-5.random.tw0000). Target-free arm: 30 records, 30
finished, 0 oversize, 0 timed out. Target arm:
60 records, 60 finished, 0 oversize,
0 timed out. Longest single F4 run 406 s; largest
recorded step matrix 0.60 GiB. Peak RSS was not recorded.

Queue log (`runs/queue.log`):

```
2026-10-18T18:53:28Z start research/target_free_s4_20261018/runs/cell-13-3.gp.tw0000
2026-10-18T18:53:29Z exit 0 research/target_free_s4_20261018/runs/cell-13-3.gp.tw0000
2026-10-18T18:53:29Z start research/target_free_s4_20261018/runs/cell-13-3.gp.tw0123
2026-10-18T18:53:29Z exit 0 research/target_free_s4_20261018/runs/cell-13-3.gp.tw0123
2026-10-18T18:53:29Z start research/target_free_s4_20261018/runs/cell-13-3.random.tw0000
2026-10-18T18:53:31Z exit 0 research/target_free_s4_20261018/runs/cell-13-3.random.tw0000
2026-10-18T18:53:31Z start research/target_free_s4_20261018/runs/cell-16-4.gp.tw0000
2026-10-18T18:57:09Z exit 0 research/target_free_s4_20261018/runs/cell-16-4.gp.tw0000
2026-10-18T18:57:09Z start research/target_free_s4_20261018/runs/cell-16-4.gp.tw0123
2026-10-18T19:03:57Z exit 0 research/target_free_s4_20261018/runs/cell-16-4.gp.tw0123
2026-10-18T19:03:57Z start research/target_free_s4_20261018/runs/cell-16-4.random.tw0000
2026-10-18T19:19:03Z exit 0 research/target_free_s4_20261018/runs/cell-16-4.random.tw0000
2026-10-18T19:19:03Z start research/target_free_s4_20261018/runs/cell-17-4.gp.tw0000
2026-10-18T19:22:35Z exit 0 research/target_free_s4_20261018/runs/cell-17-4.gp.tw0000
2026-10-18T19:22:35Z start research/target_free_s4_20261018/runs/cell-17-4.gp.tw0123
2026-10-18T19:27:51Z exit 0 research/target_free_s4_20261018/runs/cell-17-4.gp.tw0123
2026-10-18T19:27:51Z start research/target_free_s4_20261018/runs/cell-17-4.random.tw0000
2026-10-18T19:41:58Z exit 0 research/target_free_s4_20261018/runs/cell-17-4.random.tw0000
2026-10-18T19:41:58Z start research/target_free_s4_20261018/runs/cell-18-4.gp.tw0000
2026-10-18T19:46:01Z exit 0 research/target_free_s4_20261018/runs/cell-18-4.gp.tw0000
2026-10-18T19:46:01Z start research/target_free_s4_20261018/runs/cell-18-4.gp.tw0123
2026-10-18T19:52:21Z exit 0 research/target_free_s4_20261018/runs/cell-18-4.gp.tw0123
2026-10-18T19:52:21Z start research/target_free_s4_20261018/runs/cell-18-4.random.tw0000
2026-10-18T20:06:39Z exit 0 research/target_free_s4_20261018/runs/cell-18-4.random.tw0000
2026-10-18T20:06:39Z start research/target_free_s4_20261018/runs/cell-19-4.gp.tw0000
2026-10-18T20:10:14Z exit 0 research/target_free_s4_20261018/runs/cell-19-4.gp.tw0000
2026-10-18T20:10:14Z start research/target_free_s4_20261018/runs/cell-19-4.gp.tw0123
2026-10-18T20:16:34Z exit 0 research/target_free_s4_20261018/runs/cell-19-4.gp.tw0123
2026-10-18T20:16:34Z start research/target_free_s4_20261018/runs/cell-19-4.random.tw0000
2026-10-18T20:29:47Z exit 0 research/target_free_s4_20261018/runs/cell-19-4.random.tw0000
2026-10-18T20:29:47Z start research/target_free_s4_20261018/runs/cell-20-5.gp.tw0000
2026-10-18T20:30:58Z exit 134 research/target_free_s4_20261018/runs/cell-20-5.gp.tw0000
2026-10-18T20:30:58Z start research/target_free_s4_20261018/runs/cell-20-5.gp.tw0123
2026-10-18T20:32:06Z exit 134 research/target_free_s4_20261018/runs/cell-20-5.gp.tw0123
2026-10-18T20:32:06Z start research/target_free_s4_20261018/runs/cell-20-5.random.tw0000
2026-10-18T20:33:22Z exit 134 research/target_free_s4_20261018/runs/cell-20-5.random.tw0000
2026-10-18T20:33:22Z stop: no finished target-free record at (20:5); larger cells not run
2026-10-18T20:33:22Z queue done
```

== Measured values

#figure(image("s4_degrees.svg", width: 100%),
  caption: [Recorded values from `runs/cell-*.jsonl`, run run-20261018T1853Z. Left:
  F4 solving degree per draw and arm. Right: top-form dependencies of the
  target-free arm. Source: `plot.py` (layout only).])

=== Per (cell, basis, twists) readings

#table(columns: 11, inset: 3pt, stroke: 0.5pt + gray,
  [*(n, ℓ)*],
  [*basis*],
  [*twists*],
  [*target-free solving (≥ = unfinished)*],
  [*target-free top deficiency*],
  [*target solving*],
  [*target top deficiency*],
  [*target-free value*],
  [*target median*],
  [*registered reading*],
  [*bounded reading*],
  [(13, 3)],
  [gp],
  [\[0,0,0,0\]],
  [7],
  [1],
  [6, 6, 6, 6],
  [4, 4, 4, 4],
  [7],
  [6],
  [higher],
  [not lower],
  [(13, 3)],
  [gp],
  [\[0,1,2,3\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(13, 3)],
  [random],
  [\[0,0,0,0\]],
  [8, 8, 8, 8],
  [0, 0, 0, 0],
  [6, 6, 6, 6],
  [3, 3, 3, 3],
  [8],
  [6],
  [higher],
  [not lower],
  [(16, 4)],
  [gp],
  [\[0,0,0,0\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(16, 4)],
  [gp],
  [\[0,1,2,3\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(16, 4)],
  [random],
  [\[0,0,0,0\]],
  [8, 8, 8, 8],
  [0, 0, 0, 0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(17, 4)],
  [gp],
  [\[0,0,0,0\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(17, 4)],
  [gp],
  [\[0,1,2,3\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(17, 4)],
  [random],
  [\[0,0,0,0\]],
  [8, 8, 8, 8],
  [0, 0, 0, 0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(18, 4)],
  [gp],
  [\[0,0,0,0\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [1, 1, 1, 1],
  [8],
  [7],
  [higher],
  [not lower],
  [(18, 4)],
  [gp],
  [\[0,1,2,3\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(18, 4)],
  [random],
  [\[0,0,0,0\]],
  [8, 8, 8, 8],
  [0, 0, 0, 0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(19, 4)],
  [gp],
  [\[0,0,0,0\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [1, 1, 1, 1],
  [8],
  [7],
  [higher],
  [not lower],
  [(19, 4)],
  [gp],
  [\[0,1,2,3\]],
  [8],
  [0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower],
  [(19, 4)],
  [random],
  [\[0,0,0,0\]],
  [8, 8, 8, 8],
  [0, 0, 0, 0],
  [7, 7, 7, 7],
  [0, 0, 0, 0],
  [8],
  [7],
  [higher],
  [not lower])

=== Registered readings (from `dreg_score s4_free`)

```
Registered verdict (primary): killed. gp (cell, twists) reading lower: []; random reading lower: []; gp readings measured: 10.
Secondary (shape inside target-free): fewer than 2 cells — (13,3)[0000]: gp 7 vs random 8 → shape lower; (16,4)[0000]: gp 8 vs random 8 → equal; (17,4)[0000]: gp 8 vs random 8 → equal; (18,4)[0000]: gp 8 vs random 8 → equal; (19,4)[0000]: gp 8 vs random 8 → equal
P1 (target-free reads higher than target at every measured (cell, basis, twists)): holds — 15 of 15 higher
P2 (gp target-free top deficiency above the random median at every cell with both): fails — 1 of 5 above, 4 equal
P3 (secondary shape effect at ≥ 2 cells): fails
P4 (twisting [0,1,2,3] brings the gp deficiency to the random level or below): holds — (13,3): gp twisted deficiency 0 vs random plain median 0 → at or below random; (16,4): gp twisted deficiency 0 vs random plain median 0 → at or below random; (17,4): gp twisted deficiency 0 vs random plain median 0 → at or below random; (18,4): gp twisted deficiency 0 vs random plain median 0 → at or below random; (19,4): gp twisted deficiency 0 vs random plain median 0 → at or below random
```

== Observations (no interpretation)

See `RESULTS.md` for the reading against the registered rules; this report
records what ran and what was measured.
