#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 2
case "$2" in pilot|all) mode=$2;; *) exit 2;; esac
test ! -e "$1"
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
native() {
    local name=$1 expected=$2 limit=$3
    shift 3
    local call="$out/calls/$name"
    mkdir "$call"
    printf '%s\0' "$@" > "$call/argv.nul"
    set +e
    (ulimit -v 1048576; { time timeout --kill-after=3 "$limit" taskset -c "$cpu" "$@" > "$call/stdout.json" 2> "$call/stderr.txt"; } 2> "$call/cost.json")
    local code=$?
    set -e
    printf '%s\n' "$code" > "$call/exit-code.txt"
    printf '%s\n' "$expected" > "$call/expected-exit-code.txt"
    test "$code" -eq "$expected"
}
work() {
    mkdir "$out/source" "$out/input" "$out/bin" "$out/calls"
    cp fibres.cpp PROTOCOL.md run-fibres.sh "$out/source/"
    for name in projective_base.hpp correspondence_base.hpp separate_base.hpp typed_json.hpp; do
        cp "../km_pushforward_20261126/evidence/attempt-v1/source/$name" "$out/source/"
    done
    local old=../local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e/relations
    cp "$old/result.json" "$out/input/baseline.json"
    cp "$old/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cp "$old/input/backmap.json" "$out/input/backmap.json"
    git rev-parse HEAD > "$out/source-commit.txt"
    c++ --version > "$out/compiler-version.txt"
    printf 'cpu=%s\nvirtual_memory_kib=1048576\nbuild_timeout_seconds=300\nworker_timeout_seconds=600\nmode=%s\n' "$cpu" "$mode" > "$out/resource-envelope.txt"
    sha256sum "$out/source/"* "$out/input/"* > "$out/pre-build-manifest.sha256"
    native build 0 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/fibres.cpp" -lcrypto -o "$out/bin/fibres"
    chmod 0555 "$out/bin/fibres"
    sha256sum "$out/bin/fibres" "$out/source/"* "$out/input/"* > "$out/pre-call-manifest.sha256"
    if test "$mode" = pilot; then
        native pilot 0 600 "$out/bin/fibres" "$out/input/quartic.json" "$out/input/backmap.json" "$out/input/baseline.json" 0 1
    else
        local first count
        for first in 0 16 32 48 64 80 96 112 128 144 160 176 192 208 224; do
            count=16
            if test "$first" -eq 224; then count=1; fi
            native "batch-$first" 0 600 "$out/bin/fibres" "$out/input/quartic.json" "$out/input/backmap.json" "$out/input/baseline.json" "$first" "$count"
        done
    fi
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-subjects.txt"
}
set +e
{ time (set -e; work); } > "$out/stdout.txt" 2> "$out/wrapper-stderr-and-cost.txt"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit-code.txt"
exit "$code"
