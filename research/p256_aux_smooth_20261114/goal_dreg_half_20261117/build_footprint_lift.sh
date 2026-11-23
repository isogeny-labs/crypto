#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_footprint_lift.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
cert="$study_dir/evidence/retained-native/depth5-signed-structured.json"
expected=$(jq -er '.cells[]|select(.cell=="depth5-signed-structured")|.certificate_sha256' "$study_dir/evidence/retained-native/summary.json")
actual=$(sha256sum "$cert")
[[ ${actual%% *} == "$expected" ]]
jq -er '"VARS \(.variables|length) \(.variables|join(" "))", "POINTS \(.points|length)",
  (.variables as $v | .points[]| . as $p | $v|map($p[.])|join(" ")),
  ([.standard_monomials[] | .[0].powers | select((add)==4)] as $f |
    "FEATURES \($f|length)", ($f[] | "\(length) " + (to_entries|map("\(.key) \(.value)")|join(" ")))),"END"' "$cert" > "$build_dir/cache.txt"
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$study_dir/retained_basis.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$study_dir/retained_basis.cpp" > "$build_dir/retained_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror -I "$build_dir" -I "$study_dir" \
  "$study_dir/footprint_lift.cpp" -lgmpxx -lgmp -lcrypto -o "$build_dir/check" > "$build_dir/build.log" 2>&1
sha256sum "$study_dir/footprint_lift.cpp" "$study_dir/retained_basis.cpp" "$cert" "$build_dir/cache.txt" > "$build_dir/input-hashes.txt"
