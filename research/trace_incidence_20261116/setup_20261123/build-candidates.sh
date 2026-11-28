#!/usr/bin/env bash
# Rebuild optional progression/canonical candidates using build.sh's sealed objects.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd);out=${1:?qualified build directory}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps)
libs=(/workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a -L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -Wl,--gc-sections -lflint -lmpfr -lgmp -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt)
objs=("$out/optimized.o" "$out/translation.o" "$out/paper.o" "$out/pullback.o" "$out/incidence.o")
for name in target-ap target-kernel test-canonical test-canonical-flat test-flat-generation;do
 g++ "${flags[@]}" "$here/$name.cpp" "${objs[@]}" "${libs[@]}" -o "$out/$name"
done
for name in analyze analyze-target analyze-ap analyze-holdout analyze-flat project make-holdout;do
 g++ "${flags[@]}" "$here/$name.cpp" "$out/translation.o" "${libs[@]:1}" -o "$out/$name"
done
