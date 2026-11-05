#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?provide a separate output directory}
mkdir -p "$out"
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror \
  "$here/check.cpp" -lgmpxx -lgmp -o "$out/parity-check"
"$out/parity-check" | tee "$out/results.log"
diff -u "$here/expected.log" "$out/results.log"
