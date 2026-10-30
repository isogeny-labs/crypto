#!/usr/bin/env bash
set -euo pipefail
study_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
out_dir="${1:?usage: reproduce.sh NEW_OUTPUT_DIRECTORY}"
if [[ -e "$out_dir" ]]; then
  echo 'Refusing to overwrite an existing replay directory' >&2
  exit 2
fi
mkdir -p "$out_dir"
rustc --edition 2021 -O -D warnings "$study_dir/native/kernel_screen.rs" -o "$out_dir/kernel-screen"
timeout 300 "$out_dir/kernel-screen" "$out_dir/screen-v1" > "$out_dir/stdout.log" 2>&1
cmp "$study_dir/results/screen-v1/counts.csv" "$out_dir/screen-v1/counts.csv"
cmp "$study_dir/results/screen-v1/modules.jsonl" "$out_dir/screen-v1/modules.jsonl"
cmp "$study_dir/results/screen-v1.log" "$out_dir/stdout.log"
echo 'PASS: exact counts, witnesses and stdout replay'
