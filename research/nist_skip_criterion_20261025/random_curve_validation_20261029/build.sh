#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
build=${1:?usage: build.sh FRESH_BUILD_DIR}
test ! -e "$build"
mkdir -p "$build"
pari_root=${PARI_ROOT:-/workspace/tools/sage_skeleton_env}
cxx=${CXX:-g++}
"$cxx" -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
  -I"$pari_root/include" "$study/sampler.cpp" \
  -L"$pari_root/lib" -Wl,-rpath,"$pari_root/lib" \
  -l:libpari-gmp-tls.so.9 -lgmpxx -lgmp -lcrypto -o "$build/sampler"
"$cxx" -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
  "$study/cutoff_sweep.cpp" -lgmpxx -lgmp -o "$build/cutoff_sweep"
"$cxx" -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
  "$study/percentiles.cpp" -lgmpxx -lgmp -o "$build/percentiles"
"$cxx" -std=c++17 -O2 -Wall -Wextra -Werror -pedantic \
  -I"$pari_root/include" "$study/verify_sweep.cpp" \
  -L"$pari_root/lib" -Wl,-rpath,"$pari_root/lib" \
  -l:libpari-gmp-tls.so.9 -o "$build/verify_sweep"
sha256sum "$study/sampler.cpp" "$study/cutoff_sweep.cpp" \
  "$study/percentiles.cpp" "$study/verify_sweep.cpp" \
  "$build/sampler" "$build/cutoff_sweep" "$build/percentiles" \
  "$build/verify_sweep" > "$build/sha256.txt"
