#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= Run report: plain XL at degree 5 on the m·ℓ = 21 cells

*Study* `research/dreg_xl_degree5_20261018/`; preregistered in commit 88451b3e2.
*Run* run-20261018T2104Z (queue log 2026-10-18T21:04:12Z to 22:10:30Z).
*Class:* stage diagnostic; no `S`, no rho ratio, no speed claim.

== What ran
- Command: `run.sh registered`, one process per cell, `prlimit --as=8000000000 timeout 10800`, `RAYON_NUM_THREADS=2`, engine `examples/xl_dense_degree run --degree 5 --threads 2 --max-bytes 6300000000`.
- Draws: unsatisfiable draws 0, 3, 5, 7 of m3 (19, 7) (N = 40) and 0, 2, 3, 6 of m3 (21, 7) (N = 42), from `research/dreg_closure_scale_20261016/draws/`. Soundness: the 7 satisfiable draws of the same cells.
- Host: 4-core, 15 GB container shared with other jobs, not isolated; times are practicality notes only.

== Measured
All 15 runs exited 0 and stopped at the 6.3 GB pivot cap with no verdict (measured):

#table(columns: 6, align: left,
  [cell], [N], [rows], [columns], [rows inserted at cap], [engine secs],
  [(19, 7), 8 draws], [40], [218,848], [619,720], [102,464–104,640 (46.8–47.8%)], [253–292],
  [(21, 7), 7 draws], [42], [278,978], [778,324], [77,312–79,488 (27.7–28.5%)], [242–284])

Registered outcome: 8 of 8 unmeasured. Not refuted and refuted are both unsupported. Controls: positive 20 of 20 correct (degree 5 and 6), soundness 8 of 8 not refuted, 0 failures. Size correction: the brief's 34,482 rows is 218,918 rows measured (17.0 GB dense).

#figure(image("xl_degree5_cap.svg", width: 100%),
  caption: [Rows inserted when the 6.3 GB pivot cap stopped each run, % of dense rows. Recorded values, n = 15 runs, run-20261018T2104Z. Light bars: satisfiable draws.])

== Missing
No degree-5 verdict for any registered draw; no derived slope point is available. Records: `runs/registered-*.jsonl`, `runs/soundness-*.jsonl`, `runs/ctl-*.jsonl`, `runs/queue.log`.
