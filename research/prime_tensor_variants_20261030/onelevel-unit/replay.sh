#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
out_dir="${1:-replay-output}"
mkdir -p "$out_dir"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror onelevel.cpp -lgmpxx -lgmp -o "$out_dir/onelevel" >"$out_dir/build.log" 2>&1
"$out_dir/onelevel" "$out_dir" >"$out_dir/run.log" 2>&1
