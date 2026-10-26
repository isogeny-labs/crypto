#set page(paper: "a4", margin: 2cm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: none)
#show raw: set text(size: 8.5pt)

= Run report: dreg closure (4,6), run 20261018T1134Z

Study `research/dreg_closure_m4l6_20261018/`, host RunPod `wfh2xovuq5aahv` (25 vCPU, cgroup 200 GB, cap A = 184 GB), commit `9d4541e2`, draws commit `fc1eea79` (sha256 `d472512a…`). Observations only; the §4 reading is in `RESULTS.md`.

== Runs

#table(
  columns: (auto, auto, auto, auto, 1fr, auto, auto),
  align: (left, left, right, right, left, right, right),
  table.header([run], [cell], [draw], [to], [outcome], [wall], [peak RSS]),
  [measure], [A1 (4; 15, 6)], [42 (unsat)], [4], [exit 0, closure refuted at 4], [24:27], [25.8 GB],
  [measure], [A1], [214 (unsat)], [4], [exit 0, closure refuted at 4], [23:36], [25.7 GB],
  [deg5], [C1 (3; 19, 7)], [0 (unsat)], [5], [failed: allocation of 94,976 bytes failed, SIGABRT, no record], [17:28], [169.3 GB],
  [control], [A1], [0 (sat, 1,226 solutions)], [4], [exit 0, not refuted], [34:36], [21.4 GB],
)

Wall and RSS from GNU `time` (measured). The two A1 draws ran one after the other (§8 paired A1 with A2; A2 is deferred by §10).

== Recorded scans

#block(text(size: 8pt)[#table(
  columns: 10,
  align: (right, right, right, right, right, center, center, right, right, right),
  table.header([draw], [deg], [columns], [XL rank], [closure rank], [XL ref.], [closure ref.], [products], [word ops ($times 10^6$)], [secs]),
  [42], [3], [9,022], [854], [2,748], [no], [no], [5,539], [24.1], [0.197],
  [42], [4], [341,326], [23,757], [1], [no], [*yes*], [1,181,122], [2,873,337], [1,464.66],
  [214], [3], [9,022], [854], [2,748], [no], [no], [5,542], [24.3], [0.214],
  [214], [4], [341,056], [23,757], [1], [no], [*yes*], [1,178,721], [2,455,694], [1,413.40],
  [0 ctrl], [3], [11,662], [854], [2,748], [no], [no], [5,643], [26.9], [0.217],
  [0 ctrl], [4], [342,537], [23,757], [214,391], [no], [no], [1,094,741], [2,712,063], [2,072.76],
)])

Closure rank 1 is the engine's refutation signal; exact word-op counts are in `README.md`. Sample: 2 unsatisfiable draws (all found in 251 at S = −9) and 1 satisfiable control; C1 has no completed run.

== Against the projections

A1 peak RSS 25.8 GB against 44 GB projected (0.59×). C1 at degree 5 ran out of memory at 169 GB under the 184 GB cap, against a 217 GB projection: capped on this host. Unsatisfiable-draw rate at S = −9: 2/251.

#figure(
  image("closure-a1.svg", width: 100%),
  caption: [Run 20261018T1134Z, host `wfh2xovuq5aahv`. Left: columns, XL rank and closure rank at degree 4 per draw (log scale). Right: peak RSS and wall per run with the §2 projections and the 184 GB cap. Recorded values only (`plot.py`).],
)

== Missing

A2, B1, B2, C2 not run (deferred). C1: no completed run. A1: only two unsatisfiable draws exist. Raw records in `runs/`; generation log `draws/m4-15-6.log`.
