#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin"
cp ../recorded_pool_20261123/evidence/signed-v2/source/*.hpp "$out/source/"
sed '/^int main(/,$d' ../recorded_pool_20261123/signed_words.cpp > "$out/source/signed_words_arithmetic.hpp"
cp audit.cpp PROTOCOL.md check.sh "$out/source/"
baseline=../recorded_pool_20261123/evidence/signed-v2/input/baseline.json
tangents=../recorded_pool_20261123/evidence/traces-v1/result.json
prior=../recorded_pool_20261123/evidence/signed-v2/result.json
printf '%s  %s\n' \
  891d63d978d8800e245952d32ddb733d15d6156f6d080aa74d190452f9101cd4 "$baseline" \
  83bc314ae804e647366d8055d5e72e236742377a1e8355c4794a554814a55988 "$tangents" \
  926cb2041735f43c420b6e10fee4d9338ae46b98ca7a4734874a9bc58ac2bba5 "$prior" > "$out/input-pins.sha256"
sha256sum -c "$out/input-pins.sha256" > "$out/input-pins.stdout"
sha256sum "$out/source/"* > "$out/build-input-manifest.sha256"
git rev-parse HEAD > "$out/source-commit.txt"
uname -a > "$out/environment.txt"
c++ --version >> "$out/environment.txt"
openssl version >> "$out/environment.txt"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/audit.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/audit" > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
test "$status" -eq 0
chmod 0555 "$out/bin/audit"
sha256sum "$out/source/"* "$out/bin/"* > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/audit" "$baseline" "$tangents" "$prior" "$out/result.json" "$out/cancellation.identity.json" > "$out/stdout.txt" 2> "$out/stderr.txt"; } 2> "$out/cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" "$out/cancellation.identity.json" > "$out/pre-collision.sha256"
set +e
(ulimit -v 1048576; { time timeout 30 "$out/bin/audit" "$baseline" "$tangents" "$prior" "$out/result.json" "$out/cancellation.identity.json" > "$out/collision.stdout" 2> "$out/collision.stderr"; } 2> "$out/collision-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/COLLISION_EXIT_CODE.txt"
test "$status" -eq 1
sha256sum -c "$out/pre-collision.sha256" > "$out/collision-bytes.stdout"
set +e
(ulimit -v 1048576; { time timeout 30 "$out/bin/audit" --unknown > "$out/unknown.stdout" 2> "$out/unknown.stderr"; } 2> "$out/unknown-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/UNKNOWN_EXIT_CODE.txt"
test "$status" -eq 1
sha256sum audit.cpp PROTOCOL.md check.sh > "$out/post-call-source-manifest.sha256"
