#!/usr/bin/env bash
# Bounded native development tests, not a timing experiment or ecbench result.
set -euo pipefail
[[ $# == 1 ]] || { echo 'Usage: run_checks.sh FRESH_OUTPUT_DIRECTORY' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
result_dir=$1
[[ ! -e "$result_dir" ]] || { echo 'Output must be fresh (failed attempts are retained)' >&2; exit 2; }
mkdir -p "$result_dir"
result_dir=$(cd "$result_dir" && pwd)
"${CXX:-c++}" --version > "$result_dir/compiler.txt"
uname -srvmo > "$result_dir/host.txt"
pkg-config --modversion gmp openssl > "$result_dir/dependencies.txt"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  "$study_dir/aux_base.cpp" -lgmpxx -lgmp -lcrypto -o "$result_dir/aux_base" \
  > "$result_dir/build.log" 2>&1
sha256sum "$study_dir/aux_base.cpp" "$result_dir/aux_base" > "$result_dir/source-binary.sha256"
timeout 60 "$result_dir/aux_base" selftest "$result_dir/native" > "$result_dir/selftest.log" 2>&1
timeout 60 "$result_dir/aux_base" check 4 "$result_dir/native/fixture-depth4" > "$result_dir/replay.log" 2>&1
cp -r "$result_dir/native/fixture-depth4" "$result_dir/corrupt"
printf 'x' >> "$result_dir/corrupt/certificate.json"
if timeout 60 "$result_dir/aux_base" check 4 "$result_dir/corrupt" > "$result_dir/corrupt.log" 2>&1; then
  echo 'ERROR: corrupt certificate accepted' >&2; exit 1
else
  reject_status=$?
  [[ $reject_status == 1 ]] || { echo "Unexpected negative-test exit $reject_status" >&2; exit 1; }
fi
cp -r "$result_dir/native/fixture-depth4" "$result_dir/extra-file"
printf 'unexpected\n' > "$result_dir/extra-file/undeclared.json"
if timeout 60 "$result_dir/aux_base" check 4 "$result_dir/extra-file" > "$result_dir/extra-file.log" 2>&1; then
  echo 'ERROR: undeclared replay file accepted' >&2; exit 1
else
  reject_status=$?
  [[ $reject_status == 1 ]] || { echo "Unexpected negative-test exit $reject_status" >&2; exit 1; }
fi
echo 'Native development checks pass (producer assertion; no signed PCC standing).'
