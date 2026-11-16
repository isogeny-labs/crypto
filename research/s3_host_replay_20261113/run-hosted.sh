#!/usr/bin/env bash
set -euo pipefail
test "${GITHUB_ACTIONS:-}" = true
test "${GITHUB_EVENT_NAME:-}" = push
test "${GITHUB_REF:-}" = refs/heads/main
test "${GITHUB_REPOSITORY:-}" = isogeny-labs/crypto
test "${GITHUB_RUN_ATTEMPT:-}" = 1
study=$(cd "$(dirname "$0")" && pwd)
source_root=${1:?set pinned source root}
output=${2:?set a fresh absolute output directory}
if [ -e "$output" ]; then echo 'Refusing existing output directory.' >&2; exit 2; fi
mkdir -p "$output/cpp"
output=$(cd "$output" && pwd)
node "$study/prepare-cpp.mjs" "$source_root" "$output/cpp"
g++ --version > "$output/cpp/compiler.txt"
set +e
timeout 60s g++ -std=c++20 -O2 -Wall -Wextra -Werror \
  -I "$source_root/research/theorem_census_20261109/native" \
  "$output/cpp/s3-only.cpp" -o "$output/cpp/validator" \
  > "$output/cpp/build.stdout" 2> "$output/cpp/build.stderr"
cpp_build=$?
set -e
printf '%s\n' "$cpp_build" > "$output/cpp/build.exit"
cpp_run=125
cpp_compare=125
if [ "$cpp_build" -eq 0 ]; then
  sha256sum "$output/cpp/validator" > "$output/cpp/binary.sha256"
  set +e
  timeout 60s "$output/cpp/validator" > "$output/cpp/actual.jsonl" 2> "$output/cpp/actual.stderr"
  cpp_run=$?
  if [ "$cpp_run" -eq 0 ]; then
    node "$study/compare-cpp.mjs" "$output/cpp/actual.jsonl" \
      "$source_root/research/theorem_census_20261109/evidence/semaev-native.jsonl" \
      > "$output/cpp/comparison.json" 2> "$output/cpp/comparison.stderr"
    cpp_compare=$?
  fi
  set -e
fi
printf '%s\n' "$cpp_run" > "$output/cpp/run.exit"
printf '%s\n' "$cpp_compare" > "$output/cpp/comparison.exit"
for part in source.json compiler.txt binary.sha256 build.exit build.stdout build.stderr \
  actual.jsonl actual.stderr comparison.json comparison.stderr run.exit comparison.exit; do
  printf '\nS3_HOST_CPP_BEGIN %s\n' "$part"
  if [ -f "$output/cpp/$part" ]; then cat "$output/cpp/$part"; fi
  printf '\nS3_HOST_CPP_END %s\n' "$part"
done
# The unchanged Rust driver logs its own compiler, binary, host and complete exits.
set +e
S3_VALIDATOR_ARM=github-hosted bash \
  "$source_root/research/independent_s3_validator_20261109/run-driver.sh" "$output/rust"
rust_status=$?
set -e
printf '\nS3_HOST_CONTINUATION_STATUS cpp_build=%s cpp_run=%s cpp_compare=%s rust=%s\n' \
  "$cpp_build" "$cpp_run" "$cpp_compare" "$rust_status"
if [ "$cpp_build" -ne 0 ] || [ "$cpp_run" -ne 0 ] || [ "$cpp_compare" -ne 0 ] || [ "$rust_status" -ne 0 ]; then exit 1; fi
