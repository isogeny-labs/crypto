#!/usr/bin/env bash
set -euo pipefail
here="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
prior="$here/../../prime_field_followup_20261117/split4/split4.cpp"
build="$(mktemp -d)"
trap 'rm -rf "$build"' EXIT
{
    sed '/^struct Summary/,$d' "$prior"
    sed -n '/^void validate_root_grid/,/^int main()/p' "$prior" | sed '$d'
    cat "$here/reuse.cpp"
} > "$build/combined.cpp"
"${CXX:-g++}" -std=c++20 -O3 -Wall -Wextra -Werror "$build/combined.cpp" -o "$build/reuse"
timeout 120 "$build/reuse"
