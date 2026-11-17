# Affine subtraction: matched RTX 4090 results

The opt-in affine subtraction completed all 12 prescribed recoveries. Every
result passed fresh CPU certificate replay and scalar+1 rejection. For each
of the four fixed inputs, the baseline, affine candidate and baseline repeat
produced byte-identical certificates and equal relation/query counts.

The two 52-bit targets took less cold solve time with affine subtraction.
The 63-bit results were mixed: seed 101 took slightly more time and seed 202
took less. The option remains off by default; this panel does not establish a
uniform large-target improvement.

| Field bits | Seed | Baseline b (s) | Baseline repeat x (s) | Affine v (s) | Change versus mean baseline | Baseline repeat spread |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 52 | 101 | 7.213224 | 7.177102 | 6.979429 | -2.998% | 0.502% |
| 52 | 202 | 5.238853 | 5.241295 | 5.163135 | -1.468% | 0.047% |
| 63 | 101 | 2636.991553 | 2636.255738 | 2644.849780 | +0.312% | 0.028% |
| 63 | 202 | 1207.318937 | 1206.938904 | 1190.595553 | -1.370% | 0.031% |

Times are recorded cold solve seconds. Percentage change is
100 * (candidate / mean(b,x) - 1); negative means lower observed time.
Repeat spread is 100 * abs(b-x) / mean(b,x), not a confidence interval.
These are per-input descriptive L0 shared-host observations (`wall_valid=false`),
with one candidate and two baseline executions per input. There is no pooling
or cross-panel timing inference. Values come from the frozen native analyzer's
[ratios.csv](results-v1/ratios.csv); full audit times, including fresh replay
and the negative control, remain in [rows.csv](results-v1/rows.csv).

Both 63-bit targets recovered: seed 101 used 338 relations and 15,737 queries;
seed 202 used 344 relations and 13,733 queries. The affine oracle totals were
2,644.156 and 1,189.984 seconds, while linear algebra took 0.114 and 0.091 seconds.
Next, measure the costs within relation queries (coordinate conversion,
normalization and index lookup) before choosing another optimization.

## Scope and checks

The sole varied factor was the full-cache subtraction formula. Ordinary affine
storage, explicit bases, target points, table layouts, filtering, normalization,
search minima and limits were held fixed. The established EFD formula and
exceptional cases are documented in [AFFINE-QUERY.md](../../gpu/explicit-relations/AFFINE-QUERY.md).
This is generic decomposition with explicit x-coordinate factor bases.

- One RTX 4090, 24,564 MiB, driver 595.91.07, CUDA 13.2 / sm_89.
- Fields 52 and 63 bits; seeds 101 and 202; 12 slots; tau=ceil(7R/5)+1.
- Query multiplier 128, max 16 trials; unchanged memory/work limits in the protocol.
- 29 Rust tests, host/device differentials, Clippy, full service/certificate/cap checks,
  and all 39 original plus affine sanitizer checks passed before measurements.
- The independent affine-law fixture passed 37,584 cases on CPU and GPU.
- The native analyzer checked all 12 unique planned rows and equal relation/query counts.
  Its summary, ratios and SVG were recomputed byte-identically after collection.
- Canonical schema and coverage/file verification both passed with zero findings.
- The source archive and final archive passed integrity checks. The controller exited 0;
  deletion of pod `hnvmwjamem9ox9` was confirmed against fresh complete provider inventory.

Source: `333712634ae2fa0694236a530b7a5efd1be3efc5`.
[Protocol](protocol.json), [export](results-v1/export.json),
[certificate audit](results-v1/certificate-audit.json),
[custody check](results-v1/custody-check.json), and
[archive manifest](results-v1/archive-manifest.json) retain the evidence.
The complete archive is at
`s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/affine-20261030-v1/results.tgz`.
Download it, check its SHA-256 against the manifest, and extract into a fresh directory.
`tools/artifacts/artifacts.py pull` hydrates the separately stored validation log.

No local root crypto plug-in build was performed. GPU compilation and validation
ran on the allocated host. This formula-only comparison did not prescribe an
external-tool or matched-rho timing arm and makes no such comparison claim.
The 63-bit rung is the completed prescribed range, not a global hardware maximum.

## Visual record

The [PDF](reports/run-20261030T2344Z/report.pdf) and
[SVG](reports/run-20261030T2344Z/scaling.svg) describe every execution.
Raw records were pushed before report rendering; the report records that commit.
