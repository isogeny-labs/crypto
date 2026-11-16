#!/usr/bin/env bash
# Native development checks + offline replay of retained CAS data. No HTTP.
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: run_checks.sh FRESH_OUTPUT_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
result_dir=$(cd "$1" && pwd)
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/linear_reduce.cpp" -lgmpxx -lgmp -lcrypto -o "$result_dir/linear_reduce" \
  > "$result_dir/linear-build.log" 2>&1
timeout 60 "$result_dir/linear_reduce" selftest "$result_dir/native" > "$result_dir/native.log" 2>&1
diff -rq "$study_dir/evidence/native" "$result_dir/native" > "$result_dir/native-byte-replay.log"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/../magma_adapter.cpp" -lgmpxx -lgmp -lcrypto -o "$result_dir/native_replayer" \
  > "$result_dir/replayer-build.log" 2>&1
for unit in 003 004 006 007 008; do
  recorded="$study_dir/evidence/api-unit$unit"
  jq -e '.success == true and .exit_code == 0 and .truncated == false and .error == null and ((.warnings // [])|length)==0' \
    "$recorded/response.json" > "$result_dir/$unit-admission.txt"
  jq -er '.stdout' "$recorded/response.json" > "$result_dir/$unit-stdout.log"
  cmp "$recorded/stdout.log" "$result_dir/$unit-stdout.log"
done
for unit in 001 002 005; do
  jq -e '.success == false' "$study_dir/evidence/api-unit$unit/response.json" > "$result_dir/$unit-rejected.txt"
done
"$result_dir/native_replayer" replay 1 2 planted "$result_dir/003-stdout.log" --producer-unverified > "$result_dir/003-replay.json"
cmp "$study_dir/evidence/api-unit003/replay.json" "$result_dir/003-replay.json"
for unit in 004 006 007 008; do
  depth=1
  [[ $unit == 006 || $unit == 008 ]] && depth=2
  "$study_dir/replay_batch.sh" "$result_dir/native_replayer" "$depth" "$result_dir/$unit-stdout.log" "$result_dir/$unit-replay"
  for arm in dense tower paired oriented; do
    cmp "$study_dir/evidence/api-unit$unit/replay/$arm-replay.json" "$result_dir/$unit-replay/$arm-replay.json"
  done
done
cp "$result_dir/004-stdout.log" "$result_dir/corrupt-batch.log"
printf 'BEGIN_ARM unexpected\n' >> "$result_dir/corrupt-batch.log"
if "$study_dir/replay_batch.sh" "$result_dir/native_replayer" 1 "$result_dir/corrupt-batch.log" "$result_dir/rejected-batch" > "$result_dir/corrupt-batch-result.log" 2>&1; then
  echo 'ERROR: malformed batch accepted' >&2; exit 1
else
  reject_status=$?
  [[ $reject_status == 2 ]] || { echo "Unexpected batch rejection exit $reject_status" >&2; exit 1; }
fi
echo '72 native cases, coefficient certificates, retained CAS sets and negative controls pass (unsigned producer assertion).'
