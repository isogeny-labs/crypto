# Batched S3 roots accelerate the sparse-selector K512 index stage

With the high-K canonical selector enabled, batching preserved the complete
N83 K512 compact state and root-table bytes and reduced index-build time in
both paired local runs. The native [analysis](local_sparse_pilot1/analysis.json)
validated all exits, the exact source and base hashes, 10,879,232 retained
states, 21,756,398 indexed roots, 84,992 factor-base points, and identical
full BLAKE3 state and table digests across all four processes.

| Fixed order | Builder | Index build (ms) | Setup (ms) | Exit |
| --- | --- | ---: | ---: | ---: |
| 1 | Scalar | 36,191.920 | 36,996.954 | 0 |
| 2 | Batch | 21,762.306 | 22,624.197 | 0 |
| 3 | Batch | 30,547.474 | 31,467.237 | 0 |
| 4 | Scalar | 40,985.226 | 41,903.476 | 0 |

The paired scalar/batch index ratios were **1.663** and **1.342**. Mean index
time was 38,588.573 ms scalar and 26,154.890 ms batched, a 1.475 ratio of
these descriptive means. Setup totals include base loading and the field
self-test. The [raw receipts](local_sparse_pilot1/) retain source, binary and
base hashes, commands through the frozen runner, timestamps, and process
resource logs. The host was arm64 macOS with load average 75.26 at the start;
this is an ordinary-host stage diagnostic with `candidate_id: null`.

The earlier [non-sparse result](RESULT.md) measured the same base under a
different canonical selector and saw no gain. The next test should preserve
the sparse selector and measure both builders on K2816 under a frozen Tier 1
allocation. A complete IC candidate comparison must then charge factor-base
and index setup separately, run fresh same-point online IC/rho pairs, replay
every scalar, and apply the benchmark tier gates before selecting a pipeline.
