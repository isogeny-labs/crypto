#!/usr/bin/env bash
set -euo pipefail
binary=${1:?immutable executable}
output_dir=${2:?fresh output directory}
mkdir "$output_dir"
"$binary" --out "$output_dir/controls.jsonl" > "$output_dir/launcher.stdout" 2> "$output_dir/launcher.stderr"
