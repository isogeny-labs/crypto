#!/bin/bash
set -eu
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
cd "$(dirname "$0")"
test "$#" -eq 2
previous=$(cd "$1" && pwd)
mkdir "$2"
out=$(cd "$2" && pwd)
mkdir "$out/source"
cp "$previous/source/"*.hpp "$out/source/"
{ printf '#pragma once\n'; sed '/^int main(/,$d' "$previous/source/relation_pipeline.cpp"; } > "$out/source/relation_pipeline_arithmetic.hpp"
{ printf '#pragma once\n'; sed '/^int main(/,$d' "$previous/source/relation_openssl_check.cpp"; } > "$out/source/relation_openssl_arithmetic.hpp"
cp relation_transfer.cpp transfer_openssl_check.cpp RELATION_TRANSFER_PROTOCOL.md check-relation-transfer.sh "$out/source/"
sha256sum "$out/source/"* > "$out/build-input-manifest.sha256"
set +e
{ time c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
    "$out/source/relation_transfer.cpp" -lgmpxx -lgmp -lcrypto -o "$out/producer" 2> "$out/build.stderr"; } 2> "$out/build-cost.json"
status=$?
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
set -e
test "$status" -eq 0
set +e
{ time c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
    "$out/source/transfer_openssl_check.cpp" -lgmpxx -lgmp -lcrypto -o "$out/checker" 2> "$out/checker-build.stderr"; } 2> "$out/checker-build-cost.json"
status=$?
printf '%s\n' "$status" > "$out/CHECKER_BUILD_EXIT_CODE.txt"
set -e
test "$status" -eq 0
chmod 0555 "$out/producer" "$out/checker"
sha256sum "$out/source/"* "$out/producer" "$out/checker" "$previous/result.json" \
    "$previous/input/"*.json "$previous/input/inputs/"*.json "$previous/input/evidence/"*.json \
    evidence/global-correspondence-v2/result.json evidence/generic-bridge-v1/result.json \
    evidence/formal-series-v1/result.json > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/producer" "$previous/input" \
    evidence/global-correspondence-v2/result.json evidence/generic-bridge-v1/result.json \
    evidence/formal-series-v1/result.json "$previous/input/public-classes.json" "$previous/result.json" \
    > "$out/result.json" 2> "$out/stderr.txt"; } 2> "$out/producer-cost.json")
status=$?
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
set -e
test "$status" -eq 0
sha256sum "$out/result.json" > "$out/checker-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/checker" "$out/result.json" \
    > "$out/checker-result.json" 2> "$out/checker.stderr"; } 2> "$out/checker-cost.json")
status=$?
printf '%s\n' "$status" > "$out/CHECKER_EXIT_CODE.txt"
set -e
sha256sum relation_transfer.cpp transfer_openssl_check.cpp RELATION_TRANSFER_PROTOCOL.md check-relation-transfer.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
