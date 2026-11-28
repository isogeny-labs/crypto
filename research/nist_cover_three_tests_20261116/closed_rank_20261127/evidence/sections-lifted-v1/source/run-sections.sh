#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 2
test ! -e "$1"
mode=$2
if test "$mode" != old; then mode=$(realpath "$mode"); fi
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
    cp sections.cpp SECTION_PROTOCOL.md run-sections.sh fibres.cpp "$out/source/"
    sed '/^int main(/,$d' fibres.cpp > "$out/source/fibre_arithmetic.hpp"
    for name in projective_base.hpp correspondence_base.hpp separate_base.hpp typed_json.hpp; do
        cp "evidence/pilot-v1/source/$name" "$out/source/"
    done
    cp ../split_tangent_20261123/rank_roots.hpp "$out/source/"
    local old=../local_collection_20261124/evidence/attempt-v1/pipeline
    cp "$old/baseline-e2e/relations/result.json" "$out/input/baseline.json"
    cp "$old/baseline-e2e/relations/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cp "$old/baseline-e2e/relations/input/backmap.json" "$out/input/backmap.json"
    cp "$old/result.json" "$out/input/tangents.json"
    local supplied=old
    local size=226
    if test "$mode" != old; then
        cp "$mode" "$out/input/fibres.json"
        supplied="$out/input/fibres.json"
        size=$(python3 - "$supplied" "$out/input/baseline.json" <<'PY'
import sys,json
data=json.load(open(sys.argv[1]));old=json.load(open(sys.argv[2]))
points={(q['x'],q['y']) for q in old['pool']['points']}
for batch in data['batches']:
    for fibre in batch['fibres']:
        points.update((q['x'],q['y']) for q in fibre['points'])
print(len(points))
PY
)
    fi
    git rev-parse HEAD > "$out/source-commit.txt"
    c++ --version > "$out/compiler-version.txt"
    printf 'cpu=%s\nvirtual_memory_kib=1048576\nbuild_timeout_seconds=300\nworker_timeout_seconds=600\nmetadata_pool_size=%s\n' "$cpu" "$size" > "$out/resource-envelope.txt"
    sha256sum "$out/source/"* "$out/input/"* > "$out/pre-build-manifest.sha256"
    native build 0 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/sections.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/sections"
    chmod 0555 "$out/bin/sections"
    sha256sum "$out/bin/sections" "$out/source/"* "$out/input/"* > "$out/pre-call-manifest.sha256"
    local first
    for ((first=0;first<size;first+=64)); do
        native "batch-$first" 0 600 "$out/bin/sections" "$out/input/quartic.json" "$out/input/backmap.json" "$out/input/baseline.json" "$out/input/tangents.json" "$supplied" "$first" 64
    done
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-subjects.txt"
}
set +e
{ time (set -e; work); } > "$out/stdout.txt" 2> "$out/wrapper-stderr-and-cost.txt"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit-code.txt"
exit "$code"
