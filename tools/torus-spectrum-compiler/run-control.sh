#!/usr/bin/env bash
set -euo pipefail
binary=${1:?immutable executable}
output_dir=${2:?new output directory}
mkdir "$output_dir"
"$binary" "$output_dir" > "$output_dir/controls.jsonl" 2> "$output_dir/stderr.log"
if [[ $# -eq 3 ]]; then
  "$binary" --backend-admit "$3" > "$output_dir/backend-admission.jsonl" 2> "$output_dir/backend-stderr.log"
fi
