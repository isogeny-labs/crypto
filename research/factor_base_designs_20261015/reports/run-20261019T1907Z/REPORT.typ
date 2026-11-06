#set document(title: "Factor-base designs: the m = 3, t = 5 cell on p2 (T3)", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: the m = 3, t = 5 cell on p#sub[2] (run-20261019T1907Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only: nothing here prices a pipeline, reports S or shows a speedup. *Protocol:* PREREGISTRATION.md Amendment 7 (registered in commit #raw("bc31ae4a1") before any T3 row), with Amendments 1–6 for everything it does not change.

== Why this run

At m = 3, t = 5 on p#sub[1], A11 completes at D = 5 (F3) and A0 completes at D = 7 on the tower engine (T2): the largest A11-versus-A0 difference measured. §5 asks that a reading be replicated on a second prime before it is described as more than a single-prime result. Amendment 6 admitted the tower engine on p#sub[1] only, so p#sub[2] needs its own admission (T3a) before the A0 cell is measured (T3b).

== Run

- *Commands:* #raw("fb_designs_slope admit-tower research/factor_base_designs_20261015/runs --prime p2 --file T1-admission-p2") (T3a), then #raw("fb_designs_slope run --prime p2 --engine tower --arms A0 --m 3 --t-min 5 --t-max 5 --budget 3600 --grace 300 --vmem-kb 15000000 --file 'T3-{arm}-m{m}-p2'") and the same with #raw("--arms A11") on #raw("f4_fp") (no #raw("--engine")).
- *Commits measured:* T3a at #raw("bc31ae4a1b902f2a388b417c6e97aa4d845a5c1e"), T3b at #raw("8e945c6e0257da1ba10dfa18ecd74830b6695e4c"), recorded in each row.
- *Host:* "vm", Intel Xeon @ 2.10 GHz, 4 cores, 15 GB, no swap, a Claude Code cloud container. One system at a time.
- *Window:* 2026-10-19, 19:07Z to 19:36Z (UTC).
- *Limits:* F4 budget 3600 s, hard kill 3900 s, #raw("ulimit -v 15000000"). Seeds as in Amendment 2 with the p#sub[2] prime index. G2 (distinct targets) is off, as in F1.
- *Records:* #raw("runs/T1-admission-p2.jsonl"), #raw("runs/T3-A0-m3-p2.jsonl"), #raw("runs/T3-A11-m3-p2.jsonl"), their #raw(".log") files, and #raw("runs/analysis-tower.txt") (native #raw("analyze --tower")).

== T3a: admission on p#sub[2]

*Admitted.* All 22 completed planted A0 systems on p#sub[2] (F1: m = 2, t = 2–9 and m = 3, t = 2–4) complete on the tower engine with the committed #raw("f4_fp") D, without deriving 1, with the planted solution kept. The engine took 0.001 s to 6.6 s per system.

== T3b: the cell

#figure(image("m3_t5_p1_p2.svg", width: 100%),
  caption: [D against b for A0 and A11 at m = 3, on p#sub[1] (left) and p#sub[2] (right). Both targets per cell are plotted (n = 2; they coincide). The open square is A0's t = 5 cell, measured on the tower engine on each prime. Run run-20261019T1907Z; rows from #raw("runs/*.jsonl"), plotted by #raw("plot.py").])

#table(columns: 7, align: center, stroke: 0.4pt + gray,
  [*arm (engine)*], [*system*], [*D*], [*degree reached*], [*time (s)*], [*#raw("max_cols")*], [*status*],
  [A0 (tower engine)], [target 0], [7], [8], [368.6], [37 306], [ok],
  [A0 (tower engine)], [target 1], [7], [8], [370.4], [35 617], [ok],
  [A11 (#raw("f4_fp"))], [target 0], [5], [5], [486.3], [40 486], [ok],
  [A11 (#raw("f4_fp"))], [target 1], [5], [5], [498.5], [40 486], [ok],
)

== The Amendment 7 rule

"The gap replicates if both cells complete and each arm's cell D on p#sub[2] equals its p#sub[1] cell D (A0: 7, A11: 5)."

#table(columns: 4, align: center, stroke: 0.4pt + gray,
  [*arm*], [*p#sub[1] cell D*], [*p#sub[2] cell D*], [*equal?*],
  [A0 (tower engine)], [7 (T2)], [7], [yes],
  [A11 (#raw("f4_fp"))], [5 (F3)], [5], [yes],
)

*Reading: the gap replicates.* Both cells completed, and each arm's D is the same on both primes.

σ on p#sub[2] for m = 3, with the T3 cell as a fourth point (#raw("analysis-tower.txt")):

#table(columns: 5, align: (left, center, center, center, left), stroke: 0.4pt + gray,
  [*arm*], [*D over t = 2–5*], [*cells*], [*σ*], [*verdict (§1)*],
  [A0], [5, 6, 6, 7], [4], [0.6000], [no middle ground],
  [A11], [4, 4, 4, 5], [4], [0.3000], [weak],
)

Without the T3 cell, each series has 3 cells: A0 σ = 0.5000 (*no middle ground*) and A11 σ = 0.0000 (*candidate*). On p#sub[2], as on p#sub[1], the fourth cell moves A11's reading from *candidate* to *weak*.

== Predictions scorecard (Amendment 7)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*held?*],
  [T3a: admitted], [22 of 22 match], [yes],
  [both cells complete], [4 of 4 systems ok (about 370 s and 490 s)], [yes],
  [D = 7 (A0) and D = 5 (A11); the gap replicates], [7 and 5; replicates], [yes],
  [A0 σ = 0.6000 on 4 cells; A11 σ = 0.3000 on 4], [0.6000 and 0.3000], [yes],
)

== Observations

- The p#sub[1] and p#sub[2] panels are indistinguishable at every cell. This is the same pattern as the 44 earlier systems completed on both primes.
- A11's #raw("f4_fp") matrix width at the cell is 40 486 on both primes and both targets, to the column. The tower engine's width for A0 is not fixed in that way: 34 706 and 35 617 on p#sub[1], 37 306 and 35 617 on p#sub[2]. Only target 1 agrees across primes. That engine's width is not compared with #raw("f4_fp")'s, as the columns count different things. D agrees throughout.
- Replication here means the same two cells on a second prime, with the same construction. It adds a second prime, but not a different kind of instance.

== Limits

- As in T2, A0's D at this cell rests on the tower engine, admitted by agreement on smaller cells (22 on p#sub[2], 26 on p#sub[1]). #raw("f4_fp") has no completed system at this cell to compare with.
- Two primes, one cell, two planted targets per cell. Times are wall-clock on one container, and no timing comparison is made between the two engines.
- A11's two systems at this cell are not guaranteed distinct from each other (G2 is off). Their #raw("x_R") values differ, so they are two systems.

== Deviations

None from Amendment 7.
