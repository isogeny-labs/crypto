#!/usr/bin/env bash
set -euo pipefail
root=$(cd -- "$(dirname -- "$0")" && pwd)
out=${1:?fresh absolute output directory}
mkdir "$out"
bash "$root/subject/check.sh" "$out/producer"
cargo build --offline --locked --manifest-path "$root/reviewer/Cargo.toml" --target-dir "$out/build" > "$out/build.stdout" 2> "$out/build.stderr"
"$out/build/debug/independent-custody-controls" "$out/reviewer-fixtures" "$out/producer/fixtures" > "$out/controls.json" 2> "$out/controls.stderr"
jq -e '.accepted==true and .count==35 and (.rows|length)==35 and all(.rows[];.accepted==.expected_accept)' "$out/producer/controls.json"
jq -e '.accepted==true and .count==37 and (.rows|length)==37 and all(.rows[];.accepted==.expected)' "$out/controls.json"
