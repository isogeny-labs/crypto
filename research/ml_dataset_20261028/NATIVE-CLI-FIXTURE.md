# Current startup CLI correctness runner

Use the [native Rust successor](startup-integration-cli-fixture-native/README.md)
for new CLI correctness checks. Build it as Cargo example
`dataset_startup_cli_fixture`; it does not delegate cryptographic work,
correctness checks, tampering or orchestration to Python.

The older `startup-integration-cli-fixture/` is a frozen historical source
snapshot bound by its own protocol and archived execution. Its files and
earlier results are preserved unchanged. Its Python command is historical
provenance, not the current execution path. The successor has a distinct
protocol/manifest and fresh output directories; it never reseals old evidence.

Both generations cover the same fixed software assertions, including two
twists and hash-valid semantic corruption. No local test output is a cloud
dataset, research-corpus row, cryptanalytic result or performance claim.
