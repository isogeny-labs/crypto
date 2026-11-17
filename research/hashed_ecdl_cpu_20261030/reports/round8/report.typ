#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU persistent suffix-index screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round8-20261030`, October 29, 2026 PDT.

Baseline u is the optimized CPU implementation at `2bd4588fba60`; candidate v
is `7d68bcd2bf1d`, including exact compressed-point suffix indexing
on top of projective fixed-base tables, sparse pivots and reciprocal arithmetic. Candidate w uses
batch 1. This is a bundled comparison with unchanged inputs and output semantics.
Host: AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB reserved;
shared EKS VM, L0. Compiler and release settings match across arms.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.284731], [0.130490], [0.284761],
 [16-bit / 3 slots], [3.947265], [0.687532], [3.963166],
 [16-bit / 4 slots], [4.171906], [0.827703], [4.153896])
Recorded medians from 9 repetitions after 1 warmup. Derived u/v median ratios:
2.182, 5.741, 5.040. These do not meet the further 10x target and are not
strict-isolation performance evidence. Do not pool durations with other hosts.

#image("../../round8/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured, 12 warmups. Complete certificate
bytes agree across arms. Every execution checks its known scalar, replays the
certificate and rejects a corrupted scalar. The frozen native analyzer and
file-verifying export check pass. No failed or missing cells were discarded.

Complete intervals include constructor and known-target setup, scalar/MITM
tables, all attempted trials, relation collection, elimination, both certificate
checks and serialization. Process launch, archive I/O and negative control are
outside the clock. Table capacity counters are not total peak RSS.

The candidate preserves first tuple IDs using exact compressed point keys.
All 28 release tests pass, including collisions, duplicate sums, identity points,
all-target witnesses and full archived certificates. Retained table capacities
in fixture order are 8696 / 66616 / 97456 bytes, versus baseline 5112 / 50808 /
65648 bytes. The index trades extra storage for lookup work; this is not peak RSS.
Largest feasible CPU scale, matched rho/external comparisons, strict isolation
and independent review remain unresolved. The 20–40-bit ladder remains separate.

Protocol: `bf0ad0e439e52135b570e9d6563527d5ae6fe7c79b79e4eaa0db615828736cc0`.
Raw rows, summaries, certificates and checked export: `round8/results/`.
Build identities: `round8/results/build.json`; terminal receipt: `round8/status.json`.
