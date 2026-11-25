#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin"
cp inspect.cpp INSPECTION_PROTOCOL.md check-inspection.sh "$out/source/"
cp evidence/attempt-v1/pipeline/qualification/source/json.hpp "$out/source/"
original=$(realpath evidence/attempt-v1/pipeline/result.json)
instrumented=$(realpath evidence/instrumented-v1/result.json)
cpu=$(cat evidence/attempt-v1/selected-cpu.txt)
sha256sum "$out/source/"* "$original" "$instrumented" > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 300 taskset -c "$cpu" c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/inspect.cpp" -o "$out/bin/inspect" > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/bin/inspect" "$original" "$instrumented" > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 30 taskset -c "$cpu" "$out/bin/inspect" "$original" "$instrumented" "$out/result.json" > "$out/stdout.txt" 2> "$out/stderr.txt"; } 2> "$out/cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
sha256sum inspect.cpp INSPECTION_PROTOCOL.md check-inspection.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
