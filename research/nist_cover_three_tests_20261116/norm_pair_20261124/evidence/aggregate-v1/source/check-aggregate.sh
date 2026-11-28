#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/input" "$out/bin"
cp evidence/qualification-v1/source/*.hpp "$out/source/"
sed '/^int main(/,$d' ../relation_openssl_check.cpp > "$out/source/separate_arithmetic.hpp"
cp aggregate.cpp AGGREGATE_PROTOCOL.md check-aggregate.sh "$out/source/"
cp evidence/qualification-v1/input/baseline.json "$out/input/baseline.json"
cp evidence/qualification-v1/result.json "$out/input/queries.json"
sha256sum "$out/source/"* "$out/input/"*.json > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/aggregate.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/aggregate" > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
test "$status" -eq 0
chmod 0555 "$out/bin/aggregate"
sha256sum "$out/source/"* "$out/input/"*.json "$out/bin/aggregate" > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/aggregate" "$out/input/baseline.json" "$out/input/queries.json" "$out/result.json" > "$out/stdout.txt" 2> "$out/stderr.txt"; } 2> "$out/cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
sha256sum aggregate.cpp AGGREGATE_PROTOCOL.md check-aggregate.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
