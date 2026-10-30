# Commands and scope

Working tree: `/workspace/crypto-next`, initially based on
`4711f1c73887f01ef8f348d98d57e0c09b221afd`. Native catalog outputs are committed
at `a11984a573441d5bf0d60ab1d26cfc34e71badf5`; the remaining changed sources and
outputs are identified by `inputs-after.sha256`, rather than attributed to an
earlier Git revision.

The task-local Rust installation used:

```sh
export RUSTUP_HOME=/workspace/twist-tooling/rustup
export CARGO_HOME=/workspace/twist-tooling/cargo
export PATH="$CARGO_HOME/bin:$PATH"
export CARGO_BUILD_JOBS=2
```

Commands, from the repository root:

```sh
conductor check --summary 'Fix quadratic-twist catalog integration regressions in PR 2381' \
  --scope path:docs/curves path:src/bin/hyperelliptic_cover.rs \
  path:research/quadratic_twist_evidence_20261021/validation \
  path:scripts/build_cover_transfer_assessments.mjs
node scripts/build_cover_transfer_assessments.mjs --check
cargo test --release --locked --bin hyperelliptic-cover \
  tests::every_current_catalog_model_reaches_the_standalone_constructor -- --exact
cargo run --release --locked --bin curve_cover_check
target/release/curve_cover_check --check
cargo test --release --locked --lib --bin hyperelliptic-cover \
  --bin curve_cover_check --bin curve_standards
cargo run --release --locked --bin curve_standards -- --check
bash research/quadratic_twist_evidence_20261021/reports/catalog-20261022/build.sh
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

The first two reproduction checks fail before the fix (assessment exit 1,
constructor exit 101). All final checks above pass, with the Conductor command
still unavailable (exit 127). AJV's strict-schema lint option is disabled to
validate the repository's existing schema as written; instance validation still
uses the complete Draft 2020-12 contract and all errors. CI's schema job retains
its existing pinned validator and is not changed.

The library tests and Node commands spawning Git ran with the managed sandbox's
network permission, including the existing localhost tests. No benchmark or
wall-clock comparison was performed. Field/model geometry generation and replay
use the existing Rust checker; Node runs the existing assessment/metadata checks.
No Python research path, including the legacy geometry scripts, was executed.

The preservation comparison joins every prior finding by full model hash, checks
every old content-addressed map/cover object unchanged, and retains every old EC1
representation. It compares the registry bytes to the original PR head and checks
that frozen study runs, proofs, source archives, and dated reports have no diff.
The semantic comparison records all leaf changes in six claim sections and
rejects any changed support status. Both receipts are in this directory.
