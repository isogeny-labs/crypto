# R46 polynomial-root collector

[Report](REPORT.md), [PDF](report.pdf), [results](results.json),
[protocol](PROTOCOL.md), [root source](ROOT46.rs), [encoding](FORMAT.md).

The exact polynomial collector preserves all complete R45point sets through
degree13and every original source check, while replacing exhaustive lambda
trials by algebraic root candidates. Candidate counts are not total cost or
ICperformance. R45/NEXT.md gives actual divisor-evaluation and proper-map
obligations. Root certificates are conditional Lean facts, not formal native
code correctness or global geometry. Goalactive; independent review pending.
