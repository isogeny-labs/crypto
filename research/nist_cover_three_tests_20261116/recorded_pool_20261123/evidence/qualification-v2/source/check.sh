#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin" "$out/input"
cp ../evidence/opaque-km-primary-v2/source/*.hpp "$out/source/"
cp audit.cpp target.cpp PROTOCOL.md check.sh ../issue_identity.cpp "$out/source/"
sed '/^int main(/,$d' ../closure_20261123/closure.cpp > "$out/source/rank_arithmetic.hpp"
sed '/^int main(/,$d' ../issue_identity.cpp | \
  sed 's@research/nist_cover_three_tests_20261116/README.md@research/nist_cover_three_tests_20261116/recorded_pool_20261123/REPORT.md@g' \
  > "$out/source/identity_arithmetic.hpp"
cp ../replay_20261123/evidence/ci-45221063854-1/native/relations/result.json "$out/input/baseline.json"
(cd ../evidence && sha256sum -c kernel3-resume-v1.archive.sha256) > "$out/archive-check.txt"
archive=../evidence/kernel3-resume-v1.tar.gz
prefix=kernel3-resume-v1/replay/nist_cover_three_tests_20261116
tar -xzf "$archive" -C "$out/input" --strip-components=3 \
  "$prefix/inputs/p192-d7-prym.json" "$prefix/evidence/p192-d7-kernel3-plane-quartic.json"
sha256sum "$out/source/"* "$out/input/baseline.json" "$out/input/inputs/"*.json \
  "$out/input/evidence/"*.json "$archive" > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
for name in audit target; do
  set +e
  (ulimit -v 1048576; { time timeout 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
    -I "$out/source" "$out/source/$name.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/$name" \
    > "$out/$name-build.stdout" 2> "$out/$name-build.stderr"; } 2> "$out/$name-build-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$name-BUILD_EXIT_CODE.txt"
  test "$status" -eq 0
  chmod 0555 "$out/bin/$name"
done
sha256sum "$out/source/"* "$out/bin/"* "$out/input/baseline.json" "$out/input/inputs/"*.json \
  "$out/input/evidence/"*.json > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 120 "$out/bin/audit" "$out/input/baseline.json" \
  "$out/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out" \
  > "$out/audit.stdout" 2> "$out/audit.stderr"; } 2> "$out/audit-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/AUDIT_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/result.json" > "$out/target-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/bin/target" "$out/input" "$out/input/baseline.json" \
  "$out/result.json" "$out/target-result.json" > "$out/target.stdout" 2> "$out/target.stderr"; } 2> "$out/target-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/TARGET_EXIT_CODE.txt"
sha256sum audit.cpp target.cpp PROTOCOL.md check.sh ../issue_identity.cpp ../closure_20261123/closure.cpp \
  > "$out/post-call-source-manifest.sha256"
exit "$status"
