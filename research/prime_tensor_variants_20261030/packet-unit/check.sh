#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
build=${1:-/tmp/next-ternary-unit-replay}
mkdir -p "$build"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/packet_unit.cpp" -o "$build/packet_unit"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/mode_split_unit.cpp" -o "$build/mode_split_unit"
"$build/packet_unit" "$build/records"
"$build/mode_split_unit" "$build/mode-records"
for record in classes.tsv orbits.tsv marginals.tsv graphs.tsv tiles.tsv field67.tsv negative.tsv summary.json; do
    cmp "$here/records/$record" "$build/records/$record"
done
for record in matrix.tsv cases.tsv witness.tsv summary.json; do
    cmp "$here/mode-records/$record" "$build/mode-records/$record"
done
echo 'PASS byte-identical replay of all12retained unit records; no timing comparison'
