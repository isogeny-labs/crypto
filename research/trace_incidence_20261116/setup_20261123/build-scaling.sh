#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd);out=${1:?qualified build}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr};pari=${PARI_PREFIX:-/workspace/native-deps/pari-dev/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps)
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp -Wl,--gc-sections -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt)
objs=("$out/optimized.o" "$out/translation.o" "$out/paper.o" "$out/pullback.o" "$out/incidence.o" /workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a)
g++ "${flags[@]}" -I"$pari/include/x86_64-linux-gnu" "$here/make-scaling.cpp" "$out/translation.o" -L"$pari/lib/x86_64-linux-gnu" -Wl,-rpath,"$pari/lib/x86_64-linux-gnu" -lpari "${libs[@]}" -o "$out/make-scaling"
for variant in baseline bloom direct;do
 extra=();if [[ $variant == baseline ]];then extra=(-DSCALING_BASELINE);fi
 if [[ $variant == direct ]];then extra=(-DNO_BLOOM);fi
 g++ "${flags[@]}" "${extra[@]}" "$here/target-scaling.cpp" "${objs[@]}" "${libs[@]}" -o "$out/target-scaling-$variant"
 chmod 0555 "$out/target-scaling-$variant"
done
