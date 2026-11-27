# Source-pinned degree-32 PKM replay — 2026-11-24

Replay the already frozen fresh-instance workload at CLI seed 20261149 with the source-pinned binary from `SOURCE_BUILD_RECEIPT_20261124.md`, SHA-256 `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`. Use exactly 16 repeats, ordinary t5 auxiliary seed 5, `prime-abscissa:below=32`, m=2 Semaev F4 chain mode, degree bound 8, 1,000 target trials, five-second solver-call budget and one rho context run. Size-only preflights must reproduce the same instance and equal base size before timing.

Run interval, candidate, candidate, interval under the existing 240-second whole-call cap. Preserve every command, full JSON, external wall and child CPU in a new append-only store. Require all four invocations to exit successfully, use the same instance and base size, and verify the same 16 scalars. The replay passes if candidate/interval charged GAE, external wall and child CPU are below one in both mirrored pairs and combined charged GAE is below one. Compare exact relation counts, target trials and solver-operation counts with `degree32_replication_extend.jsonl`; differences must be reported rather than normalized away.

This checks source/build reproducibility on the reused workload. It is not a new instance replication, full cold same-target crossover, or wide-curve transfer result.
