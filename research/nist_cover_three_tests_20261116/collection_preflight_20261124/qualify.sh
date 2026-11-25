#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/input" "$out/bin"
cp preflight.cpp typed_json.hpp pins.hpp PROTOCOL.md qualify.sh "$out/source/"
cp ../recorded_pool_20261123/evidence/qualification-v2/input/baseline.json "$out/input/reference.json"
cp ../replay_20261123/evidence/local-v1/native/relations/result.json "$out/input/actual.json"
mkdir "$out/input/construction"
cp -r ../replay_20261123/evidence/local-v1/native/relations/input/inputs "$out/input/construction/"
cp -r ../replay_20261123/evidence/local-v1/native/relations/input/evidence "$out/input/construction/"
cp ../replay_20261123/evidence/local-v1/native/relations/input/backmap.json "$out/input/construction/"
sha256sum "$out/source/"* "$out/input/"*.json "$out/input/construction/backmap.json" "$out/input/construction/inputs/"*.json "$out/input/construction/evidence/"*.json > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/preflight.cpp" -lcrypto -o "$out/bin/preflight" > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
test "$status" -eq 0
chmod 0555 "$out/bin/preflight"
sha256sum "$out/source/"* "$out/bin/preflight" "$out/input/"*.json "$out/input/construction/backmap.json" "$out/input/construction/inputs/"*.json "$out/input/construction/evidence/"*.json > "$out/pre-call-manifest.sha256"
for mode in self-test check; do
  set +e
  (ulimit -v 1048576; { time timeout 30 "$out/bin/preflight" "--$mode" "$out/input/reference.json" "$out/input/actual.json" "$out/input/construction" "$out/$mode-result.json" > "$out/$mode.stdout" 2> "$out/$mode.stderr"; } 2> "$out/$mode-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$mode-EXIT_CODE.txt"
  test "$status" -eq 0
done
sha256sum "$out/self-test-result.json" "$out/check-result.json" > "$out/pre-collision-manifest.sha256"
for mode in unknown ambiguous collision; do
  set +e
  if [ "$mode" = unknown ]; then
    timeout 5 "$out/bin/preflight" --unknown > "$out/$mode.stdout" 2> "$out/$mode.stderr"
  elif [ "$mode" = ambiguous ]; then
    timeout 5 "$out/bin/preflight" --check --self-test > "$out/$mode.stdout" 2> "$out/$mode.stderr"
  else
    timeout 5 "$out/bin/preflight" --check "$out/input/reference.json" "$out/input/actual.json" "$out/input/construction" "$out/check-result.json" > "$out/$mode.stdout" 2> "$out/$mode.stderr"
  fi
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$mode-EXIT_CODE.txt"
  test "$status" -eq 1
done
sha256sum -c "$out/pre-collision-manifest.sha256" > "$out/collision-bytes-check.stdout"
sha256sum preflight.cpp typed_json.hpp pins.hpp PROTOCOL.md qualify.sh > "$out/post-call-source-manifest.sha256"
