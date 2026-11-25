#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin" "$out/input"
cp ../recorded_pool_20261123/evidence/traces-v1/source/*.hpp "$out/source/"
sed '/^int main(/,$d' ../recorded_pool_20261123/target_traces.cpp > "$out/source/target_traces_arithmetic.hpp"
sed '/^int main(/,$d' ../relation_openssl_check.cpp > "$out/source/separate_arithmetic.hpp"
cp collector.cpp checker.cpp rank_roots.hpp PROTOCOL.md qualify.sh "$out/source/"
cp ../closure_20261123/evidence/local-v1/result.json "$out/input/fixtures.json"
sha256sum "$out/source/"* "$out/input/fixtures.json" > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
for name in collector checker; do
  set +e
  (ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/$name.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/$name" > "$out/$name-build.stdout" 2> "$out/$name-build.stderr"; } 2> "$out/$name-build-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$name-BUILD_EXIT_CODE.txt"
  test "$status" -eq 0
  chmod 0555 "$out/bin/$name"
done
sha256sum "$out/source/"* "$out/bin/"* "$out/input/fixtures.json" > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/collector" --self-test "$out/input/fixtures.json" "$out/result.json" > "$out/collector.stdout" 2> "$out/collector.stderr"; } 2> "$out/collector-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/COLLECTOR_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" > "$out/checker-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/checker" --self-test "$out/input/fixtures.json" "$out/result.json" "$out/checker-result.json" > "$out/checker.stdout" 2> "$out/checker.stderr"; } 2> "$out/checker-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/CHECKER_EXIT_CODE.txt"
test "$status" -eq 0
for name in collector checker; do
  for mode in unknown ambiguous; do
    set +e
    if [ "$mode" = unknown ]; then
      timeout 5 "$out/bin/$name" --unknown > "$out/$name-$mode.stdout" 2> "$out/$name-$mode.stderr"
    else
      timeout 5 "$out/bin/$name" --collect --self-test > "$out/$name-$mode.stdout" 2> "$out/$name-$mode.stderr"
    fi
    status=$?
    set -e
    printf '%s\n' "$status" > "$out/$name-$mode-EXIT_CODE.txt"
    test "$status" -eq 1
  done
done
sha256sum collector.cpp checker.cpp rank_roots.hpp PROTOCOL.md qualify.sh > "$out/post-call-source-manifest.sha256"
