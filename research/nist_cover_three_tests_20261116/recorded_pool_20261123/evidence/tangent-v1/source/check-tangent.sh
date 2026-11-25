#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/input" "$out/bin"
cp evidence/signed-v2/source/*.hpp "$out/source/"
cp identity_tangent.cpp IDENTITY_TANGENT_PROTOCOL.md check-tangent.sh "$out/source/"
cp -r evidence/qualification-v2/input/. "$out/input/"
sha256sum "$out/source/"* "$out/input/baseline.json" "$out/input/evidence/"*.json "$out/input/inputs/"*.json > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/identity_tangent.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/identity_tangent" > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
test "$status" -eq 0
chmod 0555 "$out/bin/identity_tangent"
sha256sum "$out/source/"* "$out/bin/"* "$out/input/baseline.json" "$out/input/evidence/"*.json "$out/input/inputs/"*.json > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/identity_tangent" "$out/input/baseline.json" "$out/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/result.json" > "$out/stdout.txt" 2> "$out/stderr.txt"; } 2> "$out/cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
sha256sum identity_tangent.cpp IDENTITY_TANGENT_PROTOCOL.md check-tangent.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
