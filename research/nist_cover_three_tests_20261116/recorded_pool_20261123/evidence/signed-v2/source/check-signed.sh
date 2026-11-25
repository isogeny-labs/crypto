#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin" "$out/input"
cp evidence/qualification-v2/source/*.hpp "$out/source/"
cp signed_words.cpp target_signed.cpp SIGNED_WORD_PROTOCOL.md check-signed.sh "$out/source/"
sed '/^int main(/,$d' audit.cpp > "$out/source/audit_arithmetic.hpp"
sed '/^int main(/,$d' target.cpp > "$out/source/target_arithmetic.hpp"
cp -r evidence/qualification-v2/input/. "$out/input/"
sha256sum "$out/source/"* "$out/input/baseline.json" "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
for name in signed_words target_signed; do
  set +e
  (ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/$name.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/$name" > "$out/$name-build.stdout" 2> "$out/$name-build.stderr"; } 2> "$out/$name-build-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$name-BUILD_EXIT_CODE.txt"
  test "$status" -eq 0
  chmod 0555 "$out/bin/$name"
done
sha256sum "$out/source/"* "$out/bin/"* "$out/input/baseline.json" "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/signed_words" "$out/input/baseline.json" "$out/result.json" > "$out/audit.stdout" 2> "$out/audit.stderr"; } 2> "$out/audit-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/AUDIT_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" > "$out/target-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/bin/target_signed" "$out/input" "$out/input/baseline.json" "$out/result.json" "$out/target-result.json" > "$out/target.stdout" 2> "$out/target.stderr"; } 2> "$out/target-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/TARGET_EXIT_CODE.txt"
sha256sum signed_words.cpp target_signed.cpp SIGNED_WORD_PROTOCOL.md check-signed.sh audit.cpp target.cpp > "$out/post-call-source-manifest.sha256"
exit "$status"
