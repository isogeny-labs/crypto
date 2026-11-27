#!/bin/bash
# Run directly under the fixed resource limits; Conductor is waived by user.
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
test "$(cat evidence/graph-v1/exit-code.txt)" = 0
test "$(cat evidence/anchors-expanded-v1/exit-code.txt)" = 0
test "$(cat evidence/expanded-balanced-v1/exit-code.txt)" = 0
test "$(cat evidence/mixed-sign-conics-v1/exit-code.txt)" = 0
test "$(cat evidence/norms-v2/exit-code.txt)" = 0
test "$(cat evidence/four-line-conics-v1/exit-code.txt)" = 0
test "$(cat evidence/tangent-conics-v1/exit-code.txt)" = 0
test "$(cat evidence/high-contact-controls-v1/exit-code.txt)" = 0
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
    cp high-contact-scan.cpp weighted-conic-arithmetic.hpp HIGH_CONTACT_SCAN_PROTOCOL.md HIGH_CONTACT_CONTROL_PROTOCOL.md run-high-contact-scan.sh "$out/source/"
    cmp evidence/graph-v1/source/typed_json.hpp evidence/norms-v2/source/typed_json.hpp
    cmp evidence/graph-v1/source/rank_roots.hpp evidence/norms-v2/source/rank_roots.hpp
    cp evidence/graph-v1/source/*.hpp "$out/source/"
    cp evidence/norms-v2/source/{norms.cpp,norm_support.hpp,norm_arithmetic.hpp} "$out/source/"
    cp evidence/fibre-conics-v2/source/conics.cpp "$out/source/original_conics.cpp"
    sed '/^int main(/,$d' "$out/source/original_conics.cpp" > "$out/source/conic_arithmetic.hpp"
    cp evidence/graph-v1/input/{quartic.json,backmap.json,baseline.json,tangents.json,lifted-scan.json} "$out/input/"
    cp evidence/anchors-expanded-v1/calls/scan/stdout.json "$out/input/expanded.json"
    cp evidence/expanded-balanced-v1/calls/scan/stdout.json "$out/input/balanced.json"
    cp evidence/mixed-sign-conics-v1/calls/combine/stdout.json "$out/input/x-fibre.json"
    cp evidence/high-contact-controls-v1/calls/preflight/stdout.json "$out/input/controls.json"
    git rev-parse HEAD > "$out/source-commit.txt"
    printf 'cpu=%s\nvirtual_memory_kib=1048576\nbuild_timeout_seconds=300\nworker_timeout_seconds=600\n' "$cpu" > "$out/resource-envelope.txt"
    c++ --version > "$out/compiler-version.txt"
    find "$out/source" "$out/input" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/pre-build-manifest.sha256"
    native build 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/high-contact-scan.cpp" -lcrypto -lgmpxx -lgmp -o "$out/bin/high-contact-scan"
    chmod 0555 "$out/bin/high-contact-scan"
    sha256sum "$out/bin/high-contact-scan" > "$out/binary-manifest.sha256"
    local -a arguments=("$out/input/quartic.json" "$out/input/backmap.json" "$out/input/baseline.json" "$out/input/tangents.json" "$out/input/lifted-scan.json" "$out/input/expanded.json" "$out/input/balanced.json" "$out/input/x-fibre.json")
    native plan 600 "$out/bin/high-contact-scan" "${arguments[@]}" plan "$out/schedule.tsv" "$out/input/controls.json"
    sha256sum "$out/calls/plan/stdout.json" "$out/schedule.tsv" > "$out/plan-and-schedule-manifest.sha256"
    : > "$out/batches.list"
    local first count
    while read -r first count; do
        native "batch-$first" 600 "$out/bin/high-contact-scan" "${arguments[@]}" batch "$out/calls/plan/stdout.json" "$first" "$count" "$out/input/controls.json"
        printf '%s\n' "$out/calls/batch-$first/stdout.json" >> "$out/batches.list"
    done < "$out/schedule.tsv"
    native combine 600 "$out/bin/high-contact-scan" "${arguments[@]}" combine "$out/calls/plan/stdout.json" "$out/batches.list" "$out/input/controls.json"
    sha256sum "$out/batches.list" "$out"/calls/*/stdout.json > "$out/result-manifest.sha256"
    sha256sum -c "$out/plan-and-schedule-manifest.sha256" > "$out/unchanged-plan.txt"
    sha256sum -c "$out/pre-build-manifest.sha256" > "$out/unchanged-subjects.txt"
    sha256sum -c "$out/binary-manifest.sha256" > "$out/unchanged-binary.txt"
}
set +e
{ time (set -e; work); } > "$out/stdout.txt" 2> "$out/wrapper-stderr-and-cost.txt"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit-code.txt"
exit "$code"
