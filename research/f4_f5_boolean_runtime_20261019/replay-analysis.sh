#!/usr/bin/env bash
# Recompute summaries from archived receipts; no timing is repeated.
set -euo pipefail
study=research/f4_f5_boolean_runtime_20261019
build=${CARGO_TARGET_DIR:-target}
analyzers=$(mktemp -d /tmp/boolean-native-analysis.XXXXXX)
trap 'rm -rf "$analyzers"' EXIT
cargo build --locked --release -p serde_json
serde=("$build"/release/deps/libserde_json-*.rlib)
for name in analyze analyze-optimization analyze-profiles analyze-policy analyze-serial-clear analyze-serial-guard component; do
  rustc --edition 2021 -O "$study/$name.rs" -L "dependency=$build/release/deps" \
    --extern "serde_json=${serde[0]}" -o "$analyzers/$name"
done
"$analyzers/analyze" "$study/runs/initial/out" > "$study/initial-summary.json"
"$analyzers/analyze-optimization" "$study/runs/optimization/out" > "$study/optimization-summary.json"
"$analyzers/analyze" "$study/runs/final/out" > "$study/final-summary.json"
"$analyzers/analyze-profiles" "$study/profiles-corrected" > "$study/profiles-summary.json"
"$analyzers/analyze" "$study/runs/policy/out" > "$study/policy-comparison-summary.json"
"$analyzers/analyze-policy" "$study/runs/policy/out" > "$study/policy-summary.json"
"$analyzers/analyze-serial-clear" "$study/runs/serial-clear/out" > "$study/serial-clear-summary.json"
"$analyzers/analyze-serial-guard" "$study/runs/serial-guard/out" > "$study/serial-guard-summary.json"
"$analyzers/component" "$study" > "$study/component.json"
