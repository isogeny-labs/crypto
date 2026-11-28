#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
mkdir -p "$(dirname "$1")"
mkdir "$1"
out=$(cd "$1" && pwd)
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
native() {
    local label=$1 expected=$2 limit=$3
    shift 3
    local path="$out/calls/$label"
    mkdir "$path"
    set +e
    (ulimit -v 1048576; { time timeout "$limit" taskset -c "$cpu" "$@" > "$path/result.json" 2> "$path/stderr.txt"; } 2> "$path/cost.json")
    local code=$?
    set -e
    printf '%s\n' "$code" > "$path/EXIT_CODE.txt"
    printf '%s\n' "$expected" > "$path/EXPECTED_EXIT_CODE.txt"
    test "$code" -eq "$expected"
}
work() {
    mkdir "$out/source" "$out/input" "$out/bin" "$out/calls" "$out/cases"
    cp context.hpp prepare.cpp extract.cpp verify.cpp PROTOCOL.md run.sh "$out/source/"
    cp ../evidence/opaque-km-primary-v2/source/*.hpp "$out/source/"
    for item in opaque_km opaque_km_boundary; do
        cp "../$item.cpp" "$out/source/$item-source.cpp"
        { printf '#pragma once\n';sed '/^int main(/,$d' "../$item.cpp"; } > "$out/source/${item}_arithmetic.hpp"
    done
    cp ../projective_verification_20261126/normalize_km.cpp "$out/source/normalizer-source.cpp"
    { printf '#pragma once\n';sed '/^int main(/,$d' ../projective_verification_20261126/normalize_km.cpp; } > "$out/source/normalization_arithmetic.hpp"
    cp ../separate_divisor_20261124/check.cpp "$out/source/divisor-source.cpp"
    cp ../correspondence_audit_20261124/check.cpp "$out/source/correspondence-source.cpp"
    cp ../projective_verification_20261126/check.cpp "$out/source/projective-source.cpp"
    cp ../collection_preflight_20261124/typed_json.hpp "$out/source/"
    sed '/^int main(/,$d' "$out/source/divisor-source.cpp" > "$out/source/separate_base.hpp"
    sed '/^int main(/,$d' "$out/source/correspondence-source.cpp" > "$out/source/correspondence_base.hpp"
    sed '/^int main(/,$d' "$out/source/projective-source.cpp" > "$out/source/projective_base.hpp"
    previous=../local_collection_20261124/evidence/attempt-v1/pipeline/baseline-e2e
    cp "$previous/relations/input/inputs/p192-d7-prym.json" "$out/input/cover.json"
    cp "$previous/relations/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/quartic.json"
    cp "$previous/relations/input/evidence/destination-relations-v2/result.json" "$out/input/pool.json"
    cp "$previous/relations/input/backmap.json" "$out/input/backmap.json"
    cp "$previous/relations/result.json" "$out/input/relations.json"
    cp "$previous/transfer/result.json" "$out/input/transfer.json"
    cp ../evidence/formal-series-v1/result.json "$out/input/formal.json"
    cp ../evidence/actual-kernel-v1/result.json "$out/input/actual.json"
    cp ../evidence/complete-km-v3/result.json "$out/input/complete.json"
    cp ../evidence/opaque-km-boundary-v1/result.json "$out/input/boundary.json"
    cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
    printf '%s\n' "$cpu" > "$out/selected-cpu.txt"
    git rev-parse HEAD > "$out/source-commit.txt"
    c++ --version > "$out/compiler-version.txt"
    openssl version > "$out/openssl-version.txt"
    sha256sum "$out/source/"* "$out/input/"* > "$out/build-input-manifest.sha256"
    native build-prepare 0 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/prepare.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/prepare"
    native build-extract 0 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/extract.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/extract"
    native build-verify 0 300 c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/verify.cpp" -lcrypto -o "$out/bin/verify"
    chmod 0555 "$out/bin/"*
    sha256sum "$out/source/"* "$out/input/"* "$out/bin/"* > "$out/pre-call-manifest.sha256"
    native prepare 0 600 "$out/bin/prepare" "$out/input/cover.json" "$out/input/quartic.json" "$out/input/relations.json" "$out/input/formal.json" "$out/input/actual.json" "$out/input/complete.json" "$out/input/boundary.json" "$out/input/transfer.json" "$out/cases"
    sha256sum "$out/cases/"* > "$out/cases-manifest.sha256"
    for label in principal-m0 principal-m1 principal-m2 principal-m3 nonzero-m1 nonzero-m2 public-FG public-double-FG special-rr2 triple-reference double-single-reference triple-pole; do
        native "$label-extract" 0 600 "$out/bin/extract" "$out/input/cover.json" "$out/input/quartic.json" "$out/input/relations.json" "$out/input/formal.json" "$out/input/actual.json" "$out/cases/$label.json"
        native "$label-verify" 0 600 "$out/bin/verify" "$out/input/quartic.json" "$out/input/backmap.json" "$out/calls/$label-extract/result.json" "$out/cases/$label.json" "$out/input/pool.json" "$out/input/formal.json"
    done
    native off-ambient-extract 1 600 "$out/bin/extract" "$out/input/cover.json" "$out/input/quartic.json" "$out/input/relations.json" "$out/input/formal.json" "$out/input/actual.json" "$out/cases/off-ambient.json"
    sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-bytes.txt"
    sha256sum -c "$out/cases-manifest.sha256" > "$out/unchanged-cases.txt"
    sha256sum context.hpp prepare.cpp extract.cpp verify.cpp PROTOCOL.md run.sh > "$out/post-call-source-manifest.sha256"
}
set +e
{ time (set -e;work); } 2> "$out/wrapper-cost.json"
code=$?
set -e
printf '%s\n' "$code" > "$out/WRAPPER_EXIT_CODE.txt"
exit "$code"
