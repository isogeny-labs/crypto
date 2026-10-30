#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU arithmetic: complete recovery screen
Observations only; no certified latency claim or hypothesis promotion.
Run: `hashed-ecdl-cpu-round3-20261030`, October 29, 2026 PDT.

The candidate applies exact Euclidean inverses, finite-difference constructor
loops and fixed-base multiplication tables. Baseline u is the already optimized
CPU implementation at `2bd4588fba60`; candidate v is `8ca08633a12b`. This is a
bundled implementation comparison, not attribution to individual changes.
Host: Intel Xeon 6975P-C, one software thread, two reserved vCPUs and 4 GiB,
shared EKS VM (L0). Same compiler/release flags and fixed public fixtures.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u, ms], [Candidate v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.386900], [0.193202], [0.386876],
 [16-bit / 3 slots], [5.742763], [2.040574], [5.726613],
 [16-bit / 4 slots], [5.964579], [2.335476], [5.987033])
Recorded medians: 9 repetitions after 1 warmup per cell. Derived u/v median
ratios: 2.003, 2.814, 2.554. These diagnostic ratios do not reach the requested
further 10x target. No strict-isolation performance result is established.

#image("../../round3/results/latency.svg", width: 100%)

All 120 executions completed: 108 measured and 12 warmups. The frozen native
analyzer compared all complete certificate bytes across four arms. Every run
checked the known scalar, replayed the certificate, and rejected corruption.
Arm w uses candidate batch 1; v uses batch 64. Complete intervals include
constructor/known-target setup, scalar tables, all trials, relation collection,
elimination, internal and second replay, and serialization. Process launch,
archive I/O and the negative control are outside the interval.

Candidate constructor medians: 0.728172/0.691823 ms on the 16-bit fixtures;
oracle medians: 0.579318/0.811363 ms. These remain substantial counted phases.
The original 20–40-bit ladder is still unresolved. No largest-feasible-scale,
matched-rho or fastest-tool claim follows. Memory counters are table capacities,
not RSS. Do not pool these durations with earlier panels on a different host.

Protocol: `3ed2015b8b5b0607fcf565011d49c18480be79929e20b00f69606dda36dee5ac`.
Raw rows, summaries, full certificates and checked export: `round3/results/`.
Source/binary hashes: `round3/results/build.json`; job/pod receipt: `round3/status.json`.
