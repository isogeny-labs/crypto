# Sparse-pivot and reciprocal CPU screen

The frozen panel completed all 120 executions (108 measured, 12 warmups).
Full certificate bytes agree across every arm. Scalar checks, replay,
corrupted-scalar rejection and export/file-integrity verification pass.

| Fixture | Baseline median ms | Candidate median ms | Baseline repeat ms | Ratio |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.283681 | 0.138741 | 0.284081 | 2.045 |
| 16-bit, 3 slots | 3.941404 | 0.851723 | 3.946603 | 4.628 |
| 16-bit, 4 slots | 4.150535 | 1.036634 | 4.134884 | 4.004 |

Fixed baseline `2bd4588fba60`; candidate `9930a229ba35`. Nine repetitions after
one warmup per cell, AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB.
These are L0 diagnostics, not certified speedups. The further 10x goal remains
unmet. All setup, discarded trials, elimination, both replay checks and
serialization remain charged. Launch, archive I/O and negative controls are
excluded. Batch-1 controls remain in the complete raw records. Do not pool
absolute timings across panels or attribute this bundled result to one change.

[Protocol](protocol.json), [raw rows](results/rows.csv),
[summary](results/summary.csv), [export](results/export.json),
[build identities](results/build.json), [terminal receipt](status.json),
[archive receipt](results-receipt.json), [report](../reports/round6/report.pdf).

See [candidate derivation and literature](../reciprocal-candidate.md).
All 26 release tests, strict Clippy and formatting checks pass. The next
optimization should address remaining group arithmetic/table setup and replay
cost without removing verification or changing the fixed fixtures. Strict
isolation, independent review, matched rho/external comparisons and the scale
ladder remain unresolved. This is generic decomposition engineering.
