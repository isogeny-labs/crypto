# Complete-witness source and build receipt

The worker was built from the clean pre-stack execution commit
`f68b9a94ada669360f88be4d8c8a5288cca55d73`. Its five compiled source
files are listed in [source-files.sha256](source-files.sha256); all five passed
the [checkout check](source-check.log) and the separate
[snapshot check](source-snapshot-check.log). The worker's read-only snapshot
[attestation](source-attestation.json) names that commit. The five retained
factor-base input files passed [input-check.log](input-check.log) against
[inputs-before-run.sha256](inputs-before-run.sha256), including the exact
83,701,045-byte compressed object.

The native pre-stack exporter `cargo check --locked --no-default-features
--example koblitz_n83_factor_base_export` passed in 25.39 seconds
([log](cargo-check-prestack.log)). `cargo fmt --all -- --check` passed
([log](cargo-fmt-prestack.log)). The new seven-block core test passed 1/1
([matched log](core-test-matched.log)); the preceding `--exact` invocation
selected zero tests and is retained [separately](core-test.log). The touched
exporter suite passed 19/19 in 63.85 seconds ([log](example-test.log)),
including the degree-83 group-lift check.

The offline `rust:1.93.1-bookworm` arm64 release build passed in 2 minutes
22 seconds, exit 0, no OOM ([container state](linux-build.container-state.json),
[stderr](linux-build.stderr.log)). The resulting worker binary has SHA-256
`5345802d80a1ec7bfee04be5913a15536ec355a388debfde3d543db1ca4b1318`
([digest file](worker-binary.sha256)). Its build target was on the SSD. Build
time is separate from the one-CPU experimental run and from retained-base
construction.

The current stacked head's `cargo check --locked --no-default-features
--example koblitz_n83_factor_base_export` exited 101 before checking the
example because of 25 inherited library errors in unchanged ecbench/Gaudry
files ([log](cargo-check-stacked.log)). That head is not a passing validation
of the worker; the experiment is explicitly bound to the clean pre-stack
source commit above. The same three touched Rust files are byte-identical on
the execution and stacked branches. No complete-pipeline timing or factor-base
quality claim follows from these build gates.
