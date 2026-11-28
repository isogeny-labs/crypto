#!/bin/bash
# Run directly under the fixed resource limits; Conductor is waived by user.
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
test "$(cat evidence/graph-v1/exit-code.txt)" = 0
test "$(cat evidence/anchors-expanded-v1/exit-code.txt)" = 0
test "$(cat evidence/expanded-conics-v1/exit-code.txt)" = 0
test "$(cat evidence/norms-v2/exit-code.txt)" = 0
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
    cp expanded-conics.cpp EXPANDED_CONIC_PROTOCOL.md FULL_FIBRE_CONIC_PROTOCOL.md run-all-expanded-conics.sh "$out/source/"
    cmp evidence/graph-v1/source/typed_json.hpp evidence/norms-v2/source/typed_json.hpp
    cmp evidence/graph-v1/source/rank_roots.hpp evidence/norms-v2/source/rank_roots.hpp
    cp evidence/graph-v1/source/*.hpp "$out/source/"
    cp evidence/norms-v2/source/{norms.cpp,norm_support.hpp,norm_arithmetic.hpp} "$out/source/"
    cp evidence/fibre-conics-v2/source/conics.cpp "$out/source/original_conics.cpp"
    sed '/^int main(/,$d' "$out/source/original_conics.cpp" > "$out/source/conic_arithmetic.hpp"
    cp evidence/graph-v1/input/{quartic.json,backmap.json,baseline.json,tangents.json,lifted-scan.json} "$out/input/"
    cp evidence/anchors-expanded-v1/calls/scan/stdout.json "$out/input/expanded.json" "$out/input/previous-conics.json"
    cp evidence/expanded-conics-v1/calls/scan/stdout.json "$out/input/previous-conics.json"
    git rev-parse HEAD > "$out/source-commit.txt"
    printf 'cpu=%s\nvirtual_memory_kib=1048576\nbuild_timeout_seconds=300\nworker_timeout_seconds=600\n' "$cpu" > "$out/resource-envelope.txt"
    c++ --version > "$out/compiler-version.txt"
    find "$out/source" "$out/input" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/pre-build-manifest.sha256"
    native build 300 c++ -std=c++17 -DEXPANDED_CONIC_ALL=1 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/expanded-conics.cpp" -lcrypto -lgmpxx -lgmp -o "$out/bin/expanded-conics"
    chmod 0555 "$out/bin/expanded-conics"
    sha256sum "$out/bin/expanded-conics" > "$out/binary-manifest.sha256"
    native scan 600 "$out/bin/expanded-conics" "$out/input/quartic.json" "$out/input/backmap.json" "$out/input/baseline.json" "$out/input/tangents.json" "$out/input/lifted-scan.json" "$out/input/expanded.json" "$out/input/previous-conics.json"
    sha256sum -c "$out/pre-build-manifest.sha256" > "$out/unchanged-subjects.txt"
    sha256sum -c "$out/binary-manifest.sha256" > "$out/unchanged-binary.txt"
}
set +e
{ time (set -e; work); } > "$out/stdout.txt" 2> "$out/wrapper-stderr-and-cost.txt"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit-code.txt"
exit "$code"
