#set document(title: "E-K1: Gauss-period window in s = x + 1/x, run report", author: "fb_koblitz_ek1")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= E-K1: a Gauss-period window in s = x + 1/x on Koblitz analogues (run-20261019T1712Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only. Nothing here prices a pipeline, reports S or shows a speedup. The protocol is #raw("PREREGISTRATION.md") Amendment 6 (with its erratum), registered before any row. Amendment 6 follows the rule of Amendment 5 (κ is the primary score): κ = Δlog#sub[2] field_ops / Δb against the break-even m − 1.

== Run

- *Commands* (#raw("B=fb_koblitz_ek1"), #raw("R=research/factor_base_designs_20261015/runs"), one series at a time under #raw("flock /tmp/claude-0/measure.lock"), each series followed by its replay): \
  #raw("$B check --n 11 --ell-max 8; $B check --n 23 --ell-max 8") (log #raw("runs/K1-check.txt")) \
  #raw("$B run --arms A --n N --m M --out-dir $R") for A in S, X, XR, U, NS (and PC at M = 3), N in 11, 23, M in 2, 3 \
  #raw("$B replay $R --out K1-replay.jsonl --arm A --n N --m M") after each series \
  #raw("$B analyze $R --replay K1-replay.jsonl > $R/analysis-amendment6.txt")
- *Instrument:* #raw("pq_f4_f2::groebner_basis_f4") (the repository's Boolean F4 over F#sub[2]\[v\]/(v² − v), Gebauer–Möller pairs and field pairs) on the Weil restriction of every system. D = #raw("solving_degree"), field_ops = #raw("word_xors") (64-bit XORs of the row reductions), max_cols = #raw("matrix_cols_max"), max_rows = #raw("matrix_rows_max"). The same engine, 300 s budget, 420 s hard kill and #raw("ulimit -v 13000000") for every arm.
- *Binary:* built at #raw("b733807e0") (every row records it). The analysis marks for P6–P8 were added afterwards (#raw("6e1b329ce")); no rule changed.
- *Host:* "vm", Intel Xeon @ 2.80 GHz, 4 cores, 15 GB RAM, a Claude Code cloud container shared with other agents, serialised by the lock; no further isolation. *Window:* 2026-10-19, 17:12Z to 20:28Z.
- *Field and curve:* K#sub[0] over F#sub[2#super[n]], n = 11 and 23, #raw("find_irreducible_sparse(n)") modulus; ζ of order 2n + 1 = 23, 47; Gauss periods c#sub[i] = ζ#super[i] + ζ#super[−i]. #raw("KoblitzCurve::new") rejects K#sub[0] at n = 11 (#raw("#E = 4·23²")), so the curve is built directly; #raw("#E") = 2116 and 8 383 412 was confirmed by counting every abscissa.
- *Records:* #raw("runs/K1-{S,X,XR,U,NS}-n{11,23}-m{2,3}.jsonl"), #raw("runs/K1-PC-n{11,23}-m3.jsonl"), each with its #raw(".log"); #raw("runs/K1-replay.jsonl"); #raw("runs/K1-check.txt"); #raw("runs/analysis-amendment6.txt"). 306 rows, 22 of them #raw("no_plant") placeholders (see below).
- *Seeds:* #raw("0x454B31 ⊕ n≪48 ⊕ arm≪40 ⊕ m≪32 ⊕ ℓ≪16"); target k from #raw("cell ⊕ (k+1)≪8"). S, X, XR, U, NS, PC = arm 0–5.

== Exact facts checked before any measurement (class: exact computation)

- *Identity.* Eliminating σ from the repository's symmetrised polynomial (w = u² + u, s = 1/w) gives a polynomial G with G(1/s)·(Π s)#super[k] = Semaev's S#sub[m+1] of #raw("plain_terms(m)") (b = 1), on a 6-per-variable grid (degree ≤ 4 per variable, so exact) at n = 11 and 23, m = 2 and 3. So *S and X have identical equations*; they differ in the target value (s#sub[R] against x#sub[R]) and in which curve points the window contains.
- Every set (independent basis, exact point count against #raw("points_with_x") where cheap), every planted point (on the curve, coordinate in the window, sum = R), the word-field value of every polynomial at the planted point and the Boolean equations at the planted bit-vector, were checked inside each system's build. No check failed.

== Series

Cells are completed when both targets are ok. κ and σ are least-squares slopes of log#sub[2] cell field_ops and of cell D on b = log#sub[2] of the exact factor-base point count. "Limit" is the first cell where a system hit the budget (timeout) or a Macaulay matrix would exceed 1 GiB (oversize); the series stops there.

#table(columns: 8, align: (left, center, center, center, left, center, center, left), stroke: 0.4pt + gray,
  [*arm*], [*n*], [*m*], [*cells (ℓ)*], [*limit cell*], [*κ*], [*σ*], [*κ verdict (m − 1)*],
  [S], [11], [2], [9 (3–11)], [none (ℓ = n)], [3.932], [0.403], [unfavourable (1)],
  [X], [11], [2], [9 (3–11)], [none], [3.581], [0.384], [unfavourable (1)],
  [XR], [11], [2], [9 (3–11)], [none], [3.783], [0.397], [unfavourable (1)],
  [U], [11], [2], [9 (3–11)], [none], [3.642], [0.352], [unfavourable (1)],
  [NS], [11], [2], [9 (3–11)], [none], [3.849], [0.438], [unfavourable (1)],
  [S], [23], [2], [11 (3–13)], [ℓ = 14 oversize], [2.791], [0.226], [unfavourable (1)],
  [X], [23], [2], [11 (3–13)], [ℓ = 14 oversize], [2.424], [0.199], [unfavourable (1)],
  [XR], [23], [2], [11 (3–13)], [ℓ = 14 oversize], [2.849], [0.247], [unfavourable (1)],
  [U], [23], [2], [10 (4–13)], [ℓ = 14 oversize], [3.077], [0.279], [unfavourable (1)],
  [NS], [23], [2], [11 (3–13)], [ℓ = 14 oversize], [2.750], [0.467], [unfavourable (1)],
  [S], [11], [3], [2 (4–5)], [ℓ = 6 timeout], [7.331], [-], [inconclusive (2)],
  [X], [11], [3], [2 (4–5)], [ℓ = 6 timeout], [6.957], [-], [inconclusive (2)],
  [XR], [11], [3], [3 (3–5)], [ℓ = 6 timeout], [9.397], [0.000], [unfavourable (2)],
  [U], [11], [3], [3 (4–6)], [ℓ = 7 oversize], [10.036], [0.770], [unfavourable (2)],
  [NS], [11], [3], [2 (4–5)], [ℓ = 6 timeout], [7.072], [-], [inconclusive (2)],
  [PC], [11], [3], [2 (4–5)], [ℓ = 6 oversize], [3.694], [-], [inconclusive (2)],
  [S], [23], [3], [2 (4–5)], [ℓ = 6 oversize], [5.237], [-], [inconclusive (2)],
  [X], [23], [3], [2 (4–5)], [ℓ = 6 oversize], [3.778], [-], [inconclusive (2)],
  [XR], [23], [3], [3 (3–5)], [ℓ = 6 timeout], [18.707], [1.827], [unfavourable (2)],
  [U], [23], [3], [3 (4–6)], [ℓ = 7 oversize], [6.148], [0.000], [unfavourable (2)],
  [NS], [23], [3], [2 (4–5)], [ℓ = 6 timeout], [5.007], [-], [inconclusive (2)],
  [PC], [23], [3], [0], [ℓ = 4 oversize], [-], [-], [inconclusive (2)],
)

σ is shown only where at least 3 cells completed. #raw("no_plant") rows: at m = 3, ℓ = 3 holds no planted decomposition with 3 distinct summands for S, X, U, NS and PC at both n (XR's random subspace did hold one), and U at n = 23, m = 2, ℓ = 3 holds none; those cells are skipped, not limits. XR and U draw an independent random window for every ℓ, so their b is not monotone in ℓ: XR at n = 23, m = 3 has |F| = 19 at ℓ = 4 and at ℓ = 5, which makes its κ (18.707) a statement about b, not about cost.

#figure(image("ek1-field-ops-vs-b.svg", width: 100%),
  caption: [log#sub[2] field_ops (cell mean of 2 targets) against b for completed cells, per arm; dotted line: break-even slope m − 1 through the first S cell; shaded vertical lines: limit cells (their rows are in the records). Rows: runs/K1-\*.jsonl, run-20261019T1712Z.])

#figure(image("ek1-D-vs-b.svg", width: 100%),
  caption: [Solving degree D (cell mean of 2 targets) against b, same cells. At m = 3 the direct arms have input degree 6 and D = 7 at every completed cell; PC (chained, input degree 3) has D = 4; U (input degree 4) has D = 5–6.])

== Decision rules (analysis-amendment6.txt)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*rule*], [*n = 11, m = 2*], [*n = 23, m = 2*],
  [R1 κ verdicts], [all 5 arms unfavourable (κ 3.58–3.93)], [all 5 unfavourable (κ 2.42–3.08)],
  [R2 S vs X], [9 shared cells; |ΔD| ≤ 1 at every cell; κ 3.932 vs 3.581 → *differs* (|Δκ| = 0.35 ≥ 0.25)], [11 shared cells; |ΔD| ≤ 1 at every cell; κ 2.791 vs 2.424 → *differs* (|Δκ| = 0.37)],
  [R3 S vs XR], [κ 3.932 vs 3.783 → tied; XR ≥ 2× S's field_ops in 1 of 9 shared cells, S ≥ 2× XR's in 0], [κ 2.791 vs 2.849 → tied; XR ≥ 2× S's in 6 of 11, S ≥ 2× XR's in 0],
  [R3 S vs U], [κ 3.932 vs 3.642 → S higher; U ≥ 2× cheaper in 6 of 9, S in 0], [κ 2.791 vs 3.077 → S lower; S ≥ 2× cheaper in 3 of 10, U in 1],
  [R4 outcome], [*no advance* (S cost-unfavourable)], [*no advance* (S cost-unfavourable)],
  [R5 null NS vs S], [*mixed*: D equal in 7 of 9 cells; differs at ℓ = 3 (NS 1.0, S 2.0) and ℓ = 4 (NS 2.5, S 2.0)], [*mixed*: D equal in 6 of 11; NS lower in 4 (ℓ = 3–6, D 0, 0, 0, 1 against 2), higher in 1 (ℓ = 8, 3 against 2); NS ≥ 2× S's field_ops in 1 of 11],
  [R7 Gauss vs random x-window], [XR ≥ 2× X's in 1 of 9, ≤ ½ in 0], [XR ≥ 2× X's in 6 of 11, ≤ ½ in 0],
)

