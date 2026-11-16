# GPU relation-query breakdown

All **30 replay cells / 240 queries** matched the uninstrumented protocol
responses exactly on one RTX 4090. The profile retained the original full
63-bit tables and the first eight actual queries for each seed, with five
interleaved stock/staged/stock-repeat rounds. All 16 distinct queries were
misses, so these are full-table miss scans. Successful hits were covered by
the separate validation fixtures. This is a fixed-prefix diagnostic.

| Instrumented block-cycle interval | Seed 101 | Seed 202 |
|---|---:|---:|
| Cache load and input Montgomery conversion | 2.72% | 4.72% |
| Affine subtraction | 3.11% | 3.72% |
| Product-tree batch normalization | 32.22% | 36.55% |
| Affine reconstruction and output conversion | 1.80% | 1.98% |
| Negative filter and exact index lookup | 43.60% | 31.12% |
| Empty barrier/clock interval | 16.54% | 21.91% |

Lookup and normalization are the largest named work intervals. The two
conversion-related intervals together account for about 4.5% and 6.7%; these
also include the cache load and affine reconstruction. The empty interval is
substantial, so cycle shares must not be converted into fractions of the
historical 2,644/1,190-second oracle times. Block intervals include scheduling,
barriers and compiler fences, and concurrent block cycles do not add to GPU
elapsed time. The empty interval is retained without subtraction.

The staged/stock search-kernel event ratios were **0.9334** and **0.9366**.
The staged implementation also specializes the full-cache path and changes
code generation: 58 registers / 56-byte stack versus stock 48 registers /
120-byte stack, with no reported spills. The ratio therefore does not isolate
timer overhead or establish a production improvement. A useful next change is
an **uninstrumented full-cache specialization**, followed by a matched complete
recovery panel. Lookup and normalization remain the main work to investigate.

Checks passed: 37,584 host and 37,584 GPU arithmetic cases; 315 profile query
cases (261 hits, 54 exhausted misses); memcheck, racecheck and synccheck with
zero errors; 29 Rust tests and Clippy; native analysis recomputed byte-identically;
canonical export schema and coverage/file validation with zero findings.

The five optional external profiler commands in v1 exited 2 before profiling:
`tools/profile.sh` required the missing `gawk` package. Their logs and exits are
preserved. A separate [version-2 protocol](protocol-v2.json) installs the
prerequisites and runs only the external profiles against the saved transcripts.
Its outcomes are recorded separately; the v1 replay panel is not repeated.

- [Raw export](results-v1/export.json), [native summary](results-v1/summary.txt),
  [per-query data](results-v1/stages.csv), [verification](verification-v1.json).
- [Two-page report](reports/run-20261031T0730Z/report.pdf) and
  [quantitative graph](reports/run-20261031T0730Z/stages.svg). Both PDF pages were
  visually inspected after raw commit `b50e30d47458` was pushed.
- Source `ffa0da3efaf222bdcc958d45013aa469d5fbecf7`; frozen protocol
  `8b77f5a916607e7f7a0d094d194515d55457823759b9114b97808f7c6416bb17`.
- Pod `vv62a7e003x4bk`: RTX 4090, 24,564 MiB, driver 595.91.07, $0.89/hour.
  Owned pod deletion was verified against a fresh complete provider inventory.

## External profiler supplement (v2)

Both Nsight Systems traces completed and their actual service response streams
match the saved prefixes byte for byte. The separately run, unprofiled service
also matched both prefixes. Source `b50e30d47458881eba10d802b7f972a9f56541bd`
retains the unchanged GPU implementation; the wrapper, package versions and
actual service binary hash are preserved in [results-v2](results-v2/export.json).

| Native Nsight Systems observation | Seed 101 | Seed 202 |
|---|---:|---:|
| Kernel coverage between first start and last end | 99.10% | 98.70% |
| Search-kernel launches | 8 | 8 |
| Search share of summed kernel time, including setup | 76.6% | 50.5% |
| Search-kernel total, profiled ms | 1,338.874 | 695.057 |

Launch gaps were small in these traces. The short prefixes include complete
one-time table construction, so their setup/query shares do not represent a
complete recovery run. Large host `cudaMemcpy` API totals include waiting for
the search result; they are not bulk-transfer times. For seed101, the GPU copy
trace recorded only 0.033888 ms device-to-host and 0.000736 ms host-to-device.

Both Nsight Compute attempts returned **unsupported / ERR_NVGPUCTRPERM**.
The independently checked probe kernel ran correctly and met the same counter
permission restriction. Hardware instruction, occupancy and warp-stall counters
are therefore unavailable; none are estimated. No driver permission was changed.

Nsight Systems 2025.6.3 and Nsight Compute 2026.1.1 were installed through pinned
CUDA13.2.2 metapackages. Native summaries, graphs and report sources regenerated
byte-identically. Canonical schema/coverage/file checks passed with zero findings.
Pod `fmu20lo21ib6gu` was deleted and absence verified. All six planned attempts
are recorded: two baseline successes, two trace successes and two counter
refusals; the capability probe is retained separately. Both study GPUs are gone.

The [two-page supplement report](reports/run-20261031T0746Z/report.pdf) includes
both native kernel-time graphs. Both pages were visually inspected after raw
commit `5d4713f59fa3` was pushed. Committed-state progress metrics report only
diagnostic coverage: 30 exact replay cells, 240 replies, two successful timelines
and two unavailable counter profiles; no timing rank or recovery count is added.
