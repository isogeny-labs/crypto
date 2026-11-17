#!/bin/sh
# Fixed native development fixtures; no timing or DLP benchmark.
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
sed '/^int main(/,$d' point_replay.cpp > "$tmp/point_arithmetic.hpp"
sed '/^int main(/,$d' cover_replay.cpp > "$tmp/cover_arithmetic.hpp"
sed '/^int main(/,$d' kernel_point_replay.cpp > "$tmp/kernel_arithmetic.hpp"
sed '/^int main(/,$d' module_audit.cpp > "$tmp/module_arithmetic.hpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" quotient_replay.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/quotient-replay"
sha256sum quotient_replay.cpp json.hpp "$tmp/"*_arithmetic.hpp "$tmp/quotient-replay" inputs/*.json evidence/final-kernel-check/actual-cover-torsion.json evidence/final-kernel-check/kernels.jsonl ../nist_cover_separator_20261115/data/modules-192.json > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/quotient-replay"
"$tmp/quotient-replay" "$PWD" "$out/rows.jsonl" > "$out/stdout.txt" 2> "$out/stderr.txt"
sha256sum quotient_replay.cpp json.hpp > "$out/post-call-source-manifest.sha256"
cat "$out/stdout.txt"
