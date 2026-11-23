#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd);out=${1:?qualified build directory}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps)
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp -Wl,--gc-sections -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt)
objs=("$out/optimized.o" "$out/translation.o" "$out/paper.o" "$out/pullback.o" "$out/incidence.o" /workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a)
g++ "${flags[@]}" "$here/make-field-confirmation.cpp" "$out/translation.o" "${libs[@]}" -o "$out/make-field-confirmation"
"$out/make-field-confirmation" "${2:?original minimal}" "${3:?new public directory}" "${4:?new private validation}" > "$out/field-confirmation-construction.json"
g++ "${flags[@]}" "$here/test-field-generation.cpp" "${objs[@]}" "${libs[@]}" -o "$out/test-field-generation"
"$out/test-field-generation" "$3" > "$out/test-field-generation.json"
for variant in bloom direct;do
 extra=();if [[ $variant == direct ]];then extra=(-DNO_BLOOM);fi
 g++ "${flags[@]}" "${extra[@]}" "$here/target-field.cpp" "${objs[@]}" "${libs[@]}" -o "$out/target-field-$variant"
 g++ "${flags[@]}" "${extra[@]}" "$here/test-canonical-field.cpp" "${objs[@]}" "${libs[@]}" -o "$out/test-canonical-field-$variant"
 "$out/test-canonical-field-$variant" "$here/.." > "$out/test-canonical-field-$variant.json"
 chmod 0555 "$out/target-field-$variant"
done
