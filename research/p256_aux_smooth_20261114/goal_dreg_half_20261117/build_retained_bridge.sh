#!/usr/bin/env bash
# Cached-polynomial serialization and witness integration; no GB/regularity run.
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: build_retained_bridge.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
build_dir=$(cd "$1" && pwd)
mkdir "$build_dir/cache"
for depth in 4 5; do
  for arm in dense structured; do
    cert="$study_dir/evidence/retained-native/depth$depth-signed-$arm.json"
    expected=$(jq -er --arg cell "depth$depth-signed-$arm" '.cells[] | select(.cell==$cell) | .certificate_sha256' "$study_dir/evidence/retained-native/summary.json")
    actual=$(sha256sum "$cert")
    [[ ${actual%% *} == "$expected" ]]
    jq -er '"VARS \(.variables|length) \(.variables|join(" "))",
      "POINTS \(.points|length)", "GENERATORS \(.affine_generators|length)",
      (.affine_generators[] | "POLYNOMIAL \(length)",
       (.[] | "TERM \(.coefficient) \(.powers|length) " +
         (.powers|to_entries|map("\(.key) \(.value)")|join(" ")))), "END"' \
      "$cert" > "$build_dir/cache/depth$depth-signed-$arm.txt"
  done
done
frozen_dir="$study_dir/../continuation_20261116"
[[ $(grep -c '^int main(int argc,char\*\* argv) {' "$frozen_dir/linear_reduce.cpp") == 1 ]]
sed '/^int main(int argc,char\*\* argv) {/,$d' "$frozen_dir/linear_reduce.cpp" > "$build_dir/frozen_linear_prefix.inc"
timeout 60 "${CXX:-c++}" -std=c++20 -O2 -Wall -Wextra -Werror \
  -I "$build_dir" -I "$frozen_dir" "$study_dir/retained_bridge.cpp" \
  -lgmpxx -lgmp -lcrypto -o "$build_dir/export" > "$build_dir/build.log" 2>&1
timeout 60 "$build_dir/export" export "$build_dir/cache" "$build_dir/inputs" > "$build_dir/export.log" 2>&1
