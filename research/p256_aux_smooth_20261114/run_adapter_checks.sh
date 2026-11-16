#!/usr/bin/env bash
# Development tests and static export; does not execute Magma or time solvers.
set -euo pipefail
[[ $# == 1 ]] || { echo 'Usage: run_adapter_checks.sh FRESH_OUTPUT_DIRECTORY' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
[[ ! -e "$1" ]] || { echo 'Output must be fresh' >&2; exit 2; }
mkdir -p "$1"
result_dir=$(cd "$1" && pwd)
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/magma_adapter.cpp" -lgmpxx -lgmp -lcrypto -o "$result_dir/magma_adapter" \
  > "$result_dir/build.log" 2>&1
sha256sum "$study_dir/aux_base.cpp" "$study_dir/magma_adapter.cpp" "$result_dir/magma_adapter" > "$result_dir/source-binary.sha256"
timeout 60 "$result_dir/magma_adapter" selftest "$result_dir/native" > "$result_dir/selftest.log" 2>&1
timeout 60 "$result_dir/magma_adapter" export 4 "$result_dir/magma" > "$result_dir/export.log" 2>&1
timeout 60 "$result_dir/magma_adapter" export 4 "$result_dir/magma-replay" > "$result_dir/export-replay.log" 2>&1
diff -rq "$result_dir/magma" "$result_dir/magma-replay" > "$result_dir/byte-replay.log"
if "$result_dir/magma_adapter" export 4294967300 "$result_dir/overflow" > "$result_dir/overflow.log" 2>&1; then
  echo 'ERROR: overflowing depth accepted' >&2; exit 1
else
  reject_status=$?
  [[ $reject_status == 1 ]] || { echo "Unexpected negative-test exit $reject_status" >&2; exit 1; }
fi
echo 'Adapter native tests and byte replay pass; Magma was not executed.'
