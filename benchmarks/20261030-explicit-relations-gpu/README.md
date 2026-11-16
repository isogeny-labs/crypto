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

V21 completed all ten prescribed executions on one RTX4090, including both
63-bit public targets. Fresh CPU replay and corrupted-scalar rejection passed
for every cell. The [two-page V21 report](e2e-v21-report.pdf),
[SVG](e2e-scaling-v21.svg) and [150-dpi PNG](e2e-scaling-v21.png) retain every
execution; both PDF pages were inspected after raw commit `ee2a1cb4c613` was
pushed. The 63-bit targets used338/344 relations and15737/13733 queries.
Audit totals2600.976385/1192.904874 seconds are descriptive L0 observations.
The smaller52-bit bases required more queries and longer cold time than the
larger-base controls. This panel measures completion of the prescribed rung,
not a global hardware maximum. Exact10-cell coverage passes; the canonical
rectangular checker retains five findings. External comparison: not run in
this configuration/admission panel. All earlier outcomes remain unchanged.

The [affine-subtraction comparison](../../research/explicit_relations_affine_20261030/RESULTS.md)
completed all 12 recoveries on one RTX 4090. Certificates and relation/query
counts agree across both formulas and baseline repeats. The two 52-bit targets
used 3.00% and 1.47% less cold solve time; the two 63-bit targets used 0.31% more
and 1.37% less time. These are per-input L0 descriptions, not a uniform gain.
The [two-page PDF](affine-v1-report.pdf), [SVG](affine-v1-scaling.svg), and
[150-dpi PNG](affine-v1-scaling.png) retain every execution. Both pages were
visually inspected after raw commit `c79bd38363f9` was pushed. Canonical schema
and coverage checks passed with zero findings. Formula remains opt-in; next
measure the costs within relation queries. No external-tool timing arm was
prescribed for this formula-only comparison. Earlier records are unchanged.

The [query-cost profile](../../research/explicit_relations_profile_20261031/RESULTS.md)
replayed 240 queries in 30 cells with exact response equality on the same full
63-bit tables. Lookup and normalization account for the largest instrumented
block-cycle intervals; these shares include synchronization and cannot partition
production wall time. The [two-page PDF](query-profile-v1-report.pdf),
[SVG](query-profile-v1-stages.svg) and [150-dpi PNG](query-profile-v1-stages.png)
are hash-bound diagnostic companions to the affine comparison, not a new speedup
claim. All five v1 external-profiler setup failures are retained; a separate
frozen supplement repairs their prerequisites without repeating the panel.

The [profiler-only follow-up PDF](query-profile-v2-report.pdf) records two
successful Nsight Systems traces: kernel coverage 99.10% / 98.70% of their
first-to-last kernel spans, with exact replies preserved. Both Nsight Compute
attempts were refused by provider hardware-counter permissions; no counters
were estimated. [Seed101 graph](query-profile-v2-kernels-101.svg) and
[seed202 graph](query-profile-v2-kernels-202.svg), plus corresponding PNGs,
are byte/hash-bound presentation copies. Raw supplement was pushed as
`5d4713f59fa3` before rendering; both PDF pages were visually inspected.

The [uninstrumented full-cache comparison](../../research/explicit_relations_fullcache_20261101/RESULTS.md)
retains33 verified recoveries on one RTX4090: all30 small executions and the
complete first large matched set. Median paired cold time was16.75%/17.43%
lower on the small inputs and10.10% lower on the completed large input; its
baseline controls differed by0.012%. These are descriptive L0 results from
completed sets. External pod deletion interrupted the second large input;
one interrupted attempt and two unstarted cells remain explicit. The original
whole-panel analysis refuses the incomplete coverage; the separate retained-set
analysis is labelled post-interruption and unblinded. The [two-page PDF](fullcache-v2-report.pdf),
[SVG](fullcache-v2-scaling.svg) and [PNG](fullcache-v2-scaling.png) preserve that
scope. Raw evidence was pushed as2ef5d57e0e01 before rendering, and both PDF
pages were inspected. The second large input's full matched comparison remains
open; no replacement GPU is running while its external deletion is clarified.

### Full-cache query specialization: second large comparison completed

The version3 continuation completed the original 63-bit seed202 matched set on
one fresh RTX 4090. Cold times in execution order were 1183.823571s (baseline x),
1053.896752s (candidate v), and 1185.763838s (baseline b). Candidate/mean-baseline
ratio 0.8895192034 corresponds to 11.05% lower cold time; A/A spread 0.164%.
All three recoveries used 344 relations and 13,733 attempts, have identical certificates,
and passed fresh CPU replay plus corrupted-scalar rejection. One fixed input,
one round, L0 descriptive clocks; no cross-host timing pooling.

[Report](fullcache-v3-report.pdf), [plot](fullcache-v3-scaling.svg),
[PNG](fullcache-v3-scaling.png), and
[raw/export/protocol](../../research/explicit_relations_fullcache_20261101/RESULTS.md).
The frozen analysis replay and export/schema/file/coverage checks pass. Prior
interruption and payment-refusal records are retained. The worker was deleted
and absence verified; request through cleanup took 3780.073 seconds, below the
9000-second worker / 9300-second controller cap. No GPU remains running.
