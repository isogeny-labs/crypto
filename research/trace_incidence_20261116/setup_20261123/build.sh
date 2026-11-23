#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
parent=$(dirname "$here")
out=${1:?new output directory}; mkdir -p "$out"
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
static=${RUST_STATIC:-/workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps -I"$parent/accelerated")
libs=("$static" -L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -Wl,--gc-sections -lflint -lmpfr -lgmp -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt)
objs=()
for name in translation paper pullback incidence; do
 g++ "${flags[@]}" -c "$parent/native/$name.cpp" -o "$out/$name.o";objs+=("$out/$name.o")
done
g++ "${flags[@]}" -c "$parent/accelerated/accelerated.cpp" -o "$out/prior.o"
g++ "${flags[@]}" -c "$here/accelerated.cpp" -o "$out/optimized.o"
for arm in prior optimized; do
 extra=();if [[ $arm == optimized ]];then extra=(-DFAST_PUBLIC_CHECK);fi
 g++ "${flags[@]}" "${extra[@]}" "$here/benchmark.cpp" "$out/$arm.o" "${objs[@]}" "${libs[@]}" -o "$out/$arm"
 g++ "${flags[@]}" "$parent/accelerated/audit.cpp" "$out/$arm.o" "${objs[@]}" "${libs[@]}" -o "$out/audit-$arm"
 chmod 0555 "$out/$arm" "$out/audit-$arm"
 "$out/audit-$arm" "$parent" > "$out/audit-$arm.json"
done
sha256sum "$static" "$out/prior" "$out/optimized" "$out/audit-prior" "$out/audit-optimized" > "$out/qualified.sha256"
