#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
work() {
    mkdir "$out/source" "$out/input" "$out/bin"
    cp check.cpp PROTOCOL.md run.sh "$out/source/"
    cp ../separate_divisor_20261124/check.cpp "$out/source/divisor-source.cpp"
    cp ../correspondence_audit_20261124/check.cpp "$out/source/correspondence-source.cpp"
    cp ../collection_preflight_20261124/typed_json.hpp "$out/source/"
    sed '/^int main(/,$d' "$out/source/divisor-source.cpp" > "$out/source/separate_base.hpp"
    sed '/^int main(/,$d' "$out/source/correspondence-source.cpp" > "$out/source/correspondence_base.hpp"
    previous=../local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e/relations/input
    cp "$previous/backmap.json" "$out/input/backmap.json"
    cp "$previous/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    git rev-parse HEAD > "$out/source-commit.txt"
    c++ --version > "$out/compiler-version.txt"
    openssl version > "$out/openssl-version.txt"
    sha256sum "$out/source/"* "$out/input/"* > "$out/build-input-manifest.sha256"
    set +e
    (ulimit -v 1048576; { time timeout 300 taskset -c "$cpu" c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
        -I "$out/source" "$out/source/check.cpp" -lcrypto -o "$out/bin/check" \
        > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
    code=$?
    set -e
    printf '%s\n' "$code" > "$out/BUILD_EXIT_CODE.txt"
    test "$code" -eq 0 || return "$code"
    chmod 0555 "$out/bin/check"
    sha256sum "$out/source/"* "$out/input/"* "$out/bin/check" > "$out/pre-call-manifest.sha256"
    set +e
    (ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$out/bin/check" \
        "$out/input/quartic.json" "$out/input/backmap.json" \
        > "$out/result.json" 2> "$out/stderr.txt"; } 2> "$out/checker-cost.json")
    code=$?
    set -e
    printf '%s\n' "$code" > "$out/EXIT_CODE.txt"
    sha256sum check.cpp PROTOCOL.md run.sh > "$out/post-call-source-manifest.sha256"
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
    return "$code"
}
set +e
{ time (set -e; work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
