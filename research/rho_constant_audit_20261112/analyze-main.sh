#!/usr/bin/env bash
# Main analysis frozen before main data; pilot analysis remains unchanged.
set -u
binary=$(realpath "$1")
session=$(realpath "$2")
output=$3
script_dir=$(cd "$(dirname "$0")" && pwd)
bash "$script_dir/analyze.sh" "$binary" "$session" "$output"
parent_rc=$?
output=$(realpath "$output")
"$binary" db sql "$session" > "$output/index.sql" 2> "$output/index.stderr"
index_rc=$?
printf '%s\n' "$index_rc" > "$output/index.exit"
/workspace/scratch/rho-sql-analysis "$output/index.sql" "$script_dir/native/queries.sql" > "$output/native.jsonl" 2> "$output/native.stderr"
native_rc=$?
printf '%s\n' "$native_rc" > "$output/native.exit"
test "$parent_rc" -eq 0 && test "$index_rc" -eq 0 && test "$native_rc" -eq 0
