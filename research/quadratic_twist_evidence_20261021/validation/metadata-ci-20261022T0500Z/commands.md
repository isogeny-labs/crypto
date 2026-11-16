# Commands and provenance

Started from `3e442fd3310d192495e5277a278aa2a0c0aab605` in
`/workspace/crypto-next`. Exact active source bytes and catalog inputs are bound
by `inputs-after.sha256` and `preservation.json`. The prior catalog dependency at
`a11984a573441d5bf0d60ab1d26cfc34e71badf5` is retained by explicit hash checks.
Corrected catalog inputs are committed at
`228cdc0d50f2e4a2058f6fb93d6cc7f535e2692d`; the following commit refreshes the
assessment and source-bound validation without changing those inputs.

```sh
export RUSTUP_HOME=/workspace/twist-tooling/rustup
export CARGO_HOME=/workspace/twist-tooling/cargo
export PATH="$CARGO_HOME/bin:$PATH"
export CARGO_BUILD_JOBS=2
```

Commands from the repository root (each exit is recorded beside its output):

```sh
conductor check --summary 'Check PR 2381 standards metadata join and exact-head inherited Lean blocker' \
  --scope path:docs/curves --scope path:research/quadratic_twist_evidence_20261021 \
  --scope path:problems/quadratic-twist-evidence-native-parity.md
python3 scripts/update_curve_standards.py --check   # reproduces metadata-layout failure
python3 scripts/update_curve_standards.py           # metadata only, then repeated after display-label fix
python3 scripts/update_curve_standards.py --check
python3 scripts/test_cover_catalog.py
python3 scripts/test_build_lab_browser.py
python3 scripts/build_lab_browser.py --check
python3 scripts/build_lab_browser.py
python3 scripts/build_lab_browser.py --check
cargo test --release --locked --manifest-path research/quadratic_twist_evidence_20261021/native/metadata/Cargo.toml
bash research/quadratic_twist_evidence_20261021/test-metadata.sh \
  research/quadratic_twist_evidence_20261021/native/runs/run-20261021T150125Z \
  research/quadratic_twist_evidence_20261021/validation/metadata-ci-20261022T0500Z/metadata-replay
bash research/quadratic_twist_evidence_20261021/validation/metadata-ci-20261022T0500Z/test-fresh-import.sh
cargo run --release --locked --bin curve_cover_check
target/release/curve_cover_check --check
cargo run --release --locked --bin curve_standards -- --check
cargo test --release --locked --lib --bin hyperelliptic-cover --bin curve_cover_check --bin curve_standards
node research/quadratic_twist_evidence_20261021/validation/metadata-ci-20261022T0500Z/verify-preservation.mjs
bash research/quadratic_twist_evidence_20261021/reports/metadata-20261022T0500Z/build.sh
node scripts/build_cover_transfer_assessments.mjs --write
node scripts/build_cover_transfer_assessments.mjs --check
node research/cover_transfer_assessment_20261013/verify_assessment.mjs
node research/quadratic_twist_evidence_20261021/validation/catalog-ci-20261022/membership-control.mjs
env npm_config_cache=/workspace/twist-tooling/npm npm exec --yes \
  --package=ajv-cli@5.0.0 --package=ajv-formats@3.0.1 -- \
  ajv validate --spec=draft2020 -c ajv-formats --strict=false --all-errors \
  -s docs/curves/cover-transfer-assessment.schema.json \
  -d docs/curves/cover-transfer-assessments.json
```

Two more pre-edit Conductor checks, with the catalog/assessment and display
scopes, also exited 127; their logs are retained. No reservation is asserted.
The managed sandbox's network permission was enabled for Cargo's existing
localhost tests and Git-spawning metadata checks. No new cloud job was launched.

The metadata-only standards adapter calls only its existing JSON join and
catalog display path, preserving the frozen board instead of rebuilding research
measurements. The legacy arithmetic registry builder and Python research tests
were not run. AJV validates the complete existing schema; `strict=false` changes
schema linting only, without dropping instance assertions. CI keeps its existing
schema validator and no workflow gate is changed.

GitHub evidence was read through the connected GitHub tool: exact-head workflow
runs, all check runs (`per_page=100`, total 89), both failing jobs, SEA's original
receipt ZIP and PR reviews/threads/state. No CI jobs were polled or waited on.
