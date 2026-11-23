#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin"
cp identity.cpp PROTOCOL.md check-identity.sh "$out/source/"
cp ../issue_identity.cpp ../json.hpp "$out/source/"
sed '/^int main(/,$d' ../issue_identity.cpp | \
  sed 's@research/nist_cover_three_tests_20261116/README.md@research/nist_cover_three_tests_20261116/hypothesis_20261123/REPORT.md@g' \
  > "$out/source/identity_arithmetic.hpp"
sha256sum "$out/source/"* > "$out/source-manifest.sha256"
c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
  "$out/source/identity.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/identity" \
  > "$out/build.stdout" 2> "$out/build.stderr"
sha256sum "$out/source/"* "$out/bin/identity" > "$out/pre-call-manifest.sha256"
chmod 0555 "$out/bin/identity"
set +e
(ulimit -v 1048576; timeout 10 "$out/bin/identity" "$out") \
  > "$out/ids.txt" 2> "$out/stderr.txt"
status=$?
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
set -e
sha256sum identity.cpp PROTOCOL.md check-identity.sh ../issue_identity.cpp ../json.hpp \
  > "$out/post-call-source-manifest.sha256"
exit "$status"
