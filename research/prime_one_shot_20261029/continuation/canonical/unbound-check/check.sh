#!/usr/bin/env bash
# Fixed retained-output correctness check only; no external solve or timing.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-/tmp/prime-unbound-staircase-check}
mkdir -p "$out"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/staircase.cpp" -o "$out/staircase"
"$out/staircase" "$here/remainder.gb" > "$out/staircase.txt"
cmp "$here/staircase.txt" "$out/staircase.txt"
head -1 "$out/staircase.txt"
echo 'PASS retained printed staircase; no source-ideal certificate or solving-degree claim'
