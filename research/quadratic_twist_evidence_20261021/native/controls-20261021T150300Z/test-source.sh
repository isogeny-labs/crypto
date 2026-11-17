#!/usr/bin/env bash
set -euo pipefail
study_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
input_dir="${1:?usage: test-native.sh SUCCESSFUL_NATIVE_RUN FRESH_CONTROL_DIRECTORY}"
control_dir="${2:?fresh output directory required}"
[[ -d "$input_dir" && -f "$input_dir/results.json" ]] || { echo 'Expected a successful native run directory' >&2; exit 2; }
[[ ! -e "$control_dir" ]] || { echo "Refusing to overwrite $control_dir" >&2; exit 2; }
mkdir -p "$control_dir"
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/twist-native-controls.XXXXXX")"
trap 'rm -rf -- "$scratch_dir"' EXIT
exec > >(tee "$control_dir/run.log") 2>&1
date -u '+Started UTC: %Y-%m-%dT%H:%M:%SZ'
tar -xJf "$study_dir/historical/source-and-evidence.tar.xz" -C "$scratch_dir"
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$study_dir/audit.cpp" -lcrypto -o "$scratch_dir/audit"
"$scratch_dir/audit" "$scratch_dir" "$control_dir/positive.json" "$input_dir/results.json"
jq '.pairings[0].all_log_certificates[0].index_calculus.independent_relations[0].rhs += 1' "$input_dir/results.json" > "$scratch_dir/false-relation.json"
if "$scratch_dir/audit" "$scratch_dir" "$scratch_dir/rejected.json" "$scratch_dir/false-relation.json" > "$control_dir/false-new-relation.log" 2>&1; then
  echo 'FAIL: false newly collected relation accepted'; exit 1
fi
grep -q 'native complete relation equation' "$control_dir/false-new-relation.log"
echo 'PASS: false newly collected relation rejected'
jq '.pairings[2].all_log_certificates |= .[1:]' "$input_dir/results.json" > "$scratch_dir/incomplete.json"
if "$scratch_dir/audit" "$scratch_dir" "$scratch_dir/rejected.json" "$scratch_dir/incomplete.json" > "$control_dir/missing-log.log" 2>&1; then
  echo 'FAIL: missing native log certificate accepted'; exit 1
fi
grep -q 'complete native log coverage' "$control_dir/missing-log.log"
echo 'PASS: missing native log certificate rejected'
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$study_dir/native/full.cpp" -lcrypto -o "$scratch_dir/full"
jq '.exhaustive.counts.curve_models += 1' "$scratch_dir/results/results.json" > "$scratch_dir/false-reference.json"
if "$scratch_dir/full" "$scratch_dir/false-reference.json" "$scratch_dir/rejected-results.json" "$scratch_dir/rejected-parity.json" > "$control_dir/false-reference-count.log" 2>&1; then
  echo 'FAIL: false historical count accepted'; exit 1
fi
grep -q 'parity mismatch at /exhaustive/counts/curve_models' "$control_dir/false-reference-count.log"
echo 'PASS: false historical census count rejected'
date -u '+Finished UTC: %Y-%m-%dT%H:%M:%SZ'
