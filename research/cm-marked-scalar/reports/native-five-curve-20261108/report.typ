#set page(paper: "a4", margin: 15mm)
#set text(font: "Libertinus Serif", size: 9.5pt)
#set par(justify: false, leading: 0.5em)
#set heading(numbering: none)
#let data = json("plot-data.json")
#align(center)[
 #text(size: 18pt, weight: "bold")[Native closed CM relations]
 #linebreak()
 #text(size: 11pt)[P-192 primary; P-224 / P-256 / P-384 / P-521]
 #linebreak()
 2026-11-08 · T-1888 · PR 3674
]

= Checked construction
The native continuation now covers all five curves. It passes 150 point/window cases, both full curve-map identities, signed kernel Frobenius actions, the global dual-composition identity, CM norm/degree/label checks, and three orbit fixtures per curve. Eight certificate mutations fail at their intended checks, including a y-sign error with unchanged x action.

#table(columns: (1.15fr, 0.6fr, 0.65fr, 0.8fr, 0.9fr, 1fr, 1.1fr), inset: 4pt,
 [*Curve*], [*ell*], [*rho*], [*ord mu*], [*signed orbit*], [*norm bits*], [*quotient bits*],
 [P-192], [13], [2], [5], [10], [382], [378],
 [P-224], [11], [10], [17], [34], [451], [447],
 [P-256], [11], [8], [3], [6], [516], [513],
 [P-384], [19], [9], [7], [14], [769], [765],
 [P-521], [7], [6], [7], [14], [1044], [1041],
)

#block(fill: rgb("edf3f8"), inset: 8pt, width: 100%)[
 *Explicit point path*
 #linebreak()
 E → phi → E1 → eta → E2 → sigma → E → correction k
 #linebreak()
 sigma(x,y) = (x / ell², y / ell³);  sigma eta phi = [ell].
 #linebreak()
 alpha = pi − [c] = a + b pi;  a = −c, b = 1;  a+b = mu (mod n).
 #linebreak()
 k = mu / ell (mod n);  [k] sigma eta phi(P) = [mu]P.
]
The quotient psi exists globally by kernel containment, with degree norm/ell. Its residual polynomials are unexpanded. The evaluated dual/correction formula is asserted on the rational subgroup and has a different global degree. The native canonicalizer returns the least affine signed-orbit point and its verified scalar transporter.

= Exact group schedules: measured event counts, no clocks
#text(size: 8.5pt)[Blue: direct public-scalar method. Orange: fused [ell] then [mu/ell]. Each bar is the minimum *unweighted D+A* count over the fixed widths 2–7. This is a source-specific count display, not a runtime optimum.]
#v(3pt)
#for row in data [
 #let direct = row.group_count_minima.find(x => x.arm == "window")
 #let fused = row.group_count_minima.find(x => x.arm == "fused")
 #let d = direct.doubles + direct.adds
 #let f = fused.doubles + fused.adds
 #grid(columns: (1.4cm, 1fr, 1.8cm), gutter: 4pt,
  [#row.curve],
  stack(spacing: 2pt,
   rect(width: d / 650 * 11cm, height: 3mm, fill: rgb("286c9c"), stroke: none),
   rect(width: f / 650 * 11cm, height: 3mm, fill: rgb("d98235"), stroke: none)),
  [#d / #f])
 #v(4pt)
]
#text(size: 8pt)[Data: qualification/structural-v2/rows.jsonl (90 rows, all correct, timing_reported=false). Source/inputs were published before collection; binary and input checks passed. Chart source and exact data are retained together.]

#pagebreak()
= Cost vector and accounting boundary
#table(columns: (1fr, 1.55fr, 1.55fr, 1.55fr, 1.35fr), inset: 4pt,
 [*Curve*], [*Direct D,A,N*], [*Fused D,A,N*], [*Map correction D,A,N*], [*Map formulas M,A,I*],
 [P-192], [190,39,19], [195,43,23], [191,41,22], [114,104,2],
 [P-224], [218,42,14], [227,49,24], [223,47,22], [98,88,2],
 [P-256], [254,46,23], [259,52,27], [255,50,25], [98,88,2],
 [P-384], [380,70,34], [387,72,25], [383,70,24], [162,152,2],
 [P-521], [518,88,31], [524,89,40], [521,88,39], [66,56,2],
)
D/A/N count explicit group doubles/additions/negations in the named native routine, including odd-table construction and the fused small-scalar step. The displayed widths are in CONSTRUCTION.md and every scanned width remains in the raw records. The explicit maps add (8ell+10) calls to Field.mul, 8ell calls to Field.add, and two Field.inv calls. This formula follows from the actual full-Horner coefficient-array lengths plus endpoint scales. It excludes backend conversion and setup costs; it is not a complete runtime total.

The full cold timing boundary starts before the required runtime, table and map objects; it includes preparation, maps, endpoint closure, correcting scalar, output verification and destruction. Historical map generation is retained separately. No omitted setup is silently amortized.

= Execution and admission record
#table(columns: (1fr, 2.7fr), inset: 5pt,
 [*Check*], [*Observed result*],
 [Native qualification], [150 point/window cases; 15 orbit fixtures; all five global/signed/label checks pass.],
 [Mutation controls], [8 certificate mutations rejected at intended checks; synthetic analysis design rejects 11 additional corruptions.],
 [Local preflight], [0.74 and 0.75 other CPU seconds / 2 seconds; limit 0.20. Both refused before cryptographic timing.],
 [Isolation software], [Sleep-only completion and SIGTERM restoration pass; always admitted=false.],
 [EKS reconciliation], [9 failed jobs, 1 suspended; none restarted or submitted. Direct live read denied at role assumption; live caps and funds unverified.],
 [Timing protocol], [Initial two-curve draft passes native schema validation; source/binary freeze and admitted backend remain pending.],
)
The current correction still requires a full-size public scalar. The next algebraic improvement must reduce that work or provide a different directly evaluable closed relation. An admitted end-to-end runtime gain has not been demonstrated; operation counts cannot settle it.

= Custody and replay
Source and fixture publication precedes counted data (source commit 7e8109ea2a65; full receipt in structural-v2/publication.txt). New P-192/P-224/P-256 maps were constructed under extension-scope-v4 from published native source 047da426a780. P-384/P-521 preserve historical inputs; the constructor exactly reproduces the saved P-521 map coefficients and scales. P-192 uses its published explicit group because this backend omits the named entry. Earlier compile, backend and projection failures, and the corrected description of the first P-192 failure, remain preserved.

Replay: build native-complete, construct-binary and qualification-tests-binary with the study Makefile; run native --qualify fixtures-v2 and qualification-tests fixtures-v2. Native-v17/v19 contain exact receipts. These are producer-side native exact checks; no independent signed scientific review or Lean formalization is implied.

#text(size: 8pt)[Standard background: A. Sutherland, MIT 18.783 Lecture 5 (2023), slides 11–16: https://math.mit.edu/classes/18.783/2023/LectureSlides5.pdf . Full derivation, source paths, raw costs and the measurement blocker are linked in CONSTRUCTION.md and problems/cm-marked-scalar-measurement-admission.md.]
