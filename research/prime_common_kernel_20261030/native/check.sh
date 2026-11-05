#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?provide a separate output directory}
mkdir -p "$out"
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror \
  "$here/exact_controls.cpp" -lgmpxx -lgmp -o "$out/exact-controls"
"$out/exact-controls" | tee "$out/results.tsv"
diff -u "$here/expected.tsv" "$out/results.tsv"
