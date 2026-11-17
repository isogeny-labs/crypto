#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?provide output directory}
mkdir -p "$out"
g++ -std=c++20 -O2 -Wall -Wextra -Werror "$here/check.cpp" -lgmpxx -lgmp -o "$out/check"
"$out/check" | tee "$out/results.txt"
diff -u "$here/expected.txt" "$out/results.txt"
