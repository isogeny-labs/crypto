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
    cp normalize_km.cpp NORMALIZATION.md check-normalization.sh "$out/source/"
    previous=../local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e
    cp "$previous/transfer/source/"*.hpp "$out/source/"
    cp "$previous/relations/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cp "$previous/relations/result.json" "$out/input/relations.json"
    cp evidence/transport-v1/km.stdout "$out/input/class.json"
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    git rev-parse HEAD > "$out/source-commit.txt"
    sha256sum "$out/source/"* "$out/input/"* > "$out/build-input-manifest.sha256"
    set +e
    (ulimit -v 1048576; { time timeout 300 taskset -c "$cpu" c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
        -I "$out/source" "$out/source/normalize_km.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/normalize" \
        > "$out/build.stdout" 2> "$out/build.stderr"; } 2> "$out/build-cost.json")
    code=$?
    set -e
    printf '%s\n' "$code" > "$out/BUILD_EXIT_CODE.txt"
    test "$code" -eq 0 || return "$code"
    sha256sum "$out/source/"* "$out/input/"* "$out/bin/normalize" > "$out/pre-call-manifest.sha256"
    set +e
    (ulimit -v 1048576; { time timeout 600 taskset -c "$cpu" "$out/bin/normalize" "$out/input/quartic.json" "$out/input/relations.json" "$out/input/class.json" \
        > "$out/result.json" 2> "$out/stderr.txt"; } 2> "$out/checker-cost.json")
    code=$?
    set -e
    printf '%s\n' "$code" > "$out/EXIT_CODE.txt"
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
    sha256sum normalize_km.cpp NORMALIZATION.md check-normalization.sh > "$out/post-call-source-manifest.sha256"
    return "$code"
}
set +e
{ time (set -e; work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
