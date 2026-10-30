#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
build=${1:-/tmp/tensor-lemma-unit-build}
mkdir -p "$build"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/hermite_unit.cpp" -o "$build/hermite_unit" -lgmpxx -lgmp
"$build/hermite_unit" "$build/replay"
for record in cases.tsv kernels.tsv weights.tsv node-errors.tsv final-errors.tsv negative.tsv summary.json; do
    cmp "$here/records/$record" "$build/replay/$record"
done
echo 'PASS byte-identical replay of every retained scientific record; no timing comparison'
