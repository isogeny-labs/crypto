#!/bin/sh
set -eu
cd "$(dirname "$0")"
test "$#" -eq 4
root=$1
global=$2
bridge=$3
out=$4
mkdir "$out"
out=$(cd "$out" && pwd)
cp -R evidence/closed-transpose-v1/source "$out/source"
sed '/^int main(/,$d' closed_transpose.cpp > "$out/source/closed_transpose_arithmetic.hpp"
cp pole_cases.cpp POLE_CASES_PROTOCOL.md "$out/source/"
sha256sum "$out/source/"* > "$out/build-input-manifest.sha256"
set +e
c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/pole_cases.cpp" -lgmpxx -lgmp -lcrypto -o "$out/qualifier" 2> "$out/build.stderr"
build_exit=$?
set -e
printf '%s\n' "$build_exit" > "$out/BUILD_EXIT_CODE.txt"
test "$build_exit" -eq 0
chmod 0555 "$out/qualifier"
sha256sum "$out/source/"* "$out/qualifier" "$global" "$bridge" "$root/inputs/p192-d7-prym.json" "$root/evidence/p192-d7-kernel3-plane-quartic.json" > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; timeout 600 "$out/qualifier" "$root" "$global" "$bridge") > "$out/result.json" 2> "$out/stderr.txt"
run_exit=$?
set -e
printf '%s\n' "$run_exit" > "$out/EXIT_CODE.txt"
sha256sum pole_cases.cpp POLE_CASES_PROTOCOL.md complete_maps.cpp > "$out/post-call-source-manifest.sha256"
exit "$run_exit"
