# Optimized-baseline versus arithmetic-candidate screen

This completed diagnostic panel measures source `8ca08633a12b` against the
already optimized CPU baseline `2bd4588fba60`. The prior original Enumerate
implementation is not the baseline for the further 10x objective.

| Fixture | Baseline median ms | Candidate median ms | Baseline A/A median ms | Derived baseline/candidate |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.386900 | 0.193202 | 0.386876 | 2.003 |
| 16-bit, 3 slots | 5.742763 | 2.040574 | 5.726613 | 2.814 |
| 16-bit, 4 slots | 5.964579 | 2.335476 | 5.987033 | 2.554 |

All 120 executions completed (108 measured, 12 warmups). Full certificate
bytes match across all arms. Every execution checks its known scalar,
replays its certificate and rejects corruption. The frozen native analyzer
produced [stage summaries](results/summary.csv) and the [graph](results/latency.svg).
The metadata adapter emits the [checked export](results/export.json); it does
not calculate the medians. Full [report](../reports/round3/report.pdf).

Host: Intel Xeon 6975P-C, one software thread, 2 vCPU / 4 GiB reservation.
All clocks are shared-host L0 diagnostics. These ratios neither reach nor
certify the further 10x objective. The target remains active. Do not pool with
prior panels measured on AMD EPYC hardware. Matched rho, external competitors,
strict isolation and independent review remain outstanding.

The unchanged complete interval includes curve/known-target construction,
all setup including fixed-base tables, discarded trials, relation collection,
elimination, internal and second replay, and serialization. Process launch,
archive I/O and corrupted-scalar negative control are outside the clock.
Every candidate replay builds its own fixed-base tables.

Protocol hash: `3ed2015b8b5b0607fcf565011d49c18480be79929e20b00f69606dda36dee5ac`.
The source/binary/compiler identities are in [build.json](results/build.json).
The job and pod identities, terminal success and image digest are in
[status.json](status.json). [Input](input-receipt.json) and
[result](results-receipt.json) archive receipts identify durable S3 objects.
The launcher's default legacy bucket URI is not the upload destination; the
worker used the approved bucket recorded in these receipts.

## Next candidate, not measured here

The profile motivates two additional changes: use the existing nonzero
r-torsion/Hasse proof for automatic small-curve order certification instead of
scanning the field, and remove residual subtraction divisions plus Fermat
inversions from the MITM batch arithmetic. Exact parameter validation, every
relation check, and complete certificate replay remain required. These changes
are outside this panel's measured source and require a fresh frozen comparison.

The order proof uses prime r, an actual nonidentity point killed by r, and a
unique multiple of r in the Hasse interval. Ambiguous small intervals retain
exact point counting. New tests compare every compressed point on the 16-bit
fixture against the exhaustive constructor. The next candidate passes the
same 24 release tests and strict Clippy; it has no timing result yet.
