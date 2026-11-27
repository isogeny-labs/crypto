#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/input" "$out/bin"
cp ../recorded_pool_20261123/evidence/signed-v2/source/*.hpp "$out/source/"
sed '/^int main(/,$d' ../recorded_pool_20261123/signed_words.cpp > "$out/source/signed_words_arithmetic.hpp"
cp audit.cpp target.cpp classify.cpp PROTOCOL.md check.sh "$out/source/"
cp -r ../recorded_pool_20261123/evidence/signed-v2/input/. "$out/input/"
cp ../recorded_pool_20261123/evidence/traces-v1/result.json "$out/input/tangents.json"
cp ../recorded_pool_20261123/evidence/signed-v2/result.json "$out/input/prior-signed.json"
sha256sum "$out/source/"* "$out/input/"*.json "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
for name in audit target classify; do
  set +e
  (ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/$name.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/$name" > "$out/$name-build.stdout" 2> "$out/$name-build.stderr"; } 2> "$out/$name-build-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$name-BUILD_EXIT_CODE.txt"
  test "$status" -eq 0
  chmod 0555 "$out/bin/$name"
done
sha256sum "$out/source/"* "$out/bin/"* "$out/input/"*.json "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/audit" "$out/input/baseline.json" "$out/input/tangents.json" "$out/input/prior-signed.json" "$out/result.json" > "$out/audit.stdout" 2> "$out/audit.stderr"; } 2> "$out/audit-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/AUDIT_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" > "$out/target-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/bin/target" "$out/input" "$out/input/baseline.json" "$out/result.json" "$out/target-result.json" > "$out/target.stdout" 2> "$out/target.stderr"; } 2> "$out/target-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/TARGET_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" "$out/target-result.json" > "$out/classifier-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 30 "$out/bin/classify" "$out/input/baseline.json" "$out/result.json" "$out/target-result.json" "$out/classification-result.json" > "$out/classify.stdout" 2> "$out/classify.stderr"; } 2> "$out/classify-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/CLASSIFIER_EXIT_CODE.txt"
sha256sum audit.cpp target.cpp classify.cpp PROTOCOL.md check.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
