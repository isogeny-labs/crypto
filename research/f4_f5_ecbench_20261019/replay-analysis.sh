#!/usr/bin/env bash
set -euo pipefail
study=research/f4_f5_ecbench_20261019
build=${CARGO_TARGET_DIR:-target}
bin=$(mktemp -d "${TMPDIR:-/tmp}/boolean-recursive-analysis.XXXXXX")
trap 'rm -rf "$bin"' EXIT
cargo build --release --locked --lib --message-format=json > "$bin/build.jsonl"
# Use exactly the artifact from the root graph, including arbitrary_precision
# and raw_value. An unrelated default-feature rlib may coexist in the target.
serde=$(jq -r 'select(.reason=="compiler-artifact" and .target.name=="serde_json") | .filenames[] | select(endswith(".rlib"))' "$bin/build.jsonl")
test -n "$serde"
for name in analyze-native analyze-ecbench analyze-profiles analyze-lookup analyze-screen component; do
  rustc --edition 2021 -O "$study/$name.rs" -L "dependency=$build/release/deps" --extern "serde_json=$serde" -o "$bin/$name"
done
"$bin/analyze-native" "$study/runs/native/out" > "$study/native-summary.json"
"$bin/analyze-ecbench" "$study/runs/ecbench/out" > "$study/ecbench-summary.json"
"$bin/analyze-profiles" "$study/runs/profiles" > "$study/profiles-summary.json"
"$bin/analyze-lookup" "$study/runs/lookup-profiles" > "$study/lookup-summary.json"
if [ -f "$study/runs/iteration1-native/out/native-phase.status" ]; then
  "$bin/analyze-native" "$study/runs/iteration1-native/out" > "$study/iteration1-native-summary.json"
fi
if [ -d "$study/runs/iteration1-ecbench/out" ]; then
  "$bin/analyze-ecbench" "$study/runs/iteration1-ecbench/out" > "$study/iteration1-ecbench-summary.json"
fi
"$bin/analyze-native" "$study/runs/iteration2-native/out" > "$study/iteration2-native-summary.json"
"$bin/analyze-ecbench" "$study/runs/iteration2-ecbench/out" > "$study/iteration2-ecbench-summary.json"
"$bin/analyze-screen" "$study" > "$study/adaptive-screen-summary.json"
"$bin/analyze-screen" "$study" versioned-screen > "$study/versioned-screen-summary.json"
"$bin/component" "$study" > "$study/component.json"
