#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
sed '/^int main(/,$d' point_replay.cpp > "$tmp/point_arithmetic.hpp"
sed '/^int main(/,$d' destination_relations.cpp > "$tmp/destination_relations_arithmetic.hpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" quartic_km_v3.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/target-km"
sha256sum quartic_km_v3.cpp json.hpp evidence/p192-d7-kernel1-plane-quartic.json evidence/destination-relations-v2/result.json "$tmp/"*.hpp "$tmp/target-km" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/target-km"
"$tmp/target-km" evidence/p192-d7-kernel1-plane-quartic.json evidence/destination-relations-v2/result.json > "$out/result.json" 2> "$out/stderr.txt"
sha256sum quartic_km_v3.cpp json.hpp > "$out/post-call-source-manifest.sha256"
