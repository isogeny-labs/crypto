# Full-cache specialization: retained matched recoveries

The opt-in uninstrumented kernel completed 33 verified recoveries on one RTX 4090.
The completed matched sets used 16.75% and 17.43% less median paired cold time on
the two 52-bit inputs, and 10.10% less cold time on the completed 63-bit input.
The large baseline controls agreed within 0.012%. All completed matched
certificates, relation counts and attempt counts agree exactly.

| Field bits / seed | Rounds | Baseline median s | Candidate median s | Baseline-repeat median s | Median paired candidate / baseline mean |
|---|---:|---:|---:|---:|---:|
|52 /101|5|6.623034|5.581687|6.712873|0.832521|
|52 /202|5|4.957987|4.097162|4.937561|0.825690|
|63 /101|1|2591.625752|2329.970949|2591.933570|0.898985|
|63 /202|incomplete|not started|not started|interrupted|no complete comparison|

Cold solver time includes setup, relation queries, linear algebra and in-process
verification; fresh external replay is recorded separately. Small timing columns
are per-arm medians; the ratio is the median of paired ratios, not a ratio of
those medians. All clocks are L0 descriptions on this shared host. The large
input has one matched round. [Raw paired rows](retained-analysis-v2/ratios.csv)
and [statistics](retained-analysis-v2/statistics.csv) preserve the exact values.

RunPod deleted the worker at 2026-11-01T06:38:03Z during the second large input's
first baseline, before this lane's controller cleanup. The audit does not give
the initiating reason. The controller collected partial evidence, exited 93,
and verified pod absence. No replacement is running while the external deletion
is being clarified. Original worker budget remaining after conservative charges:
9,000 seconds; no new budget has been assumed.

| Stratum | Protocol planned | Actually attempted | Verified complete | Interrupted | Not started |
|---|---:|---:|---:|---:|---:|
|52-bit|30|30|30|0|0|
|63-bit|6|4|3|1|2|

The interrupted attempt has no complete clock. Both exports pass schema and
referenced-file checks. Small protocol coverage passes; large coverage retains
four findings for the infrastructure shortfall. The canonical schema requires
attempts>=1, so unstarted cells stay in the original protocol and
[interruption record](results-v2/interruption.json), not invented trial rows.
The full original analyzer correctly refuses this incomplete panel. The separate
[retained-set adapter](analyze-retained.rs) was filed after the interruption,
unblinded; it preserves the original formulas and exact completed-set checks.
Its [custody record](retained-analysis-v2/analysis-custody.json) labels it as
post-interruption descriptive analysis. A second execution reproduced all
outputs byte-for-byte. These are not substituted frozen whole-panel outputs.

GPU preflight passed 29 driver tests and Clippy, 37,584 host and 37,584 device
arithmetic cases, 315 direct query comparisons (261 hits / 54 misses), public backend
and forced-cache-fallback checks, both service/driver self-tests, and six clean
memcheck/racecheck/synccheck runs. Candidate PTX has no profiling kernel or clock
instructions. CUDA reports 40 registers/56-byte stack for the specialized kernel
versus 48 registers/120-byte stack for the generic kernel, with no spills.

Every completed recovery passed fresh CPU certificate replay and rejection of a
scalar+1 certificate. The completed large set used 338 relations and 15,737
attempts per arm. Source was clean at 0fe875d034b77fc9b9825af8217194aa2f4ccdee;
CUDA 13.2 and Rust 1.88.0 were pinned. No local root crypto/plugin build was run.
The focused committed-state metrics check passes, including the interruption
and two unstarted cells. The repository-wide exporter hit its 60 s functional
limit with both the changed and unchanged exporter on this workspace.

[Two-page report](reports/run-20261101T0412Z/report.pdf),
[small export](results-v2/export-small.json), [large export](results-v2/export.json),
and [archive manifest](results-v2/archive-manifest.json).
Raw records were pushed as 2ef5d57e0e01 before rendering; both PDF pages were
visually inspected. Hydrate larger logs with:

```sh
python3 tools/artifacts/artifacts.py pull research/explicit_relations_fullcache_20261101/results-v2
```

Version 1's stale-lock preflight failure remains preserved in its
[separate report](reports/run-20261101T0400Z/report.pdf). The second large input's
complete matched comparison is the remaining measurement task.
