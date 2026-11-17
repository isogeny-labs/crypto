# Projective fixed-base CPU screen

All 120 executions completed (108 measured, 12 warmups). Full certificate bytes
agree across every arm; scalar checks, replay, corrupted-scalar rejection and
export/file-integrity verification pass.

| Fixture | Baseline median ms | Candidate median ms | Baseline repeat ms | Ratio |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.285691 | 0.138670 | 0.285861 | 2.060 |
| 16-bit, 3 slots | 3.946047 | 0.757573 | 3.978737 | 5.209 |
| 16-bit, 4 slots | 4.170487 | 0.947734 | 4.164907 | 4.400 |

Fixed baseline `2bd4588fba60`; candidate `633a1b285c08`. Nine repetitions after
one warmup per cell, AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB.
These are L0 diagnostics, not certified speedups. The further 10x goal remains
unmet. Setup, discarded trials, elimination, both replay checks and serialization
remain charged. Launch, archive I/O and negative controls are excluded. The
batch-1 control is preserved. Do not pool times across panels or attribute this
bundled comparison separately to evaluation or table construction.

[Protocol](protocol.json), [rows](results/rows.csv), [summary](results/summary.csv),
[export](results/export.json), [build identities](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round7/report.pdf), [candidate notes](../projective-candidate.md).

All 27 release tests, strict Clippy and formatting pass. Next investigate
persistent suffix lookup and batched residual arithmetic while preserving the
first exact tuple witness. Strict isolation, independent review, matched
rho/external comparisons and the larger scale ladder remain outstanding.
