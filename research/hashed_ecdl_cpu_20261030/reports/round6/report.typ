#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU sparse-pivot and reciprocal screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round6-20261030`, October 29, 2026 PDT.

Baseline u is the optimized CPU implementation at `2bd4588fba60`; candidate v
is `9930a229ba35`, including sparse pivot tails and bounded reciprocal arithmetic
on top of cached roots, batch setup, order certification and fixed-base tables. Candidate w uses
batch 1. This is a bundled comparison with unchanged inputs and output semantics.
Host: AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB reserved;
shared EKS VM, L0. Compiler and release settings match across arms.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.283681], [0.138741], [0.284081],
 [16-bit / 3 slots], [3.941404], [0.851723], [3.946603],
 [16-bit / 4 slots], [4.150535], [1.036634], [4.134884])
Recorded medians from 9 repetitions after 1 warmup. Derived u/v median ratios:
2.045, 4.628, 4.004. These do not meet the further 10x target and are not
strict-isolation performance evidence. Do not pool durations with other hosts.

#image("../../round6/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured, 12 warmups. Complete certificate
bytes agree across arms. Every execution checks its known scalar, replays the
certificate and rejects a corrupted scalar. The frozen native analyzer and
file-verifying export check pass. No failed or missing cells were discarded.

Complete intervals include constructor and known-target setup, scalar/MITM
tables, all attempted trials, relation collection, elimination, both certificate
checks and serialization. Process launch, archive I/O and negative control are
outside the clock. Table capacity counters are not total peak RSS.

The candidate preserves pivot order and uses bounded reciprocal multiplication
for canonical residues below 2^32, retaining wide remainder for larger moduli.
All 26 release tests pass, including independent remainder checks and sparse
and dense dependency matrices compared with an eager reference. This bundled
panel does not identify either change's separate contribution.
Largest feasible CPU scale, matched rho/external comparisons, strict isolation
and independent review remain unresolved. The 20–40-bit ladder remains separate.

Protocol: `10e83377864c5ccd1303a61081c7bd815a316fd136d4c393b8ba715c0b345df7`.
Raw rows, summaries, certificates and checked export: `round6/results/`.
Build identities: `round6/results/build.json`; terminal receipt: `round6/status.json`.
