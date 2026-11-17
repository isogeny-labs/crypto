#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_complete.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
frozen_dir="$study_dir/../continuation_20261116"
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$frozen_dir/linear_reduce.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$frozen_dir/linear_reduce.cpp" > "$build_dir/frozen_linear_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  -I "$build_dir" -I "$frozen_dir" "$study_dir/complete_export.cpp" \
  -lgmpxx -lgmp -lcrypto -o "$build_dir/export" > "$build_dir/build.log" 2>&1
timeout 60 "$build_dir/export" export "$build_dir/inputs" > "$build_dir/export.log" 2>&1
(cd "$build_dir/inputs" && sha256sum ./* > "$build_dir/inputs.sha256")
