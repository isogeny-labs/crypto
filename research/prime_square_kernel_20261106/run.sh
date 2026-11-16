#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
mode=${1:?use --unit or --panel}
out=${2:?provide output directory}
test "$mode" = --unit || test "$mode" = --panel
mkdir -p "$out"
g++ -std=c++20 -O2 -Wall -Wextra -Werror "$here/check.cpp" -lgmpxx -lgmp -o "$out/check"
sha256sum "$out/check" > "$out/binary.sha256"
timeout 900 "$out/check" "$mode" > "$out/results.txt" 2> "$out/stderr.txt"
