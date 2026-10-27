# Local validation

The complete `TMPDIR=/Volumes/SSD990-2/.codex/n83-fresh-cold-tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/n83-fresh-cold-target cargo test --release --lib --locked -- --test-threads=8` run passed: 4,162 passed, 117 ignored, zero failed.
`cargo test --release --locked --example koblitz_orbit_dlp_fast` passed 21 tests; `cargo test --release --locked --example n83_rho_optimization_eval` passed 4 tests, using the same TMPDIR and target directory.
The four package Python suites, sealed dispatch check, and fresh-point exclusion audit passed. The 547,800-point K3300 pilot independently replayed its base, rank, relation, scalar and source hash. The K2304 and K1107 archive bases independently replayed, uploaded to S3 and passed fresh download SHA checks; their receipt is bound in `dispatch.json`.

The first IC-example validation attempt failed while writing a Rust metadata file to the full default `/Volumes/SSD990/llm/tmp` filesystem. A serial library run was stopped after that infrastructure failure, and the complete library and example suites were rerun with TMPDIR on SSD990-2. These first-attempt logs are retained below; they are infrastructure/interrupt receipts, not test failures.

| Compressed log | Uncompressed SHA-256 | Bytes |
| --- | --- | ---: |
| `lib_release_parallel.log.gz` | `a82422369953f0ee9bc6c24cf7b569d60b9f246b60f2b267c8ae074454a56944` | 357312 |
| `ic_example_release.log.gz` | `c05f837887b75cdfbfe638e6bcdc57fc6de540450bdb3ed564a8d6998df7279b` | 2281 |
| `rho_example_release.log.gz` | `6630822006bae3d33c5966a4f09309282caee80845bace3448f823d73a9aabc2` | 774 |
| `python_preflight.log.gz` | `1b96bbac52f9c39bb35fa190f278cb27028955de9da8e04c05920ae7f9242378` | 1101 |
| `initial_ic_example_temp_full.log.gz` | `abd22c06142fe4949bd9dcd716d93503d4a48e5ee6ba1a388c24092362200539` | 309 |
| `interrupted_serial_lib.log.gz` | `ea39b9f57d9cd485cf37f0e1bd20c73c45fa0187221e9b78318445094f23ce9c` | 134532 |
