#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
sed '/^int main(/,$d' point_replay.cpp > "$tmp/point_arithmetic.hpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" destination_relations.cpp -lgmpxx -lgmp -o "$tmp/collector"
sha256sum destination_relations.cpp json.hpp evidence/p192-d7-kernel1-plane-quartic.json "$tmp/point_arithmetic.hpp" "$tmp/collector" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/collector"
"$tmp/collector" evidence/p192-d7-kernel1-plane-quartic.json > "$out/result.json" 2> "$out/stderr.txt"
sha256sum destination_relations.cpp json.hpp > "$out/post-call-source-manifest.sha256"
