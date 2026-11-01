#!/usr/bin/env bash
# Full bounded replay plus independent arithmetic/enumeration and controls.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
frozen="$study/native/mixed-20261022T0718Z"
out=${1:?usage: run-mixed.sh NEW_OUTPUT_DIRECTORY}
mkdir -p "$out"
out=$(cd "$out" && pwd)
test ! -e "$out/results.json" || { echo 'Refusing to replace an earlier run' >&2; exit 2; }
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/mixed.cpp" -lcrypto -o "$out/mixed"
timeout 300 "$out/mixed" docs/curves/registry.json "$out/results.json"
cargo build --release --locked --manifest-path "$study/native/mixed-audit/Cargo.toml"
auditor="$study/native/mixed-audit/target/release/quadratic-twist-mixed-audit"
timeout 300 "$auditor" "$out/results.json" docs/curves/registry.json "$out/audit.json"
cmp "$frozen/results.json" "$out/results.json"
cmp "$frozen/audit.json" "$out/audit.json"
"$auditor" certify "$study/lean/TwistEmbedding.lean" "$out/certificate"
cmp "$frozen/certificate/cleared-monic-twist.json" "$out/certificate/cleared-monic-twist.json"
cmp "$frozen/certificate/receipt.json" "$out/certificate/receipt.json"
"$auditor" controls "$out/results.json" "$out/controls"
for control in false-coefficient missing-model; do
  set +e
  timeout 300 "$auditor" "$out/controls/$control.json" docs/curves/registry.json \
    "$out/controls/$control-audit.json" > "$out/controls/$control.log" 2>&1
  control_exit=$?
  set -e
  echo "$control_exit" > "$out/controls/$control.exit"
  test "$control_exit" -ne 0 && test "$control_exit" -ne 124
  test ! -e "$out/controls/$control-audit.json"
done
rg -q 'recorded relation is not zero on the actual curve' "$out/controls/false-coefficient.log"
rg -q 'missing model coverage' "$out/controls/missing-model.log"
sha256sum "$out/results.json" "$out/audit.json" "$out/controls/false-coefficient.json" \
  "$out/controls/missing-model.json" > "$out/output.sha256"
echo 'PASS: full mixed-word replay, independent ranks, exact bytes, false relation and coverage controls'
