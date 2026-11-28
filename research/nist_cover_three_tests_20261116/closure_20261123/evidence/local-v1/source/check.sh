#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin"
cp closure.cpp PROTOCOL.md check.sh ../issue_identity.cpp ../json.hpp "$out/source/"
cp ../replay_20261123/evidence/ci-replay/native/relations/result.json "$out/frozen-baseline.json"
sed '/^int main(/,$d' ../issue_identity.cpp | \
  sed 's@research/nist_cover_three_tests_20261116/README.md@research/nist_cover_three_tests_20261116/closure_20261123/REPORT.md@g' \
  > "$out/source/identity_arithmetic.hpp"
sha256sum "$out/source/"* "$out/frozen-baseline.json" > "$out/source-manifest.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
  "$out/source/closure.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/closure" \
  > "$out/build.stdout" 2> "$out/build.stderr"
chmod 0555 "$out/bin/closure"
sha256sum "$out/source/"* "$out/frozen-baseline.json" "$out/bin/closure" > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; timeout 30 "$out/bin/closure" "$out/frozen-baseline.json" "$out") \
  > "$out/result-and-id.txt" 2> "$out/stderr.txt"
status=$?
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
set -e
if [ "$status" -eq 0 ]; then
  sed -n '1p' "$out/result-and-id.txt" > "$out/result.json"
  sed -n '2p' "$out/result-and-id.txt" > "$out/identity-id.txt"
fi
sha256sum closure.cpp PROTOCOL.md check.sh ../issue_identity.cpp ../json.hpp \
  > "$out/post-call-source-manifest.sha256"
exit "$status"
