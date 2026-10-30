# Density-aware scheduling screen

The five-arm panel retains baseline u/x (`2bd4588fba60`), fixed-batch reference
y (`abb2795c95cb`), density-aware candidate v (`d0ad5a617f59`) and batch1 w.
All 150 executions completed (135 measured,15 warmups), with identical full
certificates across arms, scalar checks, replay and corrupted-scalar rejection.
The export passes completeness and file-integrity verification with zero findings.

| Fixture | Baseline u ms | Fixed y ms | Density-aware v ms | u/v | y/v |
|---|---:|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.287191 | 0.131490 | 0.120901 | 2.375 | 1.088 |
| 16-bit, 3 slots | 3.997944 | 0.668873 | 0.667152 | 5.993 | 1.003 |
| 16-bit, 4 slots | 4.192356 | 0.814393 | 0.813343 | 5.154 | 1.001 |

Nine measured repetitions after one warmup per cell. AMD EPYC9R45, one software
thread,2 vCPUs/4GiB reservation, shared-host L0. The small-fixture observed ranges
were0.120111–0.125070ms for v and0.128741–0.133461ms for y. Larger-fixture ranges
overlap; their small median differences are not evidence of improvement or
certified equivalence. Retain the candidate based on this bounded diagnostic,
without claiming strict latency validation. The further10x goal is unmet.

Setup, discarded trials, relation collection, elimination, both certificate
checks and serialization remain charged. Launch, archive I/O and negative
controls are outside the interval. No phase or fixture was removed.

[Protocol](protocol.json), [rows](results/rows.csv), [summary](results/summary.csv),
[export](results/export.json), [build identities](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round11/report.pdf), [candidate notes](../density-batch-candidate.md).
All29 release tests and strict Clippy pass. The threshold1/8 was fixed before
measurement; it depends on distinct suffix coverage, not fixture identifiers.

Unlike round10's initial export, this adapter directly records five-arm flow
counts and its frozen analyzer identity. Earlier provenance/metadata corrections
remain preserved in their original directories and are not erased by this pass.
Strict isolation, independent review, external/rho comparison and larger scale
remain open. Next investigate redundant per-query group operations and reduction
work, preserving all verification and coordinating shared-source ownership.
