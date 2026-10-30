# RTX PRO 6000 explicit-relation diagnostics

This is a browsing copy of diagnostic visual records, **not an admitted
benchmark result**. All wall clocks have isolation L0 and `wall_valid=false`.
Matched rho, fastest external-tool comparison and independent scientific
validation remain absent. The benchmark validation gate is unchanged; no
entry has been added to its admitted-results table.

| Study | Recorded scope | Visual copies | Source of truth |
| --- | --- | --- | --- |
| Block normalization | 120 executions, 12 warmups, 108 measured; product-tree inversion preparation uses 765 source-level field multiplications versus 4608 for dual scans; no cold end-to-end gain established | [PDF](perf-report.pdf), [SVG](perf-latency.svg), [150-dpi PNG](perf-latency.png) | [Normalization study](../../research/explicit_relations_perf_20261030/RESULTS.md) |
| Packed query | 120 executions, 12 warmups, 108 measured; exact witnesses and transfer-byte counts retained; timing differences are descriptive | [PDF](query-report.pdf), [SVG](query-latency.svg), [150-dpi PNG](query-latency.png) | [Query study](../../research/explicit_relations_query_20261030/RESULTS.md) |

[copies.json](copies.json) binds each PDF/SVG copy to its source and SHA-256.
PNG files are rendered from those SVG bytes at scale 150/96 and 150 dpi with
CairoSVG. The raw records, frozen protocols, original report sources, device
identities and complete archive hashes remain in the linked study directories.
The PDFs were inspected when the original reports were published; these
copies are byte-identical. Later corrections are separate records.

A [post-run strict coverage audit](../../research/explicit_relations_perf_20261030/coverage-audit.txt)
verifies both unchanged 120-row panels and rejects deliberate duplicate,
missing-warmup, seed, flag, arm and digest faults. It supplements the original
frozen analyzer without rewriting any measurements or statistics.

The subsequent [full-recovery scaling study](../../research/explicit_relations_e2e_20261030/)
retains all launch failures and bounded solver outcomes through v8: both
60-bit public targets recovered and passed fresh replay. The [seven-page PDF](e2e-report.pdf)
and graphs [v3](e2e-scaling-v3.svg), [v4](e2e-scaling-v4.svg),
[v5](e2e-scaling-v5.svg), [v7](e2e-scaling-v7.svg), [v8](e2e-scaling-v8.svg)
are accompanied by matching 150-dpi PNGs and hashes in copies.json. All seven
PDF pages were visually inspected. Raw records were committed and pushed
before rendering. These remain unadmitted L0 diagnostics, with no matched
rho or independent scientific validation.

The separate v9 extension retains seven verified recoveries from eight recorded
executions: one61-bit recovery and one61-bit memory refusal; both62-bit cells
were unattempted by the stopping rule. The [two-page v9 report](e2e-v9-report.pdf),
[SVG](e2e-scaling-v9.svg) and [PNG](e2e-scaling-v9.png) preserve this incomplete
rung. Both PDF pages were inspected. Earlier report copies remain unchanged.

V11 completed all 12 prescribed executions, including both fixed 61-bit and
62-bit targets. The [two-page v11 report](e2e-v11-report.pdf),
[SVG](e2e-scaling-v11.svg) and [150-dpi PNG](e2e-scaling-v11.png) retain every
row. Both report pages were visually inspected after raw commit `099dff2edd81`.
These are unadmitted L0 observations with no matched rho or independent
scientific validation; 62 bits is the complete prescribed rung, not a global
hardware maximum. Earlier panels and their failures remain unchanged.

V13 also completed all 12 prescribed executions, with complete right caches
in both 61-bit and both 62-bit cells. The [two-page v13 report](e2e-v13-report.pdf),
[SVG](e2e-scaling-v13.svg) and [150-dpi PNG](e2e-scaling-v13.png) retain every
execution. Both PDF pages were visually inspected after raw commit `84d5159e3635`.
These remain unadmitted L0 observations; no cross-version timing advantage,
matched-rho result or global hardware maximum is inferred. V15 failed source
checkout before compilation or trials; its failure is preserved separately.

V19 ran on one RTX4090 (24,564MiB). All eight 52-bit-field executions recovered
and passed fresh certificate replay; all four 63-bit memory/work limits remain
visible. Twelve slots used251/261 relations versus743/709 for eight slots on
the two fixed targets. The [two-page V19 report](e2e-v19-report.pdf),
[SVG](e2e-scaling-v19.svg) and [150-dpi PNG](e2e-scaling-v19.png) preserve all
12 outcomes. Report pages were inspected after raw commit `e1db39a661f3`;
these copies are hash-bound. Clocks remain L0 descriptions on fixed inputs;
intermediate field sizes53–62 were not measured here. External comparison:
not run in this configuration/admission panel. The original frozen analyzer's
raw SVG retains an inherited PRO6000 title; this derived report correctly
labels the actual4090 telemetry without rewriting raw output.
