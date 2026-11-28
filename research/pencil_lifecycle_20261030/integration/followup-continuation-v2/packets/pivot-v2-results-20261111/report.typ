#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Pivot v2 native result
Receipt audit · 2026-11-11

EliminationField and PivotSwap compiled with exit code0. Controls then failed with exit code1 at line16: Lean could not synthesize Decidable for the function equality `swapRows M 1 2 0 = M 0`. Readback and ExpectedFailure were not attempted.

#image("timing.svg",width:100%)

== Custody and checks
Pre/post checks both passed for16,736files totaling3,111,892,318bytes; their dependency hash maps match. All three phase owners reached terminal states. All three compiler subprocesses were reaped with empty remaining groups. The command ledger exactly matches the compiler receipt. Two compiled artifacts are retained and hashed. Standalone axiom readback and the negative control were not reached.

== Timing and accounting
Outer completion:165.85813155799406seconds. Caller:165.9826697029639seconds, exit1. These enclose native phases and must not be added to them. The full2,400worker-second reservation is charged:10,980charged and118,620remaining at this terminal boundary, with November11 at12:00UTC deadline. Dollar cost and scientific ratio remain null.

== Review files
summary.json, metrics.csv and command-results.csv contain exact observations. graph-data.csv supplies phase timings. audit.json records receipt checks. The raw archive preserves source, logs, receipts, dependency maps and compiled artifacts; its members are bound by raw-manifest.json. No new compiler or kernel invocation was performed by this audit.
