#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd);out=${1:?qualified build directory}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps)
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp)
g++ "${flags[@]}" "$here/test-flat.cpp" -o "$out/test-flat";"$out/test-flat" > "$out/test-flat.json"
g++ "${flags[@]}" "$here/make-confirmation.cpp" "$out/translation.o" "${libs[@]}" -o "$out/make-confirmation"
"$out/make-confirmation" "${2:?original minimal fixtures}" "${3:?new confirmation input directory}" "${4:?new private validation file}" > "$out/confirmation-construction.json"
for variant in full gen;do
 extra=();if [[ $variant == gen ]];then extra=(-DTREE_INDEX);fi
 g++ "${flags[@]}" "${extra[@]}" "$here/target-flat.cpp" "$out/optimized.o" "$out/translation.o" "$out/paper.o" "$out/pullback.o" "$out/incidence.o" /workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a "${libs[@]}" -Wl,--gc-sections -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt -o "$out/target-flat-$variant"
 chmod 0555 "$out/target-flat-$variant"
done
