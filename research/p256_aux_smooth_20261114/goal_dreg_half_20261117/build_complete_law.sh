#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_complete_law.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/membership_certificate_v2.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/membership_certificate_v2.cpp" > "$build_dir/membership_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror -I "$build_dir" -I "$study_dir" \
  "$study_dir/complete_law.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
sha256sum "$study_dir/complete_law.cpp" "$study_dir/membership_certificate_v2.cpp" "$study_dir/../aux_base.cpp" > "$build_dir/input-hashes.txt"
