#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_cocoa.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
frozen_dir="$study_dir/../continuation_20261116"
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$frozen_dir/linear_reduce.cpp") == 1 ]]
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/complete_export.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$frozen_dir/linear_reduce.cpp" > "$build_dir/frozen_linear_prefix.inc"
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/complete_export.cpp" > "$build_dir/frozen_complete_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  -I "$build_dir" -I "$frozen_dir" "$study_dir/cocoa_export.cpp" \
  -lgmpxx -lgmp -lcrypto -o "$build_dir/export" > "$build_dir/build.log" 2>&1
timeout 60 "$build_dir/export" export "$study_dir/evidence/complete-inputs" "$build_dir/inputs" > "$build_dir/export.log" 2>&1
