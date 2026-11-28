#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd);out=${1:?qualified build}
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
base=(-std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps)
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp -Wl,--gc-sections -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt)
objs=("$out/optimized.o" "$out/translation.o" "$out/paper.o" "$out/pullback.o" "$out/incidence.o" /workspace/native-deps/cargo-target/release/libtrace_incidence_accelerated.a)
g++ -O2 "${base[@]}" "$here/make-hot-confirmation.cpp" "$out/translation.o" "${libs[@]}" -o "$out/make-hot-confirmation"
"$out/make-hot-confirmation" "${2:?old scaling public}" "${3:?new public}" "${4:?new private validation}" > "$out/hot-construction.json"
for variant in baseline reserve blocked combined baseline-native combined-native;do
 opt=(-O2);extra=()
 if [[ $variant == *native ]];then opt=(-O3 -march=native);fi
 if [[ $variant == reserve || $variant == combined* ]];then extra+=(-DRESERVED_POOLS);fi
 if [[ $variant == blocked || $variant == combined* ]];then extra+=(-DBLOCKED_FILTER);fi
 g++ "${opt[@]}" "${base[@]}" "${extra[@]}" "$here/target-hot.cpp" "${objs[@]}" "${libs[@]}" -o "$out/target-hot-$variant"
 chmod 0555 "$out/target-hot-$variant"
done
for opt in generic native;do
 flags=(-O2);if [[ $opt == native ]];then flags=(-O3 -march=native);fi
 g++ "${flags[@]}" "${base[@]}" "$here/test-canonical-blocked.cpp" "${objs[@]}" "${libs[@]}" -o "$out/test-canonical-blocked-$opt"
 "$out/test-canonical-blocked-$opt" "$here/.." > "$out/test-canonical-blocked-$opt.json"
done
