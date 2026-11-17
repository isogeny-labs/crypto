#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
build=${1:?usage: build.sh FRESH_BUILD}
test ! -e "$build"
mkdir -p "$build"
for name in producer verify tamper;do
 g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$here/$name.cpp" -o "$build/$name"
done
sha256sum "$here/io.hpp" "$here/producer.cpp" "$here/verify.cpp" "$here/tamper.cpp" \
 "$build/producer" "$build/verify" "$build/tamper" > "$build/sha256.txt"
