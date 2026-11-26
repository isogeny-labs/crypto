#!/usr/bin/env bash
set -euo pipefail
binary=${1:?immutable executable}
output_dir=${2:?new output directory}
backend=${3:?pinned cvc5 executable}
mkdir "$output_dir"
"$binary" --synthetic "$backend" > "$output_dir/synthetic.jsonl" 2> "$output_dir/synthetic.stderr"
"$binary" --backend "$backend" > "$output_dir/backend.jsonl" 2> "$output_dir/backend.stderr"
"$binary" --curve "$backend" > "$output_dir/curve.jsonl" 2> "$output_dir/curve.stderr"