At m = 3 every R2–R5 comparison of S has only 2 shared completed cells and is *inconclusive* by the rule (fewer than 3); R4 is inconclusive at both n.

*R6, positive control.* PC1 (random x-window with direct S#sub[4], degree 6, against the u-frame, degree 4, at m = 3, same instrument): 4 shared cells (n = 11 ℓ = 4, 5; n = 23 ℓ = 4, 5), XR's field_ops 255×, 390×, 251× and 132× U's, D = 7 against 5 in all four. *Detected* (≥ 2× in ≥ 2 cells), so the instrument is not blind to this difference. PC2 (descriptive): X chained against X direct, same window and planted points, 2 shared cells (n = 11 ℓ = 4, 5): D = 4 against 7; chained field_ops 3.17× (ℓ = 4) and 0.22× (ℓ = 5) the direct arm's. At n = 23 PC has no completed cell (ℓ = 4 oversize).

*Trace replay.* 250 of 250 completed systems, re-measured once from their seed, reproduced D, max_cols, max_rows and steps; field_ops was also equal in all 250. Every system is therefore counted in the table above.

== Predictions scorecard

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction (Amendment 6 §9)*], [*result*], [*reading*],
  [P1 identity of the s-line polynomial], [held], [exact, checked on the grid],
  [P2 S tied with X in every (n, m)], [*failed*], [D within ±1 everywhere, but |Δκ| = 0.35, 0.37 at m = 2 (m = 3 not evaluable)],
  [P3 no arm cost-favourable], [held], [14 series with ≥ 3 cells, all unfavourable],
  [P4 PC1 detects, XR dearer than U], [held], [4 of 4 cells, ratio 132–390×],
  [P5 S does not advance], [held], [no advance at m = 2 for both n; inconclusive at m = 3],
  [P6 null reads "structure beyond shape"], [*failed*], [*mixed* at m = 2 for both n; not evaluable at m = 3],
  [P7 Gauss x-window cheaper than random in ≥ half of shared cells], [*failed*], [held at n = 23, m = 2 (6 of 11); 1 of 9 at n = 11; 0 of 2 at m = 3],
  [P8 S, X, XR, NS have < 3 cells at m = 3, n = 11], [*failed*], [XR has 3 (random ℓ = 3 window was plantable; erratum)],
)

