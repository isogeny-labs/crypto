#!/usr/bin/env bash
# Independent implementation of exported-assignment evaluation, not a solver.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
target_dir=${1:-/tmp/prime-independent-check}
mkdir -p "$target_dir"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/check_export.cpp" -o "$target_dir/check_export"
g++ --version | head -1
sha256sum "$here/check_export.cpp"
sha256sum "$target_dir/check_export"
count=0
for file in "$here"/../prototype/exports/*.ms; do
    "$target_dir/check_export" "$file" "${file%.ms}.assignment.tsv"
    count=$((count + 1))
done
test "$count" -eq 6
sample="$here/../prototype/exports/m5_t4_remainder"
awk 'BEGIN {FS=OFS="\t"} NR==2 {$2=($2+1)%17} {print}' "$sample.assignment.tsv" > "$target_dir/corrupt.tsv"
if "$target_dir/check_export" "$sample.ms" "$target_dir/corrupt.tsv"; then
    echo 'ERROR: corrupt witness accepted' >&2; exit 1
else
    code=$?; test "$code" -eq 1; echo 'PASS corrupt witness rejected with nonzero equations'
fi
head -2 "$sample.assignment.tsv" > "$target_dir/incomplete.tsv"
if "$target_dir/check_export" "$sample.ms" "$target_dir/incomplete.tsv"; then
    echo 'ERROR: incomplete witness accepted' >&2; exit 1
else
    code=$?; test "$code" -eq 2; echo 'PASS incomplete witness rejected as malformed'
fi
head -2 "$sample.ms" > "$target_dir/empty.ms"
if "$target_dir/check_export" "$target_dir/empty.ms" "$sample.assignment.tsv"; then
    echo 'ERROR: empty system accepted' >&2; exit 1
else
    code=$?; test "$code" -eq 2; echo 'PASS empty system rejected as malformed'
fi
echo 'PASS six exported assignments; three negative controls; no solving or timings'
