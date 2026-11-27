#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/bin" "$out/templates"
cp ../recorded_pool_20261123/evidence/signed-v2/source/*.hpp "$out/source/"
sed '/^int main(/,$d' ../recorded_pool_20261123/signed_words.cpp > "$out/source/signed_words_arithmetic.hpp"
sed '/^int main(/,$d' ../two_row_20261124/audit.cpp > "$out/source/two_row_arithmetic.hpp"
cp audit.cpp PROTOCOL.md check.sh "$out/source/"
baseline=../recorded_pool_20261123/evidence/signed-v2/input/baseline.json
tangents=../recorded_pool_20261123/evidence/traces-v1/result.json
signed=../recorded_pool_20261123/evidence/signed-v2/result.json
parent=../two_row_20261124/evidence/qualification-v2/result.json
printf '%s  %s\n' \
  891d63d978d8800e245952d32ddb733d15d6156f6d080aa74d190452f9101cd4 "$baseline" \
  83bc314ae804e647366d8055d5e72e236742377a1e8355c4794a554814a55988 "$tangents" \
  926cb2041735f43c420b6e10fee4d9338ae46b98ca7a4734874a9bc58ac2bba5 "$signed" \
  62e86b62c84d518209654787ef8f2c936b589b723b775f3c99e2732667b2f31a "$parent" > "$out/input-pins.sha256"
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
(ulimit -v 1048576; { time timeout 30 "$out/bin/audit" --templates "$out/templates" > "$out/templates.stdout" 2> "$out/templates.stderr"; } 2> "$out/templates-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/TEMPLATES_EXIT_CODE.txt"
test "$status" -eq 0
sha256sum "$out/templates/"* > "$out/template-manifest.sha256"
overall=0
for arm in u v w; do
  set +e
  (ulimit -v 1048576; { time timeout 120 "$out/bin/audit" "--$arm" "$baseline" "$tangents" "$signed" "$parent" "$out/templates/result.json" "$out/$arm.json" > "$out/$arm.stdout" 2> "$out/$arm.stderr"; } 2> "$out/$arm-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/$arm-EXIT_CODE.txt"
  if test "$status" -ne 0; then overall=1; fi
done
sha256sum -c "$out/template-manifest.sha256" > "$out/template-bytes.stdout"
set +e
(ulimit -v 1048576; { time timeout 30 "$out/bin/audit" --templates "$out/templates" > "$out/collision.stdout" 2> "$out/collision.stderr"; } 2> "$out/collision-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/COLLISION_EXIT_CODE.txt"
test "$status" -eq 1
sha256sum -c "$out/template-manifest.sha256" > "$out/collision-bytes.stdout"
set +e
(ulimit -v 1048576; { time timeout 30 "$out/bin/audit" --unknown > "$out/unknown.stdout" 2> "$out/unknown.stderr"; } 2> "$out/unknown-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/UNKNOWN_EXIT_CODE.txt"
test "$status" -eq 1
sha256sum audit.cpp PROTOCOL.md check.sh > "$out/post-call-source-manifest.sha256"
exit "$overall"
