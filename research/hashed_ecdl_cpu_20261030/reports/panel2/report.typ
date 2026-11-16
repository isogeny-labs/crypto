#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= CPU complete recovery: parameter cache
Observations only; no hypothesis promotion or certified latency claim.
Run: `hashed-ecdl-cpu-panel2-20261030`, October 29, 2026 PDT.

The cache compares the four validated curve parameters and exact bases, avoiding
comparison or cloning of the small-field root table. It retains lexicographic
prefix/suffix sums across targets and batches residual inversions. All arms use
the same updated shared reduction and one frozen native binary. Host: AMD EPYC
9R45, one software thread, two reserved vCPUs and 4 GiB, shared EKS VM, L0.

#table(columns: (1.3fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Original u, ms], [Cached v, ms], [A/A x, ms],
 [8-bit / 3 slots], [0.315901], [0.283191], [0.314862],
 [16-bit / 3 slots], [45.993793], [3.903194], [46.098433],
 [16-bit / 4 slots], [80.536765], [4.118365], [80.527314])
Medians from 9 measured repetitions after 1 warmup per cell. The observed median
ratios u/v are approximately 1.12, 11.78, 19.56; these are descriptive divisions,
not certified speedups. The full ranges appear below and in `summary.csv`.

#image("../../panel2/results/latency.svg", width: 100%)

All 120 executions completed,108 measured and12 warmups. The frozen analyzer
compared every complete certificate byte-for-byte across all four arms. Every
execution checked the recovered scalar, independently replayed its certificate
and rejected a corrupted scalar. Batch1 medians were 0.267431, 4.182565, 4.595888 ms;
batch 64 does extra work past the first hit in a batch and is not always fastest
on the smallest fixture. All work in the batch remains charged.

The complete interval includes curve and known-target construction, full solve
and all failed attempts/trials, two certificate checks and serialization. It
excludes launch, archive I/O and the negative control. Retained table capacities
are recorded; total peak RSS is not measured. Matched rho/external baselines and
strict CPU isolation are missing, so no fastest-tool or cryptographic-size claim
is supported. These are single-target generic-decomposition workloads.

Source `abfd53ec854c`; full source/binary identities: `panel2/results/build.json`.
Protocol: `5addf86ce983b867a869ee37c9a0a3c309566082cdb05fa2e0e46b60d4195035`.
Records: `panel2/results/export.json`, `rows.csv`, `summary.csv` and certificates.
Do not pool with panel1: the shared reduction changed between panels. Within
this panel the oracle is the only treatment factor. Larger-field feasibility is
reported separately by the bounded public-target CPU ladder.
