#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=${1:?qualified build directory}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps)
libs=(/workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a -L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -Wl,--gc-sections -lflint -lmpfr -lgmp -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt)
objects=("$out/translation.o" "$out/paper.o" "$out/pullback.o" "$out/incidence.o")
for arm in prior optimized;do
 g++ "${flags[@]}" "$here/target.cpp" "$out/$arm.o" "${objects[@]}" "${libs[@]}" -o "$out/target-$arm"
 chmod 0555 "$out/target-$arm"
done
