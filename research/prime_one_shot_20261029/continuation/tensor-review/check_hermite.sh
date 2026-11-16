#!/usr/bin/env bash
# Fixed scalar-coordinate lemma unit; no contraction/performance experiment.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-/tmp/prime-tensor-hermite-replay}
mkdir -p "$out"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/lemma-unit/hermite_unit.cpp" -o "$out/hermite_unit" -lgmpxx -lgmp
"$out/hermite_unit" "$out/records"
(cd "$out/records" && sha256sum --quiet -c "$here/lemma-unit/records.sha256")
echo 'PASS regenerated exact unit tables against all seven frozen raw hashes'
