#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?provide a separate output directory}
mkdir -p "$out"
flags=(-std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror)
if [[ ${SANITIZE:-0} == 1 ]]; then
  flags+=(-O1 -g -fsanitize=address,undefined -fno-omit-frame-pointer)
fi
g++ "${flags[@]}" "$here/exact-coded.cpp" -lgmpxx -lgmp -o "$out/exact-coded"
"$out/exact-coded" | tee "$out/results.tsv"
diff -u "$here/expected.tsv" "$out/results.tsv"
