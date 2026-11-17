#set document(title: "Factor-base designs: another prime, structured targets, m = 5", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: another prime, structured targets and m = 5 (run-20261020T1555Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only: nothing here prices a pipeline, reports S or shows a speedup. *Protocol:* PREREGISTRATION.md Amendment 9 (registered in commit #raw("ad791be5b") before any P4, Z1 or M5 row) and Amendment 10 (registered in commit #raw("e04733876") before the Z1 rows it adds), with Amendments 1–8 for everything they do not change.

== What this run takes from "not yet shown"

The Amendment 8 report listed four kinds of instance the study had not touched. This run measures three of them: *P4*, a prime of another 2-adic shape; *Z1*, structured targets; *M5*, m = 5. The fourth, other curve models (Edwards, binary-field curves), is left out, for the reasons registered in Amendment 9: each needs its own summation polynomial or field and solver, so it would not be measured by this study's instrument.

== Run

- *Commands:* P4: #raw("fb_designs_slope run --prime p4 --arms A0,A11 --m 2 --t-max 8 / --m 3 --t-max 4 --budget 300 --vmem-kb 13000000"). Z1: the same with #raw("--target-structure xr0") on p#sub[1], #raw("--m 2 --t-max 9") and #raw("--m 3 --t-max 4"). M5: #raw("run --arms A0 --m 5 --t-min 3 --t-max 3 --budget 300"). The sequence is #raw("runs/amendment9-sequence.sh").
- *Commits:* P4, Z1 (m = 3) and M5 at #raw("ad791be5b0f7bda1f000814656117f277a0bf73f"); Z1 (m = 2, t ≥ 3, Amendment 10) at #raw("e047338764dace77e3d91fa5397349ef6e37dd8d"), recorded in each row.
- *Host:* "vm", Intel Xeon @ 2.80 GHz, 4 cores, 15 GB, no swap, a Claude Code cloud container. One system at a time. Same limits as the first run: 300 s budget, 420 s hard kill, #raw("ulimit -v 13000000"), 2 planted targets per cell.
- *Window (UTC, 2026-10-20):* P4 15:55–15:58Z; Z1 15:58–15:59Z and, after Amendment 10, 16:10–16:18Z; M5 15:59–16:10Z.
- *p#sub[4]:* 10 007 297 = 2#super[8]·39 091 + 1, with p#sub[4] − 1 = 2#super[8]·13·31·97: 2-adic valuation 8 (p#sub[1], p#sub[2], p#sub[3] have 18, 27, 13), so t ≤ 8, and a smooth odd part.
- *Records:* #raw("runs/P4-*"), #raw("runs/Z1-*"), #raw("runs/M5-*") and #raw("runs/analysis-a9.txt") (native #raw("analyze --a9")).

== Graph

