#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU reciprocal residual-batch screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round9-20261030`, October 29, 2026 PDT.

Baseline u is the optimized CPU implementation at `2bd4588fba60`; candidate v
is `abb2795c95cb`, including reciprocal residual-batch arithmetic
on top of compressed suffix indexing, projective tables and sparse pivots. Candidate w uses
batch 1. This is a bundled comparison with unchanged inputs and output semantics.
Host: AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB reserved;
shared EKS VM, L0. Compiler and release settings match across arms.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.283881], [0.129490], [0.283991],
 [16-bit / 3 slots], [3.966054], [0.674992], [3.990154],
 [16-bit / 4 slots], [4.169525], [0.808663], [4.157544])
Recorded medians from 9 repetitions after 1 warmup. Derived u/v median ratios:
2.192, 5.876, 5.156. These do not meet the further 10x target and are not
strict-isolation performance evidence. Do not pool durations with other hosts.

#image("../../round9/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured, 12 warmups. Complete certificate
bytes agree across arms. Every execution checks its known scalar, replays the
certificate and rejects a corrupted scalar. The frozen native analyzer and
file-verifying export check pass. No failed or missing cells were discarded.

Complete intervals include constructor and known-target setup, scalar/MITM
tables, all attempted trials, relation collection, elimination, both certificate
checks and serialization. Process launch, archive I/O and negative control are
outside the clock. Table capacity counters are not total peak RSS.

The candidate reuses one bounded reciprocal per residual batch for small moduli,
retaining wide remainder for larger fields. All 29 release tests pass, including
independent remainder checks, exhaustive group-pair subtraction and full
archived certificates. Retained table capacities remain 8696 / 66616 / 97456
bytes versus baseline 5112 / 50808 / 65648 bytes; these are not peak RSS.
Largest feasible CPU scale, matched rho/external comparisons, strict isolation
and independent review remain unresolved. The 20–40-bit ladder remains separate.

Protocol: `2867fd7e2cc59c1664ea167601ed9cfd7c87cb8953977f9d52cdc23ac597c81b`.
Raw rows, summaries, certificates and checked export: `round9/results/`.
Build identities: `round9/results/build.json`; terminal receipt: `round9/status.json`.
