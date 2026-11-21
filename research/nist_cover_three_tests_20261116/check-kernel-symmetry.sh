#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" kernel_symmetry.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/kernel-symmetry" 2> "$out/build.stderr"
sha256sum kernel_symmetry.cpp KERNEL_SYMMETRY_PROTOCOL.md inputs/p192-d7-prym.json evidence/final-kernel-check/kernels.jsonl "$tmp/"*.hpp "$tmp/kernel-symmetry" > "$out/pre-call-manifest.sha256"
"$tmp/kernel-symmetry" "$PWD" > "$out/result.json" 2> "$out/stderr.txt"
sha256sum kernel_symmetry.cpp > "$out/post-call-source-manifest.sha256"
