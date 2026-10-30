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
retains all launch failures and bounded solver outcomes. Its current run is
still being collected; its final inspected report and graph copies will be
added after its raw records have been committed. No active-run result is
promoted by this catalog.
