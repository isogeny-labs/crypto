# Prime-field PKM tower ecbench round (2026-10-11)

This is a measured, toy-curve benchmark entry. The primary protocol, raw sessions, analysis, graphs, and reports remain in the research directory so this catalog does not duplicate the evidence.

- [Result summary and per-cell tables](../../research/pkm_tower_ecbench_20261011/RESULT.md)
- [Frozen protocol](../../research/pkm_tower_ecbench_20261011/PROTOCOL.md)
- [Analysis JSON](../../research/pkm_tower_ecbench_20261011/ANALYSIS.json)
- [Main report PDF](../../research/pkm_tower_ecbench_20261011/REPORT.pdf)
- [Run report and figures](../../research/pkm_tower_ecbench_20261011/reports/run-20261011T2244Z/)

**Scope and result.** Four prime-order curves (9–15 bits); 140/140 runs verified and replayed. Signature F5 used 0.227–0.326 of tower F4's counted multiply-adds on the same tree, while wall-time ratios ranged from 0.56 to 2.06 (faster on the smallest cell, slower on the largest). The F6 analogue saved no counted work. Every algebraic arm was slower than same-target rho. Tower multiply-adds remain unpriced in the reported lower-bound S; pricing them at 1/63 GAE per operation made the algebraic arms 670–900,000× the enum control. This is not a speedup claim.

**Environment.** x86-64 Linux cloud VM, isolation L1 or L2 by run. Operation counts are deterministic; wall times are descriptive.