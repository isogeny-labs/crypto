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
    cp ../collection_preflight_20261124/typed_json.hpp "$out/source/"
    previous=../local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e
    cp "$previous/transfer/result.json" "$out/input/transfer.json"
    cp "$previous/relations/input/inputs/p192-d7-prym.json" "$out/input/cover.json"
    cp "$previous/relations/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cpu=$(cat ../local_collection_20261124/evidence/attempt-v1/selected-cpu.txt)
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
    (ulimit -v 1048576; { time timeout 120 taskset -c "$cpu" "$out/bin/check" \
        "$out/input/cover.json" "$out/input/quartic.json" "$out/input/transfer.json" \
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
