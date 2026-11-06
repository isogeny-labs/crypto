#set document(title: "Factor-base designs: Edwards curves and the A0 m = 4, t = 4 cell", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: Edwards curves (E1) and the A0 m = 4, t = 4 cell (T5) (run-20261020T1636Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only: nothing here prices a pipeline, reports S or shows a speedup. *Protocol:* PREREGISTRATION.md Amendment 11 (registered in commit #raw("111908a13b") before any E1 or T5 row), with Amendments 1–10 for everything it does not change.

== Run

- *E1:* #raw("fb_designs_slope run --model edwards --arms A0 --m 2 --t-min 2 --t-max 9 / --m 3 --t-min 2 --t-max 4 --budget 300 --vmem-kb 13000000"). Twisted Edwards curves #raw("a x² + y² = 1 + d x² y²") on p#sub[1] = 786 433, summation polynomial in the y-coordinate (derived and checked in #raw("runs/edwards-s3-check.txt")), dense #raw("f4_fp"). 22 systems, all completed. 2026-10-20 16:34–16:36Z.
- *T5:* #raw("run --engine tower --arms A0 --m 4 --t-min 4 --t-max 4 --targets 1 --budget 21600 --grace 600 --vmem-kb 15000000"), target 0, then target 1 (#raw("--first-target 1")) because target 0 completed. Tower engine, Weierstrass, p#sub[1]. Windows: target 0 16:36–17:55Z, target 1 17:55–19:14Z. Host "vm", Xeon @ 2.80 GHz, 4 cores, 15 GB, one system at a time. Peak resident set about 7.5 GB.
- *Records:* #raw("runs/E1-*"), #raw("runs/T5-*"), #raw("runs/analysis-a11.txt") (native #raw("analyze --a11")).

== Graph

#figure(image("e1.svg", width: 100%),
  caption: [E1: per (m, t) cell shared with the committed A0 Weierstrass rows, the Edwards cell's D minus the Weierstrass D (left) and its #raw("max_cols") against Weierstrass, in percent (right). 11 cells. Both targets enter each cell mean. Rows from #raw("runs/*.jsonl"), plotted by #raw("plot.py").])

== E1: twisted Edwards curves

Edwards A0, m = 2, t = 2–9: σ = 0.220 (candidate band, 8 cells). m = 3, t = 2–4: D = 6 at all three cells, σ = 0. Against the Weierstrass rows, D is equal at 8 of the 11 shared cells, higher at 1 (m = 3, t = 2: 6 against 5) and lower at 2 (m = 2, t = 2: mean 3.5 against 4, where the two Edwards targets gave 3 and 4; m = 2, t = 6: 4 against 5). *Rule (Amendment 11): model-dependent*, because a shared D differs. The differences are one unit, go both ways, and sit at small cells; the large m = 2 cells (t = 7–9) agree at D = 5. Matrix widths differ both ways, from −52% (m = 2, t = 6) to +15% (m = 2, t = 9). Each cell has two targets, and at m = 2, t = 2 the two Edwards targets themselves differ by one unit.

== T5: A0 at m = 4, t = 4

*Both targets completed*, with identical D:

#table(columns: 6, align: center, stroke: 0.4pt + gray,
  [*target*], [*D*], [*degree reached*], [*max_cols*], [*time (s)*], [*status*],
  [0], [7], [8], [170 363], [4730], [ok],
  [1], [7], [8], [170 036], [4721], [ok])

Both systems are consistent with the planted point (#raw("planted_ok")). With the F4 cells at t = 2 (D = 6) and t = 3 (D = 7), A0 at m = 4 now has three completed cells, D = 6, 7, 7, and σ = 0.50 by least squares. By PREREGISTRATION §1 that reads *no middle ground at this arity (σ ≥ 0.5)*. Two things limit this: three cells is the minimum for a verdict, and the t = 4 value is one unit above the m = 3 value at the same t (6), as the earlier lower bound D ≥ 7 required. The widest matrix grows about 8× per unit of t at m = 4 (21 619 → 170 363 columns); a t = 5 cell was not attempted.

== Predictions scorecard (Amendment 11)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*held?*],
  [E1: model-dependent], [3 of 11 shared D differ], [yes],
  [E1: Edwards D ≥ Weierstrass D at every shared cell], [2 of 11 cells lower], [*no*],
  [T5: target 0 does not complete in 6 h / 15 GB], [completed in 4730 s, about 7.5 GB], [*no*],
)

== Reading

- *The open A0 m = 4, t = 4 cell is closed*, on the tower engine and a single container: D = 7 on both targets. The dense engine ran out of memory at this cell at 13 and 15 GB; the tower engine finished in about 79 minutes with a peak resident set near 7.5 GB. The earlier tower-engine time-out was at a 3600 s budget.
- *The m = 4 slope is not in the candidate band*: 0.50 over three cells (it was 1.0 over two). No other arm's verdict changes.
- *Edwards*: the algebraic model changes D by at most one unit at small cells and not at the large ones. Not a statement about the cost of a full attack: only A0 at m ≤ 3 was run, with no A11 analogue.

== What this still does not show

- Binary-field curves; an Edwards form of A11; Edwards at m = 4; m ≥ 5 beyond the smallest cell; any t = 5 cell at m = 4.
- T5 has two targets, one cell, one container, and time is wall-clock at 4 cores.
- The identical D = 7 on two targets is consistent with, not proof of, instance-independence at this cell.

== Deviations

Two Amendment 11 predictions failed (table above); no deviation from the registered procedure.
