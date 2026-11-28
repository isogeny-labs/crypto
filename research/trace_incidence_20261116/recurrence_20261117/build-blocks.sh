#!/usr/bin/env bash
set -euo pipefail
study=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
parent=$(dirname "$study")
out=${1:?existing qualified build directory}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps -I"$parent/native")
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp)
objects=()
for src in "$parent/native/paper.cpp" "$parent/native/pullback.cpp" "$parent/native/isogeny.cpp" "$parent/native/incidence.cpp" "$parent/runpod/incidence_lazy.cpp" "$parent/runpod/benchmark.cpp"; do
 obj="$out/$(basename "$src").o"
 g++ "${flags[@]}" -c "$src" -o "$obj"
 objects+=("$obj")
done
for mode in 4 5; do
 block=16; if ((mode==5)); then block=32; fi
 g++ "${flags[@]}" -DROOT_RECURRENCE_MODE="$mode" -DROOT_PROJECTION_BLOCK="$block" "$study/blocks.cpp" "$parent/native/translation_tests.cpp" "${libs[@]}" -o "$out/test-$mode"
 "$out/test-$mode" > "$out/test-$mode.json"
 g++ "${flags[@]}" -DROOT_RECURRENCE_MODE="$mode" -DROOT_PROJECTION_BLOCK="$block" "$study/blocks.cpp" "${objects[@]}" "${libs[@]}" -o "$out/benchmark-$mode"
 chmod 0555 "$out/test-$mode" "$out/benchmark-$mode"
done
