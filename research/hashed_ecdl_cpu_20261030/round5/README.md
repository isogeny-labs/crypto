# Cached-root and batched-table CPU screen

All 120 executions completed (108 measured, 12 warmups), with byte-identical
certificates across arms, scalar checks, replay and corrupted-scalar rejection.
The frozen export passes completeness and file-integrity verification.

| Fixture | Baseline median ms | Candidate median ms | Baseline repeat ms | Ratio |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.284421 | 0.139901 | 0.284772 | 2.033 |
| 16-bit, 3 slots | 3.939445 | 0.893223 | 3.946435 | 4.410 |
| 16-bit, 4 slots | 4.155475 | 1.088374 | 4.139796 | 3.818 |

Baseline: `2bd4588fba60`. Measured candidate: `981cf39a9827`. Nine measured
repetitions per cell after one warmup on AMD EPYC 9R45, one software thread,
2 vCPUs / 4 GiB reserved. These are L0 shared-host diagnostics. The further
10x target is not met. Do not pool absolute timings across different hosts.
All setup, relation collection, elimination, both certificate checks and
serialization are charged; process launch, archive I/O and the negative
control are outside the clock. Batch-1 ablation remains in the raw records.

See [protocol](protocol.json), [raw rows](results/rows.csv),
[summary](results/summary.csv), [checked export](results/export.json),
[build identities](results/build.json), [terminal receipt](status.json),
[archive receipt](results-receipt.json) and [report](../reports/round5/report.pdf).

## Next candidate and delivery handoff

The follow-up source change stores only normalized pivot tails, selecting
sparse index/value storage when its payload is no larger than dense storage.
It skips zero products and multiplication by one, retaining column order and
exact dependency coefficients. The existing dense eager-elimination reference
now covers four densities, 24 dimensions and three moduli. All 25 release
tests, formatting and strict Clippy pass. This change is not timed by round5.

Method context: standard modular arithmetic and elimination; the preceding
candidate uses [HAC square roots](https://cacr.uwaterloo.ca/hac/about/chap3.pdf)
and [batch/precomputation arithmetic](https://www.hyperelliptic.org/EFD/precomp.pdf).
These sources were reviewed during implementation; no novelty claim is made.
Strict-isolation measurements, independent review, matched rho/external
comparisons and the 20–40-bit scale ladder remain unresolved.

PR #2933 merged into the GPU parent branch on 2026-10-30. This follow-up
preserves its source/evidence history and adds only the remaining CPU work.
The shared daily session log is reserved by T-828; this scoped handoff records
the delivery without modifying that owner's file.
