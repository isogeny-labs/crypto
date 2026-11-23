#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU cached-root and batched-table screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round5-20261030`, October 29, 2026 PDT.

Baseline u is the optimized CPU implementation at `2bd4588fba60`; candidate v
is `981cf39a9827`, including cached square-root parameters and batched MITM table construction
on top of order certification, Euclidean inverses and fixed-base tables. Candidate w uses
batch 1. This is a bundled comparison with unchanged inputs and output semantics.
Host: AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB reserved;
shared EKS VM, L0. Compiler and release settings match across arms.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.284421], [0.139901], [0.284772],
 [16-bit / 3 slots], [3.939445], [0.893223], [3.946435],
 [16-bit / 4 slots], [4.155475], [1.088374], [4.139796])
Recorded medians from 9 repetitions after 1 warmup. Derived u/v median ratios:
2.033, 4.410, 3.818. These do not meet the further 10x target and are not
strict-isolation performance evidence. Do not pool durations with other hosts.

#image("../../round5/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured, 12 warmups. Complete certificate
bytes agree across arms. Every execution checks its known scalar, replays the
certificate and rejects a corrupted scalar. The frozen native analyzer and
file-verifying export check pass. No failed or missing cells were discarded.

Complete intervals include constructor and known-target setup, scalar/MITM
tables, all attempted trials, relation collection, elimination, both certificate
checks and serialization. Process launch, archive I/O and negative control are
outside the clock. Table capacity counters are not total peak RSS.

The next source candidate stores sparse pivot tails when their payload does not
exceed dense storage, skips zero products and preserves exact pivot order.
All 25 release tests pass, including sparse and dense matrices compared with
an independent eager reference. This panel did not measure that new change.
Largest feasible CPU scale, matched rho/external comparisons, strict isolation
and independent review remain unresolved. The 20–40-bit ladder remains separate.

Protocol: `f5f4dfce98a1ba9025fe3d8536b816450f03451e45111c99b7478aed33c3cb74`.
Raw rows, summaries, certificates and checked export: `round5/results/`.
Build identities: `round5/results/build.json`; terminal receipt: `round5/status.json`.
