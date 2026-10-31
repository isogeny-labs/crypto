#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 K = 16,627: post-root SAT search progress

*Run:* 2026-10-18 06:56:55–07:26:55 UTC. *Status:* `UNKNOWN_wall_cap`.
The exact retained signed-Frobenius base has 16,627 columns and 2,760,082
points on `icv1-f2m83-tm6151469093347-debefd74`. Source commit
`2e1ba2e4763242bb4c8870e22f93b0183c7b30f7`, six-file and input
SHA-256 manifests, frozen protocol, build receipt and exact commands are
retained beside the complete `RUN_REPORT.md`.

#image("search-progress.svg", width: 100%)

== Frozen positive arm

#table(
  columns: (auto, auto), inset: 5pt,
  [Measure], [Observed value],
  [Resource envelope], [one CPU; hard 4 GiB; zero swap; 1,800 s cap],
  [Full m=4 model], [201,598 variables; 5,915,238 clauses; 3,901 XOR rows],
  [Root propagation], [`Fixpoint` in 199.977792 ms; zero decisions/conflicts],
  [SAT entry], [39.623697 s process elapsed],
  [Progress], [176 fsynced samples; all 175 periodic phase samples in XOR propagation],
  [Last complete sample], [1,792.112144 s; 322,255 decisions, 360 conflicts, 2 restarts],
  [Outer result], [exit 124; no OOM; 2,662,309,888-byte peak],
)

== Interpretation

Decisions increased after root, so this worker repeatedly entered solver
search. Periodic phase snapshots consistently landed in XOR propagation;
they do not establish an exact CPU-time fraction or identify the costly
inner XOR operation. Work after the last sample, SAT result, returned model,
checked group lift and returned-relation rank remain unknown. The dependent
unpinned arm was not attempted. A pre-SAT rank-one group check is not a
natural-query useful-rank measurement. No m=2, m=3 or m=4 base is selected,
and no complete cold index-calculus cost or matched-rho ratio is reported.

*Raw source:* `positive/worker.json.progress.jsonl`,
`positive/worker.json.phases.jsonl`, `positive/container-state.json`,
`positive/outcome.json` and `positive-files.sha256`. The next frozen
experiment should observe completed XOR-pass timing and row work before
testing a solver policy change.
