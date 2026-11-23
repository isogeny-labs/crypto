#!/usr/bin/env bash
set -euo pipefail
study=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
parent=$(dirname "$study")
out=${1:?existing qualified build directory}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps -I"$parent/native")
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp)
objects=("$out/paper.cpp.o" "$out/pullback.cpp.o" "$out/isogeny.cpp.o" "$out/incidence.cpp.o" "$out/incidence_lazy.cpp.o" "$out/benchmark.cpp.o")
for mode in 6 7; do
 block=64; if ((mode==7)); then block=128; fi
 g++ "${flags[@]}" -DROOT_RECURRENCE_MODE="$mode" -DROOT_PROJECTION_BLOCK="$block" "$study/blocks.cpp" "$parent/native/translation_tests.cpp" "${libs[@]}" -o "$out/test-$mode"
 "$out/test-$mode" > "$out/test-$mode.json"
 g++ "${flags[@]}" -DROOT_RECURRENCE_MODE="$mode" -DROOT_PROJECTION_BLOCK="$block" "$study/blocks.cpp" "${objects[@]}" "${libs[@]}" -o "$out/benchmark-$mode"
 chmod 0555 "$out/test-$mode" "$out/benchmark-$mode"
done
