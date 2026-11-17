#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU projective fixed-base screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round7-20261030`, October 29, 2026 PDT.

Baseline u is the optimized CPU implementation at `2bd4588fba60`; candidate v
is `633a1b285c08`, including projective fixed-base evaluation and batched table normalization
on top of sparse pivots, reciprocal arithmetic and cached MITM tables. Candidate w uses
batch 1. This is a bundled comparison with unchanged inputs and output semantics.
Host: AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB reserved;
shared EKS VM, L0. Compiler and release settings match across arms.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.285691], [0.138670], [0.285861],
 [16-bit / 3 slots], [3.946047], [0.757573], [3.978737],
 [16-bit / 4 slots], [4.170487], [0.947734], [4.164907])
Recorded medians from 9 repetitions after 1 warmup. Derived u/v median ratios:
2.060, 5.209, 4.400. These do not meet the further 10x target and are not
strict-isolation performance evidence. Do not pool durations with other hosts.

#image("../../round7/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured, 12 warmups. Complete certificate
bytes agree across arms. Every execution checks its known scalar, replays the
certificate and rejects a corrupted scalar. The frozen native analyzer and
file-verifying export check pass. No failed or missing cells were discarded.

Complete intervals include constructor and known-target setup, scalar/MITM
tables, all attempted trials, relation collection, elimination, both certificate
checks and serialization. Process launch, archive I/O and negative control are
outside the clock. Table capacity counters are not total peak RSS.

The candidate uses Jacobian mixed addition with explicit exceptional cases,
one normalization per fixed-base evaluation, and batch-normalized table windows.
All 27 release tests pass, including exhaustive scaled small-curve pairs and
independent affine binary scalar checks on small and 40-bit curves.
This bundled panel does not identify either change's separate contribution.
Largest feasible CPU scale, matched rho/external comparisons, strict isolation
and independent review remain unresolved. The 20–40-bit ladder remains separate.

Protocol: `7f161ff5aa8e45865d46dd5d9ab5898070e5d0dd5cc05d4d6467c6be1ef63684`.
Raw rows, summaries, certificates and checked export: `round7/results/`.
Build identities: `round7/results/build.json`; terminal receipt: `round7/status.json`.
