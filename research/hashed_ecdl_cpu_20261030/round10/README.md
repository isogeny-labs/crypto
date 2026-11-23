# Progressive batching: mixed result, default restored

The five-arm panel adds previous candidate y (`abb2795c95cb`, fixed batch64)
while retaining baseline u/x, progressive candidate v (`ca37cf07f271`) and
batch1 w. All 150 executions completed: 135 measured,15 warmups. Full
certificates agree across all arms; replay and negative controls pass.

| Fixture | Baseline u ms | Previous fixed y ms | Progressive v ms | y/v |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.285852 | 0.129360 | 0.122401 | 1.057 |
| 16-bit, 3 slots | 3.973266 | 0.664603 | 0.681493 | 0.975 |
| 16-bit, 4 slots | 4.169826 | 0.809593 | 0.817353 | 0.991 |

Nine measured repetitions after one warmup per cell, AMD EPYC9R45, one thread,
2 vCPUs/4GiB reservation, shared-host L0. These descriptive medians do not
establish certified speedups. Progressive batching reduced the small-fixture
median but increased both larger-fixture medians in this panel. Fixed batching
is retained as the conservative default; the rejected implementation remains
in git history. The further10x goal is unmet, with baseline2bd4588fba60 unchanged.
All setup, discarded trials, elimination, both replay checks and serialization
are charged. Launch, archive I/O and negative controls are excluded.

[Protocol](protocol.json), [raw rows](results/rows.csv),
[summary](results/summary.csv), [corrected export](results/export-corrected.json),
[build identities](results/build.json), [terminal receipt](status.json),
[archive receipt](results-receipt.json), [report](../reports/round10/report.pdf).
The frozen five-arm analyzer was checked with archived/relabelled positive data
and a missing-cell negative control before measurement. It is descriptive only.

## Metadata correction

The first export inherited flow counts120 and the old four-arm analyzer hash.
The checker rejected both. Original [export.json](results/export.json) is retained
unchanged; [correct-export.py](correct-export.py) writes a separate corrected
export with counts150 and the exact frozen analyzer reference. The corrected
export passes check-export --verify-files with zero findings. Raw measurements
and frozen protocol are unchanged. Do not cite the initial export as valid.

Earlier rounds6–9 also have additive PROVENANCE-CORRECTION.md notes: descriptive
source text was stale, although arm identities and hashed build receipts agreed.
Their immutable protocols and records remain preserved with these deviations.

All 29 release tests pass after restoring fixed scheduling; formatting and
whitespace checks pass. Next consider a density-aware schedule, with the same
previous-candidate control, rather than unconditional geometric growth. Strict
isolation, independent review, external/rho comparisons and larger scale remain open.
