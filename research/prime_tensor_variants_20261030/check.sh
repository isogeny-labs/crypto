#!/usr/bin/env bash
# Native fixed development replay. No benchmark, scaling or ECDLP solve.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-/tmp/prime-ternary-development-check}
mkdir -p "$out/packet" "$out/onelevel"
(cd "$here" && sha256sum --quiet -c source-manifest.sha256)
(cd "$here/identity" && sha256sum --quiet -c source-pins.sha256)
bash "$here/verify-stored-bindings.sh"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
    "$here/packet-unit/packet_unit.cpp" -o "$out/packet/packet_unit"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
    "$here/packet-unit/mode_split_unit.cpp" -o "$out/packet/mode_split_unit"
"$out/packet/packet_unit" "$out/packet/records"
"$out/packet/mode_split_unit" "$out/packet/mode-records"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
    "$here/onelevel-unit/onelevel.cpp" -lgmpxx -lgmp -o "$out/onelevel/onelevel"
"$out/onelevel/onelevel" "$out/onelevel" > "$out/onelevel/run.log" 2>&1
(cd "$out" && sha256sum --quiet -c "$here/replay-records.sha256")
echo 'PASS all16 regenerated fixed tables/logs match frozen hashes'
echo 'PASS development checks only; general formalization, implementation and ECDLP novelty remain open'
