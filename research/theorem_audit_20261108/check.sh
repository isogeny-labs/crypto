#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
tmp=$(mktemp -d)
source_file="$root/research/ecdlp_weakness_census_20261104/pq2_rank/src/main.rs"
cat "$source_file" "$here/production-control-tests.rs" > "$tmp/control.rs"
rustc --edition=2021 --test "$tmp/control.rs" -o "$tmp/rust-control"
"$tmp/rust-control"
g++ -O2 -Wall -Wextra -Werror -std=c++17 "$here/degree_control.cpp" -o "$tmp/cpp-control"
"$tmp/cpp-control"
(cd "$here/lean" && lake build && lake env lean Audit.lean)
