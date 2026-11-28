#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
prior=$(cd evidence/attempt-v1 && pwd)
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
native() {
    local label=$1 limit=$2
    shift 2
    local path="$out/calls/$label"
    mkdir "$path"
    set +e
    (ulimit -v 1048576; { time timeout "$limit" taskset -c "$cpu" "$@" > "$path/result.json" 2> "$path/stderr.txt"; } 2> "$path/cost.json")
    local code=$?
    set -e
    printf '%s\n' "$code" > "$path/EXIT_CODE.txt"
    test "$code" -eq 0
}
work() {
    mkdir "$out/source" "$out/input" "$out/bin" "$out/calls" "$out/certificates" "$out/expected"
    cp pushforward.cpp INTERFACE.md evaluate-class.sh run-eval.sh "$out/source/"
    cp "$prior/source/verify.cpp" "$out/source/verified-source.cpp"
    cp "$prior/source/separate_base.hpp" "$prior/source/correspondence_base.hpp" "$prior/source/projective_base.hpp" "$prior/source/typed_json.hpp" "$out/source/"
    { printf '#pragma once\n';sed '/^int main(/,$d' "$out/source/verified-source.cpp"; } > "$out/source/verify_arithmetic.hpp"
    cp "$prior/input/quartic.json" "$prior/input/backmap.json" "$prior/input/pool.json" "$prior/input/formal.json" "$out/input/"
    labels=(principal-m0 principal-m1 principal-m2 principal-m3 nonzero-m1 nonzero-m2 public-FG public-double-FG special-rr2 triple-reference double-single-reference triple-pole)
    for label in "${labels[@]}";do
        cp "$prior/calls/$label-extract/result.json" "$out/certificates/$label.json"
        cp "$prior/calls/$label-verify/result.json" "$out/expected/$label.json"
    done
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    git rev-parse HEAD > "$out/source-commit.txt"
    c++ --version > "$out/compiler-version.txt"
    openssl version > "$out/openssl-version.txt"
    sha256sum "$out/source/"* "$out/input/"* "$out/certificates/"* "$out/expected/"* > "$out/build-input-manifest.sha256"
    native build 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/pushforward.cpp" -lcrypto -o "$out/bin/pushforward"
    chmod 0555 "$out/bin/pushforward"
    sha256sum "$out/source/"* "$out/input/"* "$out/certificates/"* "$out/expected/"* "$out/bin/"* > "$out/pre-call-manifest.sha256"
    for label in "${labels[@]}";do
        native "$label-eval" 600 "$out/bin/pushforward" "$out/input/quartic.json" "$out/input/backmap.json" "$out/certificates/$label.json" "$out/input/pool.json" "$out/input/formal.json"
        native "$label-compare" 600 "$out/bin/pushforward" --compare "$out/calls/$label-eval/result.json" "$out/expected/$label.json"
    done
    bash "$out/source/evaluate-class.sh" "$prior" "$out" "$prior/cases/public-FG.json" "$out/facade" > "$out/facade.stdout" 2> "$out/facade.stderr"
    native facade-compare 600 "$out/bin/pushforward" --compare "$out/facade/result.json" "$out/expected/public-FG.json"
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
    sha256sum pushforward.cpp INTERFACE.md evaluate-class.sh run-eval.sh > "$out/post-call-source-manifest.sha256"
}
set +e
{ time (set -e;work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
