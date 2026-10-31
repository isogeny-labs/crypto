#set document(title: "Factor-base designs: T4, K1 and X1", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: a stop on the tower engine, curve families, and a third prime (run-20261020T0550Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only: nothing here prices a pipeline, reports S or shows a speedup. *Protocol:* PREREGISTRATION.md Amendment 8 (registered in commit #raw("592d47257") before any T4, K1 or X1 row), with Amendments 1–7 for everything it does not change.

== What was left, and what was run

Two items were open after T3. (1) A0 at m = 4, t = 4 has no completed cell: #raw("f4_fp") runs out of memory, and the tower engine stops at 3600 s with partial D = 7. A0's m = 4 σ prediction is undecided. (2) Every instance so far is a random short-Weierstrass curve on one of two primes, on the dense #raw("f4_fp") path or the tower engine. Amendment 8 registered three tests: *T4*, a staircase stop on the tower engine for A0 at m = 4; *K1*, A0 and A11 on two curve families; and *X1*, a third prime on #raw("f4_fp")'s narrow path.

== Run

- *Commands:* K1: #raw("fb_designs_slope run --family {a0,am3} --arms A0,A11 --m 2 --t-max 9 / --m 3 --t-max 4 --budget 300 --vmem-kb 13000000"). T4a: #raw("admit-tower --stop-below 64"). T4b: #raw("run --engine tower --stop-below 64 --arms A0 --m 4 --t-min 4 --t-max 4 --budget 3600 --grace 300 --vmem-kb 15000000"). X1: #raw("run --prime p3 --arms … --budget 3600 --grace 300 --vmem-kb 15000000"). The sequence ran from one script, committed as #raw("runs/amendment8-sequence.sh"). Each row records the effect of its flags (curve family, #raw("stop_below"), prime).
- *Binary:* built from the harness at #raw("592d47257"), the Amendment 8 registration. Later source changes added analysis code only. Each row records the commit at its phase's start.
- *Host:* "vm", Intel Xeon @ 2.80 GHz, 4 cores, 15 GB, no swap, a Claude Code cloud container. This is a different container from the T1–T3 runs (2.10 GHz), so times are not comparable with theirs. One system at a time.
- *Window (UTC, 2026-10-20):* K1 05:50–06:06Z; T4a 06:06–06:07Z; X1 A0 m = 3, t = 5 06:07–06:37Z; T4b 06:37–08:37Z; X1 A11 m = 4 08:37–08:54Z; X1 A0 m = 4 08:54–09:05Z.
- *Records:* #raw("runs/K1-*"), #raw("runs/T4-admission.*"), #raw("runs/T4-A0-m4.*"), #raw("runs/X1-*"), #raw("runs/X1-oom-kernel-log.txt") and #raw("runs/analysis-a8.txt") (native #raw("analyze --a8")).

== Graph

#figure(image("k1_t4a.svg", width: 100%),
  caption: [Left: K1, each cell's #raw("max_cols") on the special curve family against the generic cell's, in percent, by the generic cell's size (22 shared cells per family). Points are spread sideways by family and arm. Right: T4a, the tower engine's stop-bounded D minus the committed #raw("f4_fp") D, one point per system (26). Run run-20261020T0550Z; rows from #raw("runs/*.jsonl"), plotted by #raw("plot.py").])

== K1: the curve family

A0 and A11 on p#sub[1], #raw("f4_fp"), m = 2 (t = 2–9) and m = 3 (t = 2–4): 88 systems, all completed. The reference is the first run's generic-family rows. The seeds differ, so towers and curves differ, and cells are compared by (arm, m, t). `a0` is `a = 0` (j = 0). `am3` is `a = −3`, the P-256 shape. `a0` changes the summation polynomial's support. `am3` does not.

#table(columns: 5, align: (left, center, center, center, left), stroke: 0.4pt + gray,
  [*family*], [*shared cells*], [*D equals generic*], [*gap at m = 3*], [*#raw("max_cols") against generic*],
  [`a0`], [22], [22 of 22], [holds (3 of 3)], [18 equal, 3 smaller, 1 larger],
  [`am3`], [22], [22 of 22], [holds (3 of 3)], [22 equal],
)

*Rule: both families are curve-independent.* Every shared cell has the generic D, and A11's D is at least 1 below A0's at every m = 3 cell. Widths in `a0`: A0 at m = 3 is smaller at t = 2–4 (363 against 368, 3268 against 3285, 12 266 against 12 314: down 1.4%, 0.5% and 0.4%), and A11 at m = 2, t = 8 is larger (8765 against 8763, up 0.02%). The other 18 are equal. `am3` matches the generic width at every cell.

== T4: a staircase stop on the tower engine

*T4a, admission: admitted on its gate.* The gate is the 4 completed planted A0 systems at m = 4 (t = 2, 3). All four complete with the committed D (6, 6, 7, 7), without deriving 1, with the planted solution kept. The other systems were measured and reported, not gating:

#table(columns: 4, align: center, stroke: 0.4pt + gray,
  [*m*], [*systems*], [*equal to committed D*], [*lower under the stop*],
  [4 (gating)], [4], [4], [0],
  [3], [6], [6], [0],
  [2], [16], [12], [4 (D = 3 against 4 at t = 2, 3; staircase 10 and 40)]
)

