#!/usr/bin/env bash
# Reproducer for the bounded exact component qualification, not a benchmark.
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: run_retained_checks.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
output_dir=$(cd "$1" && pwd)
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/retained_basis.cpp" -lgmpxx -lgmp -lcrypto -o "$output_dir/check" > "$output_dir/build.log" 2>&1
for mode in native replay; do
  (ulimit -v 2097152; timeout --kill-after=2 25 "$output_dir/check" selftest "$output_dir/$mode") \
    > "$output_dir/$mode.log" 2>&1
done
diff -rq "$output_dir/native" "$output_dir/replay" > "$output_dir/byte-replay.log"
recorded=("$study_dir"/evidence/retained-native/*.json)
generated=("$output_dir"/native/*.json)
[[ ${#recorded[@]} == 16 && ${#generated[@]} == 16 ]]
for file in "${recorded[@]}"; do cmp "$file" "$output_dir/native/$(basename "$file")"; done
while IFS=$'\t' read -r cell expected; do
  actual=$(sha256sum "$output_dir/native/$cell.json")
  [[ ${actual%% *} == "$expected" ]]
done < <(jq -er '.cells[] | [.cell,.certificate_sha256] | @tsv' "$output_dir/native/summary.json")
echo 'Retained-point exact certificates and byte replay match (unsigned producer checks).'
