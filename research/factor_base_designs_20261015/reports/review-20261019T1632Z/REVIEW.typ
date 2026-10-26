#set document(title: "Factor-base designs: close-out review", author: "factor_base_designs")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: close-out of the prime-field slope line (review-20261019T1632Z)

*Kind:* review of committed results; no new run. Observations only. Nothing here prices a pipeline, reports S or shows a speedup. Every number is copied from #raw("runs/analysis-amendment4.txt") (Amendment 4, run-20261016T0616Z), and the graph is parsed from that file by #raw("plot_kappa.py") with no further computation. The protocol is #raw("PREREGISTRATION.md") Amendments 4 and 5.

== Requested scope and what was done

- *Close-out statuses* for every measured prime-field arm: README §2 (table), with κ and the cell count copied from the analysis file.
- *Amendment 7* appended to #raw("PREREGISTRATION.md"): κ is the primary score for later rounds, with the m − 1 break-even and Amendment 4's thresholds; σ is secondary. It changes no existing verdict.
- *Owed items:* the novelty audit (#raw("NOVELTY_AUDIT.md"), abstracts and snippets only, no full text read); the Mathlib bridges (not attempted: the cache did not fit the disk before, and it is shared); A0's third m = 4 cell (not run: needs more than 13 GB of memory or isolab credentials, which this container lacks).

== κ against the break-even, per arm

#figure(image("kappa_vs_breakeven.svg", width: 100%),
  caption: [κ = Δlog#sub[2] field_ops / Δb (least-squares slope over completed cells) against the break-even m − 1, one marker per arm and arity on p#sub[1]. Numbers beside markers are completed cells. Open markers have fewer than 3 cells. Small open markers are A0 and A11 on p#sub[2] (F1). The dashed line is κ = m − 1. Points at one m − 1 are offset horizontally for legibility. Source #raw("runs/analysis-amendment4.txt"), run run-20261016T0616Z, #raw("plot_kappa.py").])

#table(columns: 7, align: (left, center, center, center, center, center, left), stroke: 0.4pt + gray,
  [*arm*], [*m = 2: κ (cells)*], [*m = 3: κ (cells)*], [*m = 4: κ (cells)*], [*κ/(m − 1), m = 2*], [*m = 3*], [*status*],
  [A0], [3.593 (8)], [7.468 (3)], [10.773 (2)], [3.59], [3.73], [unfavourable at m = 2, 3; m = 4 inconclusive],
  [N], [3.596 (8)], [7.772 (3)], [not run], [3.60], [3.89], [unfavourable at m = 2, 3],
  [A1z], [6.050 (5)], [8.905 (3)], [not run], [6.05], [4.45], [unfavourable at m = 2, 3],
  [A1g], [5.809 (5)], [9.189 (3)], [not run], [5.81], [4.59], [unfavourable at m = 2, 3],
  [A4], [4.902 (6)], [8.028 (3)], [not run], [4.90], [4.01], [unfavourable at m = 2, 3],
  [A11], [3.995 (8)], [8.769 (4)], [14.886 (3)], [3.995], [4.38], [unfavourable at m = 2, 3, 4],
  [N11], [4.014 (8)], [8.582 (3)], [not run], [4.01], [4.29], [unfavourable at m = 2, 3],
  [H1], [3.535 (8)], [7.660 (3)], [not run], [3.54], [3.83], [unfavourable at m = 2, 3],
  [H2], [3.923 (8)], [9.158 (3)], [not run], [3.92], [4.58], [unfavourable at m = 2, 3],
)

The κ/(m − 1) columns are quotients of the two committed numbers (derived, each recomputed here). At m = 4: A0 10.773 / 3 = 3.59 (2 cells, inconclusive) and A11 14.886 / 3 = 4.96 (3 cells).

Observations:
- Every series with at least 3 completed cells has κ ≥ m − 1 (23 series; Amendment 4's 22 at m = 2, 3, plus A11 at m = 4). The smallest κ/(m − 1) among them is 3.54 (H1, m = 2) and the largest is 6.05 (A1z, m = 2).
- A0: κ/(m − 1) = 3.59, 3.73, 3.59 at m = 2, 3, 4 on 8, 3 and 2 cells. A11: 3.995, 4.38, 4.96 on 8, 4 and 3 cells. This is an observation on few cells, one prime for the m = 4 points, with no interval recorded. *Nothing is extrapolated* to m > 4, larger t or deployed sizes.
- The p#sub[2] κ for A0 and A11 (3.595 and 3.995 at m = 2; 7.468 and 8.742 at m = 3) is within 0.03 of p#sub[1].

== Why κ replaces σ as the primary score (Amendment 7)

- A1z and A1g sit at D = 8 in every cell (σ = 0.0000, *candidate*), yet their κ (6.050, 5.809 at m = 2; 8.905, 9.189 at m = 3) is above A0's (3.593; 7.468). At m = 3, t = 4 their max_cols are 39 648 and 41 629 against A0's 12 314.
- A11 has σ = 0.1786 at m = 2 and 0.3000 at m = 3 on p#sub[1] (0.0000 on p#sub[2]), yet κ = 3.995, 8.769 and 14.886 at m = 2, 3, 4, above A0's at m = 2 and 3.

== Limits

- A0 at m = 4 has 2 completed cells; its t = 4 cell exhausts the 13 GB memory limit (C2, 512.8 s and 536.9 s). The third cell is owed and blocked: it also exhausted memory at 14.5 GB in L1 (Amendment 5, on main).
- Only A0 and A11 were run at m = 4, so no other arm has a status at m = 4.
- Verdicts are read at the sizes measured. The Lean break-even is a model statement. The Mathlib bridges behind K7 and K8 remain owed.
- #raw("NOVELTY_AUDIT.md") rests on abstracts, slides and search snippets. Statuses: A11, A4, H1/H2 partly prior; A1 not found in the sources checked.

== Records

#raw("runs/analysis-amendment4.txt"), #raw("runs/C1-rescore.jsonl"), #raw("runs/C2-*"), #raw("runs/C4-*"), run report #raw("reports/run-20261016T0616Z/REPORT.pdf"). Graph checked and left unchanged: the run report's #raw("ops_vs_b.svg") and #raw("ablation.svg") (no new data). Commit of this review: see the git history of this directory.
