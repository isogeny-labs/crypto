#!/usr/bin/env bash
set -euo pipefail
task_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
task_repo=$(git -C "$task_dir" rev-parse --show-toplevel)
task_out=${1:-/tmp/four-selector-receipts}
mkdir -p -- "$task_out"
task_out=$(cd -- "$task_out" && pwd)
task_stage=inputs
trap 'task_exit=$?; printf "{\"stage\":\"%s\",\"exit_code\":%d}\n" "$task_stage" "$task_exit" > "$task_out/status.json"' EXIT
cd -- "$task_dir"
sha256sum -c verification/inputs.sha256 > "$task_out/inputs.log" 2>&1
task_stage=build
lake --no-cache build > "$task_out/build.log" 2>&1
task_stage=warnings
for task_module in Recoding RecodedTop RecodedLower RecodedCanonical SupportOneHot SymmetricChord Determinantal DeterminantalTop Envelope FourTrackFixture; do
  lake env lean -DwarningAsError=true "$task_module.lean"
done > "$task_out/warnings.log" 2>&1
task_stage=axioms
lake env lean --run "$task_repo/tools/lean/AuditAxioms.lean" -- \
  CanonicalFixture DegreeBridge SecantFixture RelationDegree SecantIdeal \
  SecantIdentity SecantTopForms DenseTopForms SecantQuadratic SecantGraphTop \
  TopDegree SecantGroupLaw SplitFactorBase PencilSelector PencilTop \
  PencilCanonical PencilElimination PencilFixture \
  Recoding RecodedTop RecodedLower RecodedCanonical SupportOneHot SymmetricChord \
  Determinantal DeterminantalTop Envelope FourTrackFixture > "$task_out/axioms.log" 2>&1
task_stage=native
(
  cd native
  rustfmt --edition 2021 --config skip_children=true --check main.rs
  cargo clippy --release --offline --locked --all-targets -- -D warnings
  cargo test --release --offline --locked
) > "$task_out/native.log" 2>&1
task_stage=export-replay
cargo run --manifest-path native/Cargo.toml --release --offline --locked -- \
  emit-fixture "$task_out/replayed-fixture" > "$task_out/export.log" 2>&1
gzip -cd fixture/systems.json.gz > "$task_out/expected-systems.json"
cmp "$task_out/expected-systems.json" "$task_out/replayed-fixture/systems.json"
diff -r --exclude=systems.json --exclude=systems.json.gz fixture "$task_out/replayed-fixture" \
  > "$task_out/export-diff.log" 2>&1
task_stage=complete
