#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU order and residual arithmetic screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round4-20261030`, October 29, 2026 PDT.

Baseline u is the optimized CPU implementation at `2bd4588fba60`; candidate v
is `19748cd3248d`, including exact order-witness construction, Euclidean inverses,
fixed-base scalar tables and conditional residual subtraction. Candidate w uses
batch 1. This is a bundled comparison with unchanged inputs and output semantics.
Host: Intel Xeon Platinum 8488C, one software thread, 2 vCPUs / 4 GiB reserved;
shared EKS VM, L0. Compiler and release settings match across arms.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.471432], [0.225746], [0.467688],
 [16-bit / 3 slots], [6.777632], [2.067202], [6.804898],
 [16-bit / 4 slots], [7.142432], [2.484374], [7.134884])
Recorded medians from 9 repetitions after 1 warmup. Derived u/v median ratios:
2.088, 3.279, 2.875. These do not meet the further 10x target and are not
strict-isolation performance evidence. Do not pool durations with other hosts.

#image("../../round4/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured, 12 warmups. Complete certificate
bytes agree across arms. Every execution checks its known scalar, replays the
certificate and rejects a corrupted scalar. The frozen native analyzer and
file-verifying export check pass. No failed or missing cells were discarded.

Complete intervals include constructor and known-target setup, scalar/MITM
tables, all attempted trials, relation collection, elimination, both certificate
checks and serialization. Process launch, archive I/O and negative control are
outside the clock. Table capacity counters are not total peak RSS.

On the 16-bit candidate fixtures, constructor medians are 0.008617/0.008467 ms,
but base enumeration is 0.220313/0.244822 ms and separate replay is
0.333475/0.385913 ms. Oracle medians are 0.586966/0.823394 ms. On-demand square
roots replace the exhaustive lookup table; further changes need fresh measurement.
Largest feasible CPU scale, matched rho/external comparisons, strict isolation
and independent review remain unresolved. The 20–40-bit ladder remains separate.

Protocol: `0e878acaf6d00b2ca54b3db91e2921fd9d93f69cd8342ef4b5cdedec5d0843e2`.
Raw rows, summaries, certificates and checked export: `round4/results/`.
Build identities: `round4/results/build.json`; terminal receipt: `round4/status.json`.
