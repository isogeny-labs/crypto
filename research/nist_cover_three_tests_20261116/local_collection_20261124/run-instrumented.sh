#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 2
previous=$(cd "$1" && pwd)
mkdir -p "$(dirname "$2")"
mkdir "$2"
out=$(cd "$2" && pwd)
mkdir "$out/source" "$out/bin"
cp "$previous/pipeline/qualification/source/"*.hpp "$out/source/"
cp instrumented_collector.cpp INSTRUMENTATION_PROTOCOL.md run-instrumented.sh "$out/source/"
cpu=$(cat "$previous/selected-cpu.txt")
[[ "$cpu" =~ ^[0-9]+$ ]]
sha256sum "$out/source/"* "$previous/pipeline/input/"*.json \
 "$previous/pipeline/input/inputs/"*.json "$previous/pipeline/input/evidence/"*.json \
 "$previous/pipeline/qualification/bin/checker" > "$out/build-input-manifest.sha256"
git rev-parse HEAD > "$out/source-commit.txt"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 300 taskset -c "$cpu" c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/instrumented_collector.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/collector" > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
test "$status" -eq 0
chmod 0555 "$out/bin/collector"
sha256sum "$out/bin/collector" "$previous/pipeline/qualification/bin/checker" > "$out/pre-call-binaries.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$out/bin/collector" --collect \
 "$previous/pipeline/input" "$previous/pipeline/input/baseline.json" \
 "$previous/pipeline/input/tangents.json" "$out/result.json" > "$out/collector.stdout" 2> "$out/collector.stderr"; } 2> "$out/collector-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/COLLECTOR_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" > "$out/checker-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$previous/pipeline/qualification/bin/checker" --check \
 "$previous/pipeline/input/evidence/p192-d7-kernel3-plane-quartic.json" \
 "$previous/pipeline/input/backmap.json" "$previous/pipeline/input/baseline.json" \
 "$previous/pipeline/input/tangents.json" "$out/result.json" "$out/checker-result.json" \
 > "$out/checker.stdout" 2> "$out/checker.stderr"; } 2> "$out/checker-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/CHECKER_EXIT_CODE.txt"
sha256sum instrumented_collector.cpp INSTRUMENTATION_PROTOCOL.md run-instrumented.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
