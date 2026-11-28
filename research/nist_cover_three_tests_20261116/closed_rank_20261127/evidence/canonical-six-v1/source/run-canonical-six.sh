#!/bin/bash
# Local bounded native computation; Conductor waived by explicit user instruction.
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
test "$(cat evidence/six-v1/exit-code.txt)" = 0
test "$(cat evidence/high-contact-scan-v1/exit-code.txt)" = 0
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
native() {
    local name=$1 limit=$2
    shift 2
    mkdir "$out/calls/$name"
    printf '%s\0' "$@" > "$out/calls/$name/argv.nul"
    set +e
    (ulimit -v 1048576; { time timeout --kill-after=3 "$limit" taskset -c "$cpu" "$@" > "$out/calls/$name/stdout.json" 2> "$out/calls/$name/stderr.txt"; } 2> "$out/calls/$name/cost.json")
    local code=$?
    set -e
    printf '%s\n' "$code" > "$out/calls/$name/exit-code.txt"
    test "$code" -eq 0
}
work() {
    mkdir "$out/source" "$out/input" "$out/bin" "$out/calls"
    cp canonical-six.cpp shifted-six-arithmetic.hpp shifted-six-basis.hpp CANONICAL_SIX_PROTOCOL.md run-canonical-six.sh "$out/source/"
    cmp evidence/graph-v1/source/typed_json.hpp evidence/norms-v2/source/typed_json.hpp
    cmp evidence/graph-v1/source/rank_roots.hpp evidence/norms-v2/source/rank_roots.hpp
    cp evidence/graph-v1/source/*.hpp "$out/source/"
    cp evidence/norms-v2/source/{norms.cpp,norm_support.hpp,norm_arithmetic.hpp} "$out/source/"
    cp evidence/graph-v1/input/{quartic.json,backmap.json,baseline.json,tangents.json} "$out/input/"
    cp evidence/six-v1/calls/full/stdout.json "$out/input/prior-six.json"
    cp evidence/shifted-six-v2/calls/full/stdout.json "$out/input/shifted-six.json"
    git rev-parse HEAD > "$out/source-commit.txt"
    printf 'cpu=%s\nvirtual_memory_kib=1048576\nbuild_timeout_seconds=300\nworker_timeout_seconds=600\n' "$cpu" > "$out/resource-envelope.txt"
    c++ --version > "$out/compiler-version.txt"
    find "$out/source" "$out/input" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/pre-build-manifest.sha256"
    native build 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/canonical-six.cpp" -lcrypto -lgmpxx -lgmp -o "$out/bin/canonical-six"
    chmod 0555 "$out/bin/canonical-six"
    sha256sum "$out/bin/canonical-six" > "$out/binary-manifest.sha256"
    native full 600 "$out/bin/canonical-six" "$out/input/quartic.json" "$out/input/backmap.json" "$out/input/baseline.json" "$out/input/tangents.json" "$out/input/prior-six.json" "$out/input/shifted-six.json"
    sha256sum "$out"/calls/*/stdout.json > "$out/result-manifest.sha256"
    sha256sum -c "$out/pre-build-manifest.sha256" > "$out/unchanged-subjects.txt"
    sha256sum -c "$out/binary-manifest.sha256" > "$out/unchanged-binary.txt"
}
set +e
{ time (set -e; work); } > "$out/stdout.txt" 2> "$out/wrapper-stderr-and-cost.txt"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit-code.txt"
exit "$code"
