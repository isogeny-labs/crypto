# Attempt 03 layout

- `preflight.json`: frozen cohort, input hashes, execution/cost limits, and stop rules.
- `Cargo.toml`, `Cargo.lock`, `source-pdp.rs`, `source-top.rs`: exact offline build inputs; source files are pinned by SHA-256.
- `build-exact.sh`, `build-inputs.sha256`: clean isolated build and exact binary-hash gate.
- `execute.sh`, `launch.sh`, `run-spike38-topform.sh`: bounded controller, build gate, scientific generator/replay, and durable upload.
- `candidate-inputs/`: generated 48 deterministic systems with their generation records.
- `top-certificates/`: top-form quotient Dreg certificates and replay outputs.
- `instruments/`: binaries actually used.
- `build.log`, `build-receipt.json`, `generate.log`, `top-single.log`, `top-replay.log`, `terminal.json`, `controller-status.json`: complete run diagnostics and terminal state.
- `manifest.sha256`, `S3_PERSISTENCE_INDEX.json`: byte/hash inventory and exact S3 URI mapping.