== Observations (descriptive; no claim beyond these rows)

- Every series with at least 3 completed cells is cost-unfavourable: 14 of 14 (10 at m = 2, κ 2.42–3.93 against 1; 4 at m = 3, κ 6.15–18.71 against 2). S's κ is 3.93 (n = 11) and 2.79 (n = 23) at m = 2.
- At m = 2, D rises from 2 to 5 (n = 11) or 4 (n = 23) over the cells of S, X, XR and U, with σ between 0.20 and 0.47 for the five arms.
- S and X have identical equations and D within ±1 at every shared cell, yet their κ differ by 0.35–0.37, more than the 0.25 margin of R3. At these sizes the spread of κ between arms with identical equations exceeds the registered margin (a possible reason: b is a count of curve points and is noisy at small ℓ; not tested here).
- Against the u-frame, S's κ is higher at n = 11 and lower at n = 23; the level at equal ℓ is mostly in U's favour at n = 11 (6 of 9 cells ≥ 2×) and mostly tied at n = 23.
- The Gauss x-window is cheaper than the random x-window at equal ℓ in the middle of the n = 23, m = 2 range: XR's field_ops are 30× X's at ℓ = 7, 170× at ℓ = 10, about 60× at ℓ = 11 and within 2× at ℓ = 12 and 13. At n = 11 the two are within 2× in 8 of 9 cells (the windows are nearly the whole field from ℓ ≈ 9).
- The random-coefficient null is not harder than S in D: at n = 23, m = 2 its D is lower at ℓ = 3–6 (D = 0 in three cells: F4 added no element after the initial echelon form of the inputs), higher at ℓ = 8 by 1 and equal at the other six cells.
- At m = 3 the direct arms (S, X, XR, NS) complete only ℓ = 4–5 (D = 7, about 2 × 10#super[10] XORs at ℓ = 5, 19 to 26 s). ℓ = 6 hits the time budget (n = 11: 9 steps in 300 s, peak matrix 0.79 GB) or the 1 GiB matrix limit (n = 23). U reaches ℓ = 6; PC (chained) reaches ℓ = 5 at n = 11 and none at n = 23.
- At n = 23, m = 2 every arm hits the matrix limit at ℓ = 14 after 28 to 58 s.

== Not shown

- Nothing about n = 131, other a, the other Gauss-window class (ζ ↦ ζ#super[k]), a u-frame on a Gauss-derived V, m ≥ 4, or any engine other than #raw("groebner_basis_f4"); the budgets and the 1 GiB matrix cap decide where each series stops, so the κ values are readings over the cells that finished (ℓ ≤ 13).
- field_ops counts Boolean word XORs, not F#sub[p] multiplications; κ is compared with m − 1 and across arms of this amendment only.
- No pipeline is priced; no speedup, win or significance language is licensed. The m = 3 series of S, X, NS and PC are inconclusive by the rule, and XR's m = 3 κ values (9.40, 18.71) rest on an irregular b.
- The replay check shows reproducibility of the trace from the seed, not correctness of F4's basis beyond the planted-solution check.
