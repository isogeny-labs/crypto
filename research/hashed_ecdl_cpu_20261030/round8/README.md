# Persistent suffix-index CPU screen

All 120 executions completed (108 measured, 12 warmups). Full certificate bytes
agree across every arm; scalar checks, replay, corrupted-scalar rejection and
export/file-integrity verification pass.

| Fixture | Baseline median ms | Candidate median ms | Baseline repeat ms | Ratio |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.284731 | 0.130490 | 0.284761 | 2.182 |
| 16-bit, 3 slots | 3.947265 | 0.687532 | 3.963166 | 5.741 |
| 16-bit, 4 slots | 4.171906 | 0.827703 | 4.153896 | 5.040 |

Fixed baseline `2bd4588fba60`; candidate `7d68bcd2bf1d`. Nine repetitions after
one warmup per cell, AMD EPYC 9R45, one software thread, 2 vCPUs / 4 GiB.
These are L0 diagnostics, not certified speedups. The further 10x goal remains
unmet. Setup, discarded trials, elimination, both replay checks and serialization
remain charged. Launch, archive I/O and negative controls are excluded. The
batch-1 control is preserved. Do not pool times across panels.

| Fixture | Baseline retained table bytes | Candidate retained table bytes |
|---|---:|---:|
| 8-bit, 3 slots | 5112 | 8696 |
| 16-bit, 3 slots | 50808 | 66616 |
| 16-bit, 4 slots | 65648 | 97456 |

Actual allocated slot capacity is counted, including empty slots. These counters
exclude transient buffers and are not peak RSS. Tuple row/work caps retain their
original meaning. The observed lookup tradeoff includes more retained storage.

[Protocol](protocol.json), [rows](results/rows.csv), [summary](results/summary.csv),
[export](results/export.json), [build identities](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json),
[report](../reports/round8/report.pdf), [candidate notes](../suffix-index-candidate.md).

All 28 release tests, strict Clippy and formatting pass. Next investigate
batched residual arithmetic and per-query overhead while preserving exact first
witnesses. Shared reduction edits remain coordinated with T-906. Strict isolation,
independent review, matched external/rho comparisons and larger scale remain open.
