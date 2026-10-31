#set page(paper: "a4", margin: 17mm)
#set text(font: "Arial", size: 10pt)
#set par(leading: 0.5em)
#set table(inset: 5pt, stroke: 0.4pt + rgb("ccd4dc"))
= P-192: three residual-pivot fans
*Independent review of the frozen 17 October run · 21 October 2026*

The third complete fan contains 5,026 split cuts and 10,051 distinct quotient columns. The selected base and all three fans have 39,984 exact classes. Independent integer replays reproduced the archived geometry and union receipts. This is a finite support and matrix-structure result; it does not establish useful ordinary-query relation yield or a runtime improvement.

#figure(image("support.svg", width: 100%), caption: [Recorded class union and the exact upper bound on unordered triples, allowing repetition. Arrows denote a mathematical bound, not a measured solver pipeline.])

#table(columns: (2fr, 1fr),
 [*Frozen quantity*], [*Count*],
 [Selected quartic and quotient classes], [9,925],
 [Third fan: split / nonsplit cuts], [5,026 / 4,899],
 [Three-fan split rows / source restatements], [15,034 / 6],
 [Non-source rows with two private signed columns], [15,028],
 [Rows remaining after private-column peeling], [0],
 [Selected-plus-three-fan exact class union], [39,984],
 [Unordered three-sum support upper bound], [10,654,671,159,440],
)

== What the evidence establishes
The integer verifier reconstructs the quartic line polynomial from four direct evaluations, tests the residual points, checks the projected P-192 curve equation and group-sum identity, scans the old family and both earlier fans, and peels the signed matrix. The independent sorted-list union replay reproduces the 39,984-class count. Both current reviewer executions passed; their output receipts retain the original bytes.

The two ThreePivotSupport arithmetic statements bound the multiset count and show that count times 2^146 is below the recorded accepted-query denominator. Together with the generic unordered-image theorem, this gives a hit probability below 2^(-146) *under the stated ideal uniform accepted-query model*. It does not certify that an actual query producer follows that law. Private-column elimination leaves an empty core; useful ordinary-query rank is still unmeasured here.

== Source binding and CI repair
The frozen run manifest binds Lean dependencies to the exact file from commit `3283d46e4a`. A later atlas merge substituted a manifest with the same dependency revisions but another package name and formatting. This review restores the exact recorded file and updates the rolling study inventory. All 22 frozen run-manifest entries match; the archived run manifest and measured records are unchanged.

The two sealed Lean workflows retain their documented 60-minute budgets. The shared proof gate now passes its trust-list path through the supported environment binding. All seven gate self-tests, the 20-theorem SEA gate, 15 workflow-budget tests and the repository budget audit passed. The initial self-test invocation used an incompatible local default toolchain; the pinned 4.34.1 rerun passed all cases.

== Sources and reproduction
#link("https://github.com/aburan28/crypto/blob/2d84f8194b7947456694e58be415e87346aed615/research/p192_extension_cover_20261012/reports/run-20261017T2055Z/REPORT.md")[Frozen report and its linked raw records]. Run `verify_residual_pivot002_cross.py` and `verify_three_pivot_union.py` from the repository root. The source directory contains both verifier programs, the complete producer receipt, Lean sources and recorded axiom audits. `review-validation.json` and the adjacent logs describe this review's checks separately from the historical run. No curve-route or performance leaderboard promotion follows from this support bound.
