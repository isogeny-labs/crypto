#!/usr/bin/env bash
# Exact native component verification only; no network or measurement.
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: run_checks.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
output_dir=$(cd "$1" && pwd)
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/membership_certificate_v2.cpp" -lgmpxx -lgmp -lcrypto \
  -o "$output_dir/check" > "$output_dir/build.log" 2>&1
timeout 60 "$output_dir/check" selftest "$output_dir/native" > "$output_dir/native.log" 2>&1
for d in 1 2 4; do
  expected=$(jq -er --argjson depth "$d" '.cases[] | select(.depth==$depth) | .certificate_sha256' "$output_dir/native/summary.json")
  actual=$(sha256sum "$output_dir/native/depth$d.json")
  [[ ${actual%% *} == "$expected" ]]
done
if [[ -d "$study_dir/evidence/native" ]]; then
  diff -rq "$study_dir/evidence/native" "$output_dir/native" > "$output_dir/byte-replay.log"
fi
if "$output_dir/check" selftest "$output_dir/native" > "$output_dir/overwrite-control.log" 2>&1; then
  echo 'ERROR: overwrite accepted' >&2; exit 1
fi
grep -q 'fresh output required' "$output_dir/overwrite-control.log"
echo 'Exact component certificates, corrupt controls, hashes and overwrite rejection pass (unsigned producer).'