So the stop's D is not in general the certified D. At m = 2 a planted system has few solutions, k = 64 is reached early, and the stop-bounded D undercounts. The admission rests on the m = 4 gate, where a planted system has at least 24 solutions and the threshold is tight, and on 6 of 6 agreeing systems at m = 3.

*T4b, the cell: the stop never fired.* A0, m = 4, t = 4, tower engine, k = 64: both systems ran the full 3600 s budget, reached degree 7 (partial D = 7) at a widest matrix of 76 837 columns, and never reached a staircase of at most 64 standard monomials (#raw("staircase_at_stop") is null). So there is no stop-bounded cell, and A0's m = 4 series stays at 2 completed cells.

== X1: a third prime on the narrow path

p#sub[3] = 40961 is below 2#super[16], so #raw("f4_fp") takes its 32-bit path with half the memory per entry. It was admitted for A0 and A11 only, on C3's committed trace match (26 A0 and 30 A11 systems, all matching).

#table(columns: 6, align: center, stroke: 0.4pt + gray,
  [*cell*], [*D*], [*#raw("max_cols")*], [*time (s)*], [*status*], [*p#sub[1] reading*],
  [A0, m = 3, t = 5], [–], [–], [908, 911], [OOM-killed, both], [D = 7 (tower engine, T2, T3)],
  [A0, m = 4, t = 4], [–], [–], [327, 331], [allocation abort, both], [aborts (G1, C2)],
  [A11, m = 4, t = 4], [6, 6], [88 740, 88 740], [514, 499], [ok, ok], [D = 6, 88 740 (G1, C2)],
)

- *A11 at m = 4, t = 4* completes on p#sub[3]'s narrow path with the same D and the same widest matrix as on p#sub[1], to the column.
- *A0 at m = 3, t = 5:* both systems were killed by the host's memory cgroup at about 14 GB resident (#raw("runs/X1-oom-kernel-log.txt")), below the 15 GB address-space limit. The cross-engine check of A0's D = 7 (tower engine against #raw("f4_fp")) is therefore *inconclusive*.
- *A0 at m = 4, t = 4:* both systems aborted on an allocation failure under the 15 GB limit, as they did on p#sub[1] (at about 405 s). The narrow path's smaller entries did not make either A0 cell fit on this host.

== A0's m = 4 σ

Neither route produced a completed A0 cell at m = 4, t = 4 (T4b: no stop in 3600 s; X1: memory), so A0's m = 4 series is still D = 6, 7 over t = 2, 3, σ = 1.0000 on 2 cells, *inconclusive*, and its registered prediction "σ ≥ 0.5" is *not decided*. The bound registered in Amendment 8 still holds: a completed t = 4 cell has D ≥ 7 (both systems have already reached degree 7), so the three-point slope is at least (7 − 6)/2 = 0.5. That is a consequence of a lower bound on an unfinished system, not a measurement, and no verdict is read from it.

== Predictions scorecard (Amendment 8)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*held?*],
  [T4a: admitted], [4 of 4 at m = 4], [yes],
  [T4b: if a cell completes, D = 7], [no cell completed], [not applicable],
  [K1: both families curve-independent; the gap holds], [22 of 22 cells equal in each; gap holds in both], [yes],
  [K1: `am3` #raw("max_cols") equals generic at every shared cell], [22 of 22], [yes],
  [K1: `a0` #raw("max_cols") at most generic at every shared cell], [21 of 22; A11 at m = 2, t = 8 is larger by 2 columns], [*no*],
  [X1: A0 at m = 3, t = 5 completes with D = 7 (confirms)], [OOM-killed, both systems], [*no*],
  [X1: A0 at m = 4, t = 4 does not complete], [aborted, both systems], [yes],
  [X1: A11 at m = 4, t = 4 completes with D = 6], [D = 6, both systems], [yes],
)

== Reading

- *The instance.* Across K1 the solving degree did not depend on the curve family at any of 44 compared cells (22 per family), and the A11-below-A0 gap held in both. Widths were equal for `am3` (the generic support) and nearly so for `a0`, whose smaller support gave smaller matrices for A0 at m = 3 and one slightly larger cell for A11. Together with T3 (a second prime) and X1's A11 cell (the narrow path: same D, same width), D has now agreed across two primes, two further curve families and two code paths for the cells where each was tested. This is the pattern "cost is set by the system's shape", not a proof of it.
- *What it does not show.* Both families are short-Weierstrass curves with random `b`. Other models (Edwards, binary-field curves) and other targets (a structured, not random, `R`) are untested here, as are m ≥ 5 and primes of a different 2-adic shape.
- *The stop.* The stop's D matched the certified D wherever the threshold was tight (m = 3 and m = 4 here, 10 of 10 systems), and at m = 4, t = 4 the stop was not reached in an hour. The m = 2 undercount is a reminder that a stop-bounded D is a different quantity from a certified D.

== Limits and deviations

- The T4a log's last line reads "26 of 26 systems match". That line counts only the failures that gate the verdict. The per-arity counts above are from the rows.
- Two curve families, one prime for K1, and the generic reference comes from different seeds, so cell-by-cell equality of D is evidence of shape-dependence, not an identity.
- X1 used the registered admission on C3's committed trace match. No A1g row is read from p#sub[3].
- The A0 memory kills and aborts are host limits (a 15 GB container with a memory cgroup). A larger host could complete those cells. No result here says they cannot be completed.
- No deviations from Amendment 8.
