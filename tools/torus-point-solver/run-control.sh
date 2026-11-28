#!/usr/bin/env bash
set -euo pipefail
binary=${1:?immutable executable}
output_dir=${2:?new output directory}
backend=${3:?pinned native cvc5}
mkdir "$output_dir"
"$binary" --synthetic "$backend" > "$output_dir/synthetic.jsonl" 2> "$output_dir/synthetic.stderr"
"$binary" --backend "$backend" > "$output_dir/backend.jsonl" 2> "$output_dir/backend.stderr"
"$binary" --public "$backend" > "$output_dir/public.jsonl" 2> "$output_dir/public.stderr"
