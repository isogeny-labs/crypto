#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
stage() {
    local name=$1
    shift
    set +e
    (ulimit -v 1048576; { time "$@" > "$out/$name.stdout" 2> "$out/$name.stderr"; } 2> "$out/$name-cost.json")
    local code=$?
    set -e
    printf '%s\n' "$code" > "$out/$name-EXIT_CODE.txt"
    return "$code"
}
work() {
    mkdir "$out/source" "$out/input" "$out/bin" "$out/km-source"
    cp transport.cpp projective_km.cpp check.cpp TRANSPORT.md check-transport.sh "$out/source/"
    cp ../separate_divisor_20261124/check.cpp "$out/source/divisor-source.cpp"
    cp ../correspondence_audit_20261124/check.cpp "$out/source/correspondence-source.cpp"
    cp ../collection_preflight_20261124/typed_json.hpp "$out/source/"
    sed '/^int main(/,$d' "$out/source/divisor-source.cpp" > "$out/source/separate_base.hpp"
    sed '/^int main(/,$d' "$out/source/correspondence-source.cpp" > "$out/source/correspondence_base.hpp"
    sed '/^int main(/,$d' "$out/source/check.cpp" > "$out/source/projective_base.hpp"
    previous=../local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e
    cp "$previous/transfer/source/"*.hpp "$out/km-source/"
    cp "$previous/relations/input/backmap.json" "$out/input/backmap.json"
    cp "$previous/relations/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cp "$previous/relations/result.json" "$out/input/relations.json"
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    git rev-parse HEAD > "$out/source-commit.txt"
    c++ --version > "$out/compiler-version.txt"
    sha256sum "$out/source/"* "$out/km-source/"* "$out/input/"* > "$out/build-input-manifest.sha256"
    stage build-openssl timeout 300 taskset -c "$cpu" c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
        -I "$out/source" "$out/source/transport.cpp" -lcrypto -o "$out/bin/transport"
    stage build-km timeout 300 taskset -c "$cpu" c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
        -I "$out/km-source" "$out/source/projective_km.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/projective-km"
    sha256sum "$out/source/"* "$out/km-source/"* "$out/input/"* "$out/bin/"* > "$out/pre-call-manifest.sha256"
    stage openssl timeout 600 taskset -c "$cpu" "$out/bin/transport" "$out/input/quartic.json" "$out/input/backmap.json" "$out/input/relations.json"
    stage km timeout 600 taskset -c "$cpu" "$out/bin/projective-km" "$out/input/quartic.json" "$out/input/relations.json"
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
    sha256sum transport.cpp projective_km.cpp check.cpp TRANSPORT.md check-transport.sh > "$out/post-call-source-manifest.sha256"
}
set +e
{ time (set -e; work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
