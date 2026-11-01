#!/usr/bin/env bash
set -euo pipefail
root=$(cd -- "$(dirname -- "$0")" && pwd)
output=${1:?provide a fresh absolute output directory}
mkdir "$output"
cargo build --offline --locked --manifest-path "$root/Cargo.toml" --target-dir "$output/build" > "$output/build.stdout" 2> "$output/build.stderr"
"$output/build/debug/pencil-native-input-custody" "$output/fixtures" > "$output/controls.json" 2> "$output/controls.stderr"