#figure(image("p4_z1.svg", width: 100%),
  caption: [Per (arm, m, t) cell shared with the committed first run (generic p#sub[1]): the cell's D minus the generic D (left) and its #raw("max_cols") against the generic cell's, in percent (right), for P4 (p#sub[4], generic targets) and Z1 (p#sub[1], x#sub[R] = 0 targets). 20 cells in each group. Both targets enter each cell mean. Run run-20261020T1555Z; rows from #raw("runs/*.jsonl"), plotted by #raw("plot.py").])

== P4: another prime

A0 and A11, m = 2 (t = 2–8) and m = 3 (t = 2–4): 40 systems, all completed. *Rule: prime-independent.* At all 20 shared cells the cell D equals the p#sub[1] D, and the m = 3 gap holds (A11 at least 1 below A0 at t = 2, 3, 4). The widest matrix equals p#sub[1]'s at every one of the 20 cells, to the column.

== Z1: structured targets, x#sub[R] = 0

Each target brings its own curve, built around its points (Amendment 9). All 40 curves differ, and every target has x#sub[R] = 0, checked by the group law.

*The construction failed at m = 2, t = 2.* The 4 systems there (A0 and A11, 2 targets each) aborted after 50 331 712 trials each, with no target found. At t = 2, V has 4 elements and 6 pairs, and for this V no pair has x#sub[1] + x#sub[2] a non-zero square, which the m = 2 construction needs. The pair does not depend on the random draw, so the search could not succeed. By the stop rule that ended the m = 2 series (Amendment 1 item 5). Amendment 10 re-ran m = 2 from t = 3, and records t = 2 as *not constructible for this V*. The 4 aborted rows stay committed.

With that, Z1 has 44 systems: 40 completed (m = 2, t = 3–9: 28; m = 3, t = 2–4: 12) and the 4 aborted. *Rule: target-independent.* At all 20 shared cells the cell D equals the generic D, and the m = 3 gap holds. The matrix widths are *not* equal to the generic ones:

#table(columns: 4, align: center, stroke: 0.4pt + gray,
  [*arm, m*], [*cells*], [*width against generic*], [*range*],
  [A0, m = 2], [7], [6 smaller, 1 larger], [−21.8% to +2.5%],
  [A0, m = 3], [3], [1 equal, 1 larger, 1 smaller], [−1.3% to +2.3%],
  [A11, m = 2], [7], [2 equal, 2 smaller, 3 larger], [−0.3% to +3.2%],
  [A11, m = 3], [3], [2 smaller, 1 larger], [−21.0% to +0.6%],
)

Over the 20 cells: 3 equal, 11 smaller, 6 larger, within −21.8% and +3.2%. This differs from the curve families of K1 (Amendment 8), where widths barely moved: a target with x#sub[R] = 0 removes terms from the summation polynomial, and the matrices change with it, in both directions.

== M5: m = 5

A0 at m = 5, t = 3, the smallest cell (t = 2 has 4 elements, fewer than the 5 summands): *both systems stop at the 300 s budget*, at partial D = 8 and a widest matrix of 117 005 columns so far (315 s and 313 s). The partial D is already 1 above A0's m = 4, t = 3 value (7). The cell does not complete, so there is no D and no σ at m = 5. A11 was not run (its form is written for m ≤ 4).

== Predictions scorecard (Amendment 9)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*held?*],
  [P4: prime-independent], [20 of 20 cells equal], [yes],
  [P4: the gap holds], [3 of 3], [yes],
  [P4: #raw("max_cols") equals p#sub[1]'s at every shared cell], [20 of 20], [yes],
  [Z1: target-independent], [20 of 20 cells equal], [yes],
  [Z1: the gap holds], [3 of 3], [yes],
  [Z1 (Amendment 10): every t ≥ 3 cell constructs and completes], [all 28 m = 2 systems and all 12 m = 3 systems], [yes],
  [M5: the cell does not complete in 300 s and 13 GB], [both systems stop at 300 s], [yes],
)

== Reading

- *Instances now compared.* D agrees with the first run's generic p#sub[1] cells across a fourth prime of another 2-adic shape, two further curve families, and structured targets, as well as the earlier second prime, the tower engine and the narrow path. In each case where a cell was tested, D was equal and the A11-below-A0 gap held. This is the same pattern as in every earlier comparison.
- *Widths are less rigid than D.* They were equal across primes (P4, as for p#sub[2]) and across a curve family with the generic support (`a = −3`), nearly equal for `a = 0`, and changed by up to 22% for structured targets, where the support itself changes. So "cost is set by the system's shape" is, in these rows, a statement about D first, and about width only when the support is the same.
- *m = 5* is out of reach for the dense engine here: the smallest cell was already past 100 000 columns at its budget, with D one above m = 4's.

== What this still does not show

- *Not measured:* Edwards and binary-field curves; other structured targets (an x#sub[R] other than 0, or R of small order); primes with 2-adic valuation below 8; m ≥ 5 beyond its smallest cell; A11 at m = 5.
- *The structured target* is one kind, and its curves are built per target, so a structured-target cell differs from its generic comparison in both the target and the curve. The result says D is unchanged by this combination, not which part would matter alone.
- *The t = 2, m = 2 cell* was not constructible for the V this seed draws. A different seed could construct it.
- Two planted targets per cell, one cell per (arm, m, t), wall-clock on one container.

== Deviations

The Z1 m = 2 series needed Amendment 10 (registered before its rows) after the t = 2 construction failure. No other deviation from Amendment 9.
