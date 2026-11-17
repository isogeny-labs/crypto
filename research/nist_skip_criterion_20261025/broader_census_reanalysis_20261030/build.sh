#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
build=${1:?usage: build.sh FRESH_BUILD_DIR}
test ! -e "$build"
mkdir -p "$build"
pari_root=${PARI_ROOT:-/workspace/tools/sage_skeleton_env}
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$here/producer.cpp" -lgmpxx -lgmp -o "$build/producer"
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I"$pari_root/include" \
 "$here/verify.cpp" -L"$pari_root/lib" -Wl,-rpath,"$pari_root/lib" -l:libpari-gmp-tls.so.9 -o "$build/verify"
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$here/tamper.cpp" -o "$build/tamper"
sha256sum "$here/io.hpp" "$here/producer.cpp" "$here/verify.cpp" "$here/tamper.cpp" \
 "$build/producer" "$build/verify" "$build/tamper" > "$build/sha256.txt"
