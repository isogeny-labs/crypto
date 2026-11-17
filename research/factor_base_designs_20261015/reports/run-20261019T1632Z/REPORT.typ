#set document(title: "Factor-base designs: A0 on the tower engine (T1, T2)", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: A0's memory-bound cells on the sparse tower engine (run-20261019T1632Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only: nothing here prices a pipeline, reports S or shows a speedup. *Protocol:* PREREGISTRATION.md Amendment 6 (registered in commit #raw("02e91c9d4") before any T1 or T2 row), with Amendments 1–5 for everything it does not change.

== Why this run

Two A0 cells are missing under #raw("f4_fp") for memory: m = 3, t = 5 (F3; the cell where A11 finishes at D = 5) and m = 4, t = 4 (C2 at 13 GB, G1 at 15 GB). The repository's sparse tower engine #raw("f4_fp_tower") needs far less memory. Amendment 6 admits it as a second instrument for A0 only if it first reproduces #raw("f4_fp")'s D on every completed planted A0 system (T1), and only then measures the two missing cells (T2).

== Run

- *Commands:* #raw("fb_designs_slope admit-tower research/factor_base_designs_20261015/runs") (T1), then #raw("fb_designs_slope run --engine tower --arms A0 --m 3 --t-min 5 --t-max 5 --budget 3600 --grace 300 --vmem-kb 15000000 --file 'T2-{arm}-m{m}'") and the same with #raw("--m 4 --t-min 4 --t-max 4") (T2).
- *Commits measured:* T1 at #raw("02e91c9d413cbaba6004748b40913ca46bdc91c7"), T2 at #raw("0cc454e9c49d1d55dd789a6bec52c9ec50940bfb"), recorded in each row.
- *Host:* "vm", Intel Xeon @ 2.10 GHz, 4 cores, 15 GB, no swap, a Claude Code cloud container. One system at a time.
- *Window:* 2026-10-19, T1 16:30Z to 16:31Z; T2 16:32Z to 18:44Z (UTC).
- *Engine:* A0's Kummer equations become the ring's square rules (#raw("y_{j+1} = y_j²"), #raw("y_{t−1}² = 1")). The summation equations, reduced into the ring, are the input. Seeds and constructions are A0's own, so the systems are identical to the #raw("f4_fp") ones. Every equation is checked at the planted solution, and every tower equation is checked to vanish in the ring, before the engine runs.
- *Records:* #raw("runs/T1-admission.jsonl"), #raw("runs/T2-A0-m3.jsonl"), #raw("runs/T2-A0-m4.jsonl") and their #raw(".log") files, and #raw("runs/analysis-tower.txt") (native #raw("analyze --tower")).

== T1: admission

*Admitted.* All 26 completed planted A0 systems on p#sub[1] (the first run's m = 2, t = 2–9 and m = 3, t = 2–4, and F4's m = 4, t = 2, 3) complete on the tower engine with the committed #raw("f4_fp") D, without deriving 1, and with the planted solution kept in the basis. The engine took 0.002 s to 13.9 s per system.

The engine's #raw("max_cols") is smaller than #raw("f4_fp")'s at every one of them (for example 16 461 against 21 619 at m = 4, t = 3), and it is not the same quantity: the engines work in different rings. Matrix widths are not compared across engines in this study.

== T2: the two cells

#figure(image("a0_tower_d_vs_b.svg", width: 100%),
  caption: [D against b for A0 at m = 3 (left) and m = 4 (right) on p#sub[1]: #raw("f4_fp") cells, the T2 cells on the tower engine, and A11 (#raw("f4_fp")) beside. Both targets per cell are plotted (n = 2; they coincide). The cross is the T2 m = 4 cell, at the partial D = 7 both systems reached. Run run-20261019T1632Z; rows from #raw("runs/*.jsonl"), plotted by #raw("plot.py").])

#table(columns: 7, align: center, stroke: 0.4pt + gray,
  [*cell*], [*system*], [*D*], [*degree reached*], [*time (s)*], [*tower-engine #raw("max_cols")*], [*status*],
  [m = 3, t = 5], [target 0], [7], [8], [383.5], [34 706], [ok],
  [m = 3, t = 5], [target 1], [7], [8], [377.5], [35 617], [ok],
  [m = 4, t = 4], [target 0], [7 (partial)], [7], [3600.1], [88 925 (partial)], [timeout at the 3600 s budget],
  [m = 4, t = 4], [target 1], [7 (partial)], [8], [3600.4], [170 036 (partial)], [timeout at the 3600 s budget],
)

- *m = 3, t = 5:* both systems complete, with D = 7. With this cell, A0's m = 3 series reads D = 5, 6, 6, 7 on 4 completed cells: σ = 0.6000, *no middle ground* (§1). Without it (the 3 #raw("f4_fp") cells), σ = 0.5000, the same verdict. A11 at the same cell is D = 5 (F3), so A11's D is 2 below A0's here (descriptive).
- *m = 4, t = 4:* both systems stopped at the 3600 s budget, at partial D = 7 (the second had reached degree 8). The memory limit did not bind: spot checks showed 6–8 GB resident against the 15 GB limit, where #raw("f4_fp") aborted for memory after about 405 s. A cell with a system that did not complete does not join the fit (Amendment 6), so A0's m = 4 series stays at 2 completed cells: σ = 1.0000, *inconclusive*, and its registered prediction "σ ≥ 0.5 at m = 4" remains *not decided*.

== Predictions scorecard (Amendment 6)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*held?*],
  [T1: admitted], [26 of 26 match], [yes],
  [T2, m = 3: completes with D = 7, so the series reads 5, 6, 6, 7 (σ = 0.6)], [D = 7 on both systems; σ = 0.6000 on 4 cells], [yes],
  [T2, m = 4: does not complete within the limits], [both systems stop at 3600 s, partial D = 7], [yes],
)

== Limits of what this shows

- T1 checks agreement up to the sizes #raw("f4_fp") could complete (m = 3, t ≤ 4; m = 4, t ≤ 3). That the engine's D at m = 3, t = 5 equals #raw("f4_fp")'s is an assumption those 26 agreements support, not a measurement: #raw("f4_fp") has no completed system there to compare.
- The tower engine's own cost counter (#raw("muladds")) is recorded but is not #raw("field_ops"), so κ (Amendment 4) is not extended by these cells.
- Times are wall-clock on one 4-core container. #raw("f4_fp") and the tower engine ran under different limits here (memory against time), so no timing comparison is made between them.
- One cell and one prime. Nothing here replicates the m = 3, t = 5 reading on p#sub[2].

== Deviations

None from Amendment 6.
