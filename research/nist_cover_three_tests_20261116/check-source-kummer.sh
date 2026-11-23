#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
out=${2:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        weil3) source=weil3.cpp; dest=weil3_arithmetic ;;
        quotient_theta2) source=quotient_theta2.cpp; dest=quotient_theta2_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" source_kummer.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/source-kummer" 2> "$out/build.stderr"
sha256sum source_kummer.cpp SOURCE_KUMMER_PROTOCOL.md "$root/selected-kernel.json" "$root/evidence/quotient-theta2-v1/result.json" "$root/evidence/weil3-v2/result.json" "$tmp/"*.hpp "$tmp/source-kummer" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/source-kummer"
"$tmp/source-kummer" "$root" > "$out/result.json" 2> "$out/stderr.txt"
sha256sum source_kummer.cpp > "$out/post-call-source-manifest.sha256"
