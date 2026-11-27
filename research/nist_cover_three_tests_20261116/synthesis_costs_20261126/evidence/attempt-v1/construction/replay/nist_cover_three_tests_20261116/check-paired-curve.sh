#!/bin/sh
set -eu
cd "$(dirname "$0")"
second=${1:?Specify completed kernel2 study}
out=${2:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        weil3) source=weil3.cpp; dest=weil3_arithmetic ;;
        quotient_theta2) source=quotient_theta2.cpp; dest=quotient_theta2_arithmetic ;;
        riemann) source=riemann_quartic.cpp; dest=riemann_arithmetic ;;
        quartic_descent) source=quartic_descent.cpp; dest=quartic_descent_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" paired_curve_map.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/paired-curve" 2> "$out/build.stderr"
sha256sum paired_curve_map.cpp PAIR_CURVE_PROTOCOL.md evidence/descent-v1/result.json evidence/riemann-v1/result.json "$second/evidence/descent-v1/result.json" "$second/evidence/riemann-v1/result.json" "$tmp/"*.hpp "$tmp/paired-curve" > "$out/pre-call-manifest.sha256"
"$tmp/paired-curve" "$PWD" "$second" > "$out/result.json" 2> "$out/stderr.txt"
sha256sum paired_curve_map.cpp > "$out/post-call-source-manifest.sha256"
